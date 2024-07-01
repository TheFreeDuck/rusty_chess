use nalgebra::Vector2;

#[derive(Copy, Clone, Debug, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn subtract(&self, other: Coordinate) -> Vector2<i32> {
        Coordinate::new(self.x_i32() - other.x_i32(), self.y_i32() - other.y_i32())
    }
    pub fn difference(&self, other: Coordinate) -> Self {
        Coordinate::new((self.x_i32() - other.x_i32()).abs(), (self.y_i32() - other.y_i32()).abs())
    }

    pub fn direction(&self) -> Self {
        if self.x == 0 && self.y == 0 {
            return Coordinate::new(0, 0);
        }
    
        let direction_x = if self.x_i32() > 0 { 1 } else if self.x_i32() < 0 { -1 } else { 0 };
        let direction_y = if self.y_i32() > 0 { 1 } else if self.y_i32() < 0 { -1 } else { 0 };
    
        Coordinate::new(direction_x, direction_y)
    }

    pub fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }

    pub fn x_i32(&self) -> i32 {
        self.x as i32
    }

    pub fn y_i32(&self) -> i32 {
        self.y as i32
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}