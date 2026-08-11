use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;

const SIZE: usize = 9;
const BORDER: &str = "│";
const DASH: &str = "─";

fn border_style() -> Style {
    Style::default().fg(Color::Rgb(100, 100, 100))
}

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    render_title(frame, chunks[0]);
    render_status(frame, chunks[1], app);

    // Center board vertically in the available area
    let board_height: u16 = 13;
    let available = chunks[2].height;
    let padding = available.saturating_sub(board_height) / 2;
    let board_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(padding),
            Constraint::Length(board_height),
            Constraint::Min(0),
        ])
        .split(chunks[2]);

    render_board(frame, board_area[1], app);
    render_controls(frame, chunks[3]);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        " SUDOKU ".bold().white(),
        "│".dark_gray(),
        " zxc-sudoku ".gray(),
    ]))
    .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(title, area);
}

fn render_status(frame: &mut Frame, area: Rect, app: &App) {
    let status = if app.won {
        Line::from(vec![
            " PARABENS! Puzzle completado! ".green().bold(),
            " │ ".dark_gray(),
            " R ".yellow().bold(),
            " para novo jogo".gray(),
        ])
    } else {
        Line::from(vec![
            " Dificuldade: ".gray(),
            app.difficulty.label().cyan().bold(),
            " │ ".dark_gray(),
            " Celula: ".gray(),
            format!("({},{})", app.cursor_row + 1, app.cursor_col + 1)
                .yellow()
                .bold(),
            " │ ".dark_gray(),
            " D ".yellow().bold(),
            " trocar dificuldade".gray(),
        ])
    };

    let paragraph = Paragraph::new(status).alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(paragraph, area);
}

fn render_board(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines = Vec::new();
    let bs = border_style();

    // Pre-compute which rows, cols, blocks have duplicates
    let mut row_dup = [false; SIZE];
    let mut col_dup = [false; SIZE];
    let mut block_dup = [[false; 3]; 3];
    for (r, row_dup) in row_dup.iter_mut().enumerate() {
        *row_dup = app.game.row_has_duplicate(r);
    }
    for (c, col_dup) in col_dup.iter_mut().enumerate() {
        *col_dup = app.game.col_has_duplicate(c);
    }
    for (br, block_row) in block_dup.iter_mut().enumerate() {
        for (bc, block_cell) in block_row.iter_mut().enumerate() {
            *block_cell = app.game.block_has_duplicate(br * 3, bc * 3);
        }
    }

    // Top border: ┌─────────┬─────────┬─────────┐
    lines.push(Line::from(vec![
        Span::styled("┌", bs),
        Span::styled(format!("{DASH}{}", DASH.repeat(8)), bs),
        Span::styled("┬", bs),
        Span::styled(format!("{DASH}{}", DASH.repeat(8)), bs),
        Span::styled("┬", bs),
        Span::styled(format!("{DASH}{}", DASH.repeat(8)), bs),
        Span::styled("┐", bs),
    ]));

    for r in 0..SIZE {
        let mut spans = Vec::new();
        spans.push(Span::styled(BORDER, bs));

        for c in 0..SIZE {
            let num = app.game.board[r][c];
            let is_initial = app.game.initial[r][c];
            let is_selected = r == app.cursor_row && c == app.cursor_col;
            let is_same_row = r == app.cursor_row;
            let is_same_col = c == app.cursor_col;
            let is_same_block = (r / 3 == app.cursor_row / 3) && (c / 3 == app.cursor_col / 3);
            let is_highlighted = is_same_row || is_same_col || is_same_block;

            let has_dup = num != 0 && app.game.cell_has_duplicate(r, c);
            let row_err = row_dup[r];
            let col_err = col_dup[c];
            let block_err = block_dup[r / 3][c / 3];

            let display = if num == 0 {
                " "
            } else {
                // Safety: num is 1-9
                match num {
                    1 => "1",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    5 => "5",
                    6 => "6",
                    7 => "7",
                    8 => "8",
                    _ => "9",
                }
            };

            let mut style = Style::default();

            if has_dup {
                // Cell itself has a duplicate number
                style = style.fg(Color::Red).add_modifier(Modifier::BOLD);
            } else if is_initial {
                style = style.fg(Color::White).add_modifier(Modifier::BOLD);
            } else if num != 0 {
                style = style.fg(Color::Cyan);
            }

            // Background: red tint if row/col/block has duplicates, else cursor highlight
            if is_selected {
                style = style.bg(Color::Rgb(60, 60, 100));
            } else if row_err || col_err || block_err {
                style = style.bg(Color::Rgb(80, 20, 20));
            } else if is_highlighted {
                style = style.bg(Color::Rgb(30, 30, 50));
            }

            // Each cell: " N " = 3 chars
            spans.push(Span::styled(format!(" {display} "), style));

            // Vertical separator between 3x3 blocks (after col 2, 5)
            if c == 2 || c == 5 {
                spans.push(Span::styled(BORDER, bs));
            }
        }

        spans.push(Span::styled(BORDER, bs));
        lines.push(Line::from(spans));

        // Horizontal separator between 3x3 blocks (after row 2, 5)
        if r == 2 || r == 5 {
            lines.push(Line::from(vec![
                Span::styled("├", bs),
                Span::styled(format!("{DASH}{}", DASH.repeat(8)), bs),
                Span::styled("┼", bs),
                Span::styled(format!("{DASH}{}", DASH.repeat(8)), bs),
                Span::styled("┼", bs),
                Span::styled(format!("{DASH}{}", DASH.repeat(8)), bs),
                Span::styled("┤", bs),
            ]));
        }
    }

    // Bottom border: └─────────┴─────────┴─────────┘
    lines.push(Line::from(vec![
        Span::styled("└", bs),
        Span::styled(format!("{DASH}{}", DASH.repeat(8)), bs),
        Span::styled("┴", bs),
        Span::styled(format!("{DASH}{}", DASH.repeat(8)), bs),
        Span::styled("┴", bs),
        Span::styled(format!("{DASH}{}", DASH.repeat(8)), bs),
        Span::styled("┘", bs),
    ]));

    let board = Paragraph::new(lines).alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(board, area);
}

fn render_controls(frame: &mut Frame, area: Rect) {
    let controls = Line::from(vec![
        " Setas ".yellow().bold(),
        "navegar  ".gray(),
        " 1-9 ".yellow().bold(),
        "inserir  ".gray(),
        " Del ".yellow().bold(),
        "apagar  ".gray(),
        " H ".yellow().bold(),
        "dica  ".gray(),
        " R ".yellow().bold(),
        "novo  ".gray(),
        " D ".yellow().bold(),
        "dificuldade  ".gray(),
        " Q ".yellow().bold(),
        "sair".gray(),
    ]);

    let paragraph = Paragraph::new(controls).alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(paragraph, area);
}
