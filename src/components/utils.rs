use super::board::{Board, CellContent};

pub fn generate_thumbnail(board: &Board) -> String {
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
            <rect width="100" height="100" fill="rgb(30, 41, 59)"/>
            <g transform="translate(5,5)">{}</g>
        </svg>"#,
        board.grid.iter().enumerate().map(|(i, row)| {
            row.iter().enumerate().map(|(j, cell)| {
                let x = j as f32 * 45.0;
                let y = i as f32 * 45.0;
                match cell {
                    CellContent::Empty => format!(
                        r#"<rect x="{}" y="{}" width="40" height="40" fill="rgb(51, 65, 85)"/>"#,
                        x, y
                     ),
                     CellContent::Player => format!(
                        r#"<rect x="{}" y="{}" width="40" height="40" fill="rgb(51, 65, 85)"/>
                           <circle cx="{:.0}" cy="{:.0}" r="15" fill="rgb(37, 99, 235)"/>"#,
                        x, y, x + 20.0, y + 20.0
                     ),
                     CellContent::Trap => format!(
                        r#"<rect x="{}" y="{}" width="40" height="40" fill="rgb(51, 65, 85)"/>
                           <path d="M{} {} l30 30 m0 -30 l-30 30" stroke="rgb(220, 38, 38)" stroke-width="4"/>"#,
                        x, y, x + 5.0, y + 5.0
                     ),
                }
            }).collect::<String>()
        }).collect::<String>()
    );

    format!(
        r#"data:image/svg+xml,{}"#,
        urlencoding::encode(&svg)
    )
}
