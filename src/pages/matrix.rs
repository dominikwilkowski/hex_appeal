use leptos::prelude::*;

use crate::color::group::{Color, Groups};

#[component]
pub fn Matrix() -> impl IntoView {
	let groups = use_context::<ReadSignal<Groups>>().expect("Unable to find groups context");

	let colors = Memo::new(move |_| {
		groups.with(|groups| {
			groups
				.groups
				.iter()
				.filter(|group| group.include_default)
				.flat_map(|group| group.colors.iter().cloned())
				.collect::<Vec<Color>>()
		})
	});

	view! {
		<table class="contrast_table">
			<caption>"Contrast comparison of each color pairs"</caption>
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
