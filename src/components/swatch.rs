use leptos::prelude::*;

use crate::{
	components::del_button::DelButton,
	pages::home::{Color, Group},
};

#[component]
pub fn Swatch(
	color: Color,
	idx: ReadSignal<usize>,
	group_idx: ReadSignal<usize>,
	set_groups: WriteSignal<Vec<Group>>,
) -> impl IntoView {
	let style = format!("background:rgb({}, {}, {})", color.value.red, color.value.green, color.value.blue,);
	view! {
		<li style=style class="swatch">
			<div class="swatch_label">
				{color.name.clone()}
				<DelButton on_click=move |_| {
					let g_idx = group_idx.get_untracked();
					let c_idx = idx.get_untracked();
					set_groups
						.update(move |groups| {
							if let Some(group) = groups.get_mut(g_idx) {
								if c_idx < group.colors.len() {
									group.colors.remove(c_idx);
								}
							}
						});
				}>"Delete"</DelButton>
			</div>
		</li>
	}
}
