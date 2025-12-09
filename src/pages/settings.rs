use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::{components::del_button::DelButton, Groups};

#[component]
pub fn Settings() -> impl IntoView {
	let set_groups = use_context::<WriteSignal<Groups>>().expect("Unable to find set_groups context");
	let navigate = use_navigate();

	view! {
		<DelButton on_click=move |_| {
			set_groups
				.update(move |groups| {
					*groups = Groups::default();
				});
			navigate("/", Default::default());
		}>"Reset all data"</DelButton>
	}
}
