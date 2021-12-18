use crate::tic_tac_toe::WinCondition;

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct Board {
    pub board: [[char; 3]; 3],
}


impl Board {
    pub fn new() -> Self {
        Board {
            board: [['1', '2', '3'],
                ['4', '5', '6'],
                ['7', '8', '9']]
        }
    }
}


impl Board {
    pub fn set_position(&mut self, position: i32, current_player: char) {
        match position {
            1 => self.board[0][0] = current_player,
            2 => self.board[0][1] = current_player,
            3 => self.board[0][2] = current_player,
            4 => self.board[1][0] = current_player,
            5 => self.board[1][1] = current_player,
            6 => self.board[1][2] = current_player,
            7 => self.board[2][0] = current_player,
            8 => self.board[2][1] = current_player,
            9 => self.board[2][2] = current_player,
            _ => panic!("Please chose the valid position between 1 to 9")
        }
    }

    pub fn check_win_condition(&self, current_player: char) -> WinCondition {
        let row_condition = self.row_check(current_player);
        let column_condition = self.column_check(current_player);
        let draw_condition = self.draw_check();
        match draw_condition {
            true => WinCondition::DRAW,
            false => {
                if row_condition || column_condition {
                    WinCondition::WIN(current_player)
                } else { WinCondition::NONE }
            }
        }
    }


    fn column_check(&self, current_player: char) -> bool {
        let first_column = self.board[0][0] == current_player && self.board[1][0] == current_player && self.board[2][0] == current_player;
        let second_column = self.board[1][0] == current_player && self.board[1][1] == current_player && self.board[1][2] == current_player;
        let third_column = self.board[2][0] == current_player && self.board[2][1] == current_player && self.board[2][2] == current_player;
        first_column || second_column || third_column
    }

    fn row_check(&self, current_player: char) -> bool {
        let first_row = self.board[0][0] == current_player && self.board[0][1] == current_player && self.board[0][2] == current_player;
        let second_row = self.board[1][0] == current_player && self.board[1][1] == current_player && self.board[1][2] == current_player;
        let third_row = self.board[2][0] == current_player && self.board[2][1] == current_player && self.board[2][2] == current_player;
        first_row || second_row || third_row
    }

    pub fn get_position(&self, position: i32) -> char {
        match position {
            1 => self.board[0][0],
            2 => self.board[0][1],
            3 => self.board[0][2],
            4 => self.board[1][0],
            5 => self.board[1][1],
            6 => self.board[1][2],
            7 => self.board[2][0],
            8 => self.board[2][1],
            9 => self.board[2][2],
            _ => panic!("Please chose the valid position between 1 to 9")
        }
    }

    fn draw_check(&self) -> bool {
        for row in 0..self.board.len() {
            for col in 0..self.board[row].len() {
                if !['x', 'o'].contains(&self.board[row][col]) {
                    return false;
                }
            }
        }

        true
    }
}

