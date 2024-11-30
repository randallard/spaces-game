use leptos::*;
use leptos::prelude::*;
use leptos::logging::log;
use web_sys::{MouseEvent, Storage, window};
use serde::{Serialize, Deserialize};

mod components;
use components::board::BoardCreator;
use components::saved_boards::SavedBoards;
use components::opponent::{
    delete_opponent, Opponent, OpponentType, load_opponents, save_opponent
};

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
    let (show_board_creator, set_show_board_creator) = signal(false);
    let opponent_to_delete = RwSignal::new(None::<Opponent>);
    let opponents_trigger = RwSignal::new(false);
    let opponents = Memo::new(move |_| {
        opponents_trigger.get();
        load_opponents().unwrap_or_default()
    });

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
                    <h2 class="text-2xl font-bold mb-4">"Opponents"</h2>
                    <div class="flex flex-col gap-2">
                        <button
                            class="text-blue-400 hover:text-blue-300 text-left"
                            on:click=move |_| {
                                let opponent = Opponent::new("Random CPU".to_string(), OpponentType::Computer);
                                let _ = save_opponent(opponent);
                                opponents_trigger.update(|v| *v = !*v);
                            }
                        >
                            "+ Add CPU Opponent"
                        </button>
                        <For
                            each=move || opponents.get()
                            key=|opponent| opponent.id.clone()
                            children=move |opponent: Opponent| {
                                view! {
                                    <div class="flex items-center justify-between p-2 bg-slate-800 rounded">
                                    <div class="flex items-center gap-2 text-gray-300">
                                        <span class="w-4 h-4 rounded-full bg-blue-600 flex items-center justify-center text-xs">
                                            {if matches!(opponent.opponent_type, OpponentType::Computer) { "C" } else { "H" }}
                                        </span>
                                        {opponent.name.clone()}
                                    </div>
                                    <div class="flex gap-2">
                                        {
                                            let play_opponent = opponent.clone();
                                            let delete_opponent = opponent.clone();
                                            view! {
                                                <button
                                                    class="px-3 py-1 bg-green-600 hover:bg-green-700 rounded text-sm"
                                                    on:click=move |_| {
                                                        log!("Starting game with {}", play_opponent.name);
                                                    }
                                                >
                                                    "Play"
                                                </button>
                                                <button
                                                    class="text-red-400 hover:text-red-300 opacity-50 hover:opacity-100 transition-opacity"
                                                    on:click=move |_| opponent_to_delete.set(Some(delete_opponent.clone()))
                                                >
                                                    "Remove"
                                                </button>
                                            }
                                        }
                                    </div>
                                </div>                                }
                            }
                        />
                    </div>

                    // Confirmation Dialog
                    {move || opponent_to_delete.get().map(|opponent| view! {
                        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                            <div class="bg-slate-800 p-6 rounded-lg shadow-xl max-w-md w-full mx-4">
                                <h3 class="text-xl font-bold mb-4">"Confirm Removal"</h3>
                                <p class="text-gray-300 mb-6">
                                    "Are you sure you want to remove "
                                    <span class="font-semibold">{opponent.name.clone()}</span>
                                    " from your opponents list?"
                                </p>
                                <div class="flex justify-end gap-4">
                                    <button
                                        class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded"
                                        on:click=move |_| opponent_to_delete.set(None)
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        class="px-4 py-2 bg-red-600 hover:bg-red-700 rounded"
                                        on:click=move |_| {
                                            let _ = delete_opponent(&opponent.id);
                                            opponents_trigger.update(|v| *v = !*v);
                                            opponent_to_delete.set(None);
                                        }
                                    >
                                        "Remove"
                                    </button>
                                </div>
                            </div>
                        </div>
                    })}
                </div>
                    <div>
                        <h2 class="text-2xl font-bold mb-4">"Boards"</h2>
                        <a 
                            href="#" 
                            class="text-blue-400 hover:text-blue-300 block mb-2"
                            on:click=move |ev| {
                                ev.prevent_default();
                                set_show_board_creator.set(true);
                            }
                        >
                            "+ Create New Board"
                        </a>
                        {move || show_board_creator.get().then(|| view! {
                            <BoardCreator 
                                on_cancel=move || set_show_board_creator.set(false)
                            />
                        })}
                        <SavedBoards/>
                    </div>
                </div>
            })}
        </div>
    }
}

fn main() {
    mount_to_body(App);
}