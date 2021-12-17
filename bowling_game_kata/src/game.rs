enum Bonus {
    SPARE,
    STRIKE,
    NONE,
}

pub struct Game {
    frames: [Frame; 10],
    score: i32,
}

pub struct Frame {
    first_roll_score: i32,
    second_roll_score: i32,
    bonus: Bonus::NONE,
}


impl Game {
    pub fn new() -> Game { Game { score: 0 } }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn roll(&mut self, pins_knocked_down: i32) {
        self.score += pins_knocked_down;
    }
}



