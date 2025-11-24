use leptos::{ev::SubmitEvent, prelude::*};

use crate::{
	components::{del_button::DelButton, swatch::Swatch},
	pages::home::{Color, Group, Rgb},
};

#[component]
pub fn Group(
	groups: ReadSignal<Vec<Group>>,
	idx: ReadSignal<usize>,
	set_groups: WriteSignal<Vec<Group>>,
) -> impl IntoView {
	let (name, set_name) = signal(String::new());
	let (r, set_r) = signal(String::new());
	let (g, set_g) = signal(String::new());
	let (b, set_b) = signal(String::new());

	let colors = move || groups.with(|all| all.get(idx.get()).map(|g| g.colors.clone()).unwrap_or_default());
	let group_name = move || groups.with(|all| all.get(idx.get()).map(|g| g.name.clone()).unwrap_or_default());
	let group_included = move || groups.with(|all| all.get(idx.get()).map(|g| g.include_default).unwrap_or_default());

	let on_submit = move |ev: SubmitEvent| {
		ev.prevent_default();

		let name = name.get();
		let r = r.get().parse::<u8>().unwrap_or(0);
		let g = g.get().parse::<u8>().unwrap_or(0);
		let b = b.get().parse::<u8>().unwrap_or(0);

		set_groups.update(|all| {
			if let Some(group) = all.get_mut(idx.get()) {
				group.colors.push(Color {
					name: name.clone(),
					value: Rgb { r, g, b },
				});
			}
		});

		set_name.set(String::new());
		set_r.set(String::new());
		set_g.set(String::new());
		set_b.set(String::new());
	};

	view! {
		<h2>{group_name}</h2>
		<i>{if group_included() { "Included by default" } else { "Not included by default" }}</i>
		<Show when=move || { idx.get() > 0 }>
			<DelButton on_click=move |_| {}>"Delete"</DelButton>
		</Show>
		<ul class="group">
			<ForEnumerate
				each=move || colors()
				// TODO: name is not guaranteed to be unique
				key=|color| color.name.clone()
				children=move |idx, color| {
					view! { <Swatch color=color idx=idx /> }
				}
			/>
		</ul>

		<form class="buttons" on:submit=on_submit>
			<ul>
				<li>
					<label>
						"Name: "
						<input
							type="text"
							prop:value=name
							on:input=move |ev| {
								set_name.set(event_target_value(&ev));
							}
						/>
					</label>
				</li>
				<li>
					<label>
						"R: "
						<input
							type="number"
							min="0"
							max="255"
							prop:value=r
							on:input=move |ev| {
								set_r.set(event_target_value(&ev));
							}
						/>
					</label>
				</li>
				<li>
					<label>
						"G: "
						<input
							type="number"
							min="0"
							max="255"
							prop:value=g
							on:input=move |ev| {
								set_g.set(event_target_value(&ev));
							}
						/>
					</label>
				</li>
				<li>
					<label>
						"B: "
						<input
							type="number"
							min="0"
							max="255"
							prop:value=b
							on:input=move |ev| {
								set_b.set(event_target_value(&ev));
							}
						/>
					</label>
				</li>
				<li>
					<button type="submit">Add Color</button>
				</li>
			</ul>
		</form>
	}
}
