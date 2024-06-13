use leptos::*;
use leptos_use::*;

#[component]
pub fn NavBar() -> impl IntoView {
    let UseColorModeReturn { set_mode, mode, .. } = use_color_mode_with_options(
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
            <div class="dropdown">
                <button class="dropbtn">ColorScheme <i class="fa fa-caret-down"></i></button>

                <div class="dropdown-content">
                    <button
                        on:click=move |_| { set_mode.set(ColorMode::Custom("ctp-latte".into())) }

                        class="navbutton-left bg-ctp-overlay0/50"
                    >
                        Latte
                    </button>

                    <button
                        on:click=move |_| { set_mode.set(ColorMode::Custom("ctp-frappe".into())) }

                        class="navbutton-center bg-ctp-overlay0/50"
                    >
                        Frappe
                    </button>
                    <button
                        on:click=move |_| {
                            set_mode.set(ColorMode::Custom("ctp-macchiato".into()))
                        }

                        class="navbutton-center bg-ctp-overlay0/50"
                    >
                        Macchiato
                    </button>
                    <button
                        on:click=move |_| { set_mode.set(ColorMode::Custom("ctp-mocha".into())) }

                        class="navbutton-right bg-ctp-overlay0/50"
                    >
                        Mocha
                    </button>
                </div>
            </div>
        </nav>
    }
}
