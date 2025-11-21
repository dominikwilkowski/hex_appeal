use leptos::{ev::SubmitEvent, prelude::*};

use crate::components::group::Group;

#[derive(Clone)]
pub struct Rgb {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

#[derive(Clone)]
pub struct Color {
	pub name: String,
	pub value: Rgb,
}

#[derive(Clone)]
pub struct Group {
	pub name: String,
	pub include_default: bool,
	pub colors: Vec<Color>,
}

#[component]
pub fn Home() -> impl IntoView {
	let (name, set_name) = signal(String::new());
	let include_default = RwSignal::new(false);
	let (groups, set_groups) = signal(vec![Group {
		name: String::from("Default"),
		include_default: false,
		colors: Vec::new(),
	}]);

	let on_submit = move |ev: SubmitEvent| {
		ev.prevent_default();

		let name = name();
		set_groups.write().push(Group {
			name: name.clone(),
			include_default: include_default(),
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

			<main class="container">
				<h1>"Hex Appeal"</h1>
				<ForEnumerate
					each=move || groups()
					// TODO: name is not guaranteed to be unique
					key=|group| group.name.clone()
					children=move |idx, _| {
						view! { <Group groups=groups idx=idx set_groups=set_groups /> }
					}
				/>

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
						"Always include this group"
						<input type="checkbox" bind:checked=include_default />
					</label>
					<button type="submit">Add Group</button>
				</form>

			</main>
		</ErrorBoundary>
	}
}
