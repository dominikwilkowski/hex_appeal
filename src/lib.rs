use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod pages;

use crate::pages::{home::Home, matrix::Matrix, not_found::NotFound};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Color {
	pub name: String,
	pub value: Rgb,
}

#[derive(Clone)]
pub struct Group {
	pub name: String,
	pub include_default: bool,
	pub colors: Vec<Color>,
}

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
		<Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
		<Title text="Hex Appeal" />
		<Meta charset="UTF-8" />
		<Meta name="viewport" content="width=device-width, initial-scale=1.0" />

		<main class="container">
			<h1>"Hex Appeal"</h1>
			<Router base="/hex_appeal">
				<nav>
					<ul>
						<li>
							<A href="/hex_appeal/">Home</A>
						</li>
						<li>
							<A href="/hex_appeal/matrix">Matrix</A>
						</li>
					</ul>
				</nav>
				<Routes fallback=|| view! { <NotFound /> }>
					<Route path=path!("/") view=Home />
					<Route path=path!("/matrix") view=Matrix />
				</Routes>
			</Router>
		</main>
	}
}
