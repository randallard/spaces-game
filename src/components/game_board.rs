use std::fmt::Write;
use crate::components::board::{Board, CellContent};

use web_sys::console;
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

        // Draw pieces and traps for both players
        for (idx, &(i, j, ref content)) in player_board.sequence.iter().enumerate() {
            if self.player_collision_step.map_or(true, |collision| idx <= collision) {
                let x = j as f32 * 45.0;
                let y = i as f32 * 45.0;
                
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
                            r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(220, 38, 38)" stroke-width="4"/>"#,  // Changed to red
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
                            r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(249, 115, 22)" stroke-width="4"/>"#,  // Orange color
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

    fn initialize_svg(&self) -> String {
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

        svg
    }

    fn rotate_position(&self, row: usize, col: usize) -> (usize, usize) {
        (self.size - 1 - row, self.size - 1 - col)
    }
    
    fn calculate_step_score(&self, from_row: usize, to_row: usize, is_opponent: bool) -> i32 {
        if is_opponent {
            // For opponent, moving to a higher row number is progress
            if to_row > from_row {
                1
            } else {
                0
            }
        } else {
            // For player, moving to a lower row number is progress
            if to_row < from_row {
                1
            } else {
                0
            }
        }
    }

    fn check_trap_collision(&self, position: (usize, usize), opponent_board: &Board, step: usize) -> bool {
        let (row, col) = position;
        
        // Check existing traps
        if opponent_board.grid[row][col] == CellContent::Trap {
            console::log_1(&format!("Found existing trap at ({}, {})", row, col).into());
            return true;
        }

        // Check if trap is being placed this turn
        if step < opponent_board.sequence.len() {
            let (trap_row, trap_col, content) = opponent_board.sequence[step].clone();
            let (rotated_row, rotated_col) = self.rotate_position(trap_row, trap_col);
            if matches!(content, CellContent::Trap) && rotated_row == row && rotated_col == col {
                console::log_1(&format!("New trap being placed at ({}, {})", row, col).into());
                return true;
            }
        }

        false
    }

    fn check_piece_collision(&self, pos1: Option<(usize, usize)>, pos2: Option<(usize, usize)>) -> bool {
        if let (Some(p1), Some(p2)) = (pos1, pos2) {
            p1 == p2
        } else {
            false
        }
    }

    fn has_reached_goal(&self, position: Option<(usize, usize)>, is_opponent: bool) -> bool {
        if let Some((row, _)) = position {
            if is_opponent {
                row == self.size - 1
            } else {
                row == 0
            }
        } else {
            false
        }
    }

    fn check_and_record_collisions(
        &mut self,
        player_pos: Option<(usize, usize)>,
        opponent_pos: Option<(usize, usize)>,
        player_board: &Board,
        opponent_board: &Board,
        step: usize,
    ) {
        console::log_1(&format!("\nChecking collisions for step {}", step + 1).into());
        
        // Check player collisions if not already collided
        if self.player_collision_step.is_none() {
            if let Some(pos) = player_pos {
                console::log_1(&format!("Checking player at position {:?}", pos).into());
                
                if self.check_piece_collision(Some(pos), opponent_pos) {
                    self.player_collision_step = Some(step);
                    self.opponent_collision_step = Some(step);
                    console::log_1(&"COLLISION: Players collided!".into());
                } else if self.check_trap_collision(pos, opponent_board, step) {
                    self.player_collision_step = Some(step);
                    console::log_1(&format!("COLLISION: Player hit trap at position {:?}", pos).into());
                }
            }
        }
    
        // Check opponent collisions if not already collided
        if self.opponent_collision_step.is_none() {
            if let Some(pos) = opponent_pos {
                console::log_1(&format!("Checking opponent at position {:?}", pos).into());
                
                if !self.check_piece_collision(Some(pos), player_pos) && 
                   self.check_trap_collision(pos, player_board, step) {
                    self.opponent_collision_step = Some(step);
                    console::log_1(&format!("COLLISION: Opponent hit trap at position {:?}", pos).into());
                }
            }
        }
    }

    fn process_player_step(&mut self, board: &Board, step: usize, is_opponent: bool) -> (Option<(usize, usize)>, bool) {
        if step >= board.sequence.len() {
            return (None, false);
        }

        let (row, col, content) = board.sequence[step].clone();
        let (actual_row, actual_col) = if is_opponent {
            self.rotate_position(row, col)
        } else {
            (row, col)
        };

        match content {
            CellContent::Player => (Some((actual_row, actual_col)), false),
            CellContent::Trap => (None, true),
            _ => (None, false)
        }
    }

    fn check_goal_reached(&self, position: Option<(usize, usize)>, is_opponent: bool) -> bool {
        if let Some((row, _)) = position {
            if is_opponent {
                row == self.size - 1
            } else {
                row == 0
            }
        } else {
            false
        }
    }

    pub fn process_turn(&mut self, player_board: &Board, opponent_board: &Board) {
        console::log_1(&"=== Starting New Game Round ===".into());
        let mut player_pos: Option<(usize, usize)> = None;
        let mut opponent_pos: Option<(usize, usize)> = None;
        let mut player_goal_reached = false;
        let mut opponent_goal_reached = false;
    
        for step in 0..player_board.sequence.len().max(opponent_board.sequence.len()) {
            console::log_1(&format!("\n--- Processing Step {} ---", step + 1).into());
    
            // Process moves
            let (new_player_pos, player_set_trap) = self.process_player_step(player_board, step, false);
            let (new_opponent_pos, opponent_set_trap) = self.process_player_step(opponent_board, step, true);
    
            console::log_1(&format!("Player move: {:?} (trap set: {})", new_player_pos, player_set_trap).into());
            console::log_1(&format!("Opponent move: {:?} (trap set: {})", new_opponent_pos, opponent_set_trap).into());
    
            // Goal checks
            let player_at_goal = self.check_goal_reached(new_player_pos, false);
            let opponent_at_goal = self.check_goal_reached(new_opponent_pos, true);
    
            if player_at_goal || opponent_at_goal {
                if player_at_goal && !player_goal_reached {
                    player_goal_reached = true;
                    self.player_score += 1;
                    console::log_1(&"GOAL: Player reached goal! (+1 point)".into());
                }
                if opponent_at_goal && !opponent_goal_reached {
                    opponent_goal_reached = true;
                    self.opponent_score += 1;
                    console::log_1(&"GOAL: Opponent reached goal! (+1 point)".into());
                }
            }
    
            // Collision checks
            if let (Some(p_pos), Some(o_pos)) = (new_player_pos, new_opponent_pos) {
                if p_pos == o_pos {
                    console::log_1(&format!("COLLISION: Pieces collided at position {:?}!", p_pos).into());
                    self.player_collision_step = Some(step);
                    self.opponent_collision_step = Some(step);
                    break;
                }
            }
    
            // Trap checks
            if let Some(p_pos) = new_player_pos {
                if !player_set_trap {
                    if self.check_trap_collision(p_pos, opponent_board, step) {
                        console::log_1(&format!("COLLISION: Player hit trap at {:?}", p_pos).into());
                        self.player_collision_step = Some(step);
                    }
                }
            }
    
            if let Some(o_pos) = new_opponent_pos {
                if !opponent_set_trap {
                    if self.check_trap_collision(o_pos, player_board, step) {
                        console::log_1(&format!("COLLISION: Opponent hit trap at {:?}", o_pos).into());
                        self.opponent_collision_step = Some(step);
                    }
                }
            }
    
            if self.player_collision_step.is_some() && self.opponent_collision_step.is_some() {
                console::log_1(&"Both players hit traps - ending round".into());
                break;
            }
    
            // Score updates
            if self.player_collision_step.is_none() {
                if let (Some(new_pos), Some(old_pos)) = (new_player_pos, player_pos) {
                    let score = self.calculate_step_score(old_pos.0, new_pos.0, false);
                    if score > 0 {
                        self.player_score += score;
                        console::log_1(&format!("Player scored {} points for forward movement", score).into());
                    }
                }
                player_pos = new_player_pos;
            }
    
            if self.opponent_collision_step.is_none() {
                if let (Some(new_pos), Some(old_pos)) = (new_opponent_pos, opponent_pos) {
                    let score = self.calculate_step_score(old_pos.0, new_pos.0, true);
                    if score > 0 {
                        self.opponent_score += score;
                        console::log_1(&format!("Opponent scored {} points for forward movement", score).into());
                    }
                }
                opponent_pos = new_opponent_pos;
            }
    
            // Update final positions
            self.player_position = player_pos;
            self.opponent_position = opponent_pos;
    
            console::log_1(&format!("\nEnd of step {} status:", step + 1).into());
            console::log_1(&format!("Player position: {:?}", self.player_position).into());
            console::log_1(&format!("Opponent position: {:?}", self.opponent_position).into());
            console::log_1(&format!("Player score: {}", self.player_score).into());
            console::log_1(&format!("Opponent score: {}", self.opponent_score).into());
        }
    
        console::log_1(&"\n=== Round Complete ===".into());
        console::log_1(&format!("Final player collision step: {:?}", self.player_collision_step).into());
        console::log_1(&format!("Final opponent collision step: {:?}", self.opponent_collision_step).into());
        console::log_1(&format!("Player goal reached: {}", player_goal_reached).into());
        console::log_1(&format!("Opponent goal reached: {}", opponent_goal_reached).into());
        console::log_1(&format!("Final player score: {}", self.player_score).into());
        console::log_1(&format!("Final opponent score: {}", self.opponent_score).into());
    }

}