use leptos::prelude::*;

use crate::{
	color::group::Groups,
	components::{del_button::DelButton, new_group::NewGroup, swatch::Swatch},
};

#[component]
pub fn Group(group_idx: ReadSignal<usize>) -> impl IntoView {
	let groups = use_context::<ReadSignal<Groups>>().expect("Unable to find groups context");
	let set_groups = use_context::<WriteSignal<Groups>>().expect("Unable to find set_groups context");

	let colors = move || groups.with(|all| all.groups.get(group_idx.get()).map(|g| g.colors.clone()).unwrap_or_default());
	let group_name =
		move || groups.with(|all| all.groups.get(group_idx.get()).map(|g| g.name.clone()).unwrap_or_default());
	let group_included =
		move || groups.with(|all| all.groups.get(group_idx.get()).map(|g| g.include_default).unwrap_or_default());

	view! {
		<div class="group_wrapper">
			<h2>{group_name}</h2>
			<label>
				"Include this group in all matrices"
				<input
					type="checkbox"
					checked=group_included
					on:change=move |ev| {
						let checked = event_target_checked(&ev);
						set_groups
							.update(|all| {
								if let Some(group) = all.groups.get_mut(group_idx.get()) {
									group.include_default = checked;
								}
							});
					}
				/>
			</label>
			<Show when=move || { group_idx.get() > 0 }>
				<DelButton on_click=move |_| {
					let index = group_idx.get_untracked();
					set_groups
						.update(move |groups| {
							if index < groups.groups.len() {
								groups.groups.remove(index);
							}
						});
				}>"Delete"</DelButton>
			</Show>
			<ul class="group">
				<ForEnumerate
					each=move || colors()
					key=|color| color.id
					children=move |idx, color| {
						view! { <Swatch color=color idx=idx group_idx=group_idx /> }
					}
				/>
				<li class="new_swatch">
					<NewGroup group_idx=group_idx />
				</li>
			</ul>
		</div>
	}
}
