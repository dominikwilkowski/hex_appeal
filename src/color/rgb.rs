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

	fn srgb_channel_to_linear(srgb_channel: u8) -> f64 {
		let normalized_channel = srgb_channel as f64 / 255.0;

		if normalized_channel <= 0.04045 {
			normalized_channel / 12.92
		} else {
			((normalized_channel + 0.055) / 1.055).powf(2.4)
		}
	}

	fn relative_luminance(color: Rgb) -> f64 {
		let linear_red = Self::srgb_channel_to_linear(color.red);
		let linear_green = Self::srgb_channel_to_linear(color.green);
		let linear_blue = Self::srgb_channel_to_linear(color.blue);

		// Rec. 709 / sRGB luminance coefficients
		0.2126 * linear_red + 0.7152 * linear_green + 0.0722 * linear_blue
	}

	pub fn contrast_ratio(self, other_color: Rgb) -> f64 {
		let luminance_self = Self::relative_luminance(self);
		let luminance_other = Self::relative_luminance(other_color);

		let (lighter_luminance, darker_luminance) = if luminance_self >= luminance_other {
			(luminance_self, luminance_other)
		} else {
			(luminance_other, luminance_self)
		};

		(lighter_luminance + 0.05) / (darker_luminance + 0.05)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn contrast_ratio_test() {
		let ratio = Rgb {
			red: 255,
			green: 0,
			blue: 0,
		}
		.contrast_ratio(Rgb {
			red: 0,
			green: 255,
			blue: 0,
		});
		assert_eq!(format!("{:.3}", ratio), String::from("2.914"));

		let ratio = Rgb {
			red: 255,
			green: 0,
			blue: 0,
		}
		.contrast_ratio(Rgb {
			red: 0,
			green: 0,
			blue: 255,
		});
		assert_eq!(format!("{:.3}", ratio), String::from("2.149"));

		let ratio = Rgb {
			red: 101,
			green: 143,
			blue: 94,
		}
		.contrast_ratio(Rgb {
			red: 60,
			green: 56,
			blue: 90,
		});
		assert_eq!(format!("{:.5}", ratio), String::from("2.96202"));
	}
}
