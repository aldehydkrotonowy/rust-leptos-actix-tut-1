use crate::components::layouts::Page_Layout;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <Page_Layout />
        <h1>"Welcome to Leptos!!!!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
