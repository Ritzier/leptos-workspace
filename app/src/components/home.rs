use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <nav>
            <ul class="flex flex-row gap-x-3 gap-y-1 flex-wrap">
                <li class="hover:underline">
                    <a href="/">
                        <span class="text-black">Home</span>
                    </a>
                </li>

                <li class="hover:underline">
                    <a href="/">
                        <span class="text-black">Example</span>
                    </a>
                </li>

                <li class="hover:underline">
                    <a href="/">
                        <span class="text-black">About</span>
                    </a>
                </li>
            </ul>
        </nav>
    }
}
