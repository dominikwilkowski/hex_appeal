use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Footer() -> impl IntoView {
	let version = env!("CARGO_PKG_VERSION");

	view! {
		<footer>
			<A href="/settings">Settings</A>
			<span class="version">v{version}</span>
		</footer>
	}
}
