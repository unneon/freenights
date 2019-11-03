use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ItemData {
	pub name: String,
	pub weight: f32,
}

#[derive(Deserialize)]
pub struct ItemDatabase(pub HashMap<String, ItemData>);
