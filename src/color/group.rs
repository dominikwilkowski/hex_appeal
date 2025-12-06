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
							value: Rgb {
								red: 0,
								green: 0,
								blue: 0,
							},
						},
						Color {
							id: 2,
							name: String::from("PrimaryGray90"),
							value: Rgb {
								red: 24,
								green: 21,
								blue: 2,
							},
						},
						Color {
							id: 3,
							name: String::from("PrimaryGray80"),
							value: Rgb {
								red: 61,
								green: 58,
								blue: 5,
							},
						},
						Color {
							id: 4,
							name: String::from("PrimaryGray60"),
							value: Rgb {
								red: 113,
								green: 110,
								blue: 1,
							},
						},
						Color {
							id: 5,
							name: String::from("PrimaryGray50"),
							value: Rgb {
								red: 173,
								green: 170,
								blue: 1,
							},
						},
						Color {
							id: 6,
							name: String::from("PrimaryGray20"),
							value: Rgb {
								red: 191,
								green: 189,
								blue: 1,
							},
						},
						Color {
							id: 7,
							name: String::from("PrimaryGray15"),
							value: Rgb {
								red: 223,
								green: 221,
								blue: 2,
							},
						},
						Color {
							id: 8,
							name: String::from("PrimaryGray10"),
							value: Rgb {
								red: 241,
								green: 237,
								blue: 2,
							},
						},
						Color {
							id: 9,
							name: String::from("PrimaryGray5"),
							value: Rgb {
								red: 246,
								green: 245,
								blue: 2,
							},
						},
						Color {
							id: 10,
							name: String::from("PrimaryGray1"),
							value: Rgb {
								red: 248,
								green: 248,
								blue: 2,
							},
						},
						Color {
							id: 11,
							name: String::from("PrimaryGrayWhite"),
							value: Rgb {
								red: 255,
								green: 255,
								blue: 2,
							},
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
							value: Rgb {
								red: 70,
								green: 0,
								blue: 74,
							},
						},
						Color {
							id: 2,
							name: String::from("PrimaryPurple80"),
							value: Rgb {
								red: 121,
								green: 0,
								blue: 127,
							},
						},
						Color {
							id: 3,
							name: String::from("PrimaryPurple60"),
							value: Rgb {
								red: 156,
								green: 59,
								blue: 184,
							},
						},
						Color {
							id: 4,
							name: String::from("PrimaryPurple50"),
							value: Rgb {
								red: 216,
								green: 126,
								blue: 255,
							},
						},
						Color {
							id: 5,
							name: String::from("PrimaryPurple20"),
							value: Rgb {
								red: 255,
								green: 199,
								blue: 255,
							},
						},
						Color {
							id: 6,
							name: String::from("PrimaryPurple10"),
							value: Rgb {
								red: 244,
								green: 234,
								blue: 255,
							},
						},
						Color {
							id: 7,
							name: String::from("PrimaryPurple5"),
							value: Rgb {
								red: 249,
								green: 242,
								blue: 255,
							},
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
							value: Rgb {
								red: 0,
								green: 89,
								blue: 61,
							},
						},
						Color {
							id: 2,
							name: String::from("PrimaryGreen60"),
							value: Rgb {
								red: 0,
								green: 134,
								blue: 96,
							},
						},
						Color {
							id: 3,
							name: String::from("PrimaryGreen50"),
							value: Rgb {
								red: 0,
								green: 204,
								blue: 150,
							},
						},
						Color {
							id: 4,
							name: String::from("PrimaryGreen20"),
							value: Rgb {
								red: 153,
								green: 244,
								blue: 209,
							},
						},
						Color {
							id: 5,
							name: String::from("PrimaryGreen10"),
							value: Rgb {
								red: 214,
								green: 248,
								blue: 233,
							},
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
							value: Rgb {
								red: 0,
								green: 80,
								blue: 114,
							},
						},
						Color {
							id: 2,
							name: String::from("PrimaryCyan60"),
							value: Rgb {
								red: 0,
								green: 123,
								blue: 170,
							},
						},
						Color {
							id: 3,
							name: String::from("PrimaryCyan50"),
							value: Rgb {
								red: 0,
								green: 188,
								blue: 254,
							},
						},
						Color {
							id: 4,
							name: String::from("PrimaryCyan20"),
							value: Rgb {
								red: 181,
								green: 230,
								blue: 254,
							},
						},
						Color {
							id: 5,
							name: String::from("PrimaryCyan10"),
							value: Rgb {
								red: 216,
								green: 240,
								blue: 252,
							},
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
							value: Rgb {
								red: 137,
								green: 1,
								blue: 34,
							},
						},
						Color {
							id: 2,
							name: String::from("PrimaryRed60"),
							value: Rgb {
								red: 213,
								green: 0,
								blue: 50,
							},
						},
						Color {
							id: 3,
							name: String::from("PrimaryRed50"),
							value: Rgb {
								red: 255,
								green: 114,
								blue: 124,
							},
						},
						Color {
							id: 4,
							name: String::from("PrimaryRed20"),
							value: Rgb {
								red: 255,
								green: 207,
								blue: 206,
							},
						},
						Color {
							id: 5,
							name: String::from("PrimaryRed10"),
							value: Rgb {
								red: 255,
								green: 228,
								blue: 227,
							},
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
							value: Rgb {
								red: 88,
								green: 70,
								blue: 0,
							},
						},
						Color {
							id: 2,
							name: String::from("SecondaryMustard60"),
							value: Rgb {
								red: 135,
								green: 108,
								blue: 0,
							},
						},
						Color {
							id: 3,
							name: String::from("SecondaryMustard50"),
							value: Rgb {
								red: 206,
								green: 167,
								blue: 0,
							},
						},
						Color {
							id: 4,
							name: String::from("SecondaryMustard20"),
							value: Rgb {
								red: 254,
								green: 220,
								blue: 103,
							},
						},
						Color {
							id: 5,
							name: String::from("SecondaryMustard10"),
							value: Rgb {
								red: 255,
								green: 238,
								blue: 177,
							},
						},
						Color {
							id: 6,
							name: String::from("SecondarySalmon80"),
							value: Rgb {
								red: 112,
								green: 51,
								blue: 36,
							},
						},
						Color {
							id: 7,
							name: String::from("SecondarySalmon60"),
							value: Rgb {
								red: 200,
								green: 41,
								blue: 0,
							},
						},
						Color {
							id: 8,
							name: String::from("SecondarySalmon50"),
							value: Rgb {
								red: 255,
								green: 119,
								blue: 83,
							},
						},
						Color {
							id: 9,
							name: String::from("SecondarySalmon20"),
							value: Rgb {
								red: 255,
								green: 119,
								blue: 83,
							},
						},
						Color {
							id: 10,
							name: String::from("SecondarySalmon10"),
							value: Rgb {
								red: 255,
								green: 232,
								blue: 225,
							},
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
							value: Rgb {
								red: 8,
								green: 48,
								blue: 79,
							},
						},
						Color {
							id: 2,
							name: String::from("TertiaryOcean100"),
							value: Rgb {
								red: 15,
								green: 77,
								blue: 83,
							},
						},
						Color {
							id: 3,
							name: String::from("TertiarySeabreeze100"),
							value: Rgb {
								red: 50,
								green: 201,
								blue: 199,
							},
						},
						Color {
							id: 4,
							name: String::from("TertiaryLime100"),
							value: Rgb {
								red: 200,
								green: 240,
								blue: 113,
							},
						},
						Color {
							id: 5,
							name: String::from("TertiaryLemon100"),
							value: Rgb {
								red: 242,
								green: 236,
								blue: 72,
							},
						},
						Color {
							id: 6,
							name: String::from("TertiaryTangerine100"),
							value: Rgb {
								red: 248,
								green: 145,
								blue: 14,
							},
						},
						Color {
							id: 7,
							name: String::from("TertiaryFlamingo100"),
							value: Rgb {
								red: 255,
								green: 76,
								blue: 129,
							},
						},
						Color {
							id: 8,
							name: String::from("TertiaryCottoncandy100"),
							value: Rgb {
								red: 255,
								green: 167,
								blue: 210,
							},
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
