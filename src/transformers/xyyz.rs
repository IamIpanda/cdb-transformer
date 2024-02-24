 use std::collections::HashMap;
use std::ops::BitOr;
use std::path::Path;
use std::sync::OnceLock;

use bitflags::Flags;
use phf::phf_map;
use regex::Regex;

use crate::card::{Card, CardTransformer};
use crate::constants::*;


/*
XYZ原版格式：
INFO-JP000(QCSER)誇りと魂の龍(骄傲与灵魂之龙) 暗 8星 龙/特殊召唤 2500 2500
这张卡不能通常召唤。对方墓地有卡25张以上存在的场合才能特殊召唤。①：只要自己墓地有卡25张以上存在，这张卡的攻击力·守备力上升2500。

实际采用的格式：
骄傲与灵魂之龙(100000000) 暗 8星 龙/特殊召唤 2500 2500 (DIY)
这张卡不能通常召唤。对方墓地有卡25张以上存在的场合才能特殊召唤。①：只要自己墓地有卡25张以上存在，这张卡的攻击力·守备力上升2500。
提示文字：特殊召唤
*/

static ATTRIBUTE_NAMES: phf::Map<u32, &'static str> = phf_map! {
    0u32 => "无",
    1u32 => "地",
    2u32 => "水",
    4u32 => "炎",
    8u32 => "风",
    16u32 => "光",
    32u32 => "暗",
    64u32 => "神"
};

static RACE_NAMES: phf::Map<u32, &'static str> = phf_map! {
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

static TYPE_NAMES: phf::Map<u32, &'static str> = phf_map! {
    0u32 => "通常",
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

static LINKMARKERS_NAMES: phf::Map<i32, &'static str> = phf_map! {
    1i32 => "↙",
    2i32 => "↓",
    4i32 => "↘",
    8i32 => "←",
    16i32 => "",
    32i32 => "➡️",
    64i32 => "↖",
    128i32 => "↑",
    256i32 => "↗"
};

static OT_NAMES: phf::Map<u32, &'static str> = phf_map! {
    1u32 => "OCG",
    2u32 => "TCG",
    3u32 => "OT",
    4u32 => "Custom",
    11u32 => "SC",
    8u32 => "SCONLY"
};

static CATEGORY_NAMES: phf::Map<u64, &'static str> = phf_map! (
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

pub static SET_NAMES: OnceLock<HashMap<u16, String>> = OnceLock::new();



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
            panic!("Cannot recognize attribute {}", s)
        }
    }
    z
}

pub fn read_string_conf<P: AsRef<Path>>(paths: &Vec<P>) {   
    let mut setnames = HashMap::<u16, String>::new();
    for path in paths {
        let s = std::fs::read_to_string(path).unwrap();
        for line in s.split("\n") {
            if line.starts_with("!setname") {
                let parts = line.trim().split(" ").collect::<Vec<_>>();
                if let Ok(number) = u16::from_str_radix(parts[1].trim_start_matches("0x"), 16) {
                    setnames.insert(number, parts[2].split("\t").next().unwrap().to_string());
                }
            }
        }
    }
    SET_NAMES.set(setnames).unwrap()
}


pub struct Xyyz;

impl Xyyz {
	fn format_level(this: &Card) -> String {
		if this._type.contains(Type::Xyz) { format!("{}阶", this.level) }
		else if this._type.contains(Type::Link) { format!("LINK-{}", this.link_marker.iter().count()) }
		else { format!("{}星", this.level) }
	}

	fn set_level(card: &mut Card, str: &str) {
        if str.ends_with("星") {
            card.level = str.trim_end_matches("星").trim().parse().unwrap()
        }
		else if str.ends_with("阶") { 
            card._type = card._type | Type::Xyz;
            card.level = str.trim_end_matches("阶").trim().parse().unwrap()
        }
        else if str.starts_with("LINK-") {
            card._type = card._type | Type::Link;
            card.level = str.trim_start_matches("LINK-").trim().parse().unwrap()
        }
        else {
            panic!("Can't recognize level {}", str)
        }
	}

	fn format_number(num: i32) -> String {
		if num == -2 { "?".to_string() }
		else if num == -1 { "∞".to_string() }
		else { num.to_string() }
	}

	fn get_num(str: &str) -> i32 {
		if str == "?" { -2 }
		else if str == "∞" { -1 }
		else { str.parse().unwrap_or_default() }
	}

