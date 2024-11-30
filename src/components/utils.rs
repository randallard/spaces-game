use web_sys::window;

use std::fmt::Write;
use super::board::{Board, CellContent, SavedBoard};

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

    // Draw numbered circles for moves
    for (idx, &(i, j)) in board.moves.iter().enumerate() {
        let x = j as f32 * 45.0;
        let y = i as f32 * 45.0;
        let _ = write!(
            svg,
            r#"<circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(37, 99, 235)"/>
               <text x="{:.0}" y="{:.0}" font-size="16" fill="white" text-anchor="middle" dy=".3em">{}</text>"#,
            x + 20.0, y + 20.0, x + 20.0, y + 20.0, idx + 1
        );
    }

    svg.push_str("</g></svg>");
    format!(r#"data:image/svg+xml,{}"#, urlencoding::encode(&svg))
}

pub fn save_board(board: Board) -> Result<(), serde_json::Error> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let thumbnail = generate_thumbnail(&board);
    let saved_board = SavedBoard { board, thumbnail };    

    // Load existing boards first
    let mut saved_boards = load_saved_boards().unwrap_or_default();
    web_sys::console::log_1(&format!("Current boards: {}", saved_boards.len()).into());
    
    saved_boards.push(saved_board);
    web_sys::console::log_1(&format!("After adding: {}", saved_boards.len()).into());
    
    let json = serde_json::to_string(&saved_boards)?;
    storage.set_item("saved_boards", &json).unwrap();
    Ok(())
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