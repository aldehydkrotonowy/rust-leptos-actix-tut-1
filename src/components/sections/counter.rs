use leptos::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let click = move |_| set_count.update(|count| *count += 1);
    let red_class = move || count() % 2 == 1;

    view! {
      <button on:click=click class:red=red_class>"Test button: "{count}</button>
    }
}
