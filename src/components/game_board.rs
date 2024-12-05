use std::{fmt::Write, thread::current};
use crate::components::board::{Board, CellContent};

use web_sys::console;

// just for debugging
#[derive(Debug)]
struct Move {
    player: &'static str,
    action: &'static str,
    position: (usize, usize),
    step: usize,
}

#[derive(Debug, Clone)]
struct Square {
    row: usize,
    col: usize,
    player_trap_step: Option<usize>,
    opponent_trap_step: Option<usize>,
    player_visits: Vec<usize>,
    opponent_visits: Vec<usize>,
    collision_step: Option<usize>,
    player_trap_hit_step: Option<usize>,
    opponent_trap_hit_step: Option<usize>,
}

impl Square {
    fn new(row: usize, col: usize) -> Self {
        Square {
            row,
            col,
            player_trap_step: None,
            opponent_trap_step: None,
            player_visits: Vec::new(),
            opponent_visits: Vec::new(),
            collision_step: None,
            player_trap_hit_step: None,
            opponent_trap_hit_step: None,
        }
    }
}

#[derive(Clone, Debug)]
enum MoveType {
    Regular(usize, usize),  // row, col
    Trap(usize, usize),     // row, col
    Final,
    None,
}

#[derive(Clone)]
pub struct GameBoard {
    pub squares: Vec<Vec<Square>>,
    pub size: usize,
    pub player_sequence: Vec<(usize, usize, CellContent)>,
    pub opponent_sequence: Vec<(usize, usize, CellContent)>,
    pub player_position: Option<(usize, usize)>,
    pub opponent_position: Option<(usize, usize)>,
    pub player_collision_step: Option<usize>,
    pub opponent_collision_step: Option<usize>,
    pub player_score: i32,
    pub opponent_score: i32,
    pub processed_sequence: Vec<(usize, usize, CellContent, bool, usize)>,
}

#[derive(Debug)]
struct MoveResult {
    new_position: Option<(usize, usize)>,
    trap_placed: Option<(usize, usize)>,
    points_earned: i32,
    is_first_step: bool,
    moving_forward: bool,
    goal_reached: bool,
}
    
#[derive(Debug)]
enum TrapResult {
    NoTraps,
    Player1Hit,
    Player2Hit,
    BothHit,
}

impl GameBoard {
    pub fn new(size: usize) -> Self {
        let mut squares = Vec::with_capacity(size);
        for i in 0..size {
            let mut row = Vec::with_capacity(size);
            for j in 0..size {
                row.push(Square::new(i, j));
            }
            squares.push(row);
        }
        GameBoard {
            size,
            player_sequence: Vec::new(),
            opponent_sequence: Vec::new(),
            player_position: None,
            opponent_position: None,
            player_collision_step: None,
            opponent_collision_step: None,
            player_score: 0,
            opponent_score: 0,
            processed_sequence: Vec::new(),
            squares,
        }
    }

    fn check_collisions(&self, p1_result: &MoveResult, p2_result: &MoveResult) -> bool {
        console::log_1(&"\n=== Checking Collisions ===".into());
    
        // Check for direct piece collisions (according to flowchart node D)
        if let (Some(p1_pos), Some(p2_pos)) = (p1_result.new_position, p2_result.new_position) {
            if p1_pos == p2_pos {
                console::log_1(&format!("Players collide at position {:?}", p1_pos).into());
                return true;
            }
        }
    
        // Check trap collisions (moved from old check_traps function)
        if let Some(p1_trap) = p1_result.trap_placed {
            if let Some(p2_pos) = p2_result.new_position {
                if p1_trap == p2_pos {
                    console::log_1(&"Player 2 collides with new Player 1 trap".into());
                    return true;
                }
            }
        }
    
        if let Some(p2_trap) = p2_result.trap_placed {
            if let Some(p1_pos) = p1_result.new_position {
                if p2_trap == p1_pos {
                    console::log_1(&"Player 1 collides with new Player 2 trap".into());
                    return true;
                }
            }
        }
    
        false
    }
    
