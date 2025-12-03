use serde::{Deserialize, Serialize};

use crate::color::rgb::Rgb;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Color {
	pub id: usize,
	pub name: String,
	pub value: Rgb,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Group {
	pub id: usize,
	pub name: String,
	pub include_default: bool,
	pub color_increment: usize,
	pub colors: Vec<Color>,
}

impl Group {
	pub fn add_color(&mut self, value: Rgb, name: String) {
		self.color_increment += 1;
		self.colors.push(Color {
			id: self.color_increment,
			name,
			value,
		});
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Groups {
	pub group_increment: usize,
	pub groups: Vec<Group>,
}

impl Groups {
	pub fn add_group(&mut self, name: String, include_default: bool) {
		self.group_increment += 1;
		self.groups.push(Group {
			id: self.group_increment,
			name,
			include_default,
			color_increment: 0,
			colors: Vec::new(),
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn group_add_color_test() {
		let mut group = Group {
			id: 1,
			name: String::from("foo"),
			include_default: false,
			color_increment: 0,
			colors: Vec::new(),
		};

		group.add_color(
			Rgb {
				red: 255,
				green: 255,
				blue: 0,
			},
			String::from("Yellow"),
		);

		assert_eq!(group.color_increment, 1);
		assert_eq!(group.colors.len(), 1);
		assert_eq!(group.colors[0].id, 1);
		assert_eq!(group.colors[0].name, "Yellow");
		assert_eq!(
			group.colors[0].value,
			Rgb {
				red: 255,
				green: 255,
				blue: 0
			}
		);

		group.add_color(
			Rgb {
				red: 255,
				green: 0,
				blue: 0,
			},
			String::from("Red"),
		);

		assert_eq!(group.color_increment, 2);
		assert_eq!(group.colors.len(), 2);
		assert_eq!(group.colors[1].id, 2);

		group.colors.remove(1);

		group.add_color(
			Rgb {
				red: 0,
				green: 0,
				blue: 255,
			},
			String::from("Blue"),
		);

		assert_eq!(group.color_increment, 3);
		assert_eq!(group.colors.len(), 2);
		assert_eq!(group.colors[1].id, 3);
	}

	#[test]
	fn groups_add_test() {
		let mut groups = Groups {
			group_increment: 0,
			groups: Vec::new(),
		};

		groups.add_group(String::from("Foo"), false);

		assert_eq!(groups.group_increment, 1);
		assert_eq!(groups.groups.len(), 1);
		assert_eq!(
			groups.groups[0],
			Group {
				id: 1,
				name: String::from("Foo"),
				include_default: false,
				color_increment: 0,
				colors: Vec::new(),
			}
		);

		groups.add_group(String::from("Bar"), true);

		assert_eq!(groups.group_increment, 2);
		assert_eq!(groups.groups.len(), 2);
		assert_eq!(
			groups.groups[1],
			Group {
				id: 2,
				name: String::from("Bar"),
				include_default: true,
				color_increment: 0,
				colors: Vec::new(),
			}
		);

		groups.groups.remove(1);

		groups.add_group(String::from("Baz"), false);

		assert_eq!(groups.group_increment, 3);
		assert_eq!(groups.groups.len(), 2);
		assert_eq!(
			groups.groups[1],
			Group {
				id: 3,
				name: String::from("Baz"),
				include_default: false,
				color_increment: 0,
				colors: Vec::new(),
			}
		);
	}
}
