use leptos::*;
use leptos::prelude::*;
use leptos::callback::Callback;
use crate::components::opponent::OpponentType;
use crate::components::utils::{generate_thumbnail, generate_opponent_thumbnail};

use super::board::SavedBoard;
use super::opponent::Opponent;
use super::game_board::GameBoard;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use super::utils::load_saved_boards;
use rand; 

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum GamePhase {
    SelectingBoards,
    DisplayingBoards,  // Add this
    ShowingResults,
    RoundComplete,     // Add this
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
    pub game_board: Option<GameBoard>,  // Add this
    pub phase: GamePhase,
    pub speed: GameSpeed,
}

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
            game_board: None,  // Add this
            phase: GamePhase::SelectingBoards,
            speed: GameSpeed::Relaxed,
        }
    }
}

fn select_random_board(boards: Vec<SavedBoard>) -> Option<SavedBoard> {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    boards.choose(&mut rng).cloned()
}

#[component]
pub fn Game(
    #[prop(into)] player_name: String,
    #[prop(into)] opponent: Opponent,
    #[prop(into)] speed: GameSpeed,  
    #[prop(into)] on_exit: Callback<()>,
) -> impl IntoView {
    let game_state = RwSignal::new({
        let mut state = GameState::new(player_name, opponent);
        state.speed = speed;  // Set the speed from prop
        state
    });
    let boards = Memo::new(|_| load_saved_boards().unwrap_or_default());
    let (timer, set_timer) = signal(match game_state.get().speed {
        GameSpeed::Lightning => 1,
        GameSpeed::Quick => 5,
        GameSpeed::Relaxed => 10,
        GameSpeed::Chill => 999999, // Effectively no limit
    });

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
                                {move || {
                                    let current_speed = game_state.get().speed;
                                    let current_time = timer.get();
                                    (current_speed != GameSpeed::Chill).then(|| view! {
                                        <div class="font-mono text-lg text-orange-400 bg-slate-700 px-4 py-1 rounded-md">
                                            {move || format!("{} seconds left!", current_time)}
                                        </div>
                                    })
                                }}
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
                                                    
                                                    // Select random board for CPU opponent
                                                    if let Some(ref opponent) = current_state.player2 {
                                                        if matches!(opponent.opponent_type, OpponentType::Computer) {
                                                            let available_boards = boards.get();
                                                            if let Some(cpu_board) = select_random_board(available_boards) {
                                                                current_state.player2_board = Some(cpu_board);
                                                            }
                                                        }
                                                    }
                                                    
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
                        <div class="flex flex-col items-center gap-6">
                            // Thumbnails row
                            <div class="flex gap-8 items-start">
                                // Player's board
                                <div class="text-center">
                                    <h3 class="text-sm font-bold mb-2">
                                        <div class="flex items-center justify-center gap-2">
                                            {move || game_state.get().player1}
                                        </div>
                                    </h3>
                                    {move || game_state.get().player1_board.as_ref().map(|board| view! {
                                        <img 
                                            src=generate_thumbnail(&board.board)
                                            alt="Player board" 
                                            class="w-32 h-32 rounded border border-slate-700"
                                        />
                                    })}
                                </div>
                
                                // Opponent's board
                                <div class="text-center">
                                    <h3 class="text-sm font-bold mb-2">
                                        <div class="flex items-center justify-center gap-2">
                                            {move || game_state.get().player2.as_ref().map(|p| p.name.clone()).unwrap_or_default()}
                                        </div>
                                    </h3>
                                    {move || game_state.get().player2_board.as_ref().map(|board| view! {
                                        <img 
                                            src=generate_opponent_thumbnail(&board.board)
                                            alt="Opponent board" 
                                            class="w-32 h-32 rounded border border-slate-700"
                                        />
                                    })}
                                </div>
                            </div>
                
                            // Game board view
                            <div class="text-center">
                                <h3 class="text-xl font-bold mb-2">"Game Progress"</h3>
                                {move || {
                                    let state = game_state.get();
                                    if let (Some(board1), Some(board2)) = (&state.player1_board, &state.player2_board) {
                                        // Initialize game board if not exists
                                        if state.game_board.is_none() {
                                            let mut game_board = GameBoard::new(2);
                                            game_board.process_turn(&board1.board, &board2.board);
                                            
                                            // Update total scores
                                            let mut current_state = state.clone();
                                            current_state.player1_score += game_board.player_score;
                                            current_state.player2_score += game_board.opponent_score;
                                            current_state.game_board = Some(game_board);
                                            game_state.set(current_state);
                                        }
                                        if let Some(game_board) = &state.game_board {
                                            view! {
                                                <div class="flex flex-col items-center gap-2">
                                                    // Add player success message
                                                    {(game_board.player_position.map_or(false, |(row, _)| row == 0)).then(|| view! {
                                                        <div class="text-green-400 font-bold text-lg">
                                                            {state.player1.clone()} " Made it!"
                                                        </div>
                                                    })}
                                                    
                                                    <img 
                                                        src=game_board.generate_board_svg(&board1.board, &board2.board)
                                                        alt="Game board" 
                                                        class="w-96 h-96 rounded border border-slate-700"
                                                    />
                                                    
                                                    // Add opponent success message
                                                    {(game_board.opponent_position.map_or(false, |(row, _)| row == game_board.size - 1)).then(|| view! {
                                                        <div class="text-purple-400 font-bold text-lg">
                                                            {state.player2.as_ref().map(|p| p.name.clone()).unwrap_or_default()} " Made it!"
                                                        </div>
                                                    })}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div>"Loading..."</div> }.into_any()
                                        }
                                    } else {
                                        view! { <div>"Loading..."</div> }.into_any()
                                    }
                                }}
                            </div>                
                            // Round scores display
                            <div class="mt-4 flex justify-center gap-8">
                                {move || {
                                    let state = game_state.get();
                                    if let Some(game_board) = &state.game_board {
                                        view! {
                                            <div class="flex justify-center gap-8">
                                                <div class="text-lg">
                                                    {state.player1.clone()} 
                                                    {" (Round): "}
                                                    <span class="font-bold">
                                                        {game_board.player_score}
                                                    </span>
                                                </div>
                                                <div class="text-lg">
                                                    {state.player2.as_ref().map(|p| p.name.clone()).unwrap_or_default()} 
                                                    {" (Round): "}
                                                    <span class="font-bold">
                                                        {game_board.opponent_score}
                                                    </span>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div>"Calculating scores..."</div> }.into_any()
                                    }
                                }}
                            </div>
                
                            // Total scores display
                            <div class="mt-2 flex justify-center gap-8 text-sm text-gray-400">
                                {move || {
                                    let state = game_state.get();
                                    view! {
                                        <>
                                            <div>
                                                "Total: "
                                                <span class="font-bold">{state.player1_score}</span>
                                            </div>
                                            <div>
                                                "Total: "
                                                <span class="font-bold">{state.player2_score}</span>
                                            </div>
                                        </>
                                    }
                                }}
                            </div>
                
                            // Next round button
                            <button
                                class="px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded"
                                on:click=move |_| {
                                    let mut current_state = game_state.get();
                                    current_state.current_round += 1;
                                    if current_state.current_round <= 8 {
                                        current_state.phase = GamePhase::SelectingBoards;
                                        current_state.player1_board = None;
                                        current_state.player2_board = None;
                                        current_state.game_board = None;
                                        set_timer.set(match current_state.speed {
                                            GameSpeed::Lightning => 1,
                                            GameSpeed::Quick => 5,
                                            GameSpeed::Relaxed => 10,
                                            GameSpeed::Chill => 999999,
                                        });
                                    }
                                    game_state.set(current_state);
                                }
                            >
                                {move || if game_state.get().current_round <= 8 {
                                    "Next Round"
                                } else {
                                    "Game Complete"
                                }}
                            </button>
                        </div>
                    }.into_any(),
                    GamePhase::DisplayingBoards | GamePhase::RoundComplete => todo!(),
                }}
            </div>
        </div>
    }
}