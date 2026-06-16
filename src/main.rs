use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Clear, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tui_2048::app::{App, Direction, Screen};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    run(&mut app, &mut terminal)
}

pub fn run(app: &mut App, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !app.exit {
        terminal.draw(|frame| {
            draw(app, frame);
            if app.game_over {
                render_game_over_popup(frame, app);
            }
            if app.showing_score {
                render_scores_popup(frame, app);
            }
        })?;
        handle_events(app)?;
    }
    Ok(())
}

pub fn draw(app: &App, frame: &mut Frame) {
    frame.render_widget(app, frame.area());
}

pub fn handle_events(app: &mut App) -> io::Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event)
        }
        _ => {}
    };
    Ok(())
}

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        Screen::Playing => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('l') => app.move_nums(Direction::Right),
            KeyCode::Char('h') => app.move_nums(Direction::Left),
            KeyCode::Char('j') => app.move_nums(Direction::Down),
            KeyCode::Char('k') => app.move_nums(Direction::Up),
            KeyCode::Right => app.move_nums(Direction::Right),
            KeyCode::Left => app.move_nums(Direction::Left),
            KeyCode::Down => app.move_nums(Direction::Down),
            KeyCode::Up => app.move_nums(Direction::Up),
            KeyCode::Char('s') => app.toggle_scores(),
            KeyCode::Char('n') => app.new_game(),
            _ => {}
        },
        Screen::GameOver => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('s') => app.toggle_scores(),
            KeyCode::Char('n') => app.new_game(),
            _ => {}
        },
        Screen::Scores => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('s') => app.toggle_scores(),
            _ => {}
        },
    }
}

pub fn render_game_over_popup(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let controls = Line::from(vec![
        " New Game ".into(),
        "<n>".blue().bold(),
        " High Scores ".into(),
        "<s> ".blue().bold(),
        " Quit ".into(),
        "<q> ".blue().bold(),
    ]);

    let popup_block = Block::bordered().title("Game Over").title_bottom(controls);
    let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
    frame.render_widget(Clear, centered_area);
    let paragraph =
        Paragraph::new(format!("You finished with a score of {}", app.score)).block(popup_block);
    frame.render_widget(paragraph, centered_area);
}

pub fn render_scores_popup(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let controls = Line::from(vec![
        " New Game ".into(),
        "<n>".blue().bold(),
        " Hide Scores ".into(),
        "<s> ".blue().bold(),
        " Quit ".into(),
        "<q> ".blue().bold(),
    ]);

    let popup_block = Block::bordered()
        .title("High Scores")
        .title_bottom(controls);
    let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
    frame.render_widget(Clear, centered_area);
    let paragraph = Paragraph::new(format!("1. {}", app.score)).block(popup_block);
    frame.render_widget(paragraph, centered_area);
}