	fn format_setcode(this: &Card) -> Option<String> {
		let setnames = SET_NAMES.get()?;
		Some([this.setcode & 0xffff, (this.setcode & 0xffff0000) >> 16, (this.setcode & 0xffff00000000) >> 32, (this.setcode & 0xffff000000000000) >> 48]
			.into_iter()
			.filter(|set| *set > 0)
			.map(|set| setnames.get(&(set as u16)))
			.filter(|set| set.is_some())
			.map(|set| set.unwrap().clone())
			.collect::<Vec<_>>()
			.join("、"))
	}

	fn get_setcode(str: &str) -> u64 {
		let setnames = match SET_NAMES.get() {
			Some(s) => s,
			None => return 0
		};
		
  		let mut setcodes: u64 = 0;
		for setname in str.split("、") {
			let setcode = setnames.iter().find(|(_, v)| v == &&setname).map(|(k, _)| *k);
			if let Some(setcode) = setcode {
				setcodes = setcodes.checked_shl(16).unwrap() + setcode as u64;
			}
			else {
				println!("Can't recoginize set {}", setname)
			}
		}
		setcodes
	}

    fn format_attribute(this: &Attribute) -> String {
        if this.is_empty() { return ATTRIBUTE_NAMES[&0].to_string() }
        this.iter().map(|a| ATTRIBUTE_NAMES[&a.bits()]).collect::<Vec<_>>().join("/")
    }

    fn get_attribute(value: &str) -> Attribute {
        join_from_phf_map(&ATTRIBUTE_NAMES, value.split("/"))
    }

    fn format_race(this: &Race) -> String {
        if this.is_empty() { return RACE_NAMES[&0].to_string() }
        this.iter().map(|a| RACE_NAMES[&a.bits()]).collect::<Vec<_>>().join("/")
    }

    fn get_race(value: &str) -> Race {
        join_from_phf_map(&RACE_NAMES, value.split("/"))
    }

    fn format_type(this: &Type) -> String {
        if this.contains(Type::Monster) { }
        if this.contains(Type::Spell) { return format!("{}{}", TYPE_NAMES[&(this.bits() - Type::Spell.bits())], TYPE_NAMES[&Type::Spell.bits()]) }
        if this.contains(Type::Trap) { return format!("{}{}", TYPE_NAMES[&(this.bits() - Type::Trap.bits())], TYPE_NAMES[&Type::Trap.bits()]) }
        String::new()
    }

    fn get_type(str: &str) -> Type {
        let mut v = 0;
        let leading_length = TYPE_NAMES[&Type::Normal.bits()].len();
        let all_length = leading_length + TYPE_NAMES[&Type::Monster.bits()].len();
        if str.len() >= leading_length { v = v + search_in_phf_map(&TYPE_NAMES, &str[0..leading_length]).unwrap_or_default(); }
        if str.len() == all_length     { v = v + search_in_phf_map(&TYPE_NAMES, &str[leading_length..] ).unwrap_or_default(); }
        Type::from_bits_truncate(v)
    }

    fn format_subtype(this: &Type) -> String {
        let model_type = Type::Normal | Type::Fusion | Type::Ritual | Type::Synchro | Type::Xyz | Type::Pendulum | Type::Link | Type::Spsummon;
        let mut z1 = (*this & model_type).iter().map(|t| TYPE_NAMES[&t.bits()]).collect::<Vec<_>>();
        let sub_type = Type::Flip | Type::Token | Type::Spirit | Type::Union | Type::Toon | Type::Dual | Type::Tuner;
        let z2 = (*this & sub_type).iter().map(|t| TYPE_NAMES[&t.bits()]).collect::<Vec<_>>();
        z1.extend(z2);
        if z1.len() == 0 { String::new() }
        else { format!("/{}", z1.join("/")) }
    }
    
    fn get_subtype(value: &str) -> Type {
        join_from_phf_map(&TYPE_NAMES, value.split("/").filter(|p| p.len() > 0))
    }

    fn format_linkmarkers(this: &Linkmarkers) -> String {
        format!("[{}]", this.iter().map(|a| LINKMARKERS_NAMES[&a.bits()]).collect::<Vec<_>>().join("]["))
    }

    fn get_linkmarkers(value: &str) -> Linkmarkers {
        join_from_phf_map(&LINKMARKERS_NAMES, value[1..value.len()-1].split("]["))
    }

