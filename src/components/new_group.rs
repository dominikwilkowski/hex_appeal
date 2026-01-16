use leptos::{ev::SubmitEvent, prelude::*};

use crate::{color::group::Groups, color::rgb::Rgb};

#[component]
pub fn NewGroup(group_idx: ReadSignal<usize>) -> impl IntoView {
	let (name, set_name) = signal(String::new());
	let (r, set_r) = signal(String::new());
	let (g, set_g) = signal(String::new());
	let (b, set_b) = signal(String::new());
	let (color, set_color) = signal(String::new());
	let (show_form, set_show_form) = signal(false);

	let set_groups = use_context::<WriteSignal<Groups>>().expect("Unable to find set_groups context");

	let on_submit = move |ev: SubmitEvent| {
		ev.prevent_default();

		let name = name.get();
		let red = r.get().parse::<u8>().unwrap_or(0);
		let green = g.get().parse::<u8>().unwrap_or(0);
		let blue = b.get().parse::<u8>().unwrap_or(0);

		set_groups.update(|all| {
			if let Some(group) = all.groups.get_mut(group_idx.get()) {
				group.add_color(Rgb::new(red, green, blue), name.clone());
			}
		});

		set_name.set(String::new());
		set_r.set(String::new());
		set_g.set(String::new());
		set_b.set(String::new());
		set_color.set(String::new());
	};

	view! {
		<div class="new_group_form">
			<Show
				when=move || { show_form.get() }
				fallback=move || {
					view! { <button class="new_group_btn_open" on:click=move |_| set_show_form.set(true)>"+"</button> }
				}
			>
				<form on:submit=on_submit>
					<button class="new_group_btn_close" on:click=move |_| set_show_form.set(false)>"x"</button>
					<ul>
						<li class="new_swatch_name">
							<label>
								"Name: "
								<input
									type="text"
									required
									prop:value=name
									on:input=move |ev| {
										set_name.set(event_target_value(&ev));
									}
								/>
							</label>
						</li>
						<li class="new_swatch_rgb">
							<label>
								"R: "
								<input
									type="number"
									required
									min="0"
									max="255"
									prop:value=r
									on:input=move |ev| {
										set_r.set(event_target_value(&ev));
									}
								/>
							</label>
						</li>
						<li class="new_swatch_rgb">
							<label>
								"G: "
								<input
									type="number"
									required
									min="0"
									max="255"
									prop:value=g
									on:input=move |ev| {
										set_g.set(event_target_value(&ev));
									}
								/>
							</label>
						</li>
						<li class="new_swatch_rgb">
							<label>
								"B: "
								<input
									type="number"
									required
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
							<label>
								"Color: "
								<input
									type="color"
									prop:value=color
									on:input=move |ev| {
										let value = event_target_value(&ev);
										if let Some(rgb) = Rgb::from_hex(&value) {
											set_r.set(rgb.red.to_string());
											set_g.set(rgb.green.to_string());
											set_b.set(rgb.blue.to_string());
										}
										set_color.set(value);
									}
								/>
							</label>
						</li>
						<li>
							<button type="submit">Add Color</button>
						</li>
					</ul>
				</form>
			</Show>
		</div>
	}
}
