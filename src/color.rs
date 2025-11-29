use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Rgb {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

impl Rgb {
	pub fn from_hex(hex: &str) -> Option<Rgb> {
		let hex = hex.strip_prefix('#')?;
		if hex.len() == 3 {
			let red = u8::from_str_radix(&hex[0..1], 16).ok()?;
			let green = u8::from_str_radix(&hex[1..2], 16).ok()?;
			let blue = u8::from_str_radix(&hex[2..3], 16).ok()?;

			Some(Rgb {
				red: red * 17,
				green: green * 17,
				blue: blue * 17,
			})
		} else if hex.len() == 6 {
			let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
			let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
			let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;

			Some(Rgb { red, green, blue })
		} else {
			None
		}
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Color {
	pub name: String,
	pub value: Rgb,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Group {
	pub name: String,
	pub include_default: bool,
	pub colors: Vec<Color>,
}

impl Default for Group {
	fn default() -> Self {
		Self {
			name: String::from("Default"),
			include_default: false,
			colors: vec![
				Color {
					name: String::from("Red"),
					value: Rgb {
						red: 255,
						green: 0,
						blue: 0,
					},
				},
				Color {
					name: String::from("Green"),
					value: Rgb {
						red: 0,
						green: 255,
						blue: 0,
					},
				},
				Color {
					name: String::from("Blue"),
					value: Rgb {
						red: 0,
						green: 0,
						blue: 255,
					},
				},
			],
		}
	}
}
