use leptos::*;
use leptos_router::*;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <h1>"Examples"</h1>
        <ul>
            <li>
                <A href="/catppuccin">Catppuccin</A>
            </li>
        </ul>
    }
}
