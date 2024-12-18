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

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(getter_with_clone)]
pub struct Card {
    pub code: u32,
	pub name: String,
	pub desc: String,
	pub alias: u32,
	pub setcode: u64,
	#[wasm_bindgen(skip)]
	pub _type: Type,
	pub level: u32,
	#[wasm_bindgen(skip)]
	pub attribute: Attribute,
	#[wasm_bindgen(skip)]
	pub race: Race,
	pub attack: i32,
	pub defense: i32,
	pub lscale: u32,
	pub rscale: u32,
	#[wasm_bindgen(skip)]
	pub link_marker: Linkmarkers,
	#[wasm_bindgen(skip)]
	pub ot: OT,
	#[wasm_bindgen(skip)]
	pub category: Category,
	pub texts: Vec<String>,
	pub pack: Option<PackInfo>,
	pub range: Option<Range>	
}

pub trait CardTransformer {
	fn to_string(card: &Card) -> String;
	fn from_string(str: &str) -> Vec<Card>;
	fn merge_string<'a>(cards: impl Iterator<Item = &'a Card>) -> String {
		cards.map(|card| Self::to_string(card)).collect::<Vec<_>>().join("\n\n")
	}
}


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl Card {
	#[wasm_bindgen(getter = type)]
	pub fn _type(&self) -> u32 { self._type.bits() }
	#[wasm_bindgen(setter = type)]
	pub fn set_type(&mut self, v: u32) { self._type = Type::from_bits_retain(v) }
	#[wasm_bindgen(getter)]
	pub fn attribute(&self) -> u32 { self.attribute.bits() }
	#[wasm_bindgen(setter)]
	pub fn set_attribute(&mut self, v: u32) { self.attribute = Attribute::from_bits_retain(v) }
	#[wasm_bindgen(getter)]
	pub fn race(&self) -> u32 { self.race.bits() }
	#[wasm_bindgen(setter)]
	pub fn set_race(&mut self, v: u32) { self.race = Race::from_bits_retain(v) }
	#[wasm_bindgen(getter)]
	pub fn linkmarker(&self) -> i32 { self.link_marker.bits() }
	#[wasm_bindgen(setter)]
	pub fn set_linkmarker(&mut self, v: i32) { self.link_marker = Linkmarkers::from_bits_retain(v) }
	#[wasm_bindgen(getter)]
	pub fn ot(&self) -> u32 { self.ot.bits() }
	#[wasm_bindgen(setter)]
	pub fn set_ot(&mut self, v: u32) { self.ot = OT::from_bits_retain(v) }
	#[wasm_bindgen(getter)]
	pub fn category(&self) -> u64 { self.category.bits() }
	#[wasm_bindgen(setter)]
	pub fn set_category(&mut self, v: u64) { self.category = Category::from_bits_retain(v) }
}
