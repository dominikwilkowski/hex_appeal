use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
	view! {
		<footer>
			<A href="/settings">Settings</A>
		</footer>
	}
}
