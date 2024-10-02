use leptos::*;

#[component]
fn ButtonGeneric(label: String) -> impl IntoView {
    view! {
        <button class="py-3 px-4 font-bold text-center text-white bg-green-500 rounded-lg">
            {label}
        </button>
    }
}

#[component]
pub fn ButtonOK() -> impl IntoView {
    view! { <ButtonGeneric label="OK".to_string() /> }
}

#[component]
pub fn ButtonCancel() -> impl IntoView {
    view! { <ButtonGeneric label="Cancel".to_string() /> }
}
