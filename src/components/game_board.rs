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
    goal_reached: bool,
    trap_placed: Option<(usize, usize)>,
    points_earned: i32,
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

    fn check_collisions(&mut self, p1_result: &MoveResult, p2_result: &MoveResult) -> bool {
        console::log_1(&"\n=== Checking Collisions ===".into());
    
        // Check for direct piece collisions
        if let (Some(p1_pos), Some(p2_pos)) = (p1_result.new_position, p2_result.new_position) {
            if p1_pos == p2_pos {
                console::log_1(&format!("Piece collision detected at position {:?}!", p1_pos).into());
                self.player_collision_step = Some(self.player_sequence.len());
                self.opponent_collision_step = Some(self.opponent_sequence.len());
                return true;
            }
        }
    
        // Check if player hits opponent's new trap
        if let (Some(p1_pos), Some(trap_pos)) = (p1_result.new_position, p2_result.trap_placed) {
            if p1_pos == trap_pos {
                console::log_1(&format!("Player hit opponent's new trap at {:?}", trap_pos).into());
                self.player_collision_step = Some(self.player_sequence.len());
                return true;
            }
        }
    
        // Check if opponent hits player's new trap
        if let (Some(p2_pos), Some(trap_pos)) = (p2_result.new_position, p1_result.trap_placed) {
            if p2_pos == trap_pos {
                console::log_1(&format!("Opponent hit player's new trap at {:?}", trap_pos).into());
                self.opponent_collision_step = Some(self.opponent_sequence.len());
                return true;
            }
        }
    
        console::log_1(&"No collisions detected".into());
        false
    }

    fn check_traps(
        &self,
        p1_result: &MoveResult,
        p2_result: &MoveResult,
        player_board: &Board,
        opponent_board: &Board
    ) -> (bool, bool) {
        console::log_1(&"\n=== Checking Existing Traps ===".into());
        
        let mut p1_hit = false;
        let mut p2_hit = false;
    
        // Check if player hit any existing opponent traps
        if let Some(p1_pos) = p1_result.new_position {
            let (row, col) = p1_pos;
            if opponent_board.grid[row][col] == CellContent::Trap {
                console::log_1(&format!("Player hit existing trap at {:?}", p1_pos).into());
                p1_hit = true;
            }
        }
    
        // Check if opponent hit any existing player traps
        if let Some(p2_pos) = p2_result.new_position {
            let (row, col) = p2_pos;
            if player_board.grid[row][col] == CellContent::Trap {
                console::log_1(&format!("Opponent hit existing trap at {:?}", p2_pos).into());
                p2_hit = true;
            }
        }
    
        if !p1_hit && !p2_hit {
            console::log_1(&"No existing traps hit".into());
        }
    
        (p1_hit, p2_hit)
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
    
        // Check if we're out of moves
        let moves_exhausted = current_step >= player_board.sequence.len() && 
                             current_step >= opponent_board.sequence.len();
        if moves_exhausted {
            console::log_1(&"Round complete: No more moves".into());
            return true;
        }
    
        // Check if either player reached their goal via sequence
        let player_reached_goal = player_board.sequence.last().map_or(false, |(row, _, content)| {
            matches!(content, CellContent::Player) && *row == 0
        });
        let opponent_reached_goal = opponent_board.sequence.last().map_or(false, |(row, _, content)| {
            matches!(content, CellContent::Player) && *row == self.size - 1
        });

        if player_reached_goal {
            console::log_1(&"Round complete: Player reached goal".into());
            return true;
        }
        if opponent_reached_goal {
            console::log_1(&"Round complete: Opponent reached goal".into());
            return true;
        }
    
        console::log_1(&"Round not complete - continuing".into());
        false
    }

    fn handle_moves(&self, player_move: MoveType, opponent_move: MoveType, step: usize) -> (MoveResult, MoveResult) {
        console::log_1(&format!("\n=== Handling Moves for Step {} ===", step + 1).into());

        let player_result = match player_move {
            MoveType::Final => {
                console::log_1(&"Processing player's final move".into());
                MoveResult {
                    new_position: None,
                    goal_reached: true,
                    trap_placed: None,
                    points_earned: 1, // Bonus point for reaching goal
                }
            },
            MoveType::Regular(row, col) => {
                console::log_1(&format!("Processing player's regular move to ({}, {})", row, col).into());
                MoveResult {
                    new_position: Some((row, col)),
                    goal_reached: false,
                    trap_placed: None,
                    points_earned: if let Some((prev_row, _)) = self.player_position {
                        if row < prev_row { 1 } else { 0 }
                    } else { 0 }
                }
            },
            MoveType::Trap(row, col) => {
                console::log_1(&format!("Processing player's trap placement at ({}, {})", row, col).into());
                MoveResult {
                    new_position: self.player_position,
                    goal_reached: false,
                    trap_placed: Some((row, col)),
                    points_earned: 0,
                }
            },
            MoveType::None => {
                console::log_1(&"No move to process for player".into());
                MoveResult {
                    new_position: self.player_position,
                    goal_reached: false,
                    trap_placed: None,
                    points_earned: 0,
                }
            },
        };

        let opponent_result = match opponent_move {
            MoveType::Final => {
                console::log_1(&"Processing opponent's final move".into());
                MoveResult {
                    new_position: None,
                    goal_reached: true,
                    trap_placed: None,
                    points_earned: 1, // Bonus point for reaching goal
                }
            },
            MoveType::Regular(row, col) => {
                console::log_1(&format!("Processing opponent's regular move to ({}, {})", row, col).into());
                MoveResult {
                    new_position: Some((row, col)),
                    goal_reached: false,
                    trap_placed: None,
                    points_earned: if let Some((prev_row, _)) = self.opponent_position {
                        if row > prev_row { 1 } else { 0 }
                    } else { 0 }
                }
            },
            MoveType::Trap(row, col) => {
                console::log_1(&format!("Processing opponent's trap placement at ({}, {})", row, col).into());
                MoveResult {
                    new_position: self.opponent_position,
                    goal_reached: false,
                    trap_placed: Some((row, col)),
                    points_earned: 0,
                }
            },
            MoveType::None => {
                console::log_1(&"No move to process for opponent".into());
                MoveResult {
                    new_position: self.opponent_position,
                    goal_reached: false,
                    trap_placed: None,
                    points_earned: 0,
                }
            },
        };

        console::log_1(&format!("Move results - Player: {:?}, Opponent: {:?}", player_result, opponent_result).into());
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
        let player_move = if step < player_board.sequence.len() {
            let (row, col, content) = player_board.sequence[step].clone();
            match content {
                CellContent::Player => {
                    // Check if this is the final move in sequence
                    if player_board.sequence.last().map_or(false, |&(last_row, _, _)| {
                        row == last_row && row == 0
                    }) {
                        MoveType::Final
                    } else {
                        MoveType::Regular(row, col)
                    }
                },
                CellContent::Trap => MoveType::Trap(row, col),
                _ => MoveType::None,
            }
        } else {
            MoveType::None
        };

        let opponent_move = if step < opponent_board.sequence.len() {
            let (row, col, content) = opponent_board.sequence[step].clone();
            // Rotate opponent's position
            let (rot_row, rot_col) = self.rotate_position(row, col);
            match content {
                CellContent::Player => {
                    // Check if this is the final move in sequence
                    if opponent_board.sequence.last().map_or(false, |&(last_row, _, _)| {
                        row == last_row && row == opponent_board.size - 1
                    }) {
                        MoveType::Final
                    } else {
                        MoveType::Regular(rot_row, rot_col)
                    }
                },
                CellContent::Trap => MoveType::Trap(rot_row, rot_col),
                _ => MoveType::None,
            }
        } else {
            MoveType::None
        };

        (player_move, opponent_move)
    }

    pub fn process_turn(&mut self, player_board: &Board, opponent_board: &Board) {
        console::log_1(&"\n====== Starting New Game Round ======".into());
        
        console::log_1(&"\n=== Player Board Steps ===".into());
        for (idx, (row, col, content)) in player_board.sequence.iter().enumerate() {
            console::log_1(&format!("Step {}: row {} col {} {:?}", idx + 1, row, col, content).into());
        }
    
        console::log_1(&"\n=== Opponent Board Steps ===".into());
        for (idx, (row, col, content)) in opponent_board.sequence.iter().enumerate() {
            console::log_1(&format!("Step {}: row {} col {} {:?}", idx + 1, row, col, content).into());
        }

        let mut current_step = 0;
        
        // Store sequences for collision step tracking
        self.player_sequence = player_board.sequence.clone();
        self.opponent_sequence = opponent_board.sequence.clone();
        
        loop {
            console::log_1(&format!("\n=== Step {} ===", current_step + 1).into());
            
            // Start Turn (A) and Process Moves (P1, P2)
            let (p1_move, p2_move) = self.process_moves(player_board, opponent_board, current_step);
            
            // Handle Moves (M1, M2)
            let (p1_result, p2_result) = self.handle_moves(p1_move, p2_move, current_step);
            
            // Check Collisions (CH1, D)
            if self.check_collisions(&p1_result, &p2_result) {
                console::log_1(&"Round ended due to collision".into());
                break;
            }
            
            // Check Traps (TC)
            let (p1_hit_trap, p2_hit_trap) = self.check_traps(&p1_result, &p2_result, player_board, opponent_board);
            
            // Handle trap hits
            if p1_hit_trap {
                self.player_collision_step = Some(current_step);
                console::log_1(&"Player's turn ended due to trap".into());
            }
            if p2_hit_trap {
                self.opponent_collision_step = Some(current_step);
                console::log_1(&"Opponent's turn ended due to trap".into());
            }
            
            // Update states and scores if no traps were hit
            if !p1_hit_trap {
                self.update_player_state(&p1_result, false);
            }
            if !p2_hit_trap {
                self.update_player_state(&p2_result, true);
            }
            
            // Check if round is complete
            if self.is_round_complete(current_step, player_board, opponent_board) {
                console::log_1(&"Round complete!".into());
                break;
            }
            
            current_step += 1;
        }
    
        console::log_1(&"\n====== Round Summary ======".into());
        console::log_1(&format!("Final player score: {}", self.player_score).into());
        console::log_1(&format!("Final opponent score: {}", self.opponent_score).into());
        console::log_1(&format!("Player collision step: {:?}", self.player_collision_step).into());
        console::log_1(&format!("Opponent collision step: {:?}", self.opponent_collision_step).into());
    }
}