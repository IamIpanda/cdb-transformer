use std::collections::HashMap;
use std::ops::BitOr;
use std::path::Path;
use std::sync::{Arc, LazyLock};

use arc_swap::ArcSwap;
use bitflags::Flags;
use phf::phf_map;
use regex::Regex;

use crate::card::{Card, CardTransformer, PackInfo};
use crate::constants::*;


/*
XYZ原版格式：
INFO-JP000(QCSER)誇りと魂の龍(骄傲与灵魂之龙) 暗 8星 龙/特殊召唤 2500 2500
这张卡不能通常召唤。对方墓地有卡25张以上存在的场合才能特殊召唤。①：只要自己墓地有卡25张以上存在，这张卡的攻击力·守备力上升2500。

实际采用的格式：
骄傲与灵魂之龙(100000000) 暗 8星 龙/特殊召唤 2500 2500 (DIY)
这张卡不能通常召唤。对方墓地有卡25张以上存在的场合才能特殊召唤。①：只要自己墓地有卡25张以上存在，这张卡的攻击力·守备力上升2500。
提示文本：特殊召唤
*/

pub static ATTRIBUTE_NAMES: phf::Map<u32, &'static str> = phf_map! {
    0u32 => "无",
    1u32 => "地",
    2u32 => "水",
    4u32 => "炎",
    8u32 => "风",
    16u32 => "光",
    32u32 => "暗",
    64u32 => "神"
};

pub static RACE_NAMES: phf::Map<u32, &'static str> = phf_map! {
    0u32 => "无种族",
    1u32 => "战士",
    2u32 => "魔法使",
    4u32 => "天使",
    8u32 => "恶魔",
    16u32 => "不死",
    32u32 => "机械",
    64u32 => "水",
    128u32 => "炎",
    256u32 => "岩石",
    512u32 => "兽战士",
    1024u32 => "植物",
    2048u32 => "昆虫",
    4096u32 => "雷",
    8192u32 => "龙",
    16384u32 => "兽",
    32768u32 => "兽战士",
    65536u32 => "恐龙",
    131072u32 => "鱼",
    262144u32 => "海龙",
    524288u32 => "爬行类",
    1048576u32 => "念动力",
    2097152u32 => "神",
    4194304u32 => "创世神",
    8388608u32 => "幻龙",
    16777216u32 => "电子界",
    33554432u32 => "幻想魔",
};

pub static TYPE_NAMES: phf::Map<u32, &'static str> = phf_map! {
    // 0u32 => "通常",
    1u32 => "怪兽",
    2u32 => "魔法",
    4u32 => "陷阱",
    16u32 => "通常",
    32u32 => "效果",
    64u32 => "融合",
    128u32 => "仪式",
    256u32 => "陷阱怪兽",
    512u32 => "灵魂",
    1024u32 => "同盟",
    2048u32 => "二重",
    4096u32 => "调整",
    8192u32 => "同调",
    16384u32 => "衍生物",
    65536u32 => "速攻",
    131072u32 => "永续",
    262144u32 => "装备",
    524288u32 => "场地",
    1048576u32 => "反击",
    2097152u32 => "反转",
    4194304u32 => "卡通",
    8388608u32 => "超量",
    16777216u32 => "灵摆",
    33554432u32 => "特殊召唤",
    67108864u32 => "连接",
};
static EX_NONEFFECT_TYPE_NAME: phf::Map<u32, &'static str> = phf_map! {
    16u32 => "非效果"
};

pub static LINKMARKERS_NAMES: phf::Map<i32, &'static str> = phf_map! {
    1i32 => "↙",
    2i32 => "↓",
    4i32 => "↘",
    8i32 => "←",
    16i32 => "",
    32i32 => "→",
    64i32 => "↖",
    128i32 => "↑",
    256i32 => "↗"
};

pub static OT_NAMES: phf::Map<u32, &'static str> = phf_map! {
    1u32 => "OCG",
    2u32 => "TCG",
    3u32 => "OT",
    4u32 => "Custom",
    11u32 => "SC",
    8u32 => "SCONLY",
    1024u32 => "Draft"
};

