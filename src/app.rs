use crate::grid::Grid;

pub struct App {
    pub highest_num: u8,
    pub score: u32,
    pub game_over: bool,
    pub high_score: u32,
    pub exit: bool,
    pub grid: Grid,
    pub current_screen: Screen,
}

pub enum Screen {
    Playing,
    GameOver,
}

impl Default for App {
    fn default() -> Self {
        Self {
            highest_num: 0,
            score: 0,
            game_over: false,
            high_score: 0,
            exit: false,
            grid: Grid {
                cells: [[0, 0, 0, 0], [0, 0, 2, 0], [0, 2, 0, 0], [0, 0, 0, 0]],
            },
            current_screen: Screen::Playing,
        }
    }
}

impl App {
    pub fn new_game(&mut self) {
        self.grid.cells = [[0, 0, 0, 0], [0, 0, 2, 0], [0, 2, 0, 0], [0, 0, 0, 0]]
    }

    /// [0, 0, 2, 0] -> [2, 0, 0, 0]
    /// strip zeros, merge equal neighbors left to right
    /// then pad zeros on the right
    pub fn move_left(&mut self) {
        todo!()
    }

    pub fn move_right(&mut self) {
        todo!()
    }
    pub fn move_up(&mut self) {
        todo!()
    }

    pub fn move_down(&mut self) {
        todo!()
    }

    /// Score is calculated by the addition of current score + sum of any merged values
    pub fn calculate_score(&mut self) {
        todo!()
    }

    /// TODO: need to wire up scores to be saved in a .txt as "date score highest_num"
    /// access file and show in popup sorted by score
    pub fn show_scores(&mut self) {
        todo!()
    }

    /// TODO: calc open fields, randonmize 2 or 4, and popup in rand open
    pub fn spawn_tile(&mut self) {
        todo!()
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
