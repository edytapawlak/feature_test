use keri::derivation::self_addressing::SelfAddressing;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sai(data: &str) -> String {
	let sai = SelfAddressing::Blake3_256.derive(data.as_bytes());
   	sai.to_string()
}