use leptos::*;

#[component]
pub fn Todo_Wrapper(children: Children) -> impl IntoView {
    view! {
        <div class="h-screen">
            <div class="px-5 pt-20 mx-auto mb-10 w-full lg:px-0 lg:max-w-[90ch]">
                {children()}
            </div>
        </div>
    }
}
