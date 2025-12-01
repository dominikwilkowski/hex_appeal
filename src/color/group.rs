use serde::{Deserialize, Serialize};

use crate::color::rgb::Rgb;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Color {
	pub id: usize,
	pub name: String,
	pub value: Rgb,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Group {
	pub id: usize,
	pub name: String,
	pub include_default: bool,
	pub color_increment: usize,
	pub colors: Vec<Color>,
}

impl Group {
	pub fn add(&mut self, value: Rgb, name: String) {
		self.color_increment += 1;
		self.colors.push(Color {
			id: self.color_increment,
			name,
			value,
		});
	}
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Groups {
	pub group_increment: usize,
	pub groups: Vec<Group>,
}

impl Groups {
	pub fn add(&mut self, name: String, include_default: bool, color_increment: usize, colors: Vec<Color>) {
		self.group_increment += 1;
		self.groups.push(Group {
			id: self.group_increment,
			name,
			include_default,
			color_increment,
			colors,
		});
	}
}

impl Default for Groups {
	fn default() -> Self {
		Self {
			group_increment: 1,
			groups: vec![Group {
				id: 1,
				name: String::from("Default"),
				include_default: true,
				color_increment: 3,
				colors: vec![
					Color {
						id: 1,
						name: String::from("Red"),
						value: Rgb {
							red: 255,
							green: 0,
							blue: 0,
						},
					},
					Color {
						id: 2,
						name: String::from("Green"),
						value: Rgb {
							red: 0,
							green: 255,
							blue: 0,
						},
					},
					Color {
						id: 3,
						name: String::from("Blue"),
						value: Rgb {
							red: 0,
							green: 0,
							blue: 255,
						},
					},
				],
			}],
		}
	}
}
