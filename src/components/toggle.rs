use leptos::{ev::Event, prelude::*};

#[component]
pub fn Toggle(
	checked: impl Fn() -> bool + 'static + std::marker::Send,
	on_change: impl FnMut(Event) + 'static,
	#[prop(optional)] class: &'static str,
) -> impl IntoView {
	view! {
		<div class=format!("toggle {class}")>
			<label>
				<input type="checkbox" prop:checked=checked on:change=on_change />
				<span class="toggle_label" />
			</label>
		</div>
	}
}
