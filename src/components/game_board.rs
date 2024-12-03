use std::fmt::Write;
use crate::components::board::{Board, CellContent};

use web_sys::console;

#[derive(Debug)]
enum MoveType {
    Regular(usize, usize),  // row, col
    Trap(usize, usize),     // row, col
    Final,
    None,
}

#[derive(Clone)]
pub struct GameBoard {
    pub size: usize,
    pub player_sequence: Vec<(usize, usize, CellContent)>,
    pub opponent_sequence: Vec<(usize, usize, CellContent)>,
    pub player_position: Option<(usize, usize)>,
    pub opponent_position: Option<(usize, usize)>,
    pub player_collision_step: Option<usize>,
    pub opponent_collision_step: Option<usize>,
    pub player_score: i32,
    pub opponent_score: i32,
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
        opponent_board: &Board
    ) -> TrapResult {
        console::log_1(&"\n=== Checking Existing Traps ===".into());
        
        let mut p1_hit = false;
        let mut p2_hit = false;
    
        // Check if player 1 hit any existing opponent traps
        if let Some(p1_pos) = p1_result.new_position {
            let (row, col) = p1_pos;
            if opponent_board.grid[row][col] == CellContent::Trap {
                console::log_1(&format!("Player 1 hit existing trap at {:?}", p1_pos).into());
                p1_hit = true;
            }
        }
    
        // Check if player 2 hit any existing player traps
        if let Some(p2_pos) = p2_result.new_position {
            let (row, col) = p2_pos;
            if player_board.grid[row][col] == CellContent::Trap {
                console::log_1(&format!("Player 2 hit existing trap at {:?}", p2_pos).into());
                p2_hit = true;
            }
        }
    
        match (p1_hit, p2_hit) {
            (true, true) => TrapResult::BothHit,
            (true, false) => TrapResult::Player1Hit,
            (false, true) => TrapResult::Player2Hit,
            (false, false) => TrapResult::NoTraps,
        }
    }

    fn update_player_state(&mut self, move_result: &MoveResult, is_opponent: bool) {
        console::log_1(&format!("\n=== Updating State for {} ===", 
            if is_opponent { "Opponent" } else { "Player" }).into());
    
        // Update position
        if let Some(new_pos) = move_result.new_position {
            if is_opponent {
                self.opponent_position = Some(new_pos);
                console::log_1(&format!("Updated opponent position to {:?}", new_pos).into());
            } else {
                self.player_position = Some(new_pos);
                console::log_1(&format!("Updated player position to {:?}", new_pos).into());
            }
        }
    
        // Handle goal reached and scoring
        if move_result.goal_reached {
            console::log_1(&"Goal reached! Adding bonus point".into());
            if is_opponent {
                self.opponent_score += move_result.points_earned;
                console::log_1(&format!("Opponent score now: {}", self.opponent_score).into());
            } else {
                self.player_score += move_result.points_earned;
                console::log_1(&format!("Player score now: {}", self.player_score).into());
            }
        } else if move_result.points_earned > 0 {
            // Add points for forward movement
            if is_opponent {
                self.opponent_score += move_result.points_earned;
                console::log_1(&format!("Opponent earned {} points for forward movement. Total: {}", 
                    move_result.points_earned, self.opponent_score).into());
            } else {
                self.player_score += move_result.points_earned;
                console::log_1(&format!("Player earned {} points for forward movement. Total: {}", 
                    move_result.points_earned, self.player_score).into());
            }
        }
    }

    fn is_round_complete(&self, current_step: usize, player_board: &Board, opponent_board: &Board) -> bool {
        console::log_1(&"\n=== Checking Round Completion ===".into());
    
        // Check if both players hit something
        let both_collided = self.player_collision_step.is_some() && self.opponent_collision_step.is_some();
        if both_collided {
            console::log_1(&"Round complete: Both players collided".into());
            return true;
        }
    
        // Check current moves for goal completion
        if current_step < player_board.sequence.len() {
            let (row, _, ref content) = player_board.sequence[current_step];
            if matches!(content, CellContent::Player) && row == 0 {
                console::log_1(&"Round complete: Player reached goal this turn".into());
                return true;
            }
        }
    
        if current_step < opponent_board.sequence.len() {
            let (row, _, ref content) = opponent_board.sequence[current_step];
            if matches!(content, CellContent::Player) && row == opponent_board.size - 1 {
                console::log_1(&"Round complete: Opponent reached goal this turn".into());
                return true;
            }
        }
    
        // Check if we're out of moves
        let moves_exhausted = current_step >= player_board.sequence.len() && 
                             current_step >= opponent_board.sequence.len();
        if moves_exhausted {
            console::log_1(&"Round complete: No more moves".into());
            return true;
        }
    
        console::log_1(&"Round not complete - continuing".into());
        false
    }
    
    fn handle_moves(&self, player_move: MoveType, opponent_move: MoveType, step: usize) -> (MoveResult, MoveResult) {
        console::log_1(&format!("\n=== Handling Moves for Step {} ===", step + 1).into());
    
        let player_result = match player_move {
            MoveType::Final => MoveResult {
                new_position: None,
                trap_placed: None,
                points_earned: 0,
                is_first_step: step == 0,
                moving_forward: false,
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
                    goal_reached: row == 0,
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
                points_earned: 0,
                is_first_step: step == 0,
                moving_forward: false,
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
                    goal_reached: row == self.size - 1,
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
        for (idx, &(i, j, ref content)) in player_board.sequence.iter().enumerate() {
            if self.player_collision_step.map_or(true, |collision| idx <= collision) {
                let x = j as f32 * 45.0;
                let y = i as f32 * 45.0;
                
                // Skip the final move visualization if at top row
                if i == 0 && idx == player_board.sequence.len() - 1 {
                    continue;
                }
                
                match content {
                    CellContent::Player => {
                        if Some(idx) == self.player_collision_step {
                            let _ = write!(
                                svg,
                                r#"<circle cx="{:.0}" cy="{:.0}" r="18" fill="rgb(239, 68, 68)">
                                    <animate attributeName="r" values="18;22;18" dur="0.5s" repeatCount="2"/>
                                </circle>
                                <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                                x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                            );
                        } else {
                            let _ = write!(
                                svg,
                                r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(37, 99, 235)"/>
                                <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                                x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                            );
                        }
                    },
                    CellContent::Trap => {
                        let _ = write!(
                            svg,
                            r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(220, 38, 38)" stroke-width="4"/>"#,
                            x + 5.0, y + 5.0
                        );
                    },
                    _ => {}
                }
            }
        }
    
        // Draw opponent moves (rotated)
        for (idx, &(i, j, ref content)) in opponent_board.sequence.iter().enumerate() {
            if self.opponent_collision_step.map_or(true, |collision| idx <= collision) {
                let (rot_i, rot_j) = self.rotate_position(i, j);
                let x = rot_j as f32 * 45.0;
                let y = rot_i as f32 * 45.0;
                
                // Skip the final move visualization if at bottom row (rotated)
                if rot_i == self.size - 1 && idx == opponent_board.sequence.len() - 1 {
                    continue;
                }
                
                match content {
                    CellContent::Player => {
                        if Some(idx) == self.opponent_collision_step {
                            let _ = write!(
                                svg,
                                r#"<circle cx="{:.0}" cy="{:.0}" r="18" fill="rgb(239, 68, 68)">
                                    <animate attributeName="r" values="18;22;18" dur="0.5s" repeatCount="2"/>
                                </circle>
                                <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                                x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                            );
                        } else {
                            let _ = write!(
                                svg,
                                r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(147, 51, 234)"/>
                                <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                                x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                            );
                        }
                    },
                    CellContent::Trap => {
                        let _ = write!(
                            svg,
                            r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(249, 115, 22)" stroke-width="4"/>"#,
                            x + 5.0, y + 5.0
                        );
                    },
                    _ => {}
                }
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
            let move_type = match content {
                CellContent::Player => {
                    if player_board.sequence.last().map_or(false, |&(last_row, _, _)| {
                        row == last_row && row == 0
                    }) {
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
            };
            move_type
        } else {
            console::log_1(&format!("P1 Step {}: No more moves in sequence", step + 1).into());
            MoveType::None
        };
    
        let opponent_move = if step < opponent_board.sequence.len() {
            let (row, col, content) = opponent_board.sequence[step].clone();
            let (rot_row, rot_col) = self.rotate_position(row, col);
            let move_type = match content {
                CellContent::Player => {
                    if opponent_board.sequence.last().map_or(false, |&(last_row, _, _)| {
                        row == last_row && row == opponent_board.size - 1
                    }) {
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
            };
            move_type
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
        }
    
        console::log_1(&"\nPlayer 2 sequence:".into());
        for (i, &(row, col, ref content)) in opponent_board.sequence.iter().enumerate() {
            let content_str = match content {
                CellContent::Player => "Player",
                CellContent::Trap => "Trap",
                CellContent::Empty => "Empty",
            };
            console::log_1(&format!("Step {}: ({}, {}) - {}", i + 1, row, col, content_str).into());
        }
        
        let mut current_step = 0;
        
        self.player_sequence = player_board.sequence.clone();
        self.opponent_sequence = opponent_board.sequence.clone();        
        'main_loop: loop {
            console::log_1(&format!("\n=== Step {} ===", current_step + 1).into());
            
            // Start Turn (A) and Process Moves (P1, P2)
            let (p1_move, p2_move) = self.process_moves(player_board, opponent_board, current_step);
            console::log_1(&"\n=== Move Types ===".into());
            console::log_1(&format!("Player 1 Move: {:#?}", p1_move).into());
            console::log_1(&format!("Player 2 Move: {:#?}", p2_move).into());
            // Handle Moves (M1, M2, C1, C2, T1, T2)
            let (mut p1_result, mut p2_result) = self.handle_moves(p1_move, p2_move, current_step);
            console::log_1(&"\n=== Move Results ===".into());
            console::log_1(&format!("Player 1 Move: {:#?}", p1_result).into());
            console::log_1(&format!("Player 2 Move: {:#?}", p2_result).into());
            
            // Check Collisions (CH1, D)
            if self.check_collisions(&p1_result, &p2_result) {
                console::log_1(&"Round ended due to collision".into());
                break;
            }
            
            // Check Traps (TC)
            match self.check_traps(&p1_result, &p2_result, player_board, opponent_board) {
                TrapResult::BothHit => {
                    console::log_1(&"Both players hit traps - ending round".into());
                    break;
                },
                TrapResult::Player1Hit => {
                    console::log_1(&"Player 1 hit trap - stopping their progress".into());
                    self.player_collision_step = Some(current_step);
                },
                TrapResult::Player2Hit => {
                    console::log_1(&"Player 2 hit trap - stopping their progress".into());
                    self.opponent_collision_step = Some(current_step);
                },
                TrapResult::NoTraps => {
                    // First Step? (FS)
                    if p1_result.is_first_step || p2_result.is_first_step {
                        // Next turn without points on first step
                        console::log_1(&"First step - no points awarded".into());
                        current_step += 1;
                        continue;
                    }
                    
                    // Score points for moving forward (SC)
                    if !p1_result.is_first_step && p1_result.moving_forward {
                        p1_result.points_earned = 1;
                        console::log_1(&"Player 1 scored a point for moving forward".into());
                    }
                    if !p2_result.is_first_step && p2_result.moving_forward {
                        p2_result.points_earned = 1;
                        console::log_1(&"Player 2 scored a point for moving forward".into());
                    }
    
                    // Update scores based on results
                    self.player_score += p1_result.points_earned;
                    self.opponent_score += p2_result.points_earned;
                }
            }
            
            // Update positions if no traps were hit
            if self.player_collision_step.is_none() && p1_result.new_position.is_some() {
                self.player_position = p1_result.new_position;
            }
            if self.opponent_collision_step.is_none() && p2_result.new_position.is_some() {
                self.opponent_position = p2_result.new_position;
            }
            
            // Check if round is complete (NR)
            let p1_done = self.player_collision_step.is_some() || current_step >= player_board.sequence.len();
            let p2_done = self.opponent_collision_step.is_some() || current_step >= opponent_board.sequence.len();
            
            if p1_done && p2_done {
                console::log_1(&"Both players have completed their sequences".into());
                break;
            }
            
            // Next Turn (F -> A)
            current_step += 1;
        }
    
        console::log_1(&"\n====== Round Summary ======".into());
        console::log_1(&format!("Final player score: {}", self.player_score).into());
        console::log_1(&format!("Final opponent score: {}", self.opponent_score).into());
    }
}