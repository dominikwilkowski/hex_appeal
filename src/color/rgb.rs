use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Rgb {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub luminance: f64,
}

impl Rgb {
	pub fn new(red: u8, green: u8, blue: u8) -> Self {
		let luminance = Self::relative_luminance(red, green, blue);

		Self {
			red,
			green,
			blue,
			luminance,
		}
	}

	pub fn from_hex(hex: &str) -> Option<Rgb> {
		let hex = hex.strip_prefix('#')?;
		if hex.len() == 3 {
			let red = u8::from_str_radix(&hex[0..1], 16).ok()?;
			let green = u8::from_str_radix(&hex[1..2], 16).ok()?;
			let blue = u8::from_str_radix(&hex[2..3], 16).ok()?;

			Some(Rgb::new(red * 17, green * 17, blue * 17))
		} else if hex.len() == 6 {
			let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
			let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
			let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;

			Some(Rgb::new(red, green, blue))
		} else {
			None
		}
	}

	fn srgb_channel_to_linear(srgb_channel: u8) -> f64 {
		let normalized_channel = srgb_channel as f64 / 255.0;

		if normalized_channel <= 0.04045 {
			normalized_channel / 12.92
		} else {
			((normalized_channel + 0.055) / 1.055).powf(2.4)
		}
	}

	fn relative_luminance(red: u8, green: u8, blue: u8) -> f64 {
		let linear_red = Self::srgb_channel_to_linear(red);
		let linear_green = Self::srgb_channel_to_linear(green);
		let linear_blue = Self::srgb_channel_to_linear(blue);

		// Rec. 709 / sRGB luminance coefficients
		0.2126 * linear_red + 0.7152 * linear_green + 0.0722 * linear_blue
	}

	pub fn contrast_ratio(&self, other_color: &Rgb) -> f64 {
		let (lighter_luminance, darker_luminance) = if self.luminance >= other_color.luminance {
			(self.luminance, other_color.luminance)
		} else {
			(other_color.luminance, self.luminance)
		};

		(lighter_luminance + 0.05) / (darker_luminance + 0.05)
	}
}

impl std::fmt::Display for Rgb {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "rgb({},{},{})", self.red, self.green, self.blue)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn contrast_ratio_test() {
		let ratio = Rgb::new(255, 0, 0).contrast_ratio(&Rgb::new(0, 255, 0));
		assert_eq!(format!("{:.3}", ratio), String::from("2.914"));

		let ratio = Rgb::new(255, 0, 0).contrast_ratio(&Rgb::new(0, 0, 255));
		assert_eq!(format!("{:.3}", ratio), String::from("2.149"));

		let ratio = Rgb::new(101, 143, 94).contrast_ratio(&Rgb::new(60, 56, 90));
		assert_eq!(format!("{:.5}", ratio), String::from("2.96202"));
	}
}
