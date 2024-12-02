use leptos::*;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::{MouseEvent, Storage, window};
use serde::{Serialize, Deserialize};

mod components;
use components::board::BoardCreator;
use components::game::{Game, GameSpeed, GameState};
use components::saved_boards::SavedBoards;
use components::opponent::{
    delete_opponent, Opponent, OpponentType, load_opponents, save_opponent
};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UserData {
    name: String,
    greeting: String,
    default_game_speed: GameSpeed,
}

fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

fn load_user_data() -> Option<UserData> {
    let storage = get_local_storage()?;
    let data = match storage.get_item("user_data") {
        Ok(Some(data)) => data,
        Ok(None) => {
            web_sys::console::log_1(&"No user data found in storage".into());
            return None;
        }
        Err(e) => {
            web_sys::console::log_1(&format!("Error loading user data: {:?}", e).into());
            return None;
        }
    };
    
    match serde_json::from_str(&data) {
        Ok(user_data) => Some(user_data),
        Err(e) => {
            web_sys::console::log_1(&format!("Error parsing user data: {:?}", e).into());
            None
        }
    }
}

fn save_user_data(name: &str, greeting: &str, speed: GameSpeed) -> Result<(), serde_json::Error> {
    if let Some(storage) = get_local_storage() {
        let data = UserData {
            name: name.to_string(),
            greeting: greeting.to_string(),
            default_game_speed: speed,
        };
        let json = serde_json::to_string(&data)?;
        storage.set_item("user_data", &json).unwrap_or_else(|e| {
            web_sys::console::log_1(&format!("Failed to save to storage: {:?}", e).into());
        });
    } else {
        web_sys::console::log_1(&"No local storage available".into());
    }
    Ok(())
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greeting, set_greeting) = signal(String::new());
    let (show_form, set_show_form) = signal(true);
    let (show_profile, set_show_profile) = signal(false);
    let (default_game_speed, set_default_game_speed) = signal(GameSpeed::Quick);
    let (show_game, set_show_game) = signal(None::<(Opponent, GameSpeed)>);
    let (show_board_creator, set_show_board_creator) = signal(false);
    let opponent_to_delete = RwSignal::new(None::<Opponent>);
    let opponents_trigger = RwSignal::new(false);
    let opponents = Memo::new(move |_| {
        opponents_trigger.get();
        load_opponents().unwrap_or_default()
    });

    let cpu_opponent = Opponent::new("CPU".to_string(), OpponentType::Computer);
    let _ = save_opponent(cpu_opponent);

    if let Some(data) = load_user_data() {
        set_name.set(data.name);
        set_greeting.set(data.greeting);
        set_default_game_speed.set(data.default_game_speed); // Add this line
        set_show_form.set(false);
    }

    let handle_submit = move |_: MouseEvent| {
        if !name.get().is_empty() {
            let greeting_text = format!("Hello, {}!", name.get());
            set_greeting.set(greeting_text.clone());
            let _ = save_user_data(&name.get(), &greeting_text, GameSpeed::Relaxed);
            set_show_form.set(false);
        }
    };

    let handle_keypress = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" && !name.get().is_empty() {
            let greeting_text = format!("Hello, {}!", name.get());
            set_greeting.set(greeting_text.clone());
            let _ = save_user_data(&name.get(), &greeting_text, GameSpeed::Relaxed);
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
            {move || (!show_form.get()).then(|| view! {
                <button
                    class="text-blue-400 hover:text-blue-300 text-sm mb-4"
                    on:click=move |_| set_show_profile.set(true)
                >
                    "Edit Profile"
                </button>
            })}
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
                                            let delete_opponent = opponent.clone();
                                            view! {
                                                <div class="flex gap-1">
                                                {
                                                    let opponent_lightning = opponent.clone();
                                                    let opponent_quick = opponent.clone();
                                                    let opponent_relaxed = opponent.clone();
                                                    let opponent_chill = opponent.clone();
                                                    view! {
                                                        <button
                                                            class="px-3 py-1 bg-green-600 hover:bg-green-700 rounded-l text-sm"
                                                            on:click=move |_| set_show_game.set(Some((opponent.clone(), default_game_speed.get())))
                                                        >
                                                            "Play\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}\u{00A0}"
                                                        </button>
                                                        <div class="relative">
                                                            <button
                                                                class="px-2 py-1 bg-green-600 hover:bg-green-700 rounded-r text-sm border-l border-green-700"
                                                                on:click=move |ev| {
                                                                    ev.prevent_default();
                                                                    let target = ev.target().unwrap();
                                                                    let element = target.dyn_into::<web_sys::Element>().unwrap();
                                                                    if let Some(next_sibling) = element.next_sibling() {
                                                                        next_sibling.dyn_ref::<web_sys::HtmlElement>()
                                                                            .unwrap()
                                                                            .style()
                                                                            .set_property("display", "block")
                                                                            .unwrap();
                                                                    }
                                                                }
                                                            >
                                                                "▾"
                                                            </button>
                                                            <div 
                                                                class="absolute hidden right-0 bg-green-600 rounded-b shadow-lg z-10"
                                                                on:mouseleave=move |ev| {
                                                                    ev.target().unwrap().dyn_ref::<web_sys::HtmlElement>()
                                                                        .unwrap()
                                                                        .style()
                                                                        .set_property("display", "none")
                                                                        .unwrap();
                                                                }
                                                            >

                                                            
                                                                    <button
                                                                        class="block w-full text-left px-3 py-1 hover:bg-green-700 text-sm border-t border-green-700"
                                                                        on:click=move |_| {
                                                                            set_show_game.set(Some((opponent_lightning.clone(), GameSpeed::Lightning)));
                                                                        }
                                                                    >
                                                                        "Lightning!\u{00A0}(1s\u{00A0}to\u{00A0}choose)"
                                                                    </button>
                                                                    <button
                                                                        class="block w-full text-left px-3 py-1 hover:bg-green-700 text-sm border-t border-green-700"
                                                                        on:click=move |_| {
                                                                            set_show_game.set(Some((opponent_quick.clone(), GameSpeed::Quick)));
                                                                        }
                                                                    >
                                                                        "Quick!\u{00A0}(5s\u{00A0}to\u{00A0}choose)"
                                                                    </button>
                                                                    <button
                                                                        class="block w-full text-left px-3 py-1 hover:bg-green-700 text-sm border-t border-green-700"
                                                                        on:click=move |_| {
                                                                            set_show_game.set(Some((opponent_relaxed.clone(),GameSpeed::Relaxed)));
                                                                        }
                                                                    >
                                                                        "Relaxed\u{00A0}(10s\u{00A0}to\u{00A0}choose)"
                                                                    </button>
                                                                    <button
                                                                        class="block w-full text-left px-3 py-1 hover:bg-green-700 text-sm border-t border-green-700 rounded-b"  // Added rounded-b
                                                                        on:click=move |_| {
                                                                            set_show_game.set(Some((opponent_chill.clone(),GameSpeed::Chill)));
                                                                        }
                                                                    >
                                                                        "Totally\u{00A0}Chill\u{00A0}(no\u{00A0}limit)"
                                                                    </button>
                                                                </div>
                                                        </div>
                                                    }
                                                }
                                            </div>
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
                        {move || (!show_board_creator.get()).then(|| view! {
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
                        })}
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
        {move || show_game.get().map(|(opponent, speed)| view! {  // Change this line to destructure both values
            <Game
                player_name=name.get()
                opponent=opponent
                speed=speed  // Add this line
                on_exit=move || set_show_game.set(None)
            />
        })}
        {move || show_profile.get().then(|| view! {
            <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                <div class="bg-slate-800 p-6 rounded-lg shadow-xl max-w-md w-full mx-4 text-white">
                    <h3 class="text-xl font-bold mb-4">"Edit Profile"</h3>
                    <div class="flex flex-col gap-4">
                        <div>
                            <label class="block text-sm font-medium mb-1">
                                "Username"
                            </label>
                            <input
                                type="text"
                                class="w-full px-4 py-2 rounded bg-slate-700 border border-slate-600 text-white"
                                prop:value=name
                                on:input=move |ev| set_name.set(event_target_value(&ev))
                            />
                        </div>
                        <div>
                            <label class="block text-sm font-medium mb-1">
                                "Default Game Speed"
                            </label>
                            <select
                                class="w-full px-4 py-2 rounded bg-slate-700 border border-slate-600 text-white"
                                on:change=move |ev| {
                                    match event_target_value(&ev).as_str() {
                                        "lightning" => set_default_game_speed.set(GameSpeed::Lightning),
                                        "quick" => set_default_game_speed.set(GameSpeed::Quick),
                                        "relaxed" => set_default_game_speed.set(GameSpeed::Relaxed),
                                        "chill" => set_default_game_speed.set(GameSpeed::Chill),
                                        _ => (),
                                    }
                                }
                            >
                                <option 
                                    value="lightning" 
                                    selected=move || matches!(default_game_speed.get(), GameSpeed::Lightning)
                                    class="text-white bg-slate-700"
                                >
                                    "Lightning! (1s to choose)"
                                </option>
                                <option 
                                    value="quick"
                                    selected=move || matches!(default_game_speed.get(), GameSpeed::Quick)
                                    class="text-white bg-slate-700"
                                >
                                    "Quick! (5s to choose)"
                                </option>
                                <option 
                                    value="relaxed"
                                    selected=move || matches!(default_game_speed.get(), GameSpeed::Relaxed)
                                    class="text-white bg-slate-700"
                                >
                                    "Relaxed (10s to choose)"
                                </option>
                                <option 
                                    value="chill"
                                    selected=move || matches!(default_game_speed.get(), GameSpeed::Chill)
                                    class="text-white bg-slate-700"
                                >
                                    "Totally Chill (no limit)"
                                </option>
                            </select>
                        </div>
                        <div class="flex justify-end gap-4 mt-2">
                            <button
                                class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded"
                                on:click=move |_| set_show_profile.set(false)
                            >
                                "Cancel"
                            </button>
                            <button
                                class="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded"
                                on:click=move |_| {
                                    let greeting_text = format!("Hello, {}!", name.get());
                                    set_greeting.set(greeting_text.clone());
                                    let _ = save_user_data(&name.get(), &greeting_text, default_game_speed.get());
                                    set_show_profile.set(false);
                                }
                            >
                                "Save Changes"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        })}
    }
}

fn main() {
    mount_to_body(App);
}