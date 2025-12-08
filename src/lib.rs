use gloo_storage::Storage;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::{
	color::group::Groups,
	components::{footer::Footer, header::Header},
	pages::{home::Home, matrix::Matrix, not_found::NotFound, settings::Settings},
};

mod color;
mod components;
mod pages;

const STORAGE_KEY: &str = "hex_appeal_v1";

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	let stored_groups: Groups = gloo_storage::LocalStorage::get(STORAGE_KEY).unwrap_or_else(|_| Groups::default());
	let (groups, set_groups) = signal(stored_groups);

	Effect::new({
		move |_| {
			let current = groups.get();
			let _ = gloo_storage::LocalStorage::set(STORAGE_KEY, &current);
		}
	});

	provide_context(groups);
	provide_context(set_groups);

	view! {
		<Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
		<Title text="Hex Appeal" />
		<Meta charset="UTF-8" />
		<Meta name="viewport" content="width=device-width, initial-scale=1.0" />

		<Router>
			<Header />
			<main class="container">
				<Routes fallback=|| view! { <NotFound /> }>
					<Route path=path!("/") view=Home />
					<Route path=path!("/matrix") view=Matrix />
					<Route path=path!("/settings") view=Settings />
				</Routes>
			</main>
			<Footer />
		</Router>
	}
}
