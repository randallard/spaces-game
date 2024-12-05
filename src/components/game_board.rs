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

#[derive(Clone)]
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
                        center_x - radius/2.0, center_y, *player_step,
                        center_x + radius/2.0, center_y, *opponent_step 
                    );
                } else {
                    // Draw single player circle if only player visited
                    if has_player {
                        let step = player_visits.iter().max().unwrap();
                        let _ = write!(
                            svg,
                            r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(37, 99, 235)"/>
                            <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 20.0, y + 20.0, x + 20.0, y + 20.0, *step
                        );
                    }
                    
                    // Draw single opponent circle if only opponent visited
                    if has_opponent {
                        let step = opponent_visits.iter().max().unwrap();
                        let _ = write!(
                            svg,
                            r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(147, 51, 234)"/>
                            <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 20.0, y + 20.0, x + 20.0, y + 20.0, *step
                        );
                    }
                }                
                
                // Draw traps only if they were set before collision
                if let Some(trap_step) = square.player_trap_step {
                    if trap_step <= self.player_collision_step.unwrap_or(usize::MAX) {
                        let _ = write!(
                            svg,
                            r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(220, 38, 38)" stroke-width="4"/>"
                            <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 5.0, y + 5.0, x + 30.0, y + 30.0, trap_step + 1
                        );
                    }
                }

                if let Some(trap_step) = square.opponent_trap_step {
                    if trap_step <= self.opponent_collision_step.unwrap_or(usize::MAX) {
                        let _ = write!(
                            svg,
                            r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(249, 115, 22)" stroke-width="4"/>"
                            <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 5.0, y + 5.0, x + 30.0, y + 30.0, trap_step + 1
                        );
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
    
    fn process_moves(&mut self) {
        let max_steps = std::cmp::max(
            self.player_sequence.len(),
            self.opponent_sequence.len()
        );
    
        console::log_1(&"\n====== Starting Process Moves ======".into());
        console::log_1(&format!("Processing {} total steps", max_steps).into());
    
        'step_loop: for current_step in 0..max_steps {
            console::log_1(&format!("\n=== Step {} ===", current_step).into());
    
            // Find squares where players are at this step
            let mut player_checking_square = None;
            let mut opponent_checking_square = None;
    
            // Search through all squares to find player positions for this step
            for row in &mut self.squares {
                for square in row {
                    if square.player_visits.contains(&current_step) {
                        player_checking_square = square.position;
                        console::log_1(&format!("Found player at position: {:?}", square.position).into());
                    }
                    if square.opponent_visits.contains(&current_step) {
                        opponent_checking_square = square.position;
                        console::log_1(&format!("Found opponent at position: {:?}", square.position).into());
                    }
                }
            }
    
            // Check for collisions
            if let (Some(p_pos), Some(o_pos)) = (player_checking_square, opponent_checking_square) {
                if p_pos == o_pos {
                    console::log_1(&format!("COLLISION detected at position {:?} on step {}", p_pos, current_step).into());
                    // Update collision in the square where they collided
                    if let Some(square) = self.squares.get_mut(p_pos.0).and_then(|row| row.get_mut(p_pos.1)) {
                        square.collision_step = Some(current_step);
                    }
                    break 'step_loop;
                }
            }
    
            // Process player movement and traps
            if !self.player_round_ended {
                if let Some((row, col)) = player_checking_square {
                    let square = &mut self.squares[row][col];
                    
                    console::log_1(&format!("Checking player at ({}, {})", row, col).into());
                    
                    // Check if player hit opponent trap
                    if let Some(trap_step) = square.opponent_trap_step {
                        if trap_step < self.opponent_collision_step.unwrap_or(usize::MAX) && 
                           trap_step <= current_step {
                            console::log_1(&format!("Player hit opponent trap at ({}, {}) from step {}", row, col, trap_step).into());
                            square.player_trap_hit_step = Some(current_step);
                            self.player_collision_step = Some(current_step);
                        }
                    } else if let Some((prev_row, _)) = self.player_position {
                        // Check for forward movement
                        if prev_row > row {
                            console::log_1(&format!("Player moved forward from row {} to {}", prev_row, row).into());
                            square.player_forward_move = true;
                            self.player_score += 1;
                            console::log_1(&format!("Player score increased to {}", self.player_score).into());
                            self.player_position = Some((row, col));
                        } else {
                            console::log_1(&"Player moved but not forward".into());
                        }
                    }
                }
            }
    
            // Process opponent movement and traps
            if !self.opponent_round_ended {
                if let Some((row, col)) = opponent_checking_square {
                    let square = &mut self.squares[row][col];
                    
                    console::log_1(&format!("Checking opponent at ({}, {})", row, col).into());
                    
                    // Check if opponent hit player trap
                    if let Some(trap_step) = square.player_trap_step {
                        if trap_step < self.player_collision_step.unwrap_or(usize::MAX) && 
                           trap_step <= current_step {
                            console::log_1(&format!("Opponent hit player trap at ({}, {}) from step {}", row, col, trap_step).into());
                            square.opponent_trap_hit_step = Some(current_step);
                            self.opponent_collision_step = Some(current_step);
                        }
                    } else if let Some((prev_row, _)) = self.opponent_position {
                        // Check for forward movement
                        if prev_row < row {
                            console::log_1(&format!("Opponent moved forward from row {} to {}", prev_row, row).into());
                            square.opponent_forward_move = true;
                            self.opponent_score += 1;
                            console::log_1(&format!("Opponent score increased to {}", self.opponent_score).into());
                            self.opponent_position = Some((row, col));
                        } else {
                            console::log_1(&"Opponent moved but not forward".into());
                        }
                    }
                }
            }
    
            // Print square states after each step
            if current_step % 2 == 0 {  // Print every other step to reduce noise
                console::log_1(&"\n--- Square States ---".into());
                for row in &self.squares {
                    for square in row {
                        if let Some((r, c)) = square.position {
                            if !square.player_visits.is_empty() || 
                               !square.opponent_visits.is_empty() ||
                               square.player_trap_step.is_some() ||
                               square.opponent_trap_step.is_some() {
                                console::log_1(&format!("\nSquare ({}, {}): {:#?}", r, c, square).into());
                            }
                        }
                    }
                }
            }
        }
    
        console::log_1(&"\n====== Process Moves Complete ======".into());
        console::log_1(&format!("Final player score: {}", self.player_score).into());
        console::log_1(&format!("Final opponent score: {}", self.opponent_score).into());
    }
    
    pub fn process_turn(&mut self, player_board: &Board, opponent_board: &Board) {
        console::log_1(&"\n====== Starting New Game Round ======".into());

        // Reset game state
        self.player_round_ended = false;
        self.opponent_round_ended = false;
        self.player_position = None;
        self.opponent_position = None;
        self.player_collision_step = None;
        self.opponent_collision_step = None;
        
        // Reset sequences
        self.player_sequence = player_board.sequence.clone();
        self.opponent_sequence = opponent_board.sequence.clone();
        
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
                if let Some((row,col)) = square.position {
                    console::log_1(&format!("\nSquare ({}, {}): {:#?}", row, col, square).into());
                }
            }
        }
    
        self.process_moves();

        console::log_1(&"\n====== Round Summary ======".into());
        console::log_1(&format!("Final player score: {}", self.player_score).into());
        console::log_1(&format!("Final opponent score: {}", self.opponent_score).into());

    }        

}