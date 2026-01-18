use ratatui::{prelude::*, widgets::*};
use crate::pomo::state::{Pomo, AppScreen, SessionMode, InputMode};

const MOCHA_LAVENDER: Color = Color::Rgb(180, 190, 254);
const MOCHA_OVERLAY0: Color = Color::Rgb(108, 112, 134);
const MOCHA_SURFACE0: Color = Color::Rgb(49, 50, 68);
const MOCHA_TEXT: Color = Color::Rgb(205, 214, 244);

pub fn render(f: &mut Frame, app: &mut Pomo) {
    let main_block = Block::default().style(Style::default().bg(Color::Reset));
    f.render_widget(main_block, f.area());

    let root_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Length(1)
        ])
        .split(f.area());

    match app.screen {
        AppScreen::Timer => {
            let timer_v_center = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(15),
                    Constraint::Fill(1)
                ])
                .split(root_layout[0]);

            render_timer_screen(f, app, timer_v_center[1]);

            let footer = "tab session • t tasks • e edit time • space pause • r reset • q quit";
            f.render_widget(
                Paragraph::new(footer)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(MOCHA_OVERLAY0)),
                root_layout[1]
            );
        }
        AppScreen::Tasks => {
            render_task_screen(f, app, root_layout[1]); 
        }
    }

    if let InputMode::Insert | InputMode::Edit | InputMode::TimerEdit = app.input_mode {
        render_input_modal(f, app);
    }
}

fn render_timer_screen(f: &mut Frame, app: &Pomo, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Priority Text
            Constraint::Length(4), // Spacer
            Constraint::Length(5), // ASCII Timer
            Constraint::Length(3), // Spacer
            Constraint::Length(1), // Session Dots
            Constraint::Min(0),
        ])
        .split(area);

    let priority_text = app.tasks.iter().find(|t| !t.is_done)
        .map(|t| format!("Current Focus: {}", t.title))
        .unwrap_or_else(|| "No Active Tasks".to_string());
 
    f.render_widget(
        Paragraph::new(priority_text)
            .alignment(Alignment::Center)
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
}

// Fixed-width monolithic ASCII engine
fn format_monolithic_ascii(time: &str) -> Text<'static> {
    let mut lines = vec![String::new(); 5];
    for (idx, c) in time.chars().enumerate() {
        let art = match c {
            '0' => vec![" ██████ ", "██    ██", "██    ██", "██    ██", " ██████ "],
            '1' => vec!["   ██   ", "  ███   ", "   ██   ", "   ██   ", " ██████ "],
            '2' => vec![" ██████ ", "██    ██", "    ███ ", "  ███   ", "████████"],
            '3' => vec![" ██████ ", "      ██", "  █████ ", "      ██", " ██████ "],
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
    let modes = [(SessionMode::Work, "Focus"), (SessionMode::ShortBreak, "Short Break"), (SessionMode::LongBreak, "Long Break")];
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

pub fn render_task_screen(f: &mut Frame, app: &mut Pomo, footer_area: Rect) {
    let area = centered_rect(60, 80, f.area());

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

    f.render_stateful_widget(list, area, &mut app.task_state);

    let footer_text = "i insert • ⏎ toggle • e edit • d delete • t back";
    f.render_widget(
        Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(MOCHA_OVERLAY0)),
        footer_area
    );
}

pub fn render_input_modal(f: &mut Frame, app: &Pomo) {
    let (title, width) = match app.input_mode { 
        InputMode::TimerEdit => (" Set Minutes ", 30), 
        _ => (" Input ", 50), 
    };

    // Instead of using the utility, we define the area directly to ensure zero drift.
    let terminal_area = f.area();
    let modal_width = width.min(terminal_area.width.saturating_sub(4));
    let modal_height = 5; // Tighter vertical profile
 
    let area = Rect {
        x: terminal_area.x + (terminal_area.width.saturating_sub(modal_width)) / 2,
        y: terminal_area.y + (terminal_area.height.saturating_sub(modal_height)) / 2,
        width: modal_width,
        height: modal_height,
    };

    f.render_widget(Clear, area);

    let title_text = match app.input_mode {
        InputMode::Insert => " New Task ",
        InputMode::Edit => " Edit Task ",
        InputMode::TimerEdit => " Set Minutes ",
        _ => title
    };

    let block = Block::default()
        .title(Span::styled(title_text, Style::default().fg(MOCHA_LAVENDER).bold()))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(MOCHA_LAVENDER));

    // Nested layout for perfect internal vertical centering
    let inner_area = block.inner(area);
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(inner_area);

    let horizontal_padding = 2;
    let input_len = app.input_buffer.len() as u16;
    let max_width = vertical_chunks[1].width.saturating_sub(horizontal_padding * 2);
    let scroll = input_len.saturating_sub(max_width);

    f.render_widget(block, area);

    f.render_widget(
        Paragraph::new(app.input_buffer.as_str())
            .scroll((0, scroll))
            .block(Block::default().padding(Padding::horizontal(horizontal_padding)))
            .style(Style::default().fg(MOCHA_TEXT).bold()), 
        vertical_chunks[1]
    );

    f.set_cursor_position((
        vertical_chunks[1].x + horizontal_padding + input_len.min(max_width),
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
