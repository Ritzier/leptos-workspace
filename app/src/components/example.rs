use leptos::*;
use leptos_use::*;

#[component]
pub fn DemoPage() -> impl IntoView {
    let UseMouseReturn { x, y, .. } = use_mouse();

    view! {
        {x}
        " x "
        {y}
    }
}
