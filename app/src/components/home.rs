use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="justify-center flex">
            <div class="grid grid-cols-2 w-1/5 gap-4">
                <div class="border bg-gray-300 text-center row-span-2 col-start-2">A</div>
                <div class="border bg-gray-300 text-center">B</div>
                <div class="border bg-gray-300 text-center">C</div>
                <div class="border bg-gray-300 text-center">D</div>
            </div>
        </div>
    }
}
