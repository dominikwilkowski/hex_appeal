use serde::{Deserialize, Serialize};

use std::cmp::Ordering;

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

		self.colors.sort_by(|a, b| a.value.luminance.partial_cmp(&b.value.luminance).unwrap_or(Ordering::Equal));
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
			group_increment: 7,
			groups: vec![
				Group {
					id: 1,
					name: String::from("Neutral"),
					include_default: true,
					color_increment: 11,
					colors: vec![
						Color {
							id: 1,
							name: String::from("Black"),
							value: Rgb::new(0, 0, 0),
						},
						Color {
							id: 2,
							name: String::from("Gray90"),
							value: Rgb::new(24, 21, 22),
						},
						Color {
							id: 3,
							name: String::from("Gray80"),
							value: Rgb::new(61, 58, 59),
						},
						Color {
							id: 4,
							name: String::from("Gray60"),
							value: Rgb::new(113, 110, 111),
						},
						Color {
							id: 5,
							name: String::from("Gray50"),
							value: Rgb::new(173, 170, 171),
						},
						Color {
							id: 6,
							name: String::from("Gray20"),
							value: Rgb::new(191, 189, 190),
						},
						Color {
							id: 7,
							name: String::from("Gray15"),
							value: Rgb::new(223, 221, 222),
						},
						Color {
							id: 8,
							name: String::from("Gray10"),
							value: Rgb::new(241, 237, 238),
						},
						Color {
							id: 9,
							name: String::from("Gray5"),
							value: Rgb::new(246, 245, 245),
						},
						Color {
							id: 10,
							name: String::from("Gray1"),
							value: Rgb::new(248, 248, 248),
						},
						Color {
							id: 11,
							name: String::from("White"),
							value: Rgb::new(255, 255, 255),
						},
					],
				},
				Group {
					id: 2,
					name: String::from("Purple"),
					include_default: false,
					color_increment: 7,
					colors: vec![
						Color {
							id: 1,
							name: String::from("Purple90"),
							value: Rgb::new(70, 0, 74),
						},
						Color {
							id: 2,
							name: String::from("Purple80"),
							value: Rgb::new(121, 0, 127),
						},
						Color {
							id: 3,
							name: String::from("Purple60"),
							value: Rgb::new(156, 59, 184),
						},
						Color {
							id: 4,
							name: String::from("Purple50"),
							value: Rgb::new(216, 126, 255),
						},
						Color {
							id: 5,
							name: String::from("Purple20"),
							value: Rgb::new(255, 199, 255),
						},
						Color {
							id: 6,
							name: String::from("Purple10"),
							value: Rgb::new(244, 234, 255),
						},
						Color {
							id: 7,
							name: String::from("Purple5"),
							value: Rgb::new(249, 242, 255),
						},
					],
				},
				Group {
					id: 3,
					name: String::from("Green"),
					include_default: false,
					color_increment: 5,
					colors: vec![
						Color {
							id: 1,
							name: String::from("Green80"),
							value: Rgb::new(0, 89, 61),
						},
						Color {
							id: 2,
							name: String::from("Green60"),
							value: Rgb::new(0, 134, 96),
						},
						Color {
							id: 3,
							name: String::from("Green50"),
							value: Rgb::new(0, 204, 150),
						},
						Color {
							id: 4,
							name: String::from("Green20"),
							value: Rgb::new(153, 244, 209),
						},
						Color {
							id: 5,
							name: String::from("Green10"),
							value: Rgb::new(214, 248, 233),
						},
					],
				},
				Group {
					id: 4,
					name: String::from("Cyan"),
					include_default: false,
					color_increment: 5,
					colors: vec![
						Color {
							id: 1,
							name: String::from("Cyan80"),
							value: Rgb::new(0, 80, 114),
						},
						Color {
							id: 2,
							name: String::from("Cyan60"),
							value: Rgb::new(0, 123, 170),
						},
						Color {
							id: 3,
							name: String::from("Cyan50"),
							value: Rgb::new(0, 188, 254),
						},
						Color {
							id: 4,
							name: String::from("Cyan20"),
							value: Rgb::new(181, 230, 254),
						},
						Color {
							id: 5,
							name: String::from("Cyan10"),
							value: Rgb::new(216, 240, 252),
						},
					],
				},
				Group {
					id: 5,
					name: String::from("Red"),
					include_default: false,
					color_increment: 5,
					colors: vec![
						Color {
							id: 1,
							name: String::from("Red80"),
							value: Rgb::new(137, 1, 34),
						},
						Color {
							id: 2,
							name: String::from("Red60"),
							value: Rgb::new(213, 0, 50),
						},
						Color {
							id: 3,
							name: String::from("Red50"),
							value: Rgb::new(255, 114, 124),
						},
						Color {
							id: 4,
							name: String::from("Red20"),
							value: Rgb::new(255, 207, 206),
						},
						Color {
							id: 5,
							name: String::from("Red10"),
							value: Rgb::new(255, 228, 227),
						},
					],
				},
				Group {
					id: 6,
					name: String::from("Mustard"),
					include_default: false,
					color_increment: 10,
					colors: vec![
						Color {
							id: 1,
							name: String::from("Mustard80"),
							value: Rgb::new(88, 70, 0),
						},
						Color {
							id: 2,
							name: String::from("Mustard60"),
							value: Rgb::new(135, 108, 0),
						},
						Color {
							id: 3,
							name: String::from("Mustard50"),
							value: Rgb::new(206, 167, 0),
						},
						Color {
							id: 4,
							name: String::from("Mustard20"),
							value: Rgb::new(254, 220, 103),
						},
						Color {
							id: 5,
							name: String::from("Mustard10"),
							value: Rgb::new(255, 238, 177),
						},
					],
				},
				Group {
					id: 7,
					name: String::from("Salmon"),
					include_default: false,
					color_increment: 10,
					colors: vec![
						Color {
							id: 6,
							name: String::from("Salmon80"),
							value: Rgb::new(112, 51, 36),
						},
						Color {
							id: 7,
							name: String::from("Salmon60"),
							value: Rgb::new(200, 41, 0),
						},
						Color {
							id: 8,
							name: String::from("Salmon50"),
							value: Rgb::new(255, 119, 83),
						},
						Color {
							id: 9,
							name: String::from("Salmon20"),
							value: Rgb::new(255, 119, 83),
						},
						Color {
							id: 10,
							name: String::from("Salmon10"),
							value: Rgb::new(255, 232, 225),
						},
					],
				},
				Group {
					id: 8,
					name: String::from("Tertiary"),
					include_default: false,
					color_increment: 8,
					colors: vec![
						Color {
							id: 1,
							name: String::from("Midnight100"),
							value: Rgb::new(8, 48, 79),
						},
						Color {
							id: 2,
							name: String::from("Ocean100"),
							value: Rgb::new(15, 77, 83),
						},
						Color {
							id: 3,
							name: String::from("Seabreeze100"),
							value: Rgb::new(50, 201, 199),
						},
						Color {
							id: 4,
							name: String::from("Lime100"),
							value: Rgb::new(200, 240, 113),
						},
						Color {
							id: 5,
							name: String::from("Lemon100"),
							value: Rgb::new(242, 236, 72),
						},
						Color {
							id: 6,
							name: String::from("Tangerine100"),
							value: Rgb::new(248, 145, 14),
						},
						Color {
							id: 7,
							name: String::from("Flamingo100"),
							value: Rgb::new(255, 76, 129),
						},
						Color {
							id: 8,
							name: String::from("Cottoncandy100"),
							value: Rgb::new(255, 167, 210),
						},
					],
				},
			],
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

		group.add_color(Rgb::new(255, 255, 0), String::from("Yellow"));

		assert_eq!(group.color_increment, 1);
		assert_eq!(group.colors.len(), 1);
		assert_eq!(group.colors[0].id, 1);
		assert_eq!(group.colors[0].name, "Yellow");
		assert_eq!(group.colors[0].value, Rgb::new(255, 255, 0));

		group.add_color(Rgb::new(255, 0, 0), String::from("Red"));

		assert_eq!(group.color_increment, 2);
		assert_eq!(group.colors.len(), 2);
		assert_eq!(group.colors[0].id, 2);
		assert_eq!(group.colors[0].name, "Red");
		assert_eq!(group.colors[0].value, Rgb::new(255, 0, 0));

		group.colors.remove(1);

		group.add_color(Rgb::new(0, 0, 255), String::from("Blue"));

		assert_eq!(group.color_increment, 3);
		assert_eq!(group.colors.len(), 2);
		assert_eq!(group.colors[0].id, 3);
		assert_eq!(group.colors[0].name, "Blue");
		assert_eq!(group.colors[0].value, Rgb::new(0, 0, 255));
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
