use leptos::{ev::SubmitEvent, prelude::*};

use crate::{color::group::Groups, color::rgb::Rgb};

#[component]
pub fn NewGroup(group_idx: ReadSignal<usize>) -> impl IntoView {
	let (name, set_name) = signal(String::new());
	let (color, set_color) = signal(String::from("#fff"));
	let (swatch, set_swatch) = signal(String::from("#fff"));
	let (show_form, set_show_form) = signal(false);

	let name_input = NodeRef::<leptos::html::Input>::new();

	let set_groups = use_context::<WriteSignal<Groups>>().expect("Unable to find set_groups context");

	// move focus to input field when form is shown
	Effect::new(move |_| {
		if show_form.get() && is_browser() {
			if let Some(el) = name_input.get_untracked() {
				request_animation_frame(move || {
					let _ = el.focus();
				});
			}
		}
	});

	// show swatch color
	Effect::new(move |_| {
		if let Ok(parsed_color) = Rgb::from_import(&color.get()) {
			set_swatch.set(format!("rgb({},{},{})", parsed_color.red, parsed_color.green, parsed_color.blue));
		}
	});

	let on_submit = move |ev: SubmitEvent| {
		ev.prevent_default();

		let name = name.get();
		if let Ok(parsed_color) = Rgb::from_import(&color.get()) {
			set_groups.update(|all| {
				if let Some(group) = all.groups.get_mut(group_idx.get()) {
					group.add_color(parsed_color, name.clone());
				}
			});
		}

		set_name.set(String::new());
		set_color.set(String::from("#fff"));
	};

	view! {
		<div class="new_group_form">
			<Show
				when=move || { show_form.get() }
				fallback=move || {
					view! {
						<button
							class="new_group_btn_open unstyled_btn"
							on:click=move |_| set_show_form.set(true)
						>
							"+"
						</button>
					}
				}
			>
				<form on:submit=on_submit>
					<button
						class="new_group_btn_close unstyled_btn"
						on:click=move |_| set_show_form.set(false)
					>
						"Close"
					</button>
					<ul>
						<li class="new_swatch_color">
							<div
								class="swatch_color"
								style=format!("background-color: {}", swatch.get())
							/>
						</li>
						<li class="new_swatch_name">
							<label>
								"Name: "
								<input
									node_ref=name_input
									type="text"
									required
									prop:value=name
									on:input=move |ev| {
										set_name.set(event_target_value(&ev));
									}
								/>
							</label>
						</li>
						<li class="new_swatch_input">
							<label>
								"Color: "
								<input
									type="text"
									prop:value=color
									on:input=move |ev| {
										set_color.set(event_target_value(&ev));
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
