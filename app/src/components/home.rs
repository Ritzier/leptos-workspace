use leptos::*;
use leptos_use::*;

#[component]
pub fn HomePage() -> impl IntoView {
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
        <div class="App min-h-screen grid">
            <main class="flex flex-col justify-center items-center bg-gradient-to-b from-ctp-base to-ctp-crust p-6">
                <div class="flex mb-6 px-4 py-2 rounded-xl text-ctp-text">
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

                <div id="card" class="from-ctp-mantle to-ctp-crust outline-ctp-pink">
                    <h1 class="from-ctp-pink to-ctp-mauve">Catppuccin</h1>
                    <p class="text-ctp-subtext0">Soothing pastel theme for TailwindCSS</p>
                    <div id="palette">
                        <div class="bg-ctp-rosewater"></div>
                        <div class="bg-ctp-flamingo"></div>
                        <div class="bg-ctp-pink"></div>
                        <div class="bg-ctp-mauve"></div>
                        <div class="bg-ctp-red"></div>
                        <div class="bg-ctp-maroon"></div>
                        <div class="bg-ctp-peach"></div>
                        <div class="bg-ctp-yellow"></div>
                        <div class="bg-ctp-green"></div>
                        <div class="bg-ctp-teal"></div>
                        <div class="bg-ctp-sky"></div>
                        <div class="bg-ctp-sapphire"></div>
                        <div class="bg-ctp-blue"></div>
                        <div class="bg-ctp-lavender"></div>
                    </div>
                    <a href="https://github.com/nekowinston/catppuccin-tailwindcss">
                        <button class="bg-ctp-lavender hover:bg-ctp-mauve active:bg-ctp-mauve/75">
                            Install!
                        </button>
                    </a>
                </div>
            </main>
        </div>
    }
}
