#[derive(Copy, Clone, Debug, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }

    pub fn from_tuple_usize(tuple: (usize, usize)) -> Self {
        Coordinate::new(tuple.0, tuple.1)
    }

    pub fn vector(&self) -> Vector {
        Vector::new(self.x_i32(), self.y_i32())
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

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Vector { x, y }
    }

    pub fn subtract(&self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }
    pub fn difference(&self, other: Vector) -> Vector {
        Vector::new((self.x - other.x).abs(), (self.y - other.y).abs())
    }

    pub fn direction(&self) -> Vector {
        if self.x == 0 && self.y == 0 {
            return Vector::new(0, 0);
        }

        let direction_x = if self.x > 0 {
            1
        } else if self.x < 0 {
            -1
        } else {
            0
        };
        let direction_y = if self.y > 0 {
            1
        } else if self.y < 0 {
            -1
        } else {
            0
        };

        Vector::new(direction_x, direction_y)
    }
}
