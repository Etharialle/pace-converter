use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Run App
    let res = run_app(&mut terminal);

    // 3. Restore Terminal (Crucial!)
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

enum InputMode {
    Distance,
    Time,
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut distance_input = String::new();
    let mut time_digits = String::new();
    let mut input_mode = InputMode::Distance;
    let mut is_km = true; // true = km, false = miles

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3), // Distance
                    Constraint::Length(3), // Time
                    Constraint::Length(3), // Unit Toggle
                    Constraint::Min(3),    // Results
                ])
                .split(f.size());

            // 1. Distance Input
            let dist_style = if matches!(input_mode, InputMode::Distance) {
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
            } else {
                ratatui::style::Style::default()
            };
            f.render_widget(
                Paragraph::new(distance_input.as_str())
                    .block(Block::default().borders(Borders::ALL).title("Distance").border_style(dist_style)),
                chunks[0],
            );

            // 2. Time Input (MM:SS)
            let time_style = if matches!(input_mode, InputMode::Time) {
                ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
            } else {
                ratatui::style::Style::default()
            };
            let display_time = format_as_hms(&time_digits);
            f.render_widget(
                Paragraph::new(display_time)
                    .block(Block::default().borders(Borders::ALL).title("Time (HHMMSS)").border_style(time_style)),
                chunks[1],
            );

            // 3. Unit "Radio" Select
            let unit_text = if is_km { "[X] Kilometers  [ ] Miles" } else { "[ ] Kilometers  [X] Miles" };
            f.render_widget(
                Paragraph::new(unit_text)
                    .block(Block::default().borders(Borders::ALL).title("Unit (Press 'U' to toggle)")),
                chunks[2],
            );

            // 4. Calculation
            let pace_display = calculate_pace(&distance_input, &time_digits, is_km);
            f.render_widget(
                Paragraph::new(pace_display)
                    .block(Block::default().borders(Borders::ALL).title("Calculated Pace")),
                chunks[3],
            );
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    // Toggle Focus between Distance and Time
                    KeyCode::Tab => {
                        input_mode = match input_mode {
                            InputMode::Distance => InputMode::Time,
                            InputMode::Time => InputMode::Distance,
                        };
                    }
                    // Toggle Units
                    KeyCode::Char('u') | KeyCode::Char('U') => is_km = !is_km,
                    // Handle Typing
                    KeyCode::Char(c) => match input_mode {
                        InputMode::Distance => {
                            // Only allow numbers and one decimal point for distance
                            if c.is_ascii_digit() || (c == '.' && !distance_input.contains('.')) {
                                distance_input.push(c);
                            }
                        }
                        InputMode::Time => {
                            if c.is_ascii_digit() && time_digits.len() < 6 {
                                time_digits.push(c);
                            }
                        }
                    },
                    KeyCode::Backspace => match input_mode {
                        InputMode::Distance => { distance_input.pop(); },
                        InputMode::Time => {
                            time_digits.pop();
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

// Helper to turn 5.5 minutes into "5:30"
fn format_time(total_minutes: f64) -> String {
    let mins = total_minutes.floor();
    let secs = ((total_minutes - mins) * 60.0).round();
    format!("{}:{:02}", mins, secs)
}

fn format_as_hms(digits: &str) -> String {
    // Pad with leading zeros to ensure we always have 6 digits
    let padded = format!("{:0>6}", digits);
    let hh = &padded[0..2];
    let mm = &padded[2..4];
    let ss = &padded[4..6];
    format!("{}:{}:{}", hh, mm, ss)
}

fn calculate_pace(dist_str: &str, time_digits: &str, is_km: bool) -> String {
    let dist: f64 = dist_str.parse().unwrap_or(0.0);
    if dist <= 0.0 { return "Enter distance...".to_string(); }
    if time_digits.is_empty() { return "Enter time...".to_string(); }

    // Convert the "shift-in" digits to total seconds
    let padded = format!("{:0>6}", time_digits);
    let hrs: f64 = padded[0..2].parse().unwrap_or(0.0);
    let mins: f64 = padded[2..4].parse().unwrap_or(0.0);
    let secs: f64 = padded[4..6].parse().unwrap_or(0.0);
    
    let total_minutes = (hrs * 60.0) + mins + (secs / 60.0);
    if total_minutes <= 0.0 { return "0:00".to_string(); }

    let min_per_km = if is_km { total_minutes / dist } else { (total_minutes / dist) / 1.60934 };
    let min_per_mile = min_per_km * 1.60934;

    format!(
        "Pace: {} min/km  |  {} min/mile",
        format_time(min_per_km),
        format_time(min_per_mile)
    )
}