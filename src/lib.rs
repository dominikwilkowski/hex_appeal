use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod pages;

use crate::pages::{home::Home, not_found::NotFound};

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	view! {
		<Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
		<Title text="Hex Appeal" />
		<Meta charset="UTF-8" />
		<Meta name="viewport" content="width=device-width, initial-scale=1.0" />

		<Router base="/hex_appeal">
			<Routes fallback=|| view! { <NotFound /> }>
				<Route path=path!("/") view=Home />
			</Routes>
		</Router>
	}
}
