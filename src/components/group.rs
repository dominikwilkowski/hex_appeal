use leptos::{ev::SubmitEvent, prelude::*};

use crate::{
	components::swatch::Swatch,
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

	let on_submit = move |ev: SubmitEvent| {
		ev.prevent_default();

		let name = name();
		let r = r().parse::<u8>().unwrap_or(0);
		let g = g().parse::<u8>().unwrap_or(0);
		let b = b().parse::<u8>().unwrap_or(0);

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
		<ul class="group">
			<For
				each=move || colors()
				// TODO: name is not guaranteed to be unique
				key=|color| color.name.clone()
				children=move |color| {
					view! { <Swatch color=color /> }
				}
			/>
		</ul>

		<form class="buttons" on:submit=on_submit>
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
			<button type="submit">Add Color</button>
		</form>
	}
}
