use crate::board::Board;

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum WinCondition {
    WIN(char),
    DRAW,
    NONE,
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub struct TicTacToe {
    pub board: Board,
    pub current_player: char,
    pub winning_condition: WinCondition,
}

impl TicTacToe {
    pub fn player_move(&mut self, position: i32) {
        if self.position_is_valid(position) {
            self.board.set_position(position, self.current_player);
            let win_condition = self.board.check_win_condition(self.current_player);
            self.match_winning_condition(win_condition);
        }
    }

    fn match_winning_condition(&mut self, win_condition: WinCondition) {
        match win_condition {
            WinCondition::WIN(player) => {
                self.winning_condition = win_condition;
                println!("{} have won the game !!!", player);
            }
            WinCondition::DRAW => {
                self.winning_condition = win_condition;
                println!("The game has been drawn !!!");
            }
            WinCondition::NONE => self.set_player()
        };
    }

    fn set_player(&mut self) {
        match self.current_player {
            'x' => self.current_player = 'o',
            'o' => self.current_player = 'x',
            _ => {}
        }
    }

    fn position_is_valid(&self, position: i32) -> bool {
        !['x', 'o'].contains(&self.board.get_position(position))
    }
}


impl TicTacToe {
    pub fn new_game() -> TicTacToe {
        TicTacToe {
            board: Board::new(),
            current_player: 'x',
            winning_condition: WinCondition::NONE,
        }
    }
}