pub static CATEGORY_NAMES: phf::Map<u64, &'static str> = phf_map! (
    0x1u64 => "魔陷破坏",
    0x2u64 => "怪兽破坏",
    0x4u64 => "卡片除外",
    0x8u64 => "送去墓地",
    0x10u64 => "返回手卡",
    0x20u64 => "返回卡组",
    0x40u64 => "手卡破坏",
    0x80u64 => "卡组破坏",
    0x100u64 => "抽卡辅助",
    0x200u64 => "卡组检索",
    0x400u64 => "卡片回收",
    0x800u64 => "表示形式",
    0x1000u64 => "控制权",
    0x2000u64 => "攻守变化",
    0x4000u64 => "穿刺伤害",
    0x8000u64 => "多次攻击",
    0x10000u64 => "攻击限制",
    0x20000u64 => "直接攻击",
    0x40000u64 => "特殊召唤",
    0x80000u64 => "衍生物",
    0x100000u64 => "种族相关",
    0x200000u64 => "属性相关",
    0x400000u64 => "LP伤害",
    0x800000u64 => "LP回复",
    0x1000000u64 => "破坏耐性",
    0x2000000u64 => "效果耐性",
    0x4000000u64 => "指示物",
    0x8000000u64 => "幸运",
    0x10000000u64 => "融合相关",
    0x20000000u64 => "同调相关",
    0x40000000u64 => "超量相关",
    0x80000000u64 => "效果无效",
);

pub static SET_NAMES: LazyLock<ArcSwap<HashMap<u16, String>>> = LazyLock::new(
    || ArcSwap::new(Arc::new(HashMap::new())));


fn search_in_phf_map<K: Copy>(m: &phf::Map<K,&'static str>, v: &str) -> Option<K> {
    m.entries().find(|(_, vv)| &&v == vv).map(|(k, _)| k.clone())
}

fn join_from_phf_map<'a, K: Copy+Flags+BitOr<Output = K>>(
    m: &phf::Map<<K as Flags>::Bits,&'static str>, 
    i: impl Iterator<Item = &'a str>
) -> K {
    let mut z = K::empty();
    for s in i {
        if let Some(b) = search_in_phf_map(m, s) {
            z = z | K::from_bits_truncate(b);
        } else {
            println!("Cannot recognize attribute {}", s);
        }
    }
    z
}

fn join_from_phf_map_dual<'a, K: Copy+Flags+BitOr<Output = K>>(
    m: &phf::Map<<K as Flags>::Bits,&'static str>,
    n: &phf::Map<<K as Flags>::Bits,&'static str>,  
    i: impl Iterator<Item = &'a str>
) -> K {
    let mut z = K::empty();
    for s in i {
        if let Some(b) = search_in_phf_map(m, s) {
            z = z | K::from_bits_truncate(b);
        } else if let Some(b) = search_in_phf_map(n, s) {
            z = z | K::from_bits_truncate(b);
        } else {
            println!("Cannot recognize attribute {}", s);
        }
    }
    z
}

pub fn read_string_conf<P: AsRef<Path>>(paths: &Vec<P>) {   
    let mut setnames = HashMap::<u16, String>::new();
    for path in paths {
        let s = std::fs::read_to_string(path).unwrap();
        add_string_conf_to_hashmap(&mut setnames, &s);
    }
    SET_NAMES.store(Arc::new(setnames));
}

pub fn set_string_conf(conf: impl AsRef<str>) {
    let mut setnames = HashMap::new();
    add_string_conf_to_hashmap(&mut setnames, conf);
    SET_NAMES.store(Arc::new(setnames));
}

fn add_string_conf_to_hashmap(setnames: &mut HashMap<u16, String>, str: impl AsRef<str>) {
    for line in str.as_ref().split("\n") {
        if line.starts_with("!setname") {
            let parts = line.trim().split(" ").collect::<Vec<_>>();
            if let Ok(number) = u16::from_str_radix(parts[1].trim_start_matches("0x"), 16) {
                setnames.insert(number, parts[2].split("\t").next().unwrap().to_string());
            }
        }
    }
}


pub struct Xyyz;

impl Xyyz {
    pub fn format_level(this: &Card) -> String {
        if this._type.contains(Type::Xyz) { format!("{}阶", this.level) }
        else if this._type.contains(Type::Link) { format!("LINK-{}", this.link_marker.iter().count()) }
        else { format!("{}星", this.level) }
    }

