use ratatui::{prelude::*, widgets::*};
use crate::pomo::state::{Pomo, AppScreen, SessionMode, InputMode};

pub fn render(f: &mut Frame, app: &mut Pomo) {
    match app.screen {
        AppScreen::Timer => render_timer_screen(f, app),
        AppScreen::Tasks => render_task_screen(f, app),
    }
    
    if let InputMode::Insert | InputMode::Edit = app.input_mode {
        render_input_modal(f, app);
    }
}

fn render_timer_screen(f: &mut Frame, app: &Pomo) {
    let area = centered_rect(60, 50, f.area());
    let accent = match app.mode {
        SessionMode::Work => Color::Cyan,
        SessionMode::ShortBreak => Color::Green,
        SessionMode::LongBreak => Color::Magenta,
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Priority Card
            Constraint::Length(1), // Spacer
            Constraint::Length(3), // Buttons
            Constraint::Min(1),    // Timer
        ])
        .split(area);

    let priority_text = app.tasks.iter().find(|t| !t.is_done)
        .map(|t| format!(" Current Focus: {} ", t.title))
        .unwrap_or_else(|| " No Active Tasks ".to_string());
    
    f.render_widget(Paragraph::new(priority_text).alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(accent))), chunks[0]);

    let btn_layout = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Length(15), Constraint::Length(15), Constraint::Length(15)])
        .flex(layout::Flex::Center).split(chunks[2]);
    
    render_btn(f, "Focus", app.mode == SessionMode::Work, btn_layout[0]);
    render_btn(f, "Short", app.mode == SessionMode::ShortBreak, btn_layout[1]);
    render_btn(f, "Long", app.mode == SessionMode::LongBreak, btn_layout[2]);

    let time = format_duration(app.time_remaining);
    f.render_widget(Paragraph::new(format!("\n\n{}", time)).alignment(Alignment::Center)
        .style(Style::default().fg(Color::White).bold()), chunks[3]);
}

fn render_btn(f: &mut Frame, text: &str, active: bool, area: Rect) {
    let style = if active { Style::default().bg(Color::White).fg(Color::Black) } else { Style::default().fg(Color::Gray) };
    f.render_widget(Paragraph::new(text).alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)).style(style), area);
}

fn render_task_screen(f: &mut Frame, app: &mut Pomo) {
    let area = centered_rect(60, 80, f.area());
    let items: Vec<ListItem> = app.tasks.iter().map(|t| {
        let symbol = if t.is_done { "󰄲" } else { "󰄱" };
        ListItem::new(format!(" {} {}", symbol, t.title))
    }).collect();

    let list = List::new(items).block(Block::default().title(" Focus Priorities ").borders(Borders::ALL).border_type(BorderType::Rounded))
        .highlight_style(Style::default().bg(Color::Indexed(8)).bold()).highlight_symbol(">> ");
    f.render_stateful_widget(list, area, &mut app.task_state);
}

fn render_input_modal(f: &mut Frame, app: &Pomo) {
    let area = centered_rect(40, 10, f.area());
    f.render_widget(Clear, area);
    f.render_widget(Paragraph::new(app.input_buffer.as_str())
        .block(Block::default().title(" Input ").borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow))), area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Percentage((100-percent_y)/2), Constraint::Percentage(percent_y), Constraint::Percentage((100-percent_y)/2)]).split(r);
    Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage((100-percent_x)/2), Constraint::Percentage(percent_x), Constraint::Percentage((100-percent_x)/2)]).split(popup_layout[1])[1]
}

fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
