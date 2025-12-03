use leptos::{ev::SubmitEvent, prelude::*};

use crate::{color::group::Groups, components::group::Group};

#[component]
pub fn Home() -> impl IntoView {
	let (name, set_name) = signal(String::new());
	let include_default = RwSignal::new(false);
	let groups = use_context::<ReadSignal<Groups>>().expect("Unable to find groups context");
	let set_groups = use_context::<WriteSignal<Groups>>().expect("Unable to find set_groups context");

	let on_submit = move |ev: SubmitEvent| {
		ev.prevent_default();
		set_groups.write().add_group(name.get(), include_default.get());
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
				each=move || groups.get().groups
				key=|group| group.id
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
								required
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
