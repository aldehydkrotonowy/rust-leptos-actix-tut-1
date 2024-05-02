use leptos::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (countMe, set_countMe) = create_signal(0);

    let click = move |_| set_countMe.update(|countMe| *countMe += 1);
    let red_class = move || countMe() % 2 == 1;

    view! {
        <button on:click=click class:red=red_class>
            "Test button: "
            {countMe}
        </button>
    }
}