    fn set_level(card: &mut Card, str: &str) {
        if str.ends_with("星") {
            card.level = str.trim_end_matches("星").trim().parse().unwrap_or(0)
        }
        else if str.ends_with("阶") { 
            card._type = card._type | Type::Xyz;
            card.level = str.trim_end_matches("阶").trim().parse().unwrap_or(0)
        }
        else if str.starts_with("LINK-") {
            card._type = card._type | Type::Link;
            card.level = str.trim_start_matches("LINK-").trim().parse().unwrap_or(0)
        }
        else {
            println!("Can't recognize level {}", str)
        }
    }

    pub fn format_number(num: i32) -> String {
        if num == -2 { "?".to_string() }
        else if num == -1 { "∞".to_string() }
        else { num.to_string() }
    }

    fn get_num(str: &str) -> i32 {
        if str == "?" { -2 }
        else if str == "∞" { -1 }
        else { str.parse().unwrap_or_default() }
    }

    pub fn format_setcode(this: &Card) -> Option<String> {
        let setnames = SET_NAMES.load();
        Some([this.setcode & 0xffff, (this.setcode & 0xffff0000) >> 16, (this.setcode & 0xffff00000000) >> 32, (this.setcode & 0xffff000000000000) >> 48]
            .into_iter()
            .filter(|set| *set > 0)
            .map(|set| setnames.get(&(set as u16)).unwrap_or(&format!("0x{:X}", set)).clone())
            .collect::<Vec<_>>()
            .join("、"))
    }

    fn get_setcode(str: &str) -> u64 {
        let setnames = SET_NAMES.load();
        
        let mut setcodes: u64 = 0;
        for setname in str.split("、") {
            let setname = setname.trim();
            let setcode = if setname.starts_with("0x") {
                u16::from_str_radix(&setname[2..], 16).ok()
            } else {
                setnames.iter().find(|(_, v)| v == &&setname).map(|(k, _)| *k)
            };
            if let Some(s) = setcode {
                setcodes = setcodes.checked_shl(16).unwrap() + s as u64;
            }
            else {
                println!("Can't recoginize set {}. ", setname)
            }
        }
        setcodes
    }

    pub fn format_attribute(this: &Attribute) -> String {
        if this.is_empty() { return ATTRIBUTE_NAMES[&0].to_string() }
        this.iter().map(|a| ATTRIBUTE_NAMES[&a.bits()]).collect::<Vec<_>>().join("/")
    }

    fn get_attribute(value: &str) -> Attribute {
        join_from_phf_map(&ATTRIBUTE_NAMES, value.split("/"))
    }

    pub fn format_race(this: &Race) -> String {
        if this.is_empty() { return RACE_NAMES[&0].to_string() }
        this.iter().map(|a| RACE_NAMES[&a.bits()]).collect::<Vec<_>>().join("/")
    }

    fn get_race(value: &str) -> Race {
        join_from_phf_map(&RACE_NAMES, value.split("/"))
    }

    pub fn format_type(this: &Type) -> String {
        if this.contains(Type::Monster) { }
        if this.contains(Type::Spell) { return format!("{}{}", TYPE_NAMES.get(&(this.bits() - &Type::Spell.bits())).unwrap_or(&TYPE_NAMES[&Type::Normal.bits()]), TYPE_NAMES[&Type::Spell.bits()]) }
        if this.contains(Type::Trap) { return format!("{}{}", TYPE_NAMES.get(&(this.bits() - &Type::Trap.bits())).unwrap_or(&TYPE_NAMES[&Type::Normal.bits()]), TYPE_NAMES[&Type::Trap.bits()]) }
        String::new()
    }

    fn get_type(str: &str) -> Type {
        let mut v = 0;
        let leading_length = TYPE_NAMES[&Type::Normal.bits()].len();
        let all_length = leading_length + TYPE_NAMES[&Type::Monster.bits()].len();
        if str.len() >= leading_length && str.is_char_boundary(leading_length) { v = v + search_in_phf_map(&TYPE_NAMES, &str[0..leading_length]).unwrap_or_default(); }
        if str.len() == all_length     && str.is_char_boundary(leading_length) { v = v + search_in_phf_map(&TYPE_NAMES, &str[leading_length..] ).unwrap_or_default(); }
        v = v & !(Type::Normal.bits());
        Type::from_bits_truncate(v)
    }

