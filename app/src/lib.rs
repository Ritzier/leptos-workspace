use crate::error_template::{AppError, ErrorTemplate};
use components::{HomePage, CounterPage};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;
mod components;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    // <Route path="" view=HomePage/>
                    <Route path="" view=HomePage/>
                    <Route path="/home" view=CounterPage/>
                </Routes>
            </main>

        </Router>
    }
}


