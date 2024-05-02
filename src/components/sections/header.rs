use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex justify-between h-40 bg-slate-400">
            <div class=" bg-slate-900">
                <img class="h-40" src="assets/dna-logo.svg" alt="DNA logo svg"/>
            </div>
            <div class="flex flex-row items-stretch gap-4 bg-fuchsia-400 w-60">
                <div class="bg-green-300">hello 1</div>
                <div class="bg-green-300">hello 2</div>
                <div class="bg-green-300">hello 3</div>
                <h1 class="ribbon">"HELLO THERE MY FRIEND"</h1>
            </div>

        </header>
    }
}
