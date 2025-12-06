use leptos::prelude::*;

use crate::{
	color::group::{Color, Groups},
	components::del_button::DelButton,
};

#[component]
pub fn Swatch(color: Color, idx: ReadSignal<usize>, group_idx: ReadSignal<usize>) -> impl IntoView {
	let set_groups = use_context::<WriteSignal<Groups>>().expect("Unable to find set_groups context");
	let style = format!("background:{}", color.value);

	view! {
		<li class="swatch">
			<div class="swatch_color" style=style />
			<ul class="swatch_label">
				<li class="swatch_label_name">
					{color.name.clone()}
					<DelButton on_click=move |_| {
						let g_idx = group_idx.get_untracked();
						let c_idx = idx.get_untracked();
						set_groups
							.update(move |groups| {
								if let Some(group) = groups.groups.get_mut(g_idx) {
									if c_idx < group.colors.len() {
										group.colors.remove(c_idx);
									}
								}
							});
					}>"Delete"</DelButton>
				</li>
				<li class="swatch_label_value">
					"RGB("{color.value.red}", "{color.value.green}", "{color.value.blue}")"
				</li>
			</ul>
		</li>
	}
}
