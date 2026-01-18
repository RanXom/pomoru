use ratatui::{
    prelude::*,
    widgets::*,
};
use crate::pomo::state::{Pomo, SessionMode};

pub fn render(f: &mut Frame, app: &mut Pomo) {
    // 1. Create a centered area. 
    // We use 60% of width and 60% of height for that "isolated" feel.
    let area = centered_rect(60, 60, f.size());

    // 2. Clear the area with a transparent background
    // Color::Reset ensures we use your terminal's background/transparency
    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Reset));

    // 3. Split the centered area into Timer (Top) and Todo (Bottom)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // Timer space
            Constraint::Percentage(60), // Todo space
        ])
        .split(main_block.inner(area));

    // 4. Render the Timer
    let timer_text = format_duration(app.time_remaining);
    let timer_color = if app.is_running { Color::Green } else { Color::Yellow };
    
    let timer_para = Paragraph::new(timer_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(timer_color).add_modifier(Modifier::BOLD));

    // 5. Render the Todo List (Placeholder for now)
    let tasks: Vec<ListItem> = app.tasks
        .iter()
        .map(|t| {
            let symbol = if t.is_done { "󰄲 " } else { "󰄱 " };
            ListItem::new(format!("{} {}", symbol, t.title))
        })
        .collect();

    let list = List::new(tasks)
        .block(Block::default().title(" Priorities ").borders(Borders::TOP))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

    // Final Renders
    f.render_widget(main_block, area);
    f.render_widget(timer_para, chunks[0]);
    f.render_stateful_widget(list, chunks[1], &mut app.task_state);
}

/// Helper function to center a rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

/// Helper to format Duration into MM:SS
fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;
    format!("\n\n{:02}:{:02}", mins, secs)
}
