use leptos::*;
use leptos::prelude::*;
use leptos::callback::Callback;
use super::board::SavedBoard;
use super::opponent::Opponent;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use super::utils::load_saved_boards;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum GamePhase {
    SelectingBoards,
    ShowingResults,
}

// Add this near the top with other enums
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum GameSpeed {
    Lightning, // 1 second
    Quick,    // 5 seconds
    Relaxed,  // 10 seconds
    Chill,    // No limit
}

#[derive(Clone)]
pub struct GameState {
    pub player1: String,
    pub player2: Option<Opponent>,
    pub current_round: usize,
    pub player1_score: i32,
    pub player2_score: i32,
    pub player1_board: Option<SavedBoard>,
    pub player2_board: Option<SavedBoard>,
    pub phase: GamePhase,
    pub speed: GameSpeed,  // Add this line
}

// Update the new() function
impl GameState {
    pub fn new(player_name: String, opponent: Opponent) -> Self {
        GameState {
            player1: player_name,
            player2: Some(opponent),
            current_round: 1,
            player1_score: 0,
            player2_score: 0,
            player1_board: None,
            player2_board: None,
            phase: GamePhase::SelectingBoards,
            speed: GameSpeed::Quick,  // Default to Quick
        }
    }
}

#[component]
pub fn Game(
    #[prop(into)] player_name: String,
    #[prop(into)] opponent: Opponent,
    #[prop(into)] on_exit: Callback<()>,
) -> impl IntoView {
    let game_state = RwSignal::new(GameState::new(player_name, opponent));
    let boards = Memo::new(|_| load_saved_boards().unwrap_or_default());
    let (timer, set_timer) = signal(5);

    // Update timer every second
    if timer.get() > 0 {
        set_interval(
            move || {
                set_timer.update(|t| *t = (*t - 1).max(0));
            },
            Duration::from_secs(1),
        );
    }
    view! {
        <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
            <div class="bg-slate-800 p-6 rounded-lg shadow-xl max-w-4xl w-full mx-4 text-white">
                <div class="flex justify-between items-center mb-6">
                    <h2 class="text-2xl font-bold">
                        "Round " {move || game_state.get().current_round} " of 8"
                    </h2>
                    <button
                        class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded"
                        on:click=move |_| on_exit.run(())
                    >
                        "Exit Game"
                    </button>
                </div>
                <div class="flex justify-between text-xl mb-4">
                    <div>
                        {move || game_state.get().player1}
                        ": "
                        <span class="font-bold">
                            {move || game_state.get().player1_score}
                        </span>
                    </div>
                    <div>
                        {move || game_state.get().player2.as_ref().map(|p| p.name.clone()).unwrap_or_default()}
                        ": "
                        <span class="font-bold">
                            {move || game_state.get().player2_score}
                        </span>
                    </div>
                </div>
                {move || match game_state.get().phase {
                    GamePhase::SelectingBoards => view! {
                        <div class="mt-8">
                            <div class="flex flex-col items-center mb-4">
                                <h3 class="text-xl font-bold mb-2">
                                    "Select your board"
                                </h3>
                                <div class="font-mono text-lg text-orange-400 bg-slate-700 px-4 py-1 rounded-md">
                                    {move || format!("{} seconds left!", timer.get())}
                                </div>
                            </div>
                            <div class="grid grid-cols-4 gap-4 max-w-xl mx-auto">
                                <For
                                    each=move || boards.get()
                                    key=|board| board.thumbnail.clone()
                                    children=move |board: SavedBoard| {
                                        view! {
                                            <button
                                                class="w-24 h-24 rounded border border-slate-700 hover:border-blue-500 transition-colors"
                                                on:click=move |_| {
                                                    let mut current_state = game_state.get();
                                                    current_state.player1_board = Some(board.clone());
                                                    current_state.phase = GamePhase::ShowingResults;
                                                    game_state.set(current_state);
                                                }
                                            >
                                                <img 
                                                    src=board.thumbnail.clone()
                                                    alt="Board option" 
                                                    class="w-full h-full rounded"
                                                />
                                            </button>
                                        }
                                    }
                                />
                            </div>
                        </div>
                    }.into_any(),
                    GamePhase::ShowingResults => view! {
                        <div>"Results will go here"</div>
                    }.into_any(),
                }}
            </div>
        </div>
    }
}