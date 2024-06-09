use leptos::ev::{KeyboardEvent, keypress};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::{use_local_storage, use_local_storage_with_options, UseStorageOptions};
use leptos_use::utils::FromToStringCodec;
use leptos_use::{
    use_color_mode_with_options, use_cookie_with_options, use_debounce_fn, use_event_listener,
    use_interval, use_intl_number_format, use_preferred_dark, use_timestamp, use_window, ColorMode,
    UseColorModeOptions, UseColorModeReturn, UseCookieOptions, UseIntervalReturn,
    UseIntlNumberFormatOptions,
};

#[component]
pub fn Demo1Page() -> impl IntoView {
    let (count, set_count, _) = use_local_storage_with_options::<i32, FromToStringCodec>(
        "count-state",
        UseStorageOptions::default()
    );

    let on_click = move |_|set_count.update(|count|*count += 1);

    let nf = use_intl_number_format(
        UseIntlNumberFormatOptions::default().locale("zh-Hans-CN-u-nu-hanidec"),
    );

    let zh_count = nf.format::<i32>(count);

    let (key, set_key) = create_signal("".to_string());

    // window() doesn't work on the server so we provde use use_window()
    let _ = use_event_listener(
        use_window(),
        keypress,
        move |evt: KeyboardEvent| { set_key(evt.key()) }
    );

    let (debounce_value, set_debounce_value) = create_signal("not called");

    let debounced_fn = use_debounce_fn(
        move || {
            set_debounce_value("called");
        },
        2000.0,
    );
    debounced_fn();

    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().cookie_enabled(true));

    let timestamp = use_timestamp();

    let is_dark_preferred = use_preferred_dark();

    let (test_cookie, _) = use_cookie_with_options::<String, FromToStringCodec>(
        "test-cookie",
        UseCookieOptions::<String, _>::default()
            .max_age(3000)
            .default_value(Some("Bogus string".to_owned())),
    );

        //<Html class=move || mode.get().to_string()/>
    view! {
        <h1>Leptos-Use SSR Example</h1>

        <button on:click=on_click>Click Me: {count}</button>
        <p>Local zh-Hans-CN-u-nu-hanidec: {zh_count}</p>
        <p>Press any key: {key}</p>
        <p>Debounced called: {debounce_value}</p>
        <p>Color mode: {move || format!("{:?}", mode.get())}</p>
        <button on:click=move |_| set_mode.set(ColorMode::Light)>Change to Light</button>
        <button on:click=move |_| set_mode.set(ColorMode::Dark)>Change to Dark</button>
        <button on:click=move |_| set_mode.set(ColorMode::Auto)>Change to Auto</button>
        <p>{timestamp}</p>
        <p>Dark preferred: {is_dark_preferred}</p>
        <LocalStorageTest/>
        <p>Test cookie: {move || test_cookie().unwrap_or("<Expired>".to_string())}</p>
    }
}

#[component]
fn LocalStorageTest() -> impl IntoView {
    let UseIntervalReturn { counter, ..  } = use_interval(1_000);
    let (state, set_state, ..) = use_local_storage::<String, FromToStringCodec>("test-state");

    view! {
        <p>{counter}</p>
        <input
            class="block"
            prop:value=move || state.get()
            on:input=move |e| set_state.update(|s| *s = event_target_value(&e))
            type="text"
        />
    }
}
