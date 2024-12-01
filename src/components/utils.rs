use web_sys::{console, window};
use leptos::prelude::*;
use std::fmt::Write;
use super::board::{Board, CellContent, SavedBoard};

pub fn generate_opponent_thumbnail(board: &Board) -> String {
    let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
            <rect width="100" height="100" fill="rgb(30, 41, 59)"/>
            <g transform="translate(5,5)">"#);

    // Draw grid
    for i in 0..board.grid.len() {
        for j in 0..board.grid[i].len() {
            let x = j as f32 * 45.0;
            let y = i as f32 * 45.0;
            let _ = write!(
                svg,
                r#"<rect x="{}" y="{}" width="40" height="40" fill="rgb(51, 65, 85)"/>"#,
                x, y
            );
        }
    }

    // Draw pieces and traps with rotation
    for (idx, &(i, j, ref content)) in board.sequence.iter().enumerate() {
        // Rotate the position 180 degrees
        let size = board.grid.len();
        let (rot_i, rot_j) = (size - 1 - i, size - 1 - j);
        let x = rot_j as f32 * 45.0;
        let y = rot_i as f32 * 45.0;

        match content {
            CellContent::Player => {
                let _ = write!(
                    svg,
                    r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(147, 51, 234)"/>
                    <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                    x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                );
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

    svg.push_str("</g></svg>");
    format!(r#"data:image/svg+xml,{}"#, urlencoding::encode(&svg))
}

pub fn generate_thumbnail(board: &Board) -> String {
    let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
            <rect width="100" height="100" fill="rgb(30, 41, 59)"/>
            <g transform="translate(5,5)">"#);

    // Draw grid and traps
    for (i, row) in board.grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let x = j as f32 * 45.0;
            let y = i as f32 * 45.0;
            let _ = match cell {
                CellContent::Empty | CellContent::Player => write!(
                    svg,
                    r#"<rect x="{}" y="{}" width="40" height="40" fill="rgb(51, 65, 85)"/>"#,
                    x, y
                ),
                CellContent::Trap => write!(
                    svg,
                    r#"<rect x="{}" y="{}" width="40" height="40" fill="rgb(51, 65, 85)"/>
                       <path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(220, 38, 38)" stroke-width="4"/>"#,
                    x, y, x + 5.0, y + 5.0
                ),
            };
        }
    }

    for (idx, &(i, j, ref content)) in board.sequence.iter().enumerate() {
        let x = j as f32 * 45.0;
        let y = i as f32 * 45.0;
        let _ = match content {
            CellContent::Player => {
                write!(
                    svg,
                    r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(37, 99, 235)"/>
                       <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                    x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                )
            },
            CellContent::Trap => {
                write!(
                    svg,
                    r#"<circle cx="{:.0}" cy="{:.0}" r="12" fill="rgb(220, 38, 38)"/>
                       <text x="{:.0}" y="{:.0}" font-size="14" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                    x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                )
            },
            _ => Ok(()),
        };
    }

    svg.push_str("</g></svg>");
    format!(r#"data:image/svg+xml,{}"#, urlencoding::encode(&svg))
}

pub fn save_board(board: Board) -> Result<Vec<SavedBoard>, serde_json::Error> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let thumbnail = generate_thumbnail(&board);
    let saved_board = SavedBoard { board, thumbnail };    

    // Load existing boards first
    let mut saved_boards = load_saved_boards().unwrap_or_default();
    saved_boards.push(saved_board);
    
    let json = serde_json::to_string(&saved_boards)?;
    storage.set_item("saved_boards", &json).unwrap();

    Ok(saved_boards)
}

pub fn load_saved_boards() -> Option<Vec<SavedBoard>> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let data = storage.get_item("saved_boards").ok()??;
    serde_json::from_str(&data).ok()
}

// In utils.rs, add this function:
pub fn delete_board(index: usize) -> Result<(), serde_json::Error> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let mut saved_boards = load_saved_boards().unwrap_or_default();
    saved_boards.remove(index);
    let json = serde_json::to_string(&saved_boards)?;
    storage.set_item("saved_boards", &json).unwrap();
    Ok(())
}