    pub fn format_subtype(this: &Type) -> String {
        let model_type = Type::Normal | Type::Fusion | Type::Ritual | Type::Synchro | Type::Xyz | Type::Pendulum | Type::Link | Type::Spsummon;
        let ex_type = Type::Fusion | Type::Ritual | Type::Xyz | Type::Synchro | Type::Link;
        let mut this_intersected = this.intersection(model_type);         // Ex monster should contains 'Normal' type.
        if this.intersects(ex_type) { this_intersected.remove(Type::Normal); }  // But we still remove it for external sources.
        let mut z1 = this_intersected.iter().map(|t| TYPE_NAMES[&t.bits()]).collect::<Vec<_>>();
        if this.intersects(ex_type) && !this.contains(Type::Effect) { z1.push(EX_NONEFFECT_TYPE_NAME[&16]); } // Add 'non-effect' label for ex monsters.
        
        let sub_type = Type::Flip | Type::Token | Type::Spirit | Type::Union | Type::Toon | Type::Dual | Type::Tuner;
        let z2 = this.intersection(sub_type).iter().map(|t| TYPE_NAMES[&t.bits()]).collect::<Vec<_>>();
        z1.extend(z2);
        if z1.len() == 0 { String::new() }
        else { format!("/{}", z1.join("/")) }
    }
    
    fn get_subtype(value: &str) -> Type {
        let mut _type: Type = join_from_phf_map_dual(&EX_NONEFFECT_TYPE_NAME, &TYPE_NAMES, value.split("/").filter(|p| p.len() > 0));
        if _type.intersects(Type::Fusion | Type::Ritual | Type::Xyz | Type::Synchro | Type::Link) {
            if _type.contains(Type::Normal) { _type.remove(Type::Normal); }
            else { _type = _type.union(Type::Effect) }
        }
        else if ! _type.intersects(Type::Token | Type::Normal) { _type = _type.union(Type::Effect) }
        _type
    }

    pub fn format_linkmarkers(this: &Linkmarkers) -> String {
        if this.is_empty() { return String::new() }
        format!("[{}]", this.iter().map(|a| LINKMARKERS_NAMES[&a.bits()]).collect::<Vec<_>>().join("]["))
    }

    fn get_linkmarkers(value: &str) -> Linkmarkers {
        if value.is_empty() { return Linkmarkers::empty() }
        join_from_phf_map(&LINKMARKERS_NAMES, value[1..value.len()-1].split("]["))
    }

    pub fn format_ot(this: &OT) -> String {
        if this.bits() == (OT::OCG | OT::TCG).bits() { return String::new() }
        if let Some(s) = OT_NAMES.get(&this.bits()) { return s.to_string(); }
        this.iter().map(|a| OT_NAMES[&a.bits()]).collect::<Vec<_>>().join("&")
    }

    fn get_ot(value: &str) -> OT {
        join_from_phf_map(&OT_NAMES, value.split("&").map(|v| v.trim()))
    }

    pub fn format_category(this: &Category) -> String {
        this.iter().map(|c| CATEGORY_NAMES[&c.bits()]).collect::<Vec<_>>().join("、")
    }

    fn get_category(value: &str) -> Category {
        join_from_phf_map(&CATEGORY_NAMES, value.split("、").map(|v| v.trim()))
    }
}

impl CardTransformer for Xyyz {
    fn to_string(card: &Card) -> String {
        let mut str = String::new();
        let alias_text = if card.alias > 0 { format!("=>{}", card.alias) } else { String::new() };
        if card._type.contains(Type::Monster) {
            str += &format!("{}({}{}) {} {} {}{} {} {}", 
                card.name, 
                card.code,
                alias_text,
                Self::format_attribute(&card.attribute), 
                Self::format_level(card), 
                Self::format_race(&card.race), 
                Self::format_subtype(&card._type), 
                Self::format_number(card.attack),
                if card._type.contains(Type::Link) { String::new() } else { Self::format_number(card.defense) }
            );
            if card._type.contains(Type::Link) {
                str += &Self::format_linkmarkers(&card.link_marker)
            }
        } else {
            str += &format!("{}({}{}) {}", card.name, card.code, alias_text, Self::format_type(&card._type))
        };
        if card.ot.bits() != (OT::OCG | OT::TCG).bits() {
            str += " (";
            str += &Self::format_ot(&card.ot);
            str += ")"
        }
        if let Some(setnames) = Self::format_setcode(card) {
            if setnames.len() > 0 {
                str += &format!("\n系列：{}", setnames);
            }
        }
        str += &format!("\n{}", card.desc);
        if ! card.category.is_empty() {
            str += &format!("\n效果分类：{}", Self::format_category(&card.category));
        }
        if card.texts.len() > 0 {
            str += &format!("\n提示文本：{}", card.texts.join("、"));
        };
        str
    }