    fn check_traps(
        &self,
        p1_result: &MoveResult,
        p2_result: &MoveResult,
        player_board: &Board,
        opponent_board: &Board,
        current_step: usize  // Add current_step parameter
    ) -> TrapResult {
        console::log_1(&"\n=== Checking Existing Traps ===".into());
        
        let mut p1_hit = false;
        let mut p2_hit = false;
    
        // Check if player 1 hit any existing opponent traps (only from previous steps)
        if let Some(p1_pos) = p1_result.new_position {
            let (row, col) = p1_pos;
            let (rot_row, rot_col) = self.rotate_position(row, col);
            
            // Only consider traps placed in previous steps
            for (step, &(trap_row, trap_col, ref content)) in opponent_board.sequence.iter().enumerate() {
                if step > current_step {
                    break; // Don't check future traps
                }
                if *content == CellContent::Trap && (trap_row, trap_col) == (rot_row, rot_col) {
                    p1_hit = true;
                    break;
                }
            }
        }
    
        // Check if player 2 hit any existing player traps (only from previous steps)
        if let Some(p2_pos) = p2_result.new_position {
            let (row, col) = p2_pos;
            
            // Only consider traps placed in previous steps
            for (step, &(trap_row, trap_col, ref content)) in player_board.sequence.iter().enumerate() {
                if step > current_step {
                    break; // Don't check future traps
                }
                if *content == CellContent::Trap && (trap_row, trap_col) == (row, col) {
                    console::log_1(&format!("Player 2 hit existing trap at {:?}", p2_pos).into());
                    p2_hit = true;
                    break;
                }
            }
        }
    
        match (p1_hit, p2_hit) {
            (true, true) => TrapResult::BothHit,
            (true, false) => TrapResult::Player1Hit,
            (false, true) => TrapResult::Player2Hit,
            (false, false) => TrapResult::NoTraps,
        }
    }

    fn handle_moves(&self, player_move: MoveType, opponent_move: MoveType, step: usize) -> (MoveResult, MoveResult) {
        console::log_1(&format!("\n=== Handling Moves for Step {} ===", step + 1).into());
    
        let player_result = match player_move {
            MoveType::Final => MoveResult {
                new_position: None,
                trap_placed: None,
                points_earned: 1,
                is_first_step: step == 0,
                moving_forward: true,
                goal_reached: true,
            },
            MoveType::Regular(row, col) => {
                let moving_forward = if let Some((prev_row, _)) = self.player_position {
                    row < prev_row
                } else {
                    false
                };
        
                MoveResult {
                    new_position: Some((row, col)),
                    trap_placed: None,
                    points_earned: if moving_forward { 1 } else { 0 },
                    is_first_step: step == 0,
                    moving_forward,
                    goal_reached: false,
                }
            },
            MoveType::Trap(row, col) => MoveResult {
                new_position: self.player_position,
                trap_placed: Some((row, col)),
                points_earned: 0,
                is_first_step: step == 0,
                moving_forward: false,
                goal_reached: false,
            },
            MoveType::None => MoveResult {
                new_position: self.player_position,
                trap_placed: None,
                points_earned: 0,
                is_first_step: step == 0,
                moving_forward: false,
                goal_reached: false,
            },
        };
    
        let opponent_result = match opponent_move {
            MoveType::Final => MoveResult {
                new_position: None,
                trap_placed: None,
                points_earned: 1,
                is_first_step: step == 0,
                moving_forward: true,
                goal_reached: true,
            },
            MoveType::Regular(row, col) => {
                let moving_forward = if let Some((prev_row, _)) = self.opponent_position {
                    row > prev_row
                } else {
                    false
                };
        
                MoveResult {
                    new_position: Some((row, col)),
                    trap_placed: None,
                    points_earned: if moving_forward { 1 } else { 0 },
                    is_first_step: step == 0,
                    moving_forward,
                    goal_reached: false,
                }
            },
            MoveType::Trap(row, col) => MoveResult {
                new_position: self.opponent_position,
                trap_placed: Some((row, col)),
                points_earned: 0,
                is_first_step: step == 0,
                moving_forward: false,
                goal_reached: false,
            },
            MoveType::None => MoveResult {
                new_position: self.opponent_position,
                trap_placed: None,
                points_earned: 0,
                is_first_step: step == 0,
                moving_forward: false,
                goal_reached: false,
            },
        };

        (player_result, opponent_result)
    }

