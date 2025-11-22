use leptos::prelude::*;

use crate::pages::home::Color;

#[component]
pub fn Swatch(color: Color, idx: ReadSignal<usize>) -> impl IntoView {
	let style = format!("background:rgb({}, {}, {})", color.value.r, color.value.g, color.value.b,);
	view! { <li style=style>{color.name.clone()} <button>Detele: {idx.get()}</button></li> }
}
