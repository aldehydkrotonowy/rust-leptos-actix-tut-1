use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::sections::Counter;
use crate::pages::{Home, NotFound};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-actix-tut-1.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/*any" view=NotFound/>
                    <Route path="/counter" view=Counter/>
                </Routes>
            </main>
        </Router>
    }
}
