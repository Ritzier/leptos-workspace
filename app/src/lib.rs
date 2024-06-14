use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Body class="h-screen bg-cpt-base text-cpt-text"/>

        // <Body class="bg-ctp-base"/>
        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>

            // <NavBar/>
            // <NavBar/>
            <NavBar/>

            <Routes>
                // <Route path="" view=HomePage/>
                <Route path="/" view=HomePage/>

                <Route path="/examples" view=ExamplePage/>

                <Route path="/catppuccin" view=CatppuccinPage/>

            </Routes>
        </Router>
    }
}
