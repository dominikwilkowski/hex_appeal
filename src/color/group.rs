use serde::{Deserialize, Serialize};

use crate::color::rgb::Rgb;

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
