use crate::components::sections::{Counter, Header};
use leptos::*;

#[component]
pub fn Page_Layout() -> impl IntoView {
    view! {
        <Header />
        <div>
            <Counter />
        </div>
        <div>footer</div>
    }
}
