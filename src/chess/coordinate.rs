#[derive(Copy, Clone, Debug, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn subtract(&self, other: Coordinate) -> Self {
        Coordinate::new(self.x - other.x, self.y - other.y)
    }
    pub fn difference(&self, other: Coordinate) -> Self {
        Coordinate::new((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    pub fn direction(&self) -> Self {
        if self.x == 0 && self.y == 0 {
            return Coordinate::new(0, 0);
        }
    
        let direction_x = if self.x > 0 { 1 } else if self.x < 0 { -1 } else { 0 };
        let direction_y = if self.y > 0 { 1 } else if self.y < 0 { -1 } else { 0 };
    
        Coordinate::new(direction_x, direction_y)
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