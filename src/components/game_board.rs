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

    pub fn process_turn(&mut self, player_board: &Board, opponent_board: &Board) {

        let mut player_pos = None;
        let mut opponent_pos = None;
        let mut player_goal_step = None;
        let mut opponent_goal_step = None;

        console::log_1(&"=== Starting New Round ===".into());
        console::log_1(&format!("Player sequence length: {}", player_board.sequence.len()).into());
        console::log_1(&format!("Opponent sequence length: {}", opponent_board.sequence.len()).into());

        for step in 0..player_board.sequence.len().max(opponent_board.sequence.len()) {
            console::log_1(&format!("\n----- STEP {} -----", step + 1).into());

            let mut temp_player_pos = player_pos;
            let mut temp_opponent_pos = opponent_pos;

            // Process player move if available            
            if step < player_board.sequence.len() {
                let (row, col, content) = player_board.sequence[step].clone();
                console::log_1(&format!("Player: Processing row {} col {} ({:?})", row, col, content).into());
                
                match content {
                    CellContent::Player => {
                        if self.player_collision_step.map_or(true, |collision| step <= collision) {
                            temp_player_pos = Some((row, col));
                            console::log_1(&format!("Player position updated to ({}, {})", row, col).into());
                            
                            // Only record goal if this isn't also a collision step
                            if row == 0 && player_goal_step.is_none() && 
                            !self.check_piece_collision(Some((row, col)), temp_opponent_pos) {
                                player_goal_step = Some(step);
                                console::log_1(&"GOAL: Player reached row 0!".into());
                            }
                        }
                    },
                    CellContent::Trap => {
                        console::log_1(&format!("Player placed trap at ({}, {})", row, col).into());
                    },
                    _ => {}
                }
            }

            // Do the same for the opponent section:
            if step < opponent_board.sequence.len() {
                let (row, col, content) = opponent_board.sequence[step].clone();
                let (rot_row, rot_col) = self.rotate_position(row, col);
                console::log_1(&format!("Opponent: Processing row {} col {} ({:?})", rot_row, rot_col, content).into());
                
                match content {
                    CellContent::Player => {
                        if self.opponent_collision_step.map_or(true, |collision| step <= collision) {
                            temp_opponent_pos = Some((rot_row, rot_col));
                            console::log_1(&format!("Opponent position updated to ({}, {})", rot_row, rot_col).into());
                            
                            // Only record goal if this isn't also a collision step
                            if rot_row == self.size - 1 && opponent_goal_step.is_none() && 
                            !self.check_piece_collision(Some((rot_row, rot_col)), temp_player_pos) {
                                opponent_goal_step = Some(step);
                                console::log_1(&"GOAL: Opponent reached final row!".into());
                            }
                        }
                    },
                    CellContent::Trap => {
                        console::log_1(&format!("Opponent placed trap at ({}, {})", rot_row, rot_col).into());
                    },
                    _ => {}
                }
            }

            // Check for collisions and traps
            self.check_and_record_collisions(
                temp_player_pos,
                temp_opponent_pos,
                player_board,
                opponent_board,
                step
            );

            // Update positions if no previous collisions, or if this is the collision step
            if self.player_collision_step.map_or(true, |collision| step <= collision) {
                if let (Some(temp_pos), Some((old_row, _))) = (temp_player_pos, player_pos) {
                    let (new_row, _) = temp_pos;
                    // Only update score if no collision yet and no goal reached by opponent
                    if self.player_collision_step.is_none() && 
                    opponent_goal_step.map_or(true, |g| step <= g) {
                        let score = self.calculate_step_score(old_row, new_row, false);
                        if score > 0 {
                            self.player_score += score;
                            console::log_1(&format!("Player score +{} (moved from row {} to {})", 
                                score, old_row, new_row).into());
                        }
                    }
                }
                player_pos = temp_player_pos;
            }

            if self.opponent_collision_step.map_or(true, |collision| step <= collision) {
                if let (Some(temp_pos), Some((old_row, _))) = (temp_opponent_pos, opponent_pos) {
                    let (new_row, _) = temp_pos;
                    // Only update score if no collision yet and no goal reached by player
                    if self.opponent_collision_step.is_none() && 
                    player_goal_step.map_or(true, |g| step <= g) {
                        let score = self.calculate_step_score(old_row, new_row, true);
                        if score > 0 {
                            self.opponent_score += score;
                            console::log_1(&format!("Opponent score +{} (moved from row {} to {})", 
                                score, old_row, new_row).into());
                        }
                    }
                }
                opponent_pos = temp_opponent_pos;
            }

            // Add bonus point for reaching goal, but only if haven't collided before reaching it
            if let Some(goal_step) = player_goal_step {
                if step == goal_step && self.player_collision_step.is_none() {
                    self.player_score += 1;
                    console::log_1(&"BONUS: Player +1 for reaching goal!".into());
                }
            }

            if let Some(goal_step) = opponent_goal_step {
                if step == goal_step && self.opponent_collision_step.is_none() {
                    self.opponent_score += 1;
                    console::log_1(&"BONUS: Opponent +1 for reaching goal!".into());
                }
            }

            // Update final positions
            self.player_position = player_pos;
            self.opponent_position = opponent_pos;

            console::log_1(&format!("Current Scores - Player: {}, Opponent: {}", 
                self.player_score, self.opponent_score).into());
        }

        // Final round summary
        console::log_1(&"\n========== ROUND COMPLETE ==========".into());
        console::log_1(&format!("Player collision at step: {:?}", self.player_collision_step).into());
        console::log_1(&format!("Opponent collision at step: {:?}", self.opponent_collision_step).into());
        console::log_1(&format!("Player reached goal at step: {:?}", player_goal_step).into());
        console::log_1(&format!("Opponent reached goal at step: {:?}", opponent_goal_step).into());
        console::log_1(&format!("Final Player Position: {:?}", self.player_position).into());
        console::log_1(&format!("Final Opponent Position: {:?}", self.opponent_position).into());
        console::log_1(&format!("Final Scores - Player: {}, Opponent: {}", 
        self.player_score, self.opponent_score).into());
    }

}