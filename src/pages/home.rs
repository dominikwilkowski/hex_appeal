use leptos::{ev::SubmitEvent, prelude::*};

use crate::{components::group::Group, Color, Group, Rgb};

#[component]
pub fn Home() -> impl IntoView {
	let (name, set_name) = signal(String::new());
	let include_default = RwSignal::new(false);
	let (groups, set_groups) = signal(vec![Group {
		name: String::from("Default"),
		include_default: false,
		colors: vec![
			Color {
				name: String::from("Red"),
				value: Rgb {
					red: 255,
					green: 0,
					blue: 0,
				},
			},
			Color {
				name: String::from("Green"),
				value: Rgb {
					red: 0,
					green: 255,
					blue: 0,
				},
			},
			Color {
				name: String::from("Blue"),
				value: Rgb {
					red: 0,
					green: 0,
					blue: 255,
				},
			},
		],
	}]);

	let on_submit = move |ev: SubmitEvent| {
		ev.prevent_default();

		let name = name.get();
		set_groups.write().push(Group {
			name: name.clone(),
			include_default: include_default.get(),
			colors: Vec::new(),
		});

		set_name.set(String::new());
	};

	view! {
		<ErrorBoundary fallback=|errors| {
			view! {
				<div class="error">
					<h1>"Uh oh! Something went wrong!"</h1>
					{move || {
						cfg!(debug_assertions)
							.then(|| {
								view! {
									<h4>"ERRORS"</h4>
									<ul>
										{errors
											.get()
											.into_iter()
											.map(|(_, e)| view! { <li>{e.to_string()}</li> })
											.collect_view()}
									</ul>
								}
							})
					}}
				</div>
			}
		}>

			<ForEnumerate
				each=move || groups.get()
				// TODO: name is not guaranteed to be unique
				key=|group| group.name.clone()
				children=move |idx, _| {
					view! { <Group groups=groups group_idx=idx set_groups=set_groups /> }
				}
			/>

			<form class="new_group" on:submit=on_submit>
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
							"Always include this group"
							<input type="checkbox" bind:checked=include_default />
						</label>
					</li>
					<li>
						<button type="submit">Add Group</button>
					</li>
				</ul>
			</form>
		</ErrorBoundary>
	}
}