    fn format_ot(this: &OT) -> String {
        if this.bits() == (OT::OCG | OT::TCG).bits() { return String::new() }
        if let Some(s) = OT_NAMES.get(&this.bits()) { return s.to_string(); }
        this.iter().map(|a| OT_NAMES[&a.bits()]).collect::<Vec<_>>().join("&")
    }

    fn get_ot(value: &str) -> OT {
        join_from_phf_map(&OT_NAMES, value.split("&").map(|v| v.trim()))
    }

    fn format_category(this: &Category) -> String {
        this.iter().map(|c| CATEGORY_NAMES[&c.bits()]).collect::<Vec<_>>().join("、")
    }

    fn get_category(value: &str) -> Category {
        join_from_phf_map(&CATEGORY_NAMES, value.split("、"))
    }
}

impl CardTransformer for Xyyz {
    fn to_string(card: &Card) -> String {
        let mut str = String::new();
		if card._type.contains(Type::Monster) {
			str += &format!("{}({}{}) {} {} {}{} {} {}", 
				card.name, 
				card.code,
				if card.alias > 0 { format!("=>{}", card.alias) } else { String::new() },
				Self::format_attribute(&card.attribute), 
				Self::format_level(card), 
				Self::format_race(&card.race), 
				Self::format_subtype(&card._type), 
				Self::format_number(card.attack),
				Self::format_number(card.defense));
            if card._type.contains(Type::Link) {
                str += " ";
                str += &Self::format_linkmarkers(&card.link_marker)
            }
            if card.ot.bits() != (OT::OCG | OT::TCG).bits() {
                str += " (";
                str += &Self::format_ot(&card.ot);
                str += ")"
            }
		} else {
			str += &format!("{}({}) {}", card.name, card.code, Self::format_type(&card._type))
		};
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
        let line_regex = Regex::new(r"^(.+)\((\d+)(\s*=>\s*(\d+)\s*)?\)\s+(.+)$").unwrap();
        let parts_regex = Regex::new(r"(.) (.+) (.+?)(/.+)* (\d+|\?|∞) (\d+|\?|∞)\s*(\[.+\])?\s*(\((.+)\))?").unwrap();
        let pendulum_regex: Regex = Regex::new(r"^←(\d+)\s*【灵摆】\s*(\d+)→$").unwrap();
        for line in str.split("\n") {
            if line.starts_with("#") { continue; }
            if line.trim().len() == 0 { continue; }
            if let Some(groups) = line_regex.captures(line) {
                let name = groups.get(1).unwrap().as_str().to_string();
                let code: u32 = groups.get(2).unwrap().as_str().parse().unwrap();
                let mut card = Card {
                    code,
                    name,
                    desc: String::new(),
                    alias: if let Some(u) = groups.get(4) { u.as_str().parse().unwrap() } else { 0 },
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
                    pack: None,
                };
                let part_str = groups.get(5).unwrap().as_str();
                if let Some(parts) = parts_regex.captures(part_str) {
                    let attr_str = parts.get(1).unwrap();
                    let level_str = parts.get(2).unwrap();
                    let race_str = parts.get(3).unwrap();
                    let type_str = parts.get(4).map(|t| t.as_str()).unwrap_or("");
                    let atk_str = parts.get(5).unwrap();
                    let def_str = parts.get(6).unwrap();
                    let linkmarker_str = parts.get(7);
                    let ot_str = parts.get(9);

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
                    if let Some(ot) = ot_str {
                        card.ot = Self::get_ot(ot.as_str());
                    }
                }
                else { 
                    card._type = card._type | Self::get_type(part_str); 
                }
                if let Some(card) = current_card { cards.push(card); }
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
                    c.desc.push('\n');
                    c.desc.extend(line.chars()); 
                }
            }
        }
        if let Some(card) = current_card { cards.push(card) }
        println!("Parsed {} cards.", cards.len());
        cards
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
	let cards = std::fs::read_to_string("/Users/iami/Workshop/code/mycard/cdb-transformer/test.log").unwrap();
	let cc = Xyyz::from_string(&cards);
	println!("{:?}", cc.len());
    let s = cc.into_iter().map(|c| Xyyz::to_string(&c)).collect::<Vec<_>>().join("\n\n");
	std::fs::write("/Users/iami/Workshop/code/mycard/cdb-transformer/test2.log", s).unwrap();
}
