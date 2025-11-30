use leptos::prelude::*;

use crate::color::group::Groups;

#[component]
pub fn Matrix() -> impl IntoView {
	let _groups = use_context::<ReadSignal<Groups>>().expect("Unable to find groups context");

	view! { <div>Matrix!</div> }
}
