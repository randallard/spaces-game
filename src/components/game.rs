use serde::{Serialize, Deserialize};
use crate::components::board::SavedBoard;
use crate::components::opponent::Opponent;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player1: String,  // Current player's name
    pub player2: Option<Opponent>,  // Selected opponent
    pub current_round: usize,
    pub player1_score: i32,
    pub player2_score: i32,
    pub player1_board: Option<SavedBoard>,
    pub player2_board: Option<SavedBoard>,
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
        }
    }
}