    fn from_string(str: &str) -> Vec<Card> {
        let mut cards = Vec::new();
        let mut current_card: Option<Card> = None;
        let mut current_index = 0;
        let line_regex = Regex::new(r"^(\[.+\-.+\]\s+)?(.+)\((\d+)(\s*=>\s*(\d+)\s*)?\)\s+(.+?)\s*(\((.*)\))?$").unwrap();
        let parts_regex = Regex::new(r"^(.+?) (.+?) (.+?)((?:/.+?)*) (\d+|\?|∞) ?(\d+|\?|∞|(\[.+\])?)$").unwrap();
        let pendulum_regex: Regex = Regex::new(r"^←(\d+)\s*【灵摆】\s*(\d+)→$").unwrap();
        for line in str.split("\n") {
            let current_line_length = line.chars().count() + 1;
            if line.starts_with("#") { current_index += current_line_length; continue; }
            if line.trim().len() == 0 {
                if let Some(card) = current_card.as_mut() {
                    if let Some(range) = card.range.as_mut() {
                        range.end = current_index;
                    }
                }
                current_index += current_line_length;
                continue; 
            }
            if let Some(groups) = line_regex.captures(line) {
                let name = groups.get(2).unwrap().as_str().to_string();
                let code: u32 = groups.get(3).unwrap().as_str().parse().unwrap_or_default();
                let mut card = Card {
                    code,
                    name,
                    desc: String::new(),
                    alias: if let Some(u) = groups.get(5) { u.as_str().parse().unwrap() } else { 0 },
                    setcode: 0,
                    _type: Type::empty(),
                    level: 0,
                    attribute: Attribute::empty(),
                    race: Race::empty(),
                    attack: 0,
                    defense: 0,
                    lscale: 0,
                    rscale: 0,
                    link_marker: Linkmarkers::empty(),
                    ot: OT::OCG | OT::TCG,
                    category: Category::empty(),
                    texts: Vec::new(),
                    pack: groups.get(1).map(|u| PackInfo { id: code, pack_id: u.as_str()[1..u.as_str().len()-1].to_string(), pack: String::new(), rarity: vec![], date: String::new() }),
                    range: Some((current_index..current_index).into())
                };
                let part_str = groups.get(6).unwrap().as_str();
                if let Some(ot) = groups.get(8) {
                    card.ot = Self::get_ot(ot.as_str());
                }
                if let Some(parts) = parts_regex.captures(part_str) {
                    let attr_str = parts.get(1).unwrap();
                    let level_str = parts.get(2).unwrap();
                    let race_str = parts.get(3).unwrap();
                    let type_str = parts.get(4).map(|t| t.as_str()).unwrap_or("");
                    let atk_str = parts.get(5).unwrap();
                    let def_str = parts.get(6).unwrap();
                    let linkmarker_str = parts.get(7);

                    card.attribute = Self::get_attribute(attr_str.as_str());
                    Self::set_level(&mut card, level_str.as_str());
                    card.race = Self::get_race(race_str.as_str().into());
                    card._type = Self::get_subtype(type_str) | Type::Monster;
                    card.attack = Self::get_num(atk_str.as_str());
                    card.defense = Self::get_num(def_str.as_str());
                    if let Some(marker) = linkmarker_str {
                        card._type = card._type | Type::Link;
                        card.link_marker = Self::get_linkmarkers(marker.as_str());
                        card.defense = card.link_marker.bits();
                    }
                }
                else { 
                    card._type = card._type | Self::get_type(part_str); 
                }
                if let Some(mut card) = current_card { set_card_range(&mut card, current_index-1); cards.push(card); }
                current_card = Some(card);
            }
            else if let Some(c) = current_card.as_mut() {
                if c._type.contains(Type::Pendulum) {
                    if let Some(groups) = pendulum_regex.captures(&line) {
                        c.lscale = groups.get(1).unwrap().as_str().parse().unwrap();
                        c.rscale = groups.get(2).unwrap().as_str().parse().unwrap();
                    }
                }
                if line.starts_with("系列字段：") {
                    c.setcode = Self::get_setcode(line.trim_start_matches("系列字段："))
                }
                if line.starts_with("系列：") {
                    c.setcode = c.setcode | Self::get_setcode(line.trim_start_matches("系列："))
                }
                else if line.starts_with("效果分类：") {
                    c.category = Self::get_category(line.trim_start_matches("效果分类："))
                }
                else if line.starts_with("提示文本：") {
                    c.texts = line.trim_start_matches("提示文本：").split("、").map(|t| t.to_string()).collect();
                }
                else {
                    if c.desc.len() > 0 {
                        c.desc.push('\n');
                    }
                    c.desc.extend(line.chars()); 
                }
            }
            current_index += current_line_length; 
        }
        if let Some(mut card) = current_card {
            set_card_range(&mut card, str.len());
            cards.push(card) 
        }
        println!("Parsed {} cards.", cards.len());
        cards
    }
}

