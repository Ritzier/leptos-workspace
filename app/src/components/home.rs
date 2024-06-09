use leptos::html::Html;
use leptos::*;
use leptos_use::{
    use_color_mode_with_options, use_cycle_list_with_options, ColorMode, UseColorModeOptions, UseColorModeReturn, UseCycleListOptions, UseCycleListReturn,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } =  use_color_mode_with_options(
        UseColorModeOpions::
    )

    view! { 
        <h1>"Home"</h1> 
    }
}
