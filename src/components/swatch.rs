use leptos::prelude::*;

use crate::pages::home::Color;

#[component]
pub fn Swatch(color: Color) -> impl IntoView {
	let style = format!("background:rgb({}, {}, {})", color.value.r, color.value.g, color.value.b,);
	view! { <li style=style>{color.name.clone()}</li> }
}
