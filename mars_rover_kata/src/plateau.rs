#[derive(Debug)]
pub struct Plateau {
    x_limit: u32,
    //10
    y_limit: u32, //10
}

impl Plateau {
    pub fn new() -> Plateau {
        Plateau {
            x_limit: 10,
            y_limit: 10,
        }
    }
}
