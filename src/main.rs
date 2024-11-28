use leptos::*;
use leptos::prelude::*;
use web_sys::MouseEvent;

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greeting, set_greeting) = signal(String::new());

    let handle_submit = move |_: MouseEvent| {
        if !name.get().is_empty() {
            set_greeting.set(format!("Hello, {}!", name.get()));
        }
    };

    let handle_keypress = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !name.get().is_empty() {
            set_greeting.set(format!("Hello, {}!", name.get()));
        }
    };

    view! {
        <div class="min-h-screen bg-slate-900 text-white flex flex-col items-center justify-center gap-4">
            <h1 class="text-4xl font-bold">
                {move || if greeting.get().is_empty() {
                    "Hi! What's your name?".to_string()
                } else {
                    greeting.get()
                }}
            </h1>
            <input
                type="text"
                class="px-4 py-2 rounded bg-slate-800 border border-slate-700"
                on:input=move |ev| {
                    set_name.set(event_target_value(&ev));
                }
                on:keypress=handle_keypress
                prop:value=name
            />
            <button
                class="px-4 py-2 bg-blue-600 rounded hover:bg-blue-700"
                on:click=handle_submit
            >
                "Hello"
            </button>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}