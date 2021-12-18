
#[cfg(test)]
mod game_manager {
    use crate::board::Board;
    use crate::tic_tac_toe::{TicTacToe, WinCondition};

    #[test]
    fn test_should_return_a_new_game_with_empty_board_and_first_turn_to_player_x() {
        let expected_game = TicTacToe {
            board: Board {
                board: [['1', '2', '3'],
                    ['4', '5', '6'],
                    ['7', '8', '9']]
            },
            current_player: 'x',
            winning_condition: WinCondition::NONE,
        };
        let new_game = TicTacToe::new_game();
        assert_eq!(expected_game, new_game)
    }

    #[test]
    fn test_should_return_true_when_x_occupies_the_position_1() {
        let expected_game_state = TicTacToe {
            board: Board {
                board: [['x', '2', '3'],
                    ['4', '5', '6'],
                    ['7', '8', '9']]
            },
            current_player: 'o',
            winning_condition: WinCondition::NONE,
        };
        let mut game = TicTacToe::new_game();
        game.player_move(1);
        assert_eq!(expected_game_state, game)
    }

    #[test]
    fn test_should_return_true_when_x_occupies_2_and_o_occupies_the_position_9() {
        let expected_game_state = TicTacToe {
            board: Board {
                board: [['1', 'x', '3'],
                    ['4', '5', '6'],
                    ['7', '8', 'o']]
            },
            current_player: 'x',
            winning_condition: WinCondition::NONE,
        };
        let mut game = TicTacToe::new_game();
        game.player_move(2);
        game.player_move(9);
        assert_eq!(expected_game_state, game)
    }

    #[test]
    #[should_panic]
    fn test_should_panic_when_user_enters_a_invalid_position() {
        let mut game = TicTacToe::new_game();
        game.player_move(10);
    }

    #[test]
    fn test_a_winning_condition_for_x_when_first_row_is_similar() {
        let expected_game_state = TicTacToe {
            board: Board {
                board: [['x', 'x', 'x'],
                    ['4', '5', '6'],
                    ['7', 'o', 'o']]
            },
            current_player: 'x',
            winning_condition: WinCondition::WIN('x'),
        };
        let mut game = TicTacToe::new_game();
        game.player_move(1);
        game.player_move(8);
        game.player_move(2);
        game.player_move(9);
        game.player_move(3);
        assert_eq!(expected_game_state.winning_condition, game.winning_condition)
    }

    #[test]
    fn test_a_winning_condition_for_o_when_first_column_is_similar() {
        let expected_game_state = TicTacToe {
            board: Board {
                board: [['o', '2', '3'],
                    ['o', '5', 'x'],
                    ['o', 'x', 'x']]
            },
            current_player: 'o',
            winning_condition: WinCondition::WIN('o'),
        };
        let mut game = TicTacToe::new_game();
        game.player_move(6); // x
        game.player_move(1);
        game.player_move(9); // x
        game.player_move(4);
        game.player_move(8); // x
        game.player_move(7);
        assert_eq!(expected_game_state, game)
    }

    #[test]
    fn test_a_game_draw_condition() {
        let expected_game_state = TicTacToe {
            board: Board {
                board: [['x', 'o', 'x'],
                        ['o', 'x', 'o'],
                        ['o', 'x', 'x']]
            },
            current_player: 'x',
            winning_condition: WinCondition::DRAW,
        };
        let mut game = TicTacToe::new_game();
        game.player_move(1); //x
        game.player_move(2);
        game.player_move(3); //x
        game.player_move(4);
        game.player_move(5); //x
        game.player_move(6);
        game.player_move(9); //x
        game.player_move(7);
        game.player_move(8); //x
        assert_eq!(expected_game_state, game)
    }


}
