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
    
    fn calculate_step_score(&self, from_row: usize, to_row: usize) -> i32 {
        // Points for moving closer to goal
        if to_row < from_row {
            1
        } else {
            0
        }
    }

    fn check_trap_collision(&self, position: (usize, usize), opponent_board: &Board, step: usize) -> bool {
        let (row, col) = position;
        
        // Check existing traps
        if opponent_board.grid[row][col] == CellContent::Trap {
            return true;
        }

        // Check if trap is being placed this turn
        if step < opponent_board.sequence.len() {
            let (trap_row, trap_col, content) = opponent_board.sequence[step].clone();
            let (rotated_row, rotated_col) = self.rotate_position(trap_row, trap_col);
            if matches!(content, CellContent::Trap) && rotated_row == row && rotated_col == col {
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

    pub fn process_turn(&mut self, player_board: &Board, opponent_board: &Board) {
        let mut player_pos = None;
        let mut opponent_pos = None;

        for step in 0..player_board.sequence.len().max(opponent_board.sequence.len()) {
            // Process player move if available
            if step < player_board.sequence.len() && self.player_collision_step.is_none() {
                let (row, col, content) = player_board.sequence[step].clone();
                match content {
                    CellContent::Player => {
                        // Calculate score before updating position
                        if let Some((old_row, _)) = player_pos {
                            self.player_score += self.calculate_step_score(old_row, row);
                        }
                        player_pos = Some((row, col));
                    },
                    CellContent::Trap => {},
                    _ => {}
                }
            }

            // Process opponent move if available
            if step < opponent_board.sequence.len() && self.opponent_collision_step.is_none() {
                let (row, col, content) = opponent_board.sequence[step].clone();
                let (rot_row, rot_col) = self.rotate_position(row, col);
                match content {
                    CellContent::Player => {
                        if let Some((old_row, _)) = opponent_pos {
                            let score_change = self.calculate_step_score(old_row, rot_row);
                            self.opponent_score += score_change;
                            console::log_1(&format!("Opponent score change: +{} (moved from row {} to {})", 
                                score_change, old_row, rot_row).into());
                        }
                        opponent_pos = Some((rot_row, rot_col));
                    },
                    CellContent::Trap => {},
                    _ => {}
                }
            }

            // Check collisions
            if self.player_collision_step.is_none() {
                if self.check_piece_collision(player_pos, opponent_pos) {
                    self.player_collision_step = Some(step);
                    self.opponent_collision_step = Some(step);
                } else if let Some(pos) = player_pos {
                    if self.check_trap_collision(pos, opponent_board, step) {
                        self.player_collision_step = Some(step);
                    }
                }
            }

            if self.opponent_collision_step.is_none() {
                if let Some(pos) = opponent_pos {
                    if self.check_trap_collision(pos, player_board, step) {
                        self.opponent_collision_step = Some(step);
                    }
                }
            }

            // Update current positions
            self.player_position = player_pos;
            self.opponent_position = opponent_pos;

            // Check for reaching goal (row 0 for player, row size-1 for opponent)
            if let Some((row, _)) = player_pos {
                if row == 0 {
                    self.player_score += 1; // Bonus point for reaching goal
                    break;
                }
            }
            if let Some((row, _)) = opponent_pos {
                if row == self.size - 1 {
                    self.opponent_score += 1; // Bonus point for reaching goal
                    break;
                }
            }
        }

        console::log_1(&format!("Final scores - Player: {}, Opponent: {}", 
        self.player_score, self.opponent_score).into());

    }
}