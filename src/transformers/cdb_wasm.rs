use crate::Database;
use crate::constants::*;
use crate::card::{Card, CardTransformer};
use crate::transformers::{SQL, STR_FIELD_NAMES, CREATE_TABLE_SQL};
use js_sys::Array;
use wasm_bindgen::prelude::*;

pub struct CDB;

impl CDB {
    pub fn from_database(database: &Database) -> Vec<Card> {
        const QUERY: &str = "select * from datas join texts where datas.id = texts.id";
        let mut cards = Vec::new();
        let values = database.select_objects(QUERY);
        for h in values {
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
                let s: String = get(&h, STR_FIELD_NAMES[i]);
                if ! s.is_empty() {
                    while i > card.texts.len() { card.texts.push(String::new()) }
                    card.texts.insert(i, s.to_string())
                }
            }
            card.level = card.level & 0xff;
            cards.push(card);
        }
        cards
    }

    pub fn write_database(cards: &Vec<Card>, database: &Database) {
        database.exec(&CREATE_TABLE_SQL);
        database.exec(&cards.iter().map(|c| SQL::to_string(c)).collect::<Vec<_>>().join("\n"));
    }
}

fn get_raw(obj: &JsValue, key: &str) -> JsValue {
    match js_sys::Reflect::get(&obj, &JsValue::from_str(key)) {
        Ok(value) => return value,
        Err(_) => return JsValue::null()
    }
}

trait Get {
    fn get(obj: &JsValue, key: &str) -> Self;
}

impl Get for String {
    fn get(obj: &JsValue, key: &str) -> String {
        get_raw(obj, key).as_string().unwrap_or_default()
    }
}

impl Get for f64 {
    fn get(obj: &JsValue, key: &str) -> f64 {
        get_raw(obj, key).as_f64().unwrap_or_default()
    }
}

impl Get for u32 {
    fn get(obj: &JsValue, key: &str) -> u32 {
        get_raw(obj, key).as_f64().unwrap_or_default().ceil() as u32
    }
}

impl Get for u64 {
    fn get(obj: &JsValue, key: &str) -> u64 {
        get_raw(obj, key).as_f64().unwrap_or_default().ceil() as u64
    }
}

impl Get for i32 {
    fn get(obj: &JsValue, key: &str) -> Self {
        get_raw(obj, key).as_f64().unwrap_or_default().ceil() as i32
    }
}

fn get<T: Get>(obj: &JsValue, key: &str) -> T {
    T::get(obj, key)
}
