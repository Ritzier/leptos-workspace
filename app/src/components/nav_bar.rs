use leptos::*;
use leptos_use::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let UseColorModeReturn { set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .custom_modes(vec![
                "ctp-latte".into(),
                "ctp-frappe".into(),
                "ctp-macchiato".into(),
                "ctp-mocha".into(),
            ])
            .cookie_enabled(true),
    );
    view! {
        <nav class="navbar">
            <a href="/">Home</a>

            <a href="/examples">Example</a>

            <div class="nav-dropdown">
                <button class="nav-dropbtn">ColorScheme</button>
                <div class="nav-dropdown-content">
                    <button on:click=move |_| {
                        set_mode.set(ColorMode::Custom("ctp-latte".into()))
                    }>Latte</button>

                    <button on:click=move |_| {
                        set_mode.set(ColorMode::Custom("ctp-frappe".into()))
                    }>Frappe</button>
                    <button on:click=move |_| {
                        set_mode.set(ColorMode::Custom("ctp-macchiato".into()))
                    }>

                        Macchiato
                    </button>
                    <button on:click=move |_| {
                        set_mode.set(ColorMode::Custom("ctp-mocha".into()))
                    }>Mocha</button>
                </div>
            </div>

            <div class="navbar-dropdown">
                <span href="">A</span>
                <div class="navbar-dropdown-content">
                    <div>A</div>
                    <div>A</div>
                    <div>A</div>
                </div>
            </div>

        </nav>
    }
}
