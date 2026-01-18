use ratatui::{prelude::*, widgets::*};
use crate::pomo::state::{Pomo, AppScreen, SessionMode, InputMode};

const MOCHA_LAVENDER: Color = Color::Rgb(180, 190, 254);
const MOCHA_OVERLAY0: Color = Color::Rgb(108, 112, 134);
const MOCHA_SURFACE0: Color = Color::Rgb(49, 50, 68);
const MOCHA_TEXT: Color = Color::Rgb(205, 214, 244);

pub fn render(f: &mut Frame, app: &mut Pomo) {
    let main_block = Block::default().style(Style::default().bg(Color::Reset));
    f.render_widget(main_block, f.area());

    match app.screen {
        AppScreen::Timer => {
            let outer_v_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(18),
                    Constraint::Fill(1),
                ])
                .split(f.area());
            render_timer_screen(f, app, outer_v_chunks[1]);
        }
        AppScreen::Tasks => render_task_screen(f, app),
    }
 
    if let InputMode::Insert | InputMode::Edit = app.input_mode {
        render_input_modal(f, app);
    }
}

fn render_timer_screen(f: &mut Frame, app: &Pomo, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Length(5), 
            Constraint::Length(3), 
            Constraint::Length(1), 
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    let priority_text = app.tasks.iter().find(|t| !t.is_done)
        .map(|t| format!("Current Focus: {}", t.title))
        .unwrap_or_else(|| "No Active Tasks".to_string());
 
    f.render_widget(
        Paragraph::new(priority_text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }) // Added wrapping for the top priority bar
            .style(Style::default().fg(MOCHA_LAVENDER).bold()), 
        chunks[0]
    );

    let time_str = format_duration(app.time_remaining);
    let big_text = format_monolithic_ascii(&time_str);
    f.render_widget(
        Paragraph::new(big_text).alignment(Alignment::Center).style(Style::default().fg(MOCHA_LAVENDER)), 
        chunks[2]
    );

    render_session_dots(f, app, chunks[4]);

    let footer = "↑ +1 min  •  space pause  •  r reset  •  s skip  •  t tasks";
    f.render_widget(
        Paragraph::new(footer).alignment(Alignment::Center).style(Style::default().fg(MOCHA_OVERLAY0)), 
        chunks[6]
    );
}

// Fixed-width monolithic ASCII engine
fn format_monolithic_ascii(time: &str) -> Text<'static> {
    let mut lines = vec![String::new(); 5];
    for (idx, c) in time.chars().enumerate() {
        let art = match c {
            '0' => vec![" ██████ ", "██    ██", "██    ██", "██    ██", " ██████ "],
            '1' => vec!["   ██   ", "  ███   ", "   ██   ", "   ██   ", " ██████ "],
            '2' => vec![" ██████ ", "██    ██", "    ███ ", "  ███   ", "████████"],
            '3' => vec![" ██████ ", "██    ██", "  █████ ", "██    ██", " ██████ "],
            '4' => vec!["██    ██", "██    ██", "████████", "      ██", "      ██"],
            '5' => vec!["████████", "██      ", "███████ ", "      ██", "███████ "],
            '6' => vec![" ██████ ", "██      ", "███████ ", "██    ██", " ██████ "],
            '7' => vec!["████████", "      ██", "     ██ ", "    ██  ", "   ██   "],
            '8' => vec![" ██████ ", "██    ██", " ██████ ", "██    ██", " ██████ "],
            '9' => vec![" ██████ ", "██    ██", " ████████", "      ██", " ██████ "],
            ':' => vec!["   █    ", "        ", "   █    ", "        ", "        "], 
            _   => vec!["        "; 5],
        };
        for i in 0..5 {
            lines[i].push_str(art[i]);
            if idx < time.len() - 1 { lines[i].push_str("  "); }
        }
    }
    Text::from(lines.into_iter().map(Line::from).collect::<Vec<_>>())
}

fn render_session_dots(f: &mut Frame, app: &Pomo, area: Rect) {
    let modes = [(SessionMode::Work, "focus"), (SessionMode::ShortBreak, "shortbreak"), (SessionMode::LongBreak, "longbreak")];
    let spans = modes.iter().enumerate().map(|(i, (mode, label))| {
        let is_active = app.mode == *mode;
        let color = if is_active { MOCHA_LAVENDER } else { MOCHA_OVERLAY0 };
        let content = if is_active { format!("• {}", label) } else { label.to_string() };
        let mut s = vec![Span::styled(content, Style::default().fg(color))];
        if i < modes.len() - 1 { s.push(Span::raw("     ")); }
        s
    }).flatten().collect::<Vec<_>>();

    f.render_widget(Paragraph::new(Line::from(spans)).alignment(Alignment::Center), area);
}

pub fn render_task_screen(f: &mut Frame, app: &mut Pomo) {
    let area = centered_rect(60, 80, f.area());

    // Vertical split to house the task list and the footer helper
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Task List takes all available space
            Constraint::Length(1), // Footer takes exactly one line
        ])
        .split(area);

    let items: Vec<ListItem> = app.tasks.iter().map(|t| {
        let symbol = if t.is_done { "󰄲" } else { "󰄱" };
        ListItem::new(Text::from(format!(" {} {}", symbol, t.title)))
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .title(" Focus Priorities ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .padding(Padding::uniform(1)) 
            .border_style(Style::default().fg(MOCHA_LAVENDER)))
        .highlight_style(Style::default().bg(MOCHA_SURFACE0).fg(MOCHA_TEXT).bold())
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.task_state);

    // --- TASK MENU FOOTER ---
    // i: Insert, Enter: Done/Save, e: Edit, d: Delete
    let footer_text = "i insert • ⏎ toggle • e edit • d delete • t back";
    f.render_widget(
        Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(MOCHA_OVERLAY0)),
        chunks[1]
    );
}

pub fn render_input_modal(f: &mut Frame, app: &Pomo) {
    // We use a fixed height of 7 to ensure the "center" feels spacious
    let area = centered_rect(45, 7, f.area());
    f.render_widget(Clear, area); 

    let title = match app.input_mode { 
        InputMode::Insert => " New Task ", 
        InputMode::Edit => " Edit Task ", 
        _ => " Input " 
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(MOCHA_LAVENDER));

    // This splits the inside of the box into three: Top Space, The Text, Bottom Space.
    let inner_area = block.inner(area);
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),   // Top padding (Flexible)
            Constraint::Length(1), // The actual Input line
            Constraint::Fill(1),   // Bottom padding (Flexible)
        ])
        .split(inner_area);

    let input_len = app.input_buffer.len() as u16;
    let max_width = vertical_chunks[1].width;
    let scroll = input_len.saturating_sub(max_width);

    f.render_widget(block, area);

    f.render_widget(
        Paragraph::new(app.input_buffer.as_str())
            .scroll((0, scroll))
            .alignment(Alignment::Left) // Typing starts left but stays vertically centered
            .style(Style::default().fg(MOCHA_TEXT).bold()), 
        vertical_chunks[1]
    );

    // Cursor is now mathematically locked to the vertical center (vertical_chunks[1].y)
    f.set_cursor_position((
        vertical_chunks[1].x + input_len.min(max_width),
        vertical_chunks[1].y,
    ));
}

// --- UTILITIES ---
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
