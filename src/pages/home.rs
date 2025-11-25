use leptos::{ev::SubmitEvent, prelude::*};

use crate::components::group::Group;

#[derive(Clone)]
pub struct Rgb {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

impl Rgb {
	pub fn from_hex(hex: &str) -> Option<Rgb> {
		let hex = hex.strip_prefix('#')?;
		if hex.len() == 3 {
			let red = u8::from_str_radix(&hex[0..1], 16).ok()?;
			let green = u8::from_str_radix(&hex[1..2], 16).ok()?;
			let blue = u8::from_str_radix(&hex[2..3], 16).ok()?;

			Some(Rgb {
				red: red * 17,
				green: green * 17,
				blue: blue * 17,
			})
		} else if hex.len() == 6 {
			let red = u8::from_str_radix(&hex[0..2], 16).ok()?;
			let green = u8::from_str_radix(&hex[2..4], 16).ok()?;
			let blue = u8::from_str_radix(&hex[4..6], 16).ok()?;

			Some(Rgb { red, green, blue })
		} else {
			None
		}
	}
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

			<main class="container">
				<h1>"Hex Appeal"</h1>
				<ForEnumerate
					each=move || groups.get()
					// TODO: name is not guaranteed to be unique
					key=|group| group.name.clone()
					children=move |idx, _| {
						view! { <Group groups=groups group_idx=idx set_groups=set_groups /> }
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
