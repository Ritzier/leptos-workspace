use leptos::*;
use leptos_use::{storage::{use_local_storage_with_options, UseStorageOptions}, utils::FromToStringCodec};

#[component]
pub fn Demo1Page() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count, _) = use_local_storage_with_options::<i32, FromToStringCodec>(
        "count-state",
        UseStorageOptions::default()
    );
    // set_count += 1
    let on_click_add = move |_| set_count.update(|n| *n+=1);
    // set_count -= 1
    let on_click_min = move |_| set_count.update(|n| *n-=1);
    // set_count = 0
    let on_click_reset = move |_| set_count.update(|n| *n=0);
    view! {
        <div class="bg-ctp-latte text-ctp-mocha">
            <p>{count}</p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=on_click_min
            >
                "-1"
            </button>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=on_click_reset
            >
                "Reset"
            </button>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=on_click_add
            >
                "+1"
            </button>
        </div>
    }
}
