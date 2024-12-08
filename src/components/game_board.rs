use std::fmt::Write;
use crate::components::board::{Board, CellContent};

use web_sys::console;

#[derive(Debug, Clone)]
struct Square {
    position: Option<(usize,usize)>,
    player_trap_step: Option<usize>,
    opponent_trap_step: Option<usize>,
    player_visits: Vec<usize>,
    opponent_visits: Vec<usize>,
    collision_step: Option<usize>,
    player_trap_hit_step: Option<usize>,
    opponent_trap_hit_step: Option<usize>,
    player_forward_move: bool,
    opponent_forward_move: bool,
}

impl Square {
    fn new(row: usize, col: usize) -> Self {
        Square {
            position: Some((row,col)),
            player_trap_step: None,
            opponent_trap_step: None,
            player_visits: Vec::new(),
            opponent_visits: Vec::new(),
            collision_step: None,
            player_trap_hit_step: None,
            opponent_trap_hit_step: None,
            player_forward_move: false,
            opponent_forward_move: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameBoard {
    squares: Vec<Vec<Square>>,
    pub size: usize,
    pub player_sequence: Vec<(usize, usize, CellContent)>,
    pub opponent_sequence: Vec<(usize, usize, CellContent)>,
    pub player_position: Option<(usize, usize)>,
    pub opponent_position: Option<(usize, usize)>,
    pub player_collision_step: Option<usize>,
    pub opponent_collision_step: Option<usize>,
    pub player_score: i32,
    pub opponent_score: i32,
    pub player_round_ended: bool,
    pub opponent_round_ended: bool,
    pub player_goal_reached: bool,
    pub opponent_goal_reached: bool,
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
            squares,
            player_round_ended: false,
            opponent_round_ended: false,
            player_goal_reached: false,
            opponent_goal_reached: false,
        }
    }

    pub fn generate_board_svg(&self) -> String {
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

        

        // Draw collision markers
        for row in 0..self.size {
            for col in 0..self.size {
                if let Some(square) = self.squares.get(row).and_then(|r| r.get(col)) {
                    if square.collision_step.is_some() {
                        let x = col as f32 * 45.0 + 20.0;
                        let y = row as f32 * 45.0;
                        
                        // Top center
                        let _ = write!(svg, r#"<text x="{}" y="{}" font-size="20" fill="rgb(220, 38, 38)" text-anchor="middle">*</text>"#, x, y);
                        
                        // Bottom center
                        let _ = write!(svg, r#"<text x="{}" y="{}" font-size="20" fill="rgb(249, 115, 22)" text-anchor="middle">*</text>"#, x, y + 40.0);
                        
                        // Left center
                        let _ = write!(svg, r#"<text x="{}" y="{}" font-size="20" fill="rgb(220, 38, 38)" text-anchor="middle">*</text>"#, x - 20.0, y + 20.0);
                        
                        // Right center
                        let _ = write!(svg, r#"<text x="{}" y="{}" font-size="20" fill="rgb(249, 115, 22)" text-anchor="middle">*</text>"#, x + 20.0, y + 20.0);
                    }
                }
            }
        }
    
        // Draw pieces and traps based on squares
        for i in 0..self.size {
            for j in 0..self.size {
                let square = &self.squares[i][j];
                let x = j as f32 * 45.0;
                let y = i as f32 * 45.0;
                
                // Get valid visits (before collision)
                let player_visits: Vec<&usize> = square.player_visits.iter()
                    .filter(|&&step| step <= self.player_collision_step.unwrap_or(usize::MAX))
                    .collect();
                let opponent_visits: Vec<&usize> = square.opponent_visits.iter()
                    .filter(|&&step| step <= self.opponent_collision_step.unwrap_or(usize::MAX))
                    .collect();
                
                let has_player = !player_visits.is_empty();
                let has_opponent = !opponent_visits.is_empty();
                
                if has_player && has_opponent {
                    let center_x = x + 20.0;
                    let center_y = y + 20.0;
                    let radius = 15.0;
                    let vertical_offset = -15.0;  // Move everything up
                
                    // Blue half circle (left)
                    let _ = write!(
                        svg,
                        r#"<path d="M {},{} a {},{} 0 0 1 {},0 v {} a {},{} 0 0 1 -{},0" fill="rgb(37, 99, 235)"/>"#,
                        center_x - radius, center_y + vertical_offset,
                        radius, radius, 
                        radius, radius * 2.0,
                        radius, radius,
                        radius
                    );
                
                    // Purple half circle (right)
                    let _ = write!(
                        svg,
                        r#"<path d="M {},{} a {},{} 0 0 0 -{},0 v {} a {},{} 0 0 0 {},0" fill="rgb(147, 51, 234)"/>"#,
                        center_x + radius, center_y + vertical_offset,
                        radius, radius,
                        radius, radius * 2.0,
                        radius, radius,
                        radius
                    );
                
                    // Numbers
                    let player_step = player_visits.iter().max().unwrap();
                    let opponent_step = opponent_visits.iter().max().unwrap();
                    
                    let _ = write!(
                        svg,
                        r#"<text x="{}" y="{}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>
                           <text x="{}" y="{}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                        center_x - radius/2.0, center_y, *player_step  + 1,
                        center_x + radius/2.0, center_y, *opponent_step + 1 
                    );
                } else {
                    // Draw single player circle if only player visited
                    if has_player {
                        let step = player_visits.iter().max().unwrap();
                        let _ = write!(
                            svg,
                            r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(37, 99, 235)"/>
                            <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 20.0, y + 20.0, x + 20.0, y + 20.0, *step + 1
                        );
                    }
                    
                    // Draw single opponent circle if only opponent visited
                    if has_opponent {
                        let step = opponent_visits.iter().max().unwrap();
                        let _ = write!(
                            svg,
                            r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(147, 51, 234)"/>
                            <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 20.0, y + 20.0, x + 20.0, y + 20.0, *step + 1
                        );
                    }
                }                
                
                // Replace the existing trap drawing code with this combined approach
                // First check if both traps exist in the square
                if let (Some(p_trap), Some(o_trap)) = (square.player_trap_step, square.opponent_trap_step) {
                    if p_trap <= self.player_collision_step.unwrap_or(usize::MAX) &&
                    o_trap <= self.opponent_collision_step.unwrap_or(usize::MAX) {
                        let _ = write!(
                            svg,
                            r#"<g transform="translate({} {}) rotate(3 15 15)">
                                <path d="M0 0 l30 30 m0 -30 l-30 30" stroke="rgb(220, 38, 38)" stroke-width="4" opacity="0.6"/>
                            </g>
                            <text x="{:.0}" y="{:.0}" font-size="16" fill="rgb(220, 38, 38)" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 5.0, y + 5.0, x + 5.0, y + 20.0, p_trap + 1
                        );
                        let _ = write!(
                            svg,
                            r#"<g transform="translate({} {}) rotate(-3 15 15)">
                                <path d="M0 0 l30 30 m0 -30 l-30 30" stroke="rgb(249, 115, 22)" stroke-width="4" opacity="0.6"/>
                            </g>
                            <text x="{:.0}" y="{:.0}" font-size="16" fill="rgb(249, 115, 22)" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 5.0, y + 5.0, x + 35.0, y + 20.0, o_trap + 1
                        );
                    }
                } else {
                    // Draw single traps as before
                    if let Some(trap_step) = square.player_trap_step {
                        if trap_step <= self.player_collision_step.unwrap_or(usize::MAX) {
                            let _ = write!(
                                svg,
                                r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(220, 38, 38)" stroke-width="4" opacity="0.6"/>"
                                <text x="{:.0}" y="{:.0}" font-size="16" fill="rgb(220, 38, 38)" text-anchor="middle" dy=".3em">{}</text>"#,
                                x + 5.0, y + 5.0, x + 35.0, y + 20.0, trap_step + 1
                            );
                        }
                    }

                    if let Some(trap_step) = square.opponent_trap_step {
                        if trap_step <= self.opponent_collision_step.unwrap_or(usize::MAX) {
                            let _ = write!(
                                svg,
                                r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(249, 115, 22)" stroke-width="4" opacity="0.6"/>"
                                <text x="{:.0}" y="{:.0}" font-size="16" fill="rgb(249, 115, 22)" text-anchor="middle" dy=".3em">{}</text>"#,
                                x + 5.0, y + 5.0, x + 35.0, y + 20.0, trap_step + 1
                            );
                        }
                    }
                }
            }
        }

        svg.push_str("</g></svg>");
        format!(r#"data:image/svg+xml,{}"#, urlencoding::encode(&svg))
    }
    
    fn rotate_position(&self, row: usize, col: usize) -> (usize, usize) {
        (self.size - 1 - row, self.size - 1 - col)
    }
    
    pub fn process_turn(&mut self, player_board: &Board, opponent_board: &Board) {
        console::log_1(&"\n====== Starting New Game Round ======".into());
        
        // Log initial sequences
        console::log_1(&"\nPlayer Sequence:".into());
        for (i, &(row, col, ref content)) in player_board.sequence.iter().enumerate() {
            let content_str = match content {
                CellContent::Player => "Move",
                CellContent::Trap => "Trap",
                CellContent::Final => "Final",
                CellContent::Empty => "Empty",
            };
            console::log_1(&format!("Step {}: ({}, {}) - {}", i, row, col, content_str).into());
        }
    
        console::log_1(&"\nOpponent Sequence:".into());
        for (i, &(row, col, ref content)) in opponent_board.sequence.iter().enumerate() {
            let content_str = match content {
                CellContent::Player => "Move",
                CellContent::Trap => "Trap",
                CellContent::Final => "Final",
                CellContent::Empty => "Empty",
            };
            let (rot_row, rot_col) = self.rotate_position(row, col);
            console::log_1(&format!("Step {}: ({}, {}) rotated to ({}, {}) - {}", 
                i, row, col, rot_row, rot_col, content_str).into());
        }
        
        // Reset game state
        self.player_score = 0;
        self.opponent_score = 0;
        self.player_position = None;
        self.opponent_position = None;
        self.player_round_ended = false;
        self.opponent_round_ended = false;
        
        let max_steps = std::cmp::max(
            player_board.sequence.len(),
            opponent_board.sequence.len()
        );
    
        'step_loop: for step in 0..max_steps {
            console::log_1(&format!("\n=== Processing Step {} ===", step).into());
    
            // Process player's move
            if !self.player_round_ended && step < player_board.sequence.len() {
                let (row, col, content) = &player_board.sequence[step];
                match content {
                    CellContent::Player => {
                        console::log_1(&format!("Player moving to ({}, {})", *row, *col).into());
                        if let Some((prev_row, _)) = self.player_position {
                            if prev_row > *row {
                                self.player_score += 1;
                                console::log_1(&format!("Player scored forward move point! Score now {}", 
                                    self.player_score).into());
                            }
                        }
                        self.player_position = Some((*row, *col));
                        self.squares[*row][*col].player_visits.push(step);
                    },
                    CellContent::Trap => {
                        console::log_1(&format!("Player placed trap at ({}, {})", *row, *col).into());
                        self.squares[*row][*col].player_trap_step = Some(step);
                    },
                    CellContent::Final => {
                        console::log_1(&"Player reached goal!".into());
                        self.player_goal_reached = true;
                        self.player_score += 1;
                        console::log_1(&format!("Player scored goal point! Score now {}", 
                            self.player_score).into());
                        self.player_round_ended = true;
                    },
                    _ => {}
                }
            }
    
            // Process opponent's move
            if !self.opponent_round_ended && step < opponent_board.sequence.len() {
                let (row, col, content) = &opponent_board.sequence[step];
                let (rot_row, rot_col) = self.rotate_position(*row, *col);
                
                match content {
                    CellContent::Player => {
                        console::log_1(&format!("Opponent moving to ({}, {})", rot_row, rot_col).into());
                        if let Some((prev_row, _)) = self.opponent_position {
                            if prev_row < rot_row {
                                self.opponent_score += 1;
                                console::log_1(&format!("Opponent scored forward move point! Score now {}", 
                                    self.opponent_score).into());
                            }
                        }
                        self.opponent_position = Some((rot_row, rot_col));
                        self.squares[rot_row][rot_col].opponent_visits.push(step);
                    },
                    CellContent::Trap => {
                        console::log_1(&format!("Opponent placed trap at ({}, {})", rot_row, rot_col).into());
                        self.squares[rot_row][rot_col].opponent_trap_step = Some(step);
                    },
                    CellContent::Final => {
                        console::log_1(&"Opponent reached goal!".into());
                        self.opponent_goal_reached = true;
                        self.opponent_score += 1;
                        console::log_1(&format!("Opponent scored goal point! Score now {}", 
                            self.opponent_score).into());
                        self.opponent_round_ended = true;
                    },
                    _ => {}
                }
            }
    
            // Check for collisions
            if let (Some(p_pos), Some(o_pos)) = (self.player_position, self.opponent_position) {
                if p_pos == o_pos {
                    console::log_1(&format!("\nCOLLISION at square ({}, {})!", p_pos.0, p_pos.1).into());
                    self.squares[p_pos.0][p_pos.1].collision_step = Some(step);
                    
                    if self.player_score > 0 {
                        self.player_score -= 1;
                        console::log_1(&format!("Player lost point from collision! Score now {}", 
                            self.player_score).into());
                    }
                    if self.opponent_score > 0 {
                        self.opponent_score -= 1;
                        console::log_1(&format!("Opponent lost point from collision! Score now {}", 
                            self.opponent_score).into());
                    }
                    break 'step_loop;
                }
            }
    
            // Check for trap hits
            if !self.player_round_ended {
                if let Some((row, col)) = self.player_position {
                    if let Some(trap_step) = self.squares[row][col].opponent_trap_step {
                        if trap_step <= step {
                            console::log_1(&format!("\nPlayer hit opponent trap at ({}, {})!", row, col).into());
                            self.squares[row][col].player_trap_hit_step = Some(step);
                            if self.player_score > 0 {
                                self.player_score -= 1;
                                console::log_1(&format!("Player lost point from trap! Score now {}", 
                                    self.player_score).into());
                            }
                            self.player_round_ended = true;
                        }
                    }
                }
            }
    
            if !self.opponent_round_ended {
                if let Some((row, col)) = self.opponent_position {
                    if let Some(trap_step) = self.squares[row][col].player_trap_step {
                        if trap_step <= step {
                            console::log_1(&format!("\nOpponent hit player trap at ({}, {})!", row, col).into());
                            self.squares[row][col].opponent_trap_hit_step = Some(step);
                            if self.opponent_score > 0 {
                                self.opponent_score -= 1;
                                console::log_1(&format!("Opponent lost point from trap! Score now {}", 
                                    self.opponent_score).into());
                            }
                            self.opponent_round_ended = true;
                        }
                    }
                }
            }
    
            // Stop if both players have ended their round
            if self.player_round_ended && self.opponent_round_ended {
                console::log_1(&"\nBoth players have ended their round".into());
                break 'step_loop;
            }
    
            // Stop if either players has reched their goal
            if self.player_goal_reached || self.opponent_goal_reached {
                console::log_1(&"\nEnding round for goal reached".into());
                break 'step_loop;
            }
        }
    
        console::log_1(&"\n====== Round Summary ======".into());
        console::log_1(&format!("Final player score: {}", self.player_score).into());
        console::log_1(&format!("Final opponent score: {}", self.opponent_score).into());
    }    

}