    pub fn generate_board_svg(&self, player_board: &Board, opponent_board: &Board) -> String {

        let moves: Vec<Move> = self.processed_sequence.iter().map(|(row, col, content, is_opponent, step)| {
            Move {
                player: if *is_opponent { "Opponent" } else { "Player" },
                action: match content {
                    CellContent::Player => "Move",
                    CellContent::Trap => "Place Trap",
                    CellContent::Empty => "Empty",
                },
                position: (*row, *col),
                step: *step,
            }
        }).collect();
        
        console::log_1(&format!("\n=== Processed Sequence ===\n{:#?}", moves).into());

        let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                <rect width="100" height="100" fill="rgb(30, 41, 59)"/>
                <g transform="translate(5,5)">"#);
    
        // Draw grid
        for i in 0..self.size {
            for j in 0..self.size {
                let x = j as f32 * 45.0;
                let y = i as f32 * 45.0;
                let _ = write!(
                    svg,
                    r#"<rect x="{}" y="{}" width="40" height="40" fill="rgb(51, 65, 85)"/>"#,
                    x, y
                );
            }
        }
    
        // Draw pieces and traps for player's board
        for (row, col, content, is_opponent, current_step) in &self.processed_sequence {
            let x = *col as f32 * 45.0;
            let y = *row as f32 * 45.0;
            
            match content {
                CellContent::Player => {
                    let (color, step) = if *is_opponent {
                        ("rgb(147, 51, 234)", current_step)  // opponent color
                    } else {
                        ("rgb(37, 99, 235)", current_step)   // player color
                    };
                    let _ = write!(
                        svg,
                        r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="{}"/>
                        <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                        x + 20.0, y + 20.0, color, x + 20.0, y + 20.0, step
                    );
                },
                CellContent::Trap => {
                    let stroke_color = if *is_opponent {
                        "rgb(249, 115, 22)"  // opponent trap color
                    } else {
                        "rgb(220, 38, 38)"   // player trap color
                    };
                    let _ = write!(
                        svg,
                        r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="{}" stroke-width="4"/>"#,
                        x + 5.0, y + 5.0, stroke_color
                    );
                },
                _ => {},
            }
        }    
        svg.push_str("</g></svg>");
        format!(r#"data:image/svg+xml,{}"#, urlencoding::encode(&svg))
    }
    
    fn rotate_position(&self, row: usize, col: usize) -> (usize, usize) {
        (self.size - 1 - row, self.size - 1 - col)
    }
    
    fn process_moves(&self, player_board: &Board, opponent_board: &Board, step: usize) -> (MoveType, MoveType) {
        console::log_1(&format!("\nProcessing step {} of max {} steps", 
            step + 1, 
            std::cmp::max(player_board.sequence.len(), opponent_board.sequence.len())
        ).into());
    
        let player_move = if step < player_board.sequence.len() {
            let (row, col, content) = player_board.sequence[step].clone();
            match content {
                CellContent::Player => {
                    if step == player_board.sequence.len() - 1 {
                        console::log_1(&format!("P1 Step {}: Final move", step + 1).into());
                        MoveType::Final
                    } else {
                        console::log_1(&format!("P1 Step {}: Move to ({}, {})", step + 1, row, col).into());
                        MoveType::Regular(row, col)
                    }
                },
                CellContent::Trap => {
                    console::log_1(&format!("P1 Step {}: Place trap at ({}, {})", step + 1, row, col).into());
                    MoveType::Trap(row, col)
                },
                _ => {
                    console::log_1(&format!("P1 Step {}: No move", step + 1).into());
                    MoveType::None
                },
            }
        } else {
            console::log_1(&format!("P1 Step {}: No more moves in sequence", step + 1).into());
            MoveType::None
        };
    
        let opponent_move = if step < opponent_board.sequence.len() {
            let (row, col, content) = opponent_board.sequence[step].clone();
            let (rot_row, rot_col) = self.rotate_position(row, col);
            match content {
                CellContent::Player => {
                    if step == opponent_board.sequence.len() - 1 {
                        console::log_1(&format!("P2 Step {}: Final move", step + 1).into());
                        MoveType::Final
                    } else {
                        console::log_1(&format!("P2 Step {}: Move to ({}, {}) (rotated from ({}, {}))", 
                            step + 1, rot_row, rot_col, row, col).into());
                        MoveType::Regular(rot_row, rot_col)
                    }
                },
                CellContent::Trap => {
                    console::log_1(&format!("P2 Step {}: Place trap at ({}, {}) (rotated from ({}, {}))", 
                        step + 1, rot_row, rot_col, row, col).into());
                    MoveType::Trap(rot_row, rot_col)
                },
                _ => {
                    console::log_1(&format!("P2 Step {}: No move", step + 1).into());
                    MoveType::None
                },
            }
        } else {
            console::log_1(&format!("P2 Step {}: No more moves in sequence", step + 1).into());
            MoveType::None
        };
    
        (player_move, opponent_move)
    }

    pub fn process_turn(&mut self, player_board: &Board, opponent_board: &Board) {
        console::log_1(&"\n====== Starting New Game Round ======".into());
        
        // Debug: Print full sequences
        console::log_1(&"\nPlayer 1 sequence:".into());
        for (i, &(row, col, ref content)) in player_board.sequence.iter().enumerate() {
            let content_str = match content {
                CellContent::Player => "Player",
                CellContent::Trap => "Trap",
                CellContent::Empty => "Empty",
            };
            console::log_1(&format!("Step {}: ({}, {}) - {}", i + 1, row, col, content_str).into());

            // Record player moves/traps
            match content {
                CellContent::Player => {
                    self.squares[row][col].player_visits.push(i);
                },
                CellContent::Trap => {
                    self.squares[row][col].player_trap_step = Some(i);
                },
                _ => {}
            }
        }
    
        console::log_1(&"\nPlayer 2 sequence:".into());
        for (i, &(row, col, ref content)) in opponent_board.sequence.iter().enumerate() {
            let content_str = match content {
                CellContent::Player => "Player",
                CellContent::Trap => "Trap",
                CellContent::Empty => "Empty",
            };
            console::log_1(&format!("Step {}: ({}, {}) - {}", i + 1, row, col, content_str).into());

            // Record opponent moves/traps (with rotation)
            let (rot_row, rot_col) = self.rotate_position(row, col);
            match content {
                CellContent::Player => {
                    self.squares[rot_row][rot_col].opponent_visits.push(i);
                },
                CellContent::Trap => {
                    self.squares[rot_row][rot_col].opponent_trap_step = Some(i);
                },
                _ => {}
            }
        }

        console::log_1(&"\n=== Square States ===".into());
        for row in &self.squares {
            for square in row {
                console::log_1(&format!("\nSquare ({}, {}): {:#?}", square.row, square.col, square).into());
            }
        }
    
        console::log_1(&"\n====== Round Summary ======".into());
        console::log_1(&format!("Final player score: {}", self.player_score).into());
        console::log_1(&format!("Final opponent score: {}", self.opponent_score).into());

    }        

}