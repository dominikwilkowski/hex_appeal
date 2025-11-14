use crate::components::counter_btn::Button;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
	view! {
		<ErrorBoundary fallback=|errors| {
			view! {
				<h1>"Uh oh! Something went wrong!"</h1>

				{
					view! {
						if cfg!(debug_assertions)
						{
							view! {
								<h4>"ERRORS"</h4>
								<ul>
									{move || {
										errors
											.get()
											.into_iter()
											.map(|(_, e)| view! { <li>{e.to_string()}</li> })
											.collect_view()
									}}
								</ul>
							}
						}
					}
				}
			}
		}>

			<main class="container">
				<h1>"Welcome to hex appeal"</h1>

				<div class="buttons">
					<Button />
					<Button increment=5 />
				</div>

			</main>
		</ErrorBoundary>
	}
}