pub fn generate_game_board(player_board: &Board, opponent_board: &Board) -> String {
    console::log_1(&"=== New Game Board Generation ===".into());
    console::log_1(&"Player Board sequence:".into());
    for (idx, (i, j, content)) in player_board.sequence.iter().enumerate() {
        console::log_1(&format!("Step {}: row {} col {} {:?}", idx+1, i, j, content).into());
    }

    console::log_1(&"\nOpponent Board sequence:".into());
    for (idx, (i, j, content)) in opponent_board.sequence.iter().enumerate() {
        console::log_1(&format!("Step {}: row {} col {} {:?}", idx+1, i, j, content).into());
    }

    console::log_1(&"\nCollision Processing:".into());

    let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
            <rect width="100" height="100" fill="rgb(30, 41, 59)"/>
            <g transform="translate(5,5)">"#);

    // Draw grid
    for i in 0..player_board.grid.len() {
        for j in 0..player_board.grid[i].len() {
            let x = j as f32 * 45.0;
            let y = i as f32 * 45.0;
            let _ = write!(
                svg,
                r#"<rect x="{}" y="{}" width="40" height="40" fill="rgb(51, 65, 85)"/>"#,
                x, y
            );
        }
    }

    // Function to calculate opponent's rotated position
    let rotate_position = |row: usize, col: usize, size: usize| -> (usize, usize) {
        (size - 1 - row, size - 1 - col)
    };

    // Track current positions and collisions
    let mut player_current_pos = None;
    let mut opponent_current_pos = None;
    let mut player_collision = None;
    let mut opponent_collision = None;

    // Function to draw collision starburst
    let draw_collision_starburst = |svg: &mut String, x: f32, y: f32, idx: usize| {
        let _ = write!(
            svg,
            r#"
            <g>
                <!-- Blue rays (player) -->
                <path d="M{x} {y} l-15 0 M{x} {y} l15 0"
                      stroke="rgb(37, 99, 235)" stroke-width="4">
                    <animate attributeName="stroke-width" 
                             values="4;8;4" 
                             dur="0.5s" 
                             repeatCount="2"/>
                </path>
                <!-- Purple rays (opponent) -->
                <path d="M{x} {y} l0 -15 M{x} {y} l0 15"
                      stroke="rgb(147, 51, 234)" stroke-width="4">
                    <animate attributeName="stroke-width" 
                             values="4;8;4" 
                             dur="0.5s" 
                             repeatCount="2"/>
                </path>
                <!-- Blue diagonal rays -->
                <path d="M{x} {y} l-10 -10 M{x} {y} l10 10"
                      stroke="rgb(37, 99, 235)" stroke-width="4">
                    <animate attributeName="stroke-width" 
                             values="4;8;4" 
                             dur="0.5s" 
                             repeatCount="2"/>
                </path>
                <!-- Purple diagonal rays -->
                <path d="M{x} {y} l-10 10 M{x} {y} l10 -10"
                      stroke="rgb(147, 51, 234)" stroke-width="4">
                    <animate attributeName="stroke-width" 
                             values="4;8;4" 
                             dur="0.5s" 
                             repeatCount="2"/>
                </path>
                <text x="{x}" y="{y}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{idx}</text>
            </g>
            "#,
            x = x + 20.0, y = y + 20.0, idx = idx + 1
        );
    };

    // Process moves until collision
    for (idx, move_pair) in player_board.sequence.iter()
        .zip(opponent_board.sequence.iter().chain(std::iter::repeat(&(0,0,CellContent::Empty))))
        .enumerate()
    {
        let (player_move, opponent_move) = move_pair;

        console::log_1(&format!("\nProcessing Step {}:", idx + 1).into());
        console::log_1(&format!("Player current pos: {:?}", player_current_pos).into());
        console::log_1(&format!("Opponent current pos: {:?}", opponent_current_pos).into());

        // Skip if both collisions occurred
        if player_collision.is_some() && opponent_collision.is_some() {
            break;
        }

        // Process player move if no collision yet
        if player_collision.is_none() {
            let (i, j, ref content) = *player_move;
            
            match content {
                CellContent::Player => {
                    player_current_pos = Some((i, j));
                },
                CellContent::Trap => {},
                _ => {}
            }

            // Check for collision using current position
            if let Some((player_i, player_j)) = player_current_pos {
                // Check if current player position collides with opponent position
                if let Some((opp_i, opp_j)) = opponent_current_pos {
                    if player_i == opp_i && player_j == opp_j {
                        player_collision = Some(idx);
                        console::log_1(&format!("Player collision with opponent at step {}", idx + 1).into());
                        
                        let x = j as f32 * 45.0 + 20.0;
                        let y = i as f32 * 45.0 + 20.0;
                        draw_collision_starburst(&mut svg, x, y, idx);
                        continue;
                    }
                }
                
                // Check if current player position hits a trap
                let (op_i, op_j, ref op_content) = *opponent_move;
                let (op_row, op_col) = rotate_position(op_i, op_j, opponent_board.grid.len());
                if opponent_board.grid[op_row][op_col] == CellContent::Trap || 
                   matches!(op_content, CellContent::Trap) && op_row == player_i && op_col == player_j {
                    player_collision = Some(idx);
                    console::log_1(&format!("Player collision with trap at step {}", idx + 1).into());
                    continue;
                }
            }
            
            // Draw the piece or trap if no collision or it's the collision point
            let x = j as f32 * 45.0;
            let y = i as f32 * 45.0;
            
            if player_collision == Some(idx) {
                // Draw trap collision
                let _ = write!(
                    svg,
                    r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(249, 115, 22)" stroke-width="4"/>
                       <circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(37, 99, 235)"/>
                       <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                    x + 5.0, y + 5.0,
                    x + 20.0, y + 20.0,
                    x + 20.0, y + 20.0, idx + 1
                );
            } else if player_collision.is_none() {
                match content {
                    CellContent::Player => {
                        let _ = write!(
                            svg,
                            r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(37, 99, 235)"/>
                               <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                        );
                    },
                    CellContent::Trap => {
                        let _ = write!(
                            svg,
                            r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(37, 99, 235)" stroke-width="4"/>"#,
                            x + 5.0, y + 5.0
                        );
                    },
                    _ => {}
                }
            }
        }

        // Process opponent move if no collision yet
        if opponent_collision.is_none() {
            let (i, j, ref content) = *opponent_move;
            let (rot_i, rot_j) = rotate_position(i, j, opponent_board.grid.len());
            
            match content {
                CellContent::Player => {
                    opponent_current_pos = Some((rot_i, rot_j));
                },
                CellContent::Trap => {},
                _ => {}
            }

            // Check for collision using current position
            if let Some((opp_i, opp_j)) = opponent_current_pos {
                // Check if current opponent position collides with player position
                if let Some((player_i, player_j)) = player_current_pos {
                    if opp_i == player_i && opp_j == player_j {
                        opponent_collision = Some(idx);
                        console::log_1(&format!("Opponent collision with player at step {}", idx + 1).into());
                        
                        let x = rot_j as f32 * 45.0 + 20.0;
                        let y = rot_i as f32 * 45.0 + 20.0;
                        draw_collision_starburst(&mut svg, x, y, idx);
                        continue;
                    }
                }
                
                // Check if current opponent position hits a trap
                let (pl_i, pl_j, ref pl_content) = *player_move;
                if player_board.grid[opp_i][opp_j] == CellContent::Trap || 
                   matches!(pl_content, CellContent::Trap) && pl_i == i && pl_j == j {
                    opponent_collision = Some(idx);
                    console::log_1(&format!("Opponent collision with trap at step {}", idx + 1).into());
                    continue;
                }
            }
            
            // Draw the piece or trap if no collision or it's the collision point
            let x = rot_j as f32 * 45.0;
            let y = rot_i as f32 * 45.0;
            
            if opponent_collision == Some(idx) {
                // Draw trap collision
                let _ = write!(
                    svg,
                    r#"<path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(37, 99, 235)" stroke-width="4"/>
                       <circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(147, 51, 234)"/>
                       <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                    x + 5.0, y + 5.0,
                    x + 20.0, y + 20.0,
                    x + 20.0, y + 20.0, idx + 1
                );
            } else if opponent_collision.is_none() {
                match content {
                    CellContent::Player => {
                        let _ = write!(
                            svg,
                            r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(147, 51, 234)"/>
                               <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                            x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                        );
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
    }

    console::log_1(&"\nFinal State:".into());
    console::log_1(&format!("Player collision at step: {:?}", player_collision.map(|x| x + 1)).into());
    console::log_1(&format!("Opponent collision at step: {:?}", opponent_collision.map(|x| x + 1)).into());

    svg.push_str("</g></svg>");
    format!(r#"data:image/svg+xml,{}"#, urlencoding::encode(&svg))
}