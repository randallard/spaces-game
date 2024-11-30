use leptos::*;
use leptos::prelude::*;

use crate::components::utils::{delete_board,generate_thumbnail,load_saved_boards};
use crate::components::board::SavedBoard;

pub static BOARD_TRIGGER: std::sync::OnceLock<RwSignal<bool>> = std::sync::OnceLock::new();

pub fn get_board_trigger() -> RwSignal<bool> {
    *BOARD_TRIGGER.get_or_init(|| RwSignal::new(false))
}

#[component]
pub fn SavedBoards() -> impl IntoView {
    let trigger = get_board_trigger();
    let boards = Memo::new(move |_| {
        // Use trigger to force recomputation
        trigger.get();
        load_saved_boards().unwrap_or_default()
    });

    let delete = move |index: usize| {
        let _ = delete_board(index);
        trigger.update(|v| *v = !*v);  // Toggle to trigger refresh
    };

    view! {
        <div class="grid grid-cols-4 gap-4 mt-4">
            <For
                each=move || boards.get()
                key=|board| generate_thumbnail(&board.board)
                children=move |board: SavedBoard| {
                    view! {
                        <div class="relative">
                            <img 
                                src=board.thumbnail.clone()
                                alt="Saved board" 
                                class="w-24 h-24 rounded border border-slate-700"
                            />
                            <button
                                class="absolute -top-2 -right-2 bg-red-600 hover:bg-red-700 rounded-full w-6 h-6 flex items-center justify-center"
                                on:click=move |_| {
                                    if let Some(index) = boards.get().iter().position(|b| b.thumbnail == board.thumbnail) {
                                        delete(index)
                                    }
                                }
                            >
                                "×"
                            </button>
                        </div>
                    }
                }
            />
        </div>
    }
}