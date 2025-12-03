use leptos::prelude::*;

use crate::color::{mode::Mode, rgb::Rgb};

#[component]
pub fn ContrastBadge(color1: Rgb, color2: Rgb) -> impl IntoView {
	let mode = use_context::<ReadSignal<Mode>>().expect("Unable to find mode context");

	let ratio = color1.contrast_ratio(&color2);
	let passes_large_text = move || mode.get().ratio_large_text(ratio);
	let passes_small_text = move || mode.get().ratio_small_text(ratio);
	let is_same_color = color1 == color2;
	let style = format!(
		"color:rgb({},{},{});background-color:rgb({},{},{})",
		color1.red, color1.green, color1.blue, color2.red, color2.green, color2.blue
	);

	view! {
		<div
			class=format!(
				"contrast_badge{}{}",
				if !is_same_color { "" } else { " contrast_badge_same" },
				if !passes_small_text() { " contrast_badge_nope" } else { "" },
			)
			title=format!("Contrast ratio 1:{ratio:.2}")
			style=if is_same_color || !passes_small_text() { String::new() } else { style }
		>
			<Show when=move || !is_same_color>
				<div class="contrast_badge" title=format!("Contrast ratio 1:{ratio:.2}")>
					<div>
						<Show when=move || passes_small_text()>"Aa"</Show>
						<Show when=move || passes_small_text() && !passes_large_text()>
							<span class="contrast_badge_large_text">"⚠️"</span>
						</Show>
					</div>
				</div>
			</Show>
		</div>
	}
}
