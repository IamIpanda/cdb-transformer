use serde::Serialize;
use serde::Deserialize;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::constants::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackInfo {
	pub id: u32,
	pub pack_id: String,
	pub pack: String,
	pub rarity: Vec<String>,
	pub date: String
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Range {
	pub start: usize,
	pub end: usize
}

impl From<std::ops::Range<usize>> for Range {
	fn from(value: std::ops::Range<usize>) -> Self {
		Self { start: value.start, end: value.end }
	}
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub code: u32,
	pub name: String,
	pub desc: String,
	pub alias: u32,
	pub setcode: u64,
	pub _type: Type,
	pub level: u32,
	pub attribute: Attribute,
	pub race: Race,
	pub attack: i32,
	pub defense: i32,
	pub lscale: u32,
	pub rscale: u32,
	pub link_marker: Linkmarkers,
	pub ot: OT,
	pub category: Category,
	pub texts: Vec<String>,
	pub pack: Option<PackInfo>,
	pub range: Option<Range>
}

impl Card {
	pub fn new() -> Card {
		return Card {
			code: 0,
			name: String::new(),
			desc: String::new(),
			alias: 0,
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
			range: None
		};
	}
}

pub trait CardTransformer {
	fn to_string(card: &Card) -> String;
	fn from_string(str: &str) -> Vec<Card>;
	fn merge_string<'a>(cards: impl Iterator<Item = &'a Card>) -> String {
		cards.map(|card| Self::to_string(card)).collect::<Vec<_>>().join("\n\n")
	}
}
