use leptos::{ev::MouseEvent, prelude::*};

use std::time::Duration;

#[component]
pub fn DelButton(mut on_click: impl FnMut(MouseEvent) + 'static, children: ChildrenFn) -> impl IntoView {
	let (double, set_double) = signal(false);
	let (handle, set_handle) = signal::<Option<TimeoutHandle>>(None);

	on_cleanup({
		move || {
			if let Some(timeout_handle) = handle.get_untracked() {
				timeout_handle.clear();
			}
		}
	});

	view! {
		<button on:click=move |event| {
			if double.get() {
				if let Some(timeout_handle) = handle.get_untracked() {
					timeout_handle.clear();
					set_handle.set(None);
				}
				on_click(event);
				set_double.set(false);
			} else {
				set_double.set(true);
				if let Some(timeout_handle) = handle.get_untracked() {
					timeout_handle.clear();
				}
				if let Ok(timeout_handle) = set_timeout_with_handle(
					move || {
						if double.get() {
							set_double.set(false);
						}
					},
					Duration::from_millis(2_000),
				) {
					set_handle.set(Some(timeout_handle));
				}
			}
		}>
			<Show when=move || { !double.get() } fallback=|| view! { "Really?" }>
				{children()}
			</Show>
		</button>
	}
}
