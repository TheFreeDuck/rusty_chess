#[derive(Copy, Clone, Debug, Eq)]
pub struct Coordinate {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Coordinate {
    pub fn subtract(&self, other: Coordinate) -> Self {
        Coordinate::new(self.x - other.x, self.y - other.y)
    }
    pub fn difference(&self, other: Coordinate) -> Self {
        Coordinate::new((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    pub fn direction(&self) -> Self {
        let movement_direction = Coordinate::new(if self.x > 0 { 1 } else { -1 }, if self.y > 0 { 1 } else { -1 });

        movement_direction
    }

    pub fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    pub fn x_usize(&self) -> usize {
        self.x as usize
    }

    pub fn y_usize(&self) -> usize {
        self.y as usize
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}