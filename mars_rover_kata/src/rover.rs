#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn set_x(&mut self, x: u32) {
        self.x = x
    }
    fn get_x(&self) -> u32 {
        self.x
    }

    fn set_y(&mut self, y: u32) {
        self.y = y
    }
    fn get_y(&self) -> u32 {
        self.y
    }
}

#[derive(Debug)]
enum Direction {
    N,
    W,
    E,
    S,
}


#[derive(Debug)]
pub(crate) struct Rover {
    position: Position,
    direction: Direction,
}

impl Rover {
    pub fn new(s: String) -> Rover {
        let (position, direction) = Rover::parse_rover_input(s);
        Rover { position, direction }
    }
    fn parse_rover_input(input: String) -> (Position, Direction) {
        let input_chars: Vec<char> = input.chars().collect();
        let position: Position = Position {
            x: input_chars[0].to_digit(10).expect("Given X coordinate is invalid !!!"),
            y: input_chars[2].to_digit(10).expect("Given Y coordinate is invalid !!!"),
        };
        let direction = match input_chars[4] {
            'N' => Direction::N,
            'E' => Direction::E,
            'W' => Direction::W,
            'S' => Direction::S,
            _ => panic!("Given Direction is invalid !!!"),
        };

        (position, direction)
    }
    pub fn execute(&mut self, message: String) -> String {
        let message: Vec<char> = message.chars().collect();
        for command in message {
            match command {
                'M' => &self.move_forward(),
                'L' => &self.rotate_left(),
                'R' => &self.rotate_right(),
                _ => {
                    eprintln!("Command {} not found executing next one..", command);
                    &()
                }
            };
        }

        format!("{} {} {:?}", &self.position.x, &self.position.y, &self.direction)
    }
    fn move_forward(&mut self) {
        match &mut self.direction {
            Direction::N => if self.position.get_y() < 5 {self.position.set_y(self.position.get_y() + 1) },
            Direction::S => if self.position.get_y() - 1 >= 1 { self.position.set_y(self.position.get_y() - 1) },
            Direction::W => if self.position.get_x() - 1 >= 1 { self.position.set_x(self.position.get_x() - 1) },
            Direction::E => if self.position.get_x() + 1 < 5 { self.position.set_x(self.position.get_x() + 1) }
        }
    }
    fn rotate_left(&mut self) {
        match &self.direction {
            Direction::N => self.direction = Direction::W,
            Direction::W => self.direction = Direction::S,
            Direction::E => self.direction = Direction::N,
            Direction::S => self.direction = Direction::E
        }
    }
    fn rotate_right(&mut self) {
        match &self.direction {
            Direction::N => self.direction = Direction::E,
            Direction::W => self.direction = Direction::N,
            Direction::E => self.direction = Direction::S,
            Direction::S => self.direction = Direction::W
        }
    }
}
