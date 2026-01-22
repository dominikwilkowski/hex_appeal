use leptos::prelude::*;

use crate::{
	color::{group::Groups, mode::Mode},
	components::{contrast_table::ContrastTable, toggle::Toggle},
};

#[component]
pub fn Matrix() -> impl IntoView {
	let groups = use_context::<ReadSignal<Groups>>().expect("Unable to find groups context");
	let (mode, set_mode) = signal(Mode::DoubleA);

	provide_context(mode);

	view! {
		"Mode: AA "
		<Toggle
			class="mode_toggle"
			checked=move || mode.get() == Mode::DoubleA
			on_change=move |ev| {
				let checked = event_target_checked(&ev);
				set_mode.set(if checked { Mode::DoubleA } else { Mode::TripleA });
			}
		/>
		" AAA"

		<ForEnumerate
			each=move || groups.get().groups
			key=|group| group.id
			children=move |idx, _| {
				view! { <ContrastTable group_idx=idx /> }
			}
		/>
	}
}
