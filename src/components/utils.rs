use web_sys::window;
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
                    r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(220, 38, 38)"/>
                       <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                    x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                );
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

    // Track collisions
    let mut player_collision = None;
    let mut opponent_collision = None;

    // Process moves until collision
    for (idx, move_pair) in player_board.sequence.iter()
        .zip(opponent_board.sequence.iter().chain(std::iter::repeat(&(0,0,CellContent::Empty))))
        .enumerate()
    {
        let (player_move, opponent_move) = move_pair;

        // Skip if both collisions occurred
        if player_collision.is_some() && opponent_collision.is_some() {
            break;
        }

        // Process player move if no collision yet
        if player_collision.is_none() {
            let (i, j, ref content) = *player_move;
            let x = j as f32 * 45.0;
            let y = i as f32 * 45.0;

            // Check for collision with opponent
            let (op_row, op_col, _) = *opponent_move;
            let (op_row, op_col) = rotate_position(op_row, op_col, player_board.grid.len());
            if opponent_board.grid[op_row][op_col] == CellContent::Trap {
                player_collision = Some(idx);
            }

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

        // Process opponent move if no collision yet
        if opponent_collision.is_none() {
            let (i, j, ref content) = *opponent_move;
            let x = j as f32 * 45.0;
            let y = i as f32 * 45.0;

            // Check for collision with player's trap
            let (pl_row, pl_col, _) = *player_move;            
            let (pl_row, pl_col) = rotate_position(pl_row, pl_col, opponent_board.grid.len());
            if player_board.grid[pl_row][pl_col] == CellContent::Trap {
                opponent_collision = Some(idx);
            }

            match content {
                CellContent::Player => {
                    let _ = write!(
                        svg,
                        r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(220, 38, 38)"/>
                           <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
                        x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
                    );
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

    svg.push_str("</g></svg>");
    format!(r#"data:image/svg+xml,{}"#, urlencoding::encode(&svg))
}