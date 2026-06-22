#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use ratatui::layout::{Constraint, Direction, Layout, Rect};
    use ratatui::{Terminal, backend::TestBackend};

    use crate::app::{merge_row_horizontal, merge_row_vertical};
    use crate::ui::popups::{render_game_over_popup, render_scores_popup};
    use crate::{
        app::{App, GameStyle, Screen},
        ui::{grid::Grid, popups::render_game_style_popup},
    };

    fn build_app() -> App {
        App {
            highest_num: 0,
            score: 0,
            game_over: false,
            showing_score: false,
            high_score: 0,
            exit: false,
            grid: Grid {
                cells: [[0, 0, 0, 0], [0, 0, 2, 0], [0, 2, 0, 0], [0, 0, 0, 0]],
            },
            current_screen: Screen::Playing,
            game_style: GameStyle::Normal,
            chosen_game_style: true,
            game_style_index: 0,
            time_remaining_seconds: 0,
            game_start_time: None,
        }
    }
    fn get_centered_popup_area(area: Rect) -> Rect {
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(vertical_layout[1]);

        horizontal_layout[1]
    }

    #[test]
    fn test_render_app() {
        // set_var is unsafe in Rust 1.80+ because modifying environment variables
        // while other threads might be reading them causes data races.
        // It is considered acceptable here for a single test environment if it's the only one
        // needing HOME, or if run with `cargo test -- --test-threads=1`.
        unsafe {
            std::env::set_var("HOME", env!("CARGO_MANIFEST_DIR"));
        }
        let app = build_app();
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_app_game_style_not_chosen() {
        unsafe {
            std::env::set_var("HOME", env!("CARGO_MANIFEST_DIR"));
        }
        let mut app = build_app();
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
        terminal
            .draw(|frame| {
                frame.render_widget(&app, frame.area());

                let popup_area = get_centered_popup_area(frame.area());

                frame.render_widget(ratatui::widgets::Clear, popup_area);

                render_game_style_popup(frame, &mut app);
            })
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_app_scores() {
        unsafe {
            std::env::set_var("HOME", env!("CARGO_MANIFEST_DIR"));
        }
        let app = build_app();
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
        terminal
            .draw(|frame| {
                frame.render_widget(&app, frame.area());

                let popup_area = get_centered_popup_area(frame.area());

                frame.render_widget(ratatui::widgets::Clear, popup_area);

                render_scores_popup(frame);
            })
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_app_game_over() {
        unsafe {
            std::env::set_var("HOME", env!("CARGO_MANIFEST_DIR"));
        }
        let app = build_app();
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
        terminal
            .draw(|frame| {
                frame.render_widget(&app, frame.area());

                let popup_area = get_centered_popup_area(frame.area());

                frame.render_widget(ratatui::widgets::Clear, popup_area);

                render_game_over_popup(frame, &app);
            })
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_app_move_left() {
        unsafe {
            std::env::set_var("HOME", env!("CARGO_MANIFEST_DIR"));
        }
        let mut app = build_app();
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();

        terminal
            .draw(|frame| {
                let mut cells = app.grid.cells;
                for row in &mut cells {
                    *row = merge_row_horizontal(&mut app, *row, crate::app::Direction::Left)
                }
                app.grid.cells = cells;
                frame.render_widget(&app, frame.area());
            })
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_app_move_right() {
        unsafe {
            std::env::set_var("HOME", env!("CARGO_MANIFEST_DIR"));
        }
        let mut app = build_app();
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
        terminal
            .draw(|frame| {
                let mut cells = app.grid.cells;
                for row in &mut cells {
                    *row = merge_row_horizontal(&mut app, *row, crate::app::Direction::Right)
                }
                app.grid.cells = cells;
                frame.render_widget(&app, frame.area());
            })
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_app_move_up() {
        unsafe {
            std::env::set_var("HOME", env!("CARGO_MANIFEST_DIR"));
        }
        let mut app = build_app();
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
        terminal
            .draw(|frame| {
                app.grid = merge_row_vertical(&mut app, crate::app::Direction::Up);
                frame.render_widget(&app, frame.area());
            })
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_app_move_down() {
        unsafe {
            std::env::set_var("HOME", env!("CARGO_MANIFEST_DIR"));
        }
        let mut app = build_app();
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
        terminal
            .draw(|frame| {
                app.grid = merge_row_vertical(&mut app, crate::app::Direction::Down);
                frame.render_widget(&app, frame.area());
            })
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
}
