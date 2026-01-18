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
                    Constraint::Length(16),
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
            Constraint::Length(1), // Priority Text
            Constraint::Length(4), // Large top spacer
            Constraint::Length(5), // MONOLITHIC TIMER
            Constraint::Length(3), // Padding
            Constraint::Length(1), // Session Dots
            Constraint::Min(1),    // Fill
            Constraint::Length(1), // Footer Help
        ])
        .split(area);

    // PRIORITY
    let priority_text = app.tasks.iter().find(|t| !t.is_done)
        .map(|t| format!("Current Focus: {}", t.title))
        .unwrap_or_else(|| "No Active Tasks".to_string());
    
    f.render_widget(
        Paragraph::new(priority_text).alignment(Alignment::Center).style(Style::default().fg(MOCHA_LAVENDER).bold()), 
        chunks[0]
    );

    // TIMER
    let time_str = format_duration(app.time_remaining);
    let big_text = format_monolithic_ascii(&time_str);
    f.render_widget(
        Paragraph::new(big_text).alignment(Alignment::Center).style(Style::default().fg(MOCHA_LAVENDER)), 
        chunks[2]
    );

    // SESSION INDICATOR
    render_session_dots(f, app, chunks[4]);

    // FOOTER HELP
    let footer = "↑ +1 min  •  space pause  •  r reset  •  s skip  •  t tasks";
    f.render_widget(
        Paragraph::new(footer).alignment(Alignment::Center).style(Style::default().fg(MOCHA_OVERLAY0)), 
        chunks[6]
    );
}

fn render_marker(f: &mut Frame, text: &str, active: bool, area: Rect) {
    let style = if active { 
        Style::default().fg(MOCHA_LAVENDER).bold() 
    } else { 
        Style::default().fg(MOCHA_OVERLAY0) 
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(style);
    
    f.render_widget(
        Paragraph::new(text).alignment(Alignment::Center).block(block).style(style), 
        area
    );
}

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
    let items: Vec<ListItem> = app.tasks.iter().map(|t| {
        let symbol = if t.is_done { "󰄲" } else { "󰄱" };
        ListItem::new(format!(" {} {}", symbol, t.title))
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .title(" Focus Priorities ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(MOCHA_LAVENDER)))
        .highlight_style(Style::default().bg(MOCHA_SURFACE0).fg(MOCHA_TEXT).bold())
        .highlight_symbol(">> ");
    
    f.render_stateful_widget(list, area, &mut app.task_state);
}

pub fn render_input_modal(f: &mut Frame, app: &Pomo) {
    let area = centered_rect(40, 10, f.area());
    f.render_widget(Clear, area); 
    let title = match app.input_mode { 
        InputMode::Insert => " New Task ", 
        InputMode::Edit => " Edit Task ", 
        _ => " Input " 
    };
    
    f.render_widget(
        Paragraph::new(app.input_buffer.as_str())
            .block(Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(MOCHA_LAVENDER))), 
        area
    );
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
