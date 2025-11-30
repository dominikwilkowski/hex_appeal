use leptos::{ev::SubmitEvent, prelude::*};

use crate::{color::group::Group, components::group::Group};

#[component]
pub fn Home() -> impl IntoView {
	let (name, set_name) = signal(String::new());
	let include_default = RwSignal::new(false);
	let groups = use_context::<ReadSignal<Vec<Group>>>().expect("Unable to find groups context");
	let set_groups = use_context::<WriteSignal<Vec<Group>>>().expect("Unable to find set_groups context");

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
					view! { <Group group_idx=idx /> }
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
