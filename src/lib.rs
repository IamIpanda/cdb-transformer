pub mod card;
pub mod constants;
pub mod transformers;

#[cfg(target_arch="wasm32")]
use std::ops::Deref;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn parse(text: &str) -> Vec<card::Card> {
    <transformers::Xyyz as card::CardTransformer>::from_string(text)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn set_string_conf(text: &str) {
    transformers::set_string_conf(text);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn format(card: &card::Card) -> String {
    <transformers::Xyyz as card::CardTransformer>::to_string(card)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn format_subtype(_type: u32) -> String {
    transformers::Xyyz::format_subtype(&constants::Type::from_bits_retain(_type))
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn format_race(race: u32) -> String {
    transformers::Xyyz::format_race(&constants::Race::from_bits_retain(race))
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn race_names() -> wasm_bindgen::JsValue {
    serde_wasm_bindgen::to_value(&transformers::RACE_NAMES).unwrap()
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn type_names() -> wasm_bindgen::JsValue {
    serde_wasm_bindgen::to_value(&transformers::TYPE_NAMES).unwrap()
}
#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn attribute_names() -> wasm_bindgen::JsValue {
    serde_wasm_bindgen::to_value(&transformers::ATTRIBUTE_NAMES).unwrap()
}
#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn linker_names() -> wasm_bindgen::JsValue {
    serde_wasm_bindgen::to_value(&transformers::LINKMARKERS_NAMES).unwrap()
}
#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn ot_names() -> wasm_bindgen::JsValue {
    serde_wasm_bindgen::to_value(&transformers::OT_NAMES).unwrap()
}
#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn category_names() -> wasm_bindgen::JsValue {
    serde_wasm_bindgen::to_value(&transformers::CATEGORY_NAMES).unwrap()
}
#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn set_names() -> wasm_bindgen::JsValue {
    serde_wasm_bindgen::to_value(transformers::SET_NAMES.deref()).unwrap()
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Database;

    #[wasm_bindgen(method, js_class = "Database", js_name = selectObjects)]
    pub fn select_objects(this: &Database, sql: &str) -> js_sys::Array;

    #[wasm_bindgen(method, js_class = "Database")] 
    pub fn exec(this: &Database, sql: &str) -> Database;
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn parse_database(db: &Database) -> Vec<card::Card> {
    transformers::CDB::from_database(db)
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn write_database(db: &Database, text: &str) {
    let cards = parse(text);
    transformers::CDB::write_database(&cards, db)
}


