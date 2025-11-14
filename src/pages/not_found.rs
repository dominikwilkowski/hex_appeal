use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
	view! {
		<h1>"Uh oh!"</h1>
		<h3>"We couldn't find that page!"</h3>
	}
}
