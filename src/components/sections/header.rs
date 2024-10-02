use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="flex justify-between h-12gi bg-slate-400">
            <div class="justify-center align-middle bg-emerald-400">
                <img
                    class="h-10"
                    src="assets/dna-logo.svg"
                    alt="DNA logo svg"
                />
            </div>
            <div class="flex flex-row gap-4 items-stretch w-auto bg-fuchsia-400">
                <div class="bg-green-300">hello 1</div>
                <div class="bg-green-300">hello 2</div>
                <div class="bg-green-300">hello 3</div>
                <div class="bg-green-300">hello 2</div>
                <div class="bg-green-300">hello 3</div>
                <h1 class="ribbon">"HELLO THERE MY FRIEND"</h1>
                <div class="inline-block p-10">
                    <details class="relative before:hidden before:open:block">
                        <summary role="button" class="list-none cursor-pointer">
                            <img
                                class="w-10 h-10"
                                src="data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNTEyIiBoZWlnaHQ9IjUxMiIgdmlld0JveD0iMCAwIDUxMiA1MTIiIHN0eWxlPSJjb2xvcjojMjhBRTlCIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGNsYXNzPSJoLWZ1bGwgdy1mdWxsIj48cmVjdCB3aWR0aD0iNTEyIiBoZWlnaHQ9IjUxMiIgeD0iMCIgeT0iMCIgcng9IjMwIiBmaWxsPSJ0cmFuc3BhcmVudCIgc3Ryb2tlPSJ0cmFuc3BhcmVudCIgc3Ryb2tlLXdpZHRoPSIwIiBzdHJva2Utb3BhY2l0eT0iMTAwJSIgcGFpbnQtb3JkZXI9InN0cm9rZSI+PC9yZWN0Pjxzdmcgd2lkdGg9IjUxMnB4IiBoZWlnaHQ9IjUxMnB4IiB2aWV3Qm94PSIwIDAgMzIgMzIiIGZpbGw9IiMyOEFFOUIiIHg9IjAiIHk9IjAiIHJvbGU9ImltZyIgc3R5bGU9ImRpc3BsYXk6aW5saW5lLWJsb2NrO3ZlcnRpY2FsLWFsaWduOm1pZGRsZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj48ZyBmaWxsPSIjMjhBRTlCIj48cGF0aCBmaWxsPSJjdXJyZW50Q29sb3IiIGQ9Ik0xNiA4YTUgNSAwIDEgMCA1IDVhNSA1IDAgMCAwLTUtNVoiPjwvcGF0aD48cGF0aCBmaWxsPSJjdXJyZW50Q29sb3IiIGQ9Ik0xNiAyYTE0IDE0IDAgMSAwIDE0IDE0QTE0LjAxNiAxNC4wMTYgMCAwIDAgMTYgMlptNy45OTIgMjIuOTI2QTUuMDAyIDUuMDAyIDAgMCAwIDE5IDIwaC02YTUuMDAyIDUuMDAyIDAgMCAwLTQuOTkyIDQuOTI2YTEyIDEyIDAgMSAxIDE1Ljk4NSAwWiI+PC9wYXRoPjwvZz48L3N2Zz48L3N2Zz4="
                            />
                        </summary>
                        // dropdown menu https://codepen.io/sinfullycoded/pen/MWLRgPj
                        <ul class="absolute right-2 p-0 min-w-max bg-white rounded-md shadow">
                            <li class="p-2 bg-slate-100">
                                <span class="block font-semibold">username</span>
                                <span class="block font-light">email address</span>
                            </li>
                            <li class="p-2 hover:bg-sky-100">
                                <a href="#">account</a>
                            </li>
                            <li class="p-2 hover:bg-sky-100">
                                <a href="#">settings</a>
                            </li>
                            <li class="p-2 hover:bg-sky-100">
                                <a href="#">logout</a>
                            </li>
                        </ul>
                    </details>
                </div>

            </div>

        </header>
    }
}
