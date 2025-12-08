use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Header() -> impl IntoView {
	view! {
		<header>
			<h1>"Hex Appeal"</h1>
			<nav>
				<ul>
					<li>
						<A href="/">Home</A>
					</li>
					<li>
						<A href="/matrix">Matrix</A>
					</li>
				</ul>
			</nav>
		</header>
	}
}
