use crate::constants::*;

#[derive(Debug)]
pub struct PackInfo {
	pack_id: String,
	pack: String,
	rarity: Vec<String>,
	date: String
}

#[derive(Debug)]
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
	pub pack: Option<PackInfo>
}

pub trait CardTransformer {
	fn to_string(card: &Card) -> String;
	fn from_string(str: &str) -> Vec<Card>;
}