fn set_card_range(card: &mut Card, end: usize) {
    if let Some(range) = card.range.as_mut() {
        if range.start >= range.end {
            range.end = end;
        }
    }
}

#[test]
fn read_string_conf_test() {
    read_string_conf(&vec!["/Users/iami/Workshop/code/mycard/ygopro/strings.conf"]);
    println!("{:?}", SET_NAMES)
}

#[test]
fn test_format() {
    read_string_conf(&vec!["/Users/iami/Workshop/code/mycard/ygopro/strings.conf"]);
    let cards = super::CDB::from_string("/Users/iami/Workshop/code/mycard/ygopro-database/locales/zh-CN/cards.cdb");
    let s = cards.into_iter().map(|c| Xyyz::to_string(&c)).collect::<Vec<_>>().join("\n\n");
    std::fs::write("/Users/iami/Workshop/code/mycard/cdb-transformer/test.log", s).unwrap();
} 

#[test]
fn test_parse() {
    read_string_conf(&vec!["/Users/iami/Workshop/code/mycard/ygopro/strings.conf"]);
    let cards = std::fs::read_to_string("/Users/iami/Workshop/code/mycard/MyDIY/MyDIY.txt").unwrap();
    let cc = Xyyz::from_string(&cards);
    println!("{:?}", cc.len());
    let s = cc.into_iter().map(|c| Xyyz::to_string(&c)).collect::<Vec<_>>().join("\n\n");
    std::fs::write("/Users/iami/Workshop/code/mycard/cdb-transformer/test2.log", s).unwrap();
}

#[test]
fn test_parse_text() {
    let text = "
游戏王九项修改器(65123333) 地/水/炎/风/光/暗 LINK-0 创世神/连接 0  (Custom)
——————第一页——————

日食爆龙(172017203) 暗 LINK-2 龙/连接 1600 [↓][↘] (Custom)
光属性龙族怪兽+暗属性龙族怪兽
①：这张卡连接召唤成功时发动。这张卡以外的场上的怪兽的攻击力只要这张卡在场上表侧表示变成0。为这个效果下降的攻击力每有500，把1个「燃料指示物」在这张卡上放置。
②：1回合1次，把这张卡上1个「燃料指示物」取除才能发动。从卡组选光·暗属性龙族怪兽各1只除外。这个效果发动的回合，自己不能把龙族怪兽以外的怪兽效果发动。
③：龙族怪兽的效果发动时，把这张卡解放才能发动。为这张卡连接召唤的那一组素材在墓地集齐的场合，把那些怪兽在自己场上特殊召唤。那之后，可以把这张卡放置过的「燃料指示物」数量的「燃料指示物」在自己场上1只「主宰龙 无穷烈日」上放置。

#这是一行注释
S.A.R.A.(172016020) 无 3星 龙/调整 0 1800 (Custom)
这张卡的属性是最后召唤·特殊召唤·反转召唤的怪兽的属性。
①：对方的主要阶段，把手卡的这张卡送去墓地才能发动。从卡组选1张和这张卡属性相同的攻0/防1800的调整怪兽加入手卡。这个效果发动的回合，这张卡是公开表示的场合，自己不是与这张卡属性相同的怪兽的效果不能发动。

原质阿尔法(114514269) 暗 3星 恶魔/通常 1000 1000 (Custom)
系列：原质炉
起点之果。

青眼白龙(89631141=>89631139) 通常魔法
以高攻击力著称的传说之龙。任何对手都能粉碎，其破坏力不可估量。";
    let cc = Xyyz::from_string(text);
    println!("{:?}", cc[4])
}
