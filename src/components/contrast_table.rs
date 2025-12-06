use leptos::prelude::*;

use crate::{
	color::group::{Color, Groups},
	components::contrast_badge::ContrastBadge,
};

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
			<caption>"Contrast comparison of "<strong>{move || group_name()}</strong></caption>
			<colgroup>
				<col class="col-label" />
				<col class="col-value" span=move || colors.with(|colors_vec| colors_vec.len()) />
			</colgroup>
			<thead>
				<tr>
					<th />
					{move || {
						colors
							.with(|colors_vec| {
								colors_vec
									.iter()
									.map(|color| view! { <th>{color.name.clone()}</th> })
									.collect_view()
							})
					}}
				</tr>
			</thead>
			<tbody>
				{move || {
					colors
						.with(|colors_vec| {
							colors_vec
								.iter()
								.map(|row_color| {
									let row_name = row_color.name.clone();

									view! {
										<tr>
											<th>{row_name}</th>
											{colors_vec
												.iter()
												.map(|col_color| {
													view! {
														<td>
															<ContrastBadge
																color1=row_color.value
																color2=col_color.value
															/>
														</td>
													}
												})
												.collect_view()}
										</tr>
									}
								})
								.collect_view()
						})
				}}
			</tbody>
		</table>
	}
}
