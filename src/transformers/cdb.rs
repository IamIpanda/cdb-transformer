use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use sqlite::Connection;

use crate::card::{Card, CardTransformer};
use crate::constants::*;

use crate::transformers::{CREATE_TABLE_SQL, SQL, STR_FIELD_NAMES};

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
                code: get(&h, "id"),
                name: get(&h, "name"),
                desc: get(&h, "desc"),
                alias: get(&h, "alias"),
                setcode: get(&h, "setcode"),
                _type: Type::from_bits_retain(get(&h, "type")),
                level: 0,
                attribute: Attribute::empty(),
                race: Race::empty(),
                attack: 0,
                defense: 0,
                lscale: 0,
                rscale: 0,
                link_marker: Linkmarkers::empty(),
                ot: OT::from_bits_retain(get(&h, "ot")),
                category: Category::from_bits_truncate(get(&h, "category")),
                texts: Vec::new(),
                pack: None,
                range: None
            };
            if card._type.contains(Type::Monster) {
                card.level = get(&h, "level");
                card.attribute = Attribute::from_bits_retain(get(&h, "attribute"));
                card.race = Race::from_bits_retain(get(&h, "race"));
                card.attack = get(&h, "atk");
                card.defense = get(&h, "def");
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
            cards.push(card);
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

trait Get<S> {
    fn get(obj: &S, key: &str) -> Self;
}

fn get<S, T>(obj: &S, key: &str) -> T where T: Get<S> {
    T::get(obj, key)
}

impl<S> Get<HashMap<&&str, &&str>> for S where S: FromStr, <S as FromStr>::Err: Debug {
    fn get(obj: &HashMap<&&str, &&str>, key: &str) -> S {
        obj.get(&key).unwrap().parse().unwrap()
    }
}

