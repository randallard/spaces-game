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
    pub player_round_ended: bool,
    pub opponent_round_ended: bool,
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
            squares,
            player_round_ended: false,
            opponent_round_ended: false,
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
                
                // Draw player moves
                if !square.player_visits.is_empty() {
                    let step = square.player_visits[0]; // Use first visit for numbering
                    let _ = write!(
                        svg,
                        r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(37, 99, 235)"/>
                        <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                        x + 20.0, y + 20.0, x + 20.0, y + 20.0, step + 1
                    );
                }
    
                // Draw opponent moves
                if !square.opponent_visits.is_empty() {
                    let step = square.opponent_visits[0]; // Use first visit for numbering
                    let _ = write!(
                        svg,
                        r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(147, 51, 234)"/>
                        <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                        x + 20.0, y + 20.0, x + 20.0, y + 20.0, step + 1
                    );
                }
    
                // Draw player traps
                if let Some(step) = square.player_trap_step {
                    let _ = write!(
                        svg,
                        r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(220, 38, 38)" stroke-width="4"/>"#,
                        x + 5.0, y + 5.0
                    );
                }
    
                // Draw opponent traps
                if let Some(step) = square.opponent_trap_step {
                    let _ = write!(
                        svg,
                        r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(249, 115, 22)" stroke-width="4"/>"#,
                        x + 5.0, y + 5.0
                    );
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