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
							name: String::from("PrimaryGrayBlack"),
							value: Rgb::new(0, 0, 0),
						},
						Color {
							id: 2,
							name: String::from("PrimaryGray90"),
							value: Rgb::new(24, 21, 2),
						},
						Color {
							id: 3,
							name: String::from("PrimaryGray80"),
							value: Rgb::new(61, 58, 5),
						},
						Color {
							id: 4,
							name: String::from("PrimaryGray60"),
							value: Rgb::new(113, 110, 1),
						},
						Color {
							id: 5,
							name: String::from("PrimaryGray50"),
							value: Rgb::new(173, 170, 1),
						},
						Color {
							id: 6,
							name: String::from("PrimaryGray20"),
							value: Rgb::new(191, 189, 1),
						},
						Color {
							id: 7,
							name: String::from("PrimaryGray15"),
							value: Rgb::new(223, 221, 2),
						},
						Color {
							id: 8,
							name: String::from("PrimaryGray10"),
							value: Rgb::new(241, 237, 2),
						},
						Color {
							id: 9,
							name: String::from("PrimaryGray5"),
							value: Rgb::new(246, 245, 2),
						},
						Color {
							id: 10,
							name: String::from("PrimaryGray1"),
							value: Rgb::new(248, 248, 2),
						},
						Color {
							id: 11,
							name: String::from("PrimaryGrayWhite"),
							value: Rgb::new(255, 255, 2),
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
							name: String::from("PrimaryPurple90"),
							value: Rgb::new(70, 0, 74),
						},
						Color {
							id: 2,
							name: String::from("PrimaryPurple80"),
							value: Rgb::new(121, 0, 127),
						},
						Color {
							id: 3,
							name: String::from("PrimaryPurple60"),
							value: Rgb::new(156, 59, 184),
						},
						Color {
							id: 4,
							name: String::from("PrimaryPurple50"),
							value: Rgb::new(216, 126, 255),
						},
						Color {
							id: 5,
							name: String::from("PrimaryPurple20"),
							value: Rgb::new(255, 199, 255),
						},
						Color {
							id: 6,
							name: String::from("PrimaryPurple10"),
							value: Rgb::new(244, 234, 255),
						},
						Color {
							id: 7,
							name: String::from("PrimaryPurple5"),
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
							name: String::from("PrimaryGreen80"),
							value: Rgb::new(0, 89, 61),
						},
						Color {
							id: 2,
							name: String::from("PrimaryGreen60"),
							value: Rgb::new(0, 134, 96),
						},
						Color {
							id: 3,
							name: String::from("PrimaryGreen50"),
							value: Rgb::new(0, 204, 150),
						},
						Color {
							id: 4,
							name: String::from("PrimaryGreen20"),
							value: Rgb::new(153, 244, 209),
						},
						Color {
							id: 5,
							name: String::from("PrimaryGreen10"),
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
							name: String::from("PrimaryCyan80"),
							value: Rgb::new(0, 80, 114),
						},
						Color {
							id: 2,
							name: String::from("PrimaryCyan60"),
							value: Rgb::new(0, 123, 170),
						},
						Color {
							id: 3,
							name: String::from("PrimaryCyan50"),
							value: Rgb::new(0, 188, 254),
						},
						Color {
							id: 4,
							name: String::from("PrimaryCyan20"),
							value: Rgb::new(181, 230, 254),
						},
						Color {
							id: 5,
							name: String::from("PrimaryCyan10"),
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
							name: String::from("PrimaryRed80"),
							value: Rgb::new(137, 1, 34),
						},
						Color {
							id: 2,
							name: String::from("PrimaryRed60"),
							value: Rgb::new(213, 0, 50),
						},
						Color {
							id: 3,
							name: String::from("PrimaryRed50"),
							value: Rgb::new(255, 114, 124),
						},
						Color {
							id: 4,
							name: String::from("PrimaryRed20"),
							value: Rgb::new(255, 207, 206),
						},
						Color {
							id: 5,
							name: String::from("PrimaryRed10"),
							value: Rgb::new(255, 228, 227),
						},
					],
				},
				Group {
					id: 6,
					name: String::from("Secondary"),
					include_default: false,
					color_increment: 10,
					colors: vec![
						Color {
							id: 1,
							name: String::from("SecondaryMustard80"),
							value: Rgb::new(88, 70, 0),
						},
						Color {
							id: 2,
							name: String::from("SecondaryMustard60"),
							value: Rgb::new(135, 108, 0),
						},
						Color {
							id: 3,
							name: String::from("SecondaryMustard50"),
							value: Rgb::new(206, 167, 0),
						},
						Color {
							id: 4,
							name: String::from("SecondaryMustard20"),
							value: Rgb::new(254, 220, 103),
						},
						Color {
							id: 5,
							name: String::from("SecondaryMustard10"),
							value: Rgb::new(255, 238, 177),
						},
						Color {
							id: 6,
							name: String::from("SecondarySalmon80"),
							value: Rgb::new(112, 51, 36),
						},
						Color {
							id: 7,
							name: String::from("SecondarySalmon60"),
							value: Rgb::new(200, 41, 0),
						},
						Color {
							id: 8,
							name: String::from("SecondarySalmon50"),
							value: Rgb::new(255, 119, 83),
						},
						Color {
							id: 9,
							name: String::from("SecondarySalmon20"),
							value: Rgb::new(255, 119, 83),
						},
						Color {
							id: 10,
							name: String::from("SecondarySalmon10"),
							value: Rgb::new(255, 232, 225),
						},
					],
				},
				Group {
					id: 7,
					name: String::from("Tertiary"),
					include_default: false,
					color_increment: 8,
					colors: vec![
						Color {
							id: 1,
							name: String::from("TertiaryMidnight100"),
							value: Rgb::new(8, 48, 79),
						},
						Color {
							id: 2,
							name: String::from("TertiaryOcean100"),
							value: Rgb::new(15, 77, 83),
						},
						Color {
							id: 3,
							name: String::from("TertiarySeabreeze100"),
							value: Rgb::new(50, 201, 199),
						},
						Color {
							id: 4,
							name: String::from("TertiaryLime100"),
							value: Rgb::new(200, 240, 113),
						},
						Color {
							id: 5,
							name: String::from("TertiaryLemon100"),
							value: Rgb::new(242, 236, 72),
						},
						Color {
							id: 6,
							name: String::from("TertiaryTangerine100"),
							value: Rgb::new(248, 145, 14),
						},
						Color {
							id: 7,
							name: String::from("TertiaryFlamingo100"),
							value: Rgb::new(255, 76, 129),
						},
						Color {
							id: 8,
							name: String::from("TertiaryCottoncandy100"),
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
		assert_eq!(group.colors[1].id, 2);

		group.colors.remove(1);

		group.add_color(Rgb::new(0, 0, 255), String::from("Blue"));

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
