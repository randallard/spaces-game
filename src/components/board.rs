use leptos::*;
use leptos::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum CellContent {
    Empty,
    Player,
    Trap,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    grid: Vec<Vec<CellContent>>,
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            grid: vec![vec![CellContent::Empty; size]; size],
            size,
        }
    }
}

#[component]
pub fn BoardCreator(
    #[prop(into)] on_cancel: Callback<()>,
) -> impl IntoView {
    let board = RwSignal::new(Board::new(2));
    let current_turn = RwSignal::new(0);
    let finished = RwSignal::new(false);

    let handle_cell_click = move |row: usize, col: usize| {
        let mut current_board = board.get();
        if current_turn.get() == 0 && row == current_board.size - 1 {
            // First turn - only allow placing player in bottom row
            current_board.grid[row][col] = CellContent::Player;
            board.set(current_board);
            current_turn.set(1);
        } else if !finished.get() {
            // Subsequent turns - allow placing trap or moving player
            let player_pos = find_player(&current_board);
            if let Some((player_row, player_col)) = player_pos {
                if player_row == 0 || is_adjacent(player_row, player_col, row, col) {
                    if row == usize::MAX {  // Special case for final move
                        finished.set(true);
                    } else {
                        current_board.grid[player_row][player_col] = CellContent::Empty;
                        current_board.grid[row][col] = CellContent::Player;
                        board.set(current_board);
                        current_turn.update(|t| *t += 1);
                    }
                }
            }
        }
    };

    let rows = move || (0..2).collect::<Vec<_>>();
    let cols = move || (0..2).collect::<Vec<_>>();

    view! {
        <div class="flex flex-col gap-4">
            {move || {
                let player_pos = find_player(&board.get());
                if let Some((row, _)) = player_pos {
                    if row == 0 {
                        view! {
                            <button
                                class="w-full h-8 bg-green-600 hover:bg-green-700 rounded mb-2"
                                on:click=move |_| handle_cell_click(usize::MAX, 0)
                            >
                                "Final Move"
                            </button>
                        }.into_any()
                    } else {
                        view! { <div class="h-8 mb-2"></div> }.into_any()
                    }
                } else {
                    view! { <div class="h-8 mb-2"></div> }.into_any()
                }
            }}        
            <div class="grid grid-cols-2 gap-1 bg-slate-800 p-2 rounded">
                <For
                    each=rows
                    key=|row| *row
                    children=move |row| {
                        view! {
                            <For
                                each=cols
                                key=|col| *col
                                children=move |col| {                                    
                                    view! {
                                        <button
                                        class="w-16 h-16 flex items-center justify-center bg-slate-700 hover:bg-slate-600 text-2xl"
                                        disabled=move || {
                                            if current_turn.get() == 0 {
                                                row != board.get().size - 1
                                            } else {
                                                let player_pos = find_player(&board.get());
                                                !matches!(player_pos, Some((p_row, p_col)) if is_adjacent(p_row, p_col, row, col))
                                            }
                                        }
                                        on:click=move |_| handle_cell_click(row, col)
                                    >
                                    {move || {
                                        if current_turn.get() == 0 && row == board.get().size - 1 {
                                            view! { <span>"Start"</span> }.into_any()
                                        } else if let Some((p_row, p_col)) = find_player(&board.get()) {
                                            match board.get().grid[row][col] {
                                                CellContent::Empty if is_adjacent(p_row, p_col, row, col) => 
                                                    view! {
                                                        <div class="flex flex-col gap-1">
                                                            <button class="px-2 py-1 bg-blue-600 rounded text-sm"
                                                                on:click=move |ev| {
                                                                    ev.stop_propagation();
                                                                    handle_cell_click(row, col)
                                                                }
                                                            >"Move"</button>
                                                            <button class="px-2 py-1 bg-red-600 rounded text-sm"
                                                                on:click=move |ev| {
                                                                    ev.stop_propagation();
                                                                    let mut current_board = board.get();
                                                                    current_board.grid[row][col] = CellContent::Trap;
                                                                    board.set(current_board);
                                                                    current_turn.update(|t| *t += 1);
                                                                }
                                                            >"Trap"</button>
                                                        </div>
                                                    }.into_any(),
                                                    CellContent::Empty => view! { <span>" "</span> }.into_any(),
                                                    CellContent::Player => view! { <span>"○"</span> }.into_any(),
                                                    CellContent::Trap => view! { <span>"×"</span> }.into_any(),
                                            }
                                        } else {
                                            view! { <span>"·"</span> }.into_any()
                                        }
                                    }}
                                    </button>
                                    }
                                }
                            />
                        }
                    }
                />
            </div>
            <div class="text-gray-300">
                {move || if current_turn.get() == 0 {
                    "Choose a starting square"
                } else if finished.get() {
                    "Board complete!"
                } else {
                    "Select an adjascent square to move your piece or place a trap."
                }}
            </div>
            <div class="flex gap-2">
                <button 
                    class="px-4 py-2 bg-gray-600 rounded hover:bg-gray-700"
                    on:click=move |_| on_cancel.run(())
                >
                    "Cancel"
                </button>
                {move || finished.get().then(|| view! {
                    <button class="px-4 py-2 bg-blue-600 rounded hover:bg-blue-700">
                        "Save Board"
                    </button>
                })}
            </div>
        </div>
    }
}

fn find_player(board: &Board) -> Option<(usize, usize)> {
    for i in 0..board.size {
        for j in 0..board.size {
            if matches!(board.grid[i][j], CellContent::Player) {
                return Some((i, j));
            }
        }
    }
    None
}

fn is_adjacent(x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    let dx = if x1 >= x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 >= y2 { y1 - y2 } else { y2 - y1 };
    (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
}