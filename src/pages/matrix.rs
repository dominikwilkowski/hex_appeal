use leptos::prelude::*;

use crate::{color::group::Groups, components::contrast_table::ContrastTable};

#[component]
pub fn Matrix() -> impl IntoView {
	let groups = use_context::<ReadSignal<Groups>>().expect("Unable to find groups context");

	view! {
		<ForEnumerate
			each=move || groups.get().groups
			key=|group| group.id
			children=move |idx, _| {
				view! { <ContrastTable group_idx=idx /> }
			}
		/>
	}
}
