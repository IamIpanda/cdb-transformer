use crate::card::Card;
use crate::card::CardTransformer;

use super::Xyyz;

pub struct Script;

pub static MAX_LINE_LENGTH: std::sync::OnceLock<usize> = std::sync::OnceLock::<usize>::new();
fn len(s: &str) -> usize {
    s.chars().map(|c| if c.is_ascii() {1} else {2}).sum()
}

impl CardTransformer for Script {
    fn to_string(card: &crate::card::Card) -> String {
        let text = Xyyz::to_string(card);
        let max_line_length = MAX_LINE_LENGTH.get_or_init(|| 80).clone();
        let mut new_lines = vec![];
        for line in text.split("\n") {
            let mut current_line = String::new();
            for sentence in line.split_inclusive("。") {
                if sentence.len() == 0 { continue; }
                if len(&current_line) + len(&sentence) > max_line_length && len(&current_line) > 0 {
                    new_lines.push(current_line);
                    current_line = "   ".to_string();
                }
                current_line = current_line + sentence
            }
            new_lines.push(current_line);
        }
        let max_length = new_lines.iter().map(|line| len(line)).max().unwrap_or(0);
        let wrapped_lines = "-".repeat(max_length + 5);
        wrapped_lines.clone() + "\n" + new_lines.into_iter().map(|s| "--- ".to_string() + &s).collect::<Vec<_>>().join("\n").as_str() + "\n" + &wrapped_lines
    }

    fn from_string(str: &str) -> Vec<crate::card::Card> {
        let mut context = false;
        let mut text = String::new();
        for origin_line in str.replace("\r","").split("\n") {
            let line = origin_line.trim();
            if line.starts_with("----") { 
                if context { break; }
                else { context = true; }
            }
            if !context { continue; }
            if line.starts_with("---") {
                let trimmed_line = line.trim_start_matches('-');
                if !trimmed_line.starts_with("  ") && text.len() != 0 { text = text + "\n"; }
                text = text + trimmed_line.trim();
            }
        }
        Xyyz::from_string(&text)
    }
}

impl Script {
    pub fn save_to(cards: &Vec<Card>, path: &str) {
        let leading_description = regex::Regex::new("^-{4,}(\n--.*)*\n-{4,}").unwrap();
        for card in cards {
            let target = Script::to_string(card);
            let real_path = path.replace("{id}", &card.code.to_string());
            let origin_content = std::fs::read_to_string(&real_path).unwrap_or(String::new());
            let content = match leading_description.replace(&origin_content, target.clone()) {
                std::borrow::Cow::Borrowed(_) => { target + &origin_content },
                std::borrow::Cow::Owned(o) => {o},
            };
            std::fs::write(real_path, content).ok();
        }
    }
}

#[test]
fn test_parse() {
    let cards = Script::from_string("
    --------------------------------------------------------
    --- 王家的人柱(172016025) 通常陷阱 (Custom)
    --- ①：当自己场上有「王家长眠之谷」存在时才能发动。
    ---    双方玩家把卡组·额外卡组中的怪兽卡全部送去墓地。
    --------------------------------------------------------
local s,id,o=GetID()
    ");
    println!("{}", Script::to_string(&cards[0]))
}

#[test]
fn test_format() {
    let card = Card {
        code: 10000,
        name: "测试".to_string(),
        desc: "①：抽1张卡。再选1张手卡丢弃。抽1张卡。再选1张手卡丢弃。抽1张卡。再选1张手卡丢弃。抽1张卡。再选1张手卡丢弃。抽1张卡。再选1张手卡丢弃。抽1张卡。再选1张手卡丢弃。\n②：回复100基本分。".to_string(),
        alias: 0,
        setcode: 0,
        _type: crate::constants::Type::Spell,
        level: 0,
        attribute: crate::constants::Attribute::empty(),
        race: crate::constants::Race::empty(),
        attack: 0,
        defense: 0,
        lscale: 0,
        rscale: 0,
        link_marker: crate::constants::Linkmarkers::empty(),
        ot: crate::constants::OT::OCG | crate::constants::OT::TCG,
        category: crate::constants::Category::empty(),
        texts: vec![],
        pack: None,
        range: None
    };
    println!("{}", Script::to_string(&card))
}
