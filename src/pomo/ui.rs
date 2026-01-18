use ratatui::{
    prelude::*,
    widgets::*,
};
use crate::pomo::state::Pomo;

pub fn render(f: &mut Frame, app: &mut Pomo) {
    let area = centered_rect(50, 50, f.area());

    let accent_color = app.mode.get_color();

    let main_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(accent_color))
        .style(Style::default().bg(Color::Reset));

    let inner_area = main_block.inner(area);
    f.render_widget(main_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), 
            Constraint::Min(1),    
        ])
        .split(inner_area);

    // Timer
    let timer_text = format_duration(app.time_remaining);
    let timer_para = Paragraph::new(timer_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
    
    f.render_widget(timer_para, chunks[0]);

    // Priority Tasks
    let tasks: Vec<ListItem> = app.tasks
        .iter()
        .map(|t| {
            let symbol = if t.is_done { "󰄲" } else { "󰄱" };
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", symbol), Style::default().fg(accent_color)),
                Span::raw(t.title.clone()),
            ]))
        })
        .collect();

    let list = List::new(tasks)
        .highlight_style(Style::default().bg(Color::Indexed(8)).fg(Color::White))
        .highlight_symbol(" ");

    f.render_stateful_widget(list, chunks[1], &mut app.task_state);
}

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

fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let mins = secs / 60;
    let secs = secs % 60;
    format!("{:02}:{:02}", mins, secs)
}
