use leptos::{ev::MouseEvent, prelude::*};

use std::time::Duration;

#[component]
pub fn DelButton(mut on_click: impl FnMut(MouseEvent) + 'static, children: ChildrenFn) -> impl IntoView {
	let (double, set_double) = signal(false);

	view! {
		<button on:click=move |event| {
			if double.get() {
				on_click(event);
				set_double.set(false);
			} else {
				set_double.set(true);
				set_timeout(
					move || {
						if double.get() {
							set_double.set(false);
						}
					},
					Duration::from_millis(2_000),
				);
			}
		}>
			<Show when=move || { !double.get() } fallback=|| view! { "Really?" }>
				{children()}
			</Show>
		</button>
	}
}
