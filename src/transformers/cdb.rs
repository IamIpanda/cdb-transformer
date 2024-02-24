use std::collections::HashMap;

use sqlite::Connection;

use crate::card::{Card, CardTransformer};
use crate::constants::*;

use super::{CREATE_TABLE_SQL, SQL};

pub const STR_FIELD_NAMES: [&str; 16] = ["str1","str2","str3","str4","str5","str6","str7","str8","str9","str10","str11","str12","str13","str14","str15","str16"];

pub struct CDB;

impl CDB {
    pub fn save_to(cards: &Vec<Card>, path: &str) {
        let connection = Connection::open(path).expect("Failed to open file.");
        connection.execute(CREATE_TABLE_SQL).expect("create table failed");
        let str = cards.iter().map(|c| SQL::to_string(c)).collect::<Vec<_>>().join("\n");
        connection.execute(str).expect("execute sql failed");
    }

    pub fn from_connection(connection: Connection) -> Vec<Card> {
        const QUERY: &str = "select * from datas join texts where datas.id = texts.id";
        let mut cards = Vec::new();
        connection.iterate(QUERY, |iter| {
            let mut h = HashMap::new();
            for (name, value) in iter {
                if let Some(value) = value.as_ref() {
                    h.insert(name, value);
                }
            };
            let mut card = Card {
                code: h.get(&&"id").unwrap().parse().unwrap(),
                name: h.get(&&"name").unwrap().to_string(),
                desc: h.get(&&"desc").unwrap().to_string(),
                alias: h.get(&&"alias").unwrap().parse().unwrap_or_default(),
                setcode: h.get(&&"setcode").unwrap().parse().unwrap(),
                _type: Type::from_bits_retain(h.get(&&"type").unwrap().parse::<u32>().unwrap()),
                level: 0,
                attribute: Attribute::empty(),
                race: Race::empty(),
                attack: 0,
                defense: 0,
                lscale: 0,
                rscale: 0,
                link_marker: Linkmarkers::empty(),
                ot: OT::from_bits_retain(h.get(&&"ot").unwrap().parse().unwrap()),
                category: Category::from_bits_truncate((h.get(&&"category").unwrap().parse::<i64>().unwrap() as u64).wrapping_add(u64::MAX/2+1)),
                texts: Vec::new(),
                pack: None,
            };
            if card._type.contains(Type::Monster) {
                card.level = h.get(&&"level").unwrap().parse().unwrap();
                card.attribute = Attribute::from_bits_retain(h.get(&&"attribute").unwrap().parse().unwrap());
                card.race = Race::from_bits_retain(h.get(&&"race").unwrap().parse().unwrap());
                card.attack = h.get(&&"atk").unwrap().parse().unwrap();
                card.defense = h.get(&&"def").unwrap().parse().unwrap();
            }
            if card._type.contains(Type::Link) {
                card.link_marker = Linkmarkers::from_bits_retain(card.defense);
                card.defense = card.level as i32;
            }
            if card._type.contains(Type::Pendulum) {
                card.lscale = (card.level >> 24) & 0xff;
                card.rscale = (card.level >> 16) & 0xff;
            }
            for i in 0..15 {
                if let Some(s) = h.get(&&STR_FIELD_NAMES[i]) {
                    if ! s.is_empty() {
                        while i > card.texts.len() { card.texts.push(String::new()) }
                        card.texts.insert(i, s.to_string())
                    }
                }
            }
            card.level = card.level & 0xff;
            println!("{}", card.desc);
            true
        }).ok();
        cards
    }
}

impl CardTransformer for CDB {
    fn to_string(_: &Card) -> String {
        unimplemented!()
    }

    fn from_string(from: &str) -> Vec<Card> {
        CDB::from_connection(sqlite::open(from).expect("Cannot open sqlite file"))
    }
}
