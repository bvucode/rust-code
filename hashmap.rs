use std::collections::HashMap;

fn main() {
	let mut h = HashMap::new();
	let key = "apple";
	let value = "fruit";
	h.insert(key, value);
	println!("{:?}", h);
}