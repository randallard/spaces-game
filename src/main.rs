use leptos::*;
use leptos::prelude::*;
use web_sys::{MouseEvent, Storage, window};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct UserData {
    name: String,
    greeting: String,
}

fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

fn load_user_data() -> Option<UserData> {
    let storage = get_local_storage()?;
    let data = storage.get_item("user_data").ok()??;
    serde_json::from_str(&data).ok()
}

fn save_user_data(name: &str, greeting: &str) -> Result<(), serde_json::Error> {
    if let Some(storage) = get_local_storage() {
        let data = UserData {
            name: name.to_string(),
            greeting: greeting.to_string(),
        };
        let json = serde_json::to_string(&data)?;
        let _ = storage.set_item("user_data", &json);
    }
    Ok(())
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greeting, set_greeting) = signal(String::new());
    let (show_form, set_show_form) = signal(true);

    if let Some(data) = load_user_data() {
        set_name.set(data.name);
        set_greeting.set(data.greeting);
        set_show_form.set(false);
    }

    let handle_submit = move |_: MouseEvent| {
        if !name.get().is_empty() {
            let greeting_text = format!("Hello, {}!", name.get());
            set_greeting.set(greeting_text.clone());
            let _ = save_user_data(&name.get(), &greeting_text);
            set_show_form.set(false);
        }
    };

    let handle_keypress = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !name.get().is_empty() {
            let greeting_text = format!("Hello, {}!", name.get());
            set_greeting.set(greeting_text.clone());
            let _ = save_user_data(&name.get(), &greeting_text);
            set_show_form.set(false);
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
            {move || show_form.get().then(|| view! {
                <>
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
                </>
            })}
            {move || (!show_form.get()).then(|| view! {
                <div class="grid grid-cols-2 gap-8 w-full max-w-4xl px-4">
                    <div>
                        <h2 class="text-2xl font-bold mb-4">"Friends"</h2>
                        <a href="#" class="text-blue-400 hover:text-blue-300 block mb-2">"+ Invite a Friend"</a>
                    </div>
                    <div>
                        <h2 class="text-2xl font-bold mb-4">"Boards"</h2>
                        <a href="#" class="text-blue-400 hover:text-blue-300 block mb-2">"+ Create New Board"</a>
                    </div>
                </div>
            })}
        </div>
    }
}

fn main() {
    mount_to_body(App);
}