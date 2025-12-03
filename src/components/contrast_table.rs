use leptos::prelude::*;

use crate::color::group::{Color, Groups};

#[component]
pub fn ContrastTable(group_idx: ReadSignal<usize>) -> impl IntoView {
	let groups = use_context::<ReadSignal<Groups>>().expect("Unable to find groups context");

	let group_name =
		move || groups.with(|all| all.groups.get(group_idx.get()).map(|g| g.name.clone()).unwrap_or_default());

	let colors = Memo::new(move |_| {
		let selected_idx = group_idx.get();

		groups.with(|groups_state| {
			groups_state
				.groups
				.iter()
				.enumerate()
				.filter(|(idx, group)| *idx == selected_idx || group.include_default)
				.flat_map(|(_, group)| group.colors.iter().cloned())
				.collect::<Vec<Color>>()
		})
	});

	view! {
		<table class="contrast_table">
			<caption>"Contrast comparison of "<strong>{group_name()}</strong></caption>
			<thead>
				<tr>
					<th />
					<For
						each=move || colors.get()
						key=|color| color.id
						children=move |color: Color| {
							view! { <th>{color.name}</th> }
						}
					/>
				</tr>
			</thead>
			<tbody>
				<For
					each=move || colors.get()
					key=|color| color.id
					children=move |row_color| {
						let colors = colors;

						view! {
							<tr>
								<th>{row_color.name.clone()}</th>

								<For
									each=move || colors.get()
									key=|col_color| col_color.id
									children=move |col_color| {
										let value = row_color.value.clone();
										let ratio = row_color
											.value
											.contrast_ratio(&col_color.value);
										view! {
											<td>
												<Show
													when=move || { value != col_color.value }
													fallback=|| view! { "-" }
												>
													{format!("{ratio:.2}")}
												</Show>
											</td>
										}
									}
								/>
							</tr>
						}
					}
				/>
			</tbody>
		</table>
	}
}
