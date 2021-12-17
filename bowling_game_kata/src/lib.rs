mod game;


#[cfg(test)]
mod tests {
    use crate::game::Game;

    #[test]
    fn should_return_total_score_as_0_when_all_the_frame_scores_are_0() {
        let mut game = Game::new();
        let pins_knocked_down = 0;
        roll(&mut game, pins_knocked_down, 20);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn should_return_total_score_as_20_when_all_the_frame_scores_are_1() {
        let mut game = Game::new();
        let pins_knocked_down = 1;
        roll(&mut game, pins_knocked_down, 20);
        assert_eq!(game.score(), 20);
    }

    #[test]
    fn should_calculate_the_spare_bonus() {
        let mut game = Game::new();
        roll(&mut game, 5, 1);
        roll(&mut game, 5, 1);
        roll(&mut game, 3, 1);
        roll(&mut game, 0, 17);
        assert_eq!(game.score(), 16);
    }


    fn roll(game: &mut Game, pins_knocked_down: i32, times: i32) {
        for _i in 0..times {
            game.roll(pins_knocked_down);
        }
    }
}

