use std::{io, path::PathBuf, time::Duration};

use crate::{
    SCORES_PATH,
    app::{App, Direction, GameStyle, Screen, write_scores_to_file},
    ui::popups::{render_game_over_popup, render_game_style_popup, render_scores_popup},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

pub fn run(app: &mut App, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !app.exit {
        app.update_timer();

        terminal.draw(|frame| {
            draw(app, frame);

            if !app.chosen_game_style {
                render_game_style_popup(frame, app);
            }

            if app.game_over {
                render_game_over_popup(frame, app);
            }

            if app.showing_score {
                render_scores_popup(frame);
            }
        })?;
        if app.game_over {
            let home = std::env::var("HOME").unwrap_or("~".to_string());

            let mut path = PathBuf::from(home);
            path.push(SCORES_PATH);

            let _ = write_scores_to_file(app, path);
        }

        handle_events(app)?;
    }
    ratatui::restore();
    Ok(())
}

pub fn draw(app: &App, frame: &mut Frame) {
    frame.render_widget(app, frame.area());
}

pub fn handle_events(app: &mut App) -> io::Result<()> {
    if event::poll(Duration::from_millis(16))? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                handle_key_event(app, key_event)
            }
            _ => {}
        };
    }
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
            KeyCode::Char('n') => match app.game_style {
                GameStyle::Normal => app.new_game(),
                GameStyle::Timed5 => app.new_game_timed5(),
                GameStyle::Timed10 => app.new_game_timed10(),
            },
            _ => {}
        },
        Screen::GameOver => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('s') => app.toggle_scores(),
            KeyCode::Char('n') => match app.game_style {
                GameStyle::Normal => app.new_game(),
                GameStyle::Timed5 => app.new_game_timed5(),
                GameStyle::Timed10 => app.new_game_timed10(),
            },
            _ => {}
        },
        Screen::Scores => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('s') => app.toggle_scores(),
            KeyCode::Char('a') => {
                todo!() // all scores
            }
            KeyCode::Char('f') => {
                todo!() // timed 5 scores
            }
            KeyCode::Char('t') => {
                todo!() // timed 10 scores
            }
            KeyCode::Char('d') => {
                todo!() // sort by date
            }
            KeyCode::Char('h') => {
                todo!() // sort by high to low
            }
            _ => {}
        },
        Screen::GameStyle => match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if app.game_style_index < 2 {
                    app.game_style_index += 1;
                };
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if app.game_style_index > 0 {
                    app.game_style_index -= 1;
                };
            }
            KeyCode::Enter => {
                app.chosen_game_style = true;

                match app.game_style_index {
                    0 => app.new_game(),
                    1 => app.new_game_timed5(),
                    2 => app.new_game_timed10(),
                    _ => {}
                }
            }
            KeyCode::Char('q') => app.exit(),
            _ => {}
        },
    }
}
