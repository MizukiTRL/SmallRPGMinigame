pub struct Obstacle {
    pub pos: (i32, i32),
}

impl Obstacle {
    pub fn new(pos: (i32, i32)) -> Self {
        Obstacle { pos: pos }
    }
}
