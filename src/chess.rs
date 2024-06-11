use std::collections::btree_set::Difference;


#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    fn is_legal_move(&self, from: Coordinate, to: Coordinate, board: &Board) -> bool {
        if let Some(destination_piece) = board.squares[to.x_usize()][to.y_usize()] {
            if self.get_color() == destination_piece.get_color() {
                return false;
            }
        }

        match self {
            Piece::Pawn(color) => Piece::is_legal_pawn_move(from, to, board, *color),
            Piece::Knight(color) => Piece::is_legal_knight_move(from, to, board, *color),
            Piece::Bishop(color) => Piece::is_legal_bishop_move(from, to, board, *color),
            Piece::Rook(color) => Piece::is_legal_rook_move(from, to, board, *color),
            Piece::Queen(color) => Piece::is_legal_queen_move(from, to, board, *color),
            Piece::King(color) => Piece::is_legal_king_move(from, to, board, *color),
        }
    }

    fn is_legal_pawn_move(
        from: Coordinate,
        to: Coordinate,
        board: &Board,
        color: Color,
    ) -> bool {
        let is_capture = match board.squares[to.x_usize()][to.y_usize()] {
            Some(_) => board.squares[to.x_usize()][to.y_usize()].unwrap().get_color() != color,
            None => false,
        };
        if is_capture {
            if color == Color::White {
                if to.x - from.x == 1 && to.y - from.y == 1 || to.y - from.y == -1 {
                    return true
                }
            }
            if color == Color::Black {
                if to.x - from.x == -1 && to.y - from.y == 1 || to.y - from.y == -1 {
                    return true
                }
            }
            return false
        }

        if from.x != to.x{
            return false
        }

        let can_double_move = from.y == 1 && color == Color::White || from.y == 6 && color == Color::Black;
        if from.y.abs_diff(to.y) <= 2 && can_double_move || from.y.abs_diff(to.y) <= 1{
            return true;
        }

        false
    }

    fn is_legal_knight_move(
        from: Coordinate,
        to: Coordinate,
        board: &Board,
        color: Color,
    ) -> bool {
        let difference = from.difference(to);
       if difference == Coordinate::new(2,1) || difference == Coordinate::new(1,2){
            return true;
       }

       false
    }

    fn is_legal_bishop_move(
        from: Coordinate,
        to: Coordinate,
        board: &Board,
        color: Color,
    ) -> bool {
        let difference = from.difference(to);
        if difference.x == difference.y {
            return true;
        }
        false
    }

    fn is_legal_rook_move(
        from: Coordinate,
        to: Coordinate,
        board: &Board,
        color: Color,
    ) -> bool {
        // Placeholder logic
        let movement_vector = to.subtract(from);
        if movement_vector.x == 0{
            let mut is_blocked = false;
            if movement_vector.y > 0{
                for i in from.y + 1..to.y{
                    is_blocked = match board.squares[from.x_usize()][i as usize]{
                        Some(_) => return false,
                        None => false,
                    }
                }
            }else{
                for i in (to.y + 1..from.y).rev(){
                    is_blocked = match board.squares[from.x_usize()][i as usize]{
                        Some(_) => return false,
                        None => false,
                    }
                }
            }
            
            return !is_blocked;
        }

        if movement_vector.y == 0 {
            let mut is_blocked = false;
            if movement_vector.x > 0{
                for i in from.x + 1..to.x{
                    is_blocked = match board.squares[i as usize][from.y_usize()]{
                        Some(_) => {return false},
                        None => false,
                    }
                }
            }else{
                for i in (to.y + 1..from.x).rev(){
                    is_blocked = match board.squares[i as usize][from.y_usize()]{
                        Some(_) => return false,
                        None => false,
                    }
                }
            }
            
            return !is_blocked;
        }
        false
    }

    fn is_legal_queen_move(
        from: Coordinate,
        to: Coordinate,
        board: &Board,
        color: Color,
    ) -> bool {
        // Placeholder logic
        true
    }

    fn is_legal_king_move(
        from: Coordinate,
        to: Coordinate,
        board: &Board,
        color: Color,
    ) -> bool {
        let difference = from.difference(to);
        if difference.x == 1 || difference.y == 1{
            return true;
        }
        false
    }

    fn is_in_check(&self,color: Color, board: &Board) -> bool{
        match color{
            Color::Black => {
                for x in 0..8{
                    for y in 0..8{
                        let square = board.squares[x][y];
                        if let Some(piece) = square{
                        }
                    }
                }
            },
            Color::White => todo!(),
        }
        false
    }

    fn get_color(&self) -> Color {
        match self {
            Piece::Pawn(color) => *color,
            Piece::Knight(color) => *color,
            Piece::Bishop(color) => *color,
            Piece::Rook(color) => *color,
            Piece::Queen(color) => *color,
            Piece::King(color) => *color,
        }
    }

}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    Black,
    White,
}

#[derive(Copy, Clone, Debug)]
pub struct Coordinate{
    x: i32,
    y: i32,
}

impl Coordinate {
    fn subtract(&self, other: Coordinate) -> Self{
        Coordinate::new(self.x - other.x, self.y - other.y)
    }
    fn difference(&self, other: Coordinate) -> Self {
        Coordinate::new(
            (self.x - other.x).abs(),
            (self.y - other.y).abs(),
        )
    }
    

    pub fn new(x: i32, y: i32) -> Self {
        Coordinate {x, y}
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

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
    side_to_move: Color,
}

impl Board {
    pub fn starting_positions() -> Board {
        let mut squares = [[None; 8]; 8];

        squares[0][0] = Some(Piece::Rook(Color::White));
        squares[1][0] = Some(Piece::Knight(Color::White));
        squares[2][0] = Some(Piece::Bishop(Color::White));
        squares[3][0] = Some(Piece::Queen(Color::White));
        squares[4][0] = Some(Piece::King(Color::White));
        squares[5][0] = Some(Piece::Bishop(Color::White));
        squares[6][0] = Some(Piece::Knight(Color::White));
        squares[7][0] = Some(Piece::Rook(Color::White));

        for i in 0..8 {
            squares[i][1] = Some(Piece::Pawn(Color::White));
        }

        squares[0][7] = Some(Piece::Rook(Color::Black));
        squares[1][7] = Some(Piece::Knight(Color::Black));
        squares[2][7] = Some(Piece::Bishop(Color::Black));
        squares[3][7] = Some(Piece::Queen(Color::Black));
        squares[4][7] = Some(Piece::King(Color::Black));
        squares[5][7] = Some(Piece::Bishop(Color::Black));
        squares[6][7] = Some(Piece::Knight(Color::Black));
        squares[7][7] = Some(Piece::Rook(Color::Black));

        for i in 0..8 {
            squares[i][6] = Some(Piece::Pawn(Color::Black));
        }

        Board {
            squares,
            side_to_move: Color::White,
        }
    }

    pub fn rook() -> Board {
        let mut squares = [[None; 8]; 8];
        squares[0][7] = Some(Piece::Rook(Color::White));

        squares[0][1] = Some(Piece::Rook(Color::White));
        squares[6][7] = Some(Piece::Rook(Color::White));
        Board {
            squares,
            side_to_move: Color::White,
        }
    }

    pub fn move_piece(&mut self, from: Coordinate, to: Coordinate) -> Result<(), &str> {
        if from == to {
            return Err("Piece did not move");
        }
        if to.x >= 8 && to.y >= 8{
            return Err("Out of bounds");
        }
        match self.squares[from.x_usize()][from.y_usize()] {
            Some(piece) => {
                if piece.is_legal_move(from, to, self) {
                    self.squares[to.x_usize()][to.y_usize()] = self.squares[from.x_usize()][from.y_usize()].take();
                    return Ok(());
                }
                Err("Illegal move!")
            }
            None => Err("There is not a piece here!"),
        }
    }

    pub fn display_as_text(&self) {
        for i in (0..8).rev() {
            for j in 0..8 {
                let piece = self.squares[j][i];

                // read color of piece here and set the color of text
                match piece {
                    Some(Piece::Pawn(Color::Black)) => print!("[p]"),
                    Some(Piece::Knight(Color::Black)) => print!("[n]"),
                    Some(Piece::Bishop(Color::Black)) => print!("[b]"),
                    Some(Piece::Rook(Color::Black)) => print!("[r]"),
                    Some(Piece::Queen(Color::Black)) => print!("[q]"),
                    Some(Piece::King(Color::Black)) => print!("[k]"),

                    Some(Piece::Pawn(Color::White)) => print!("[P]"),
                    Some(Piece::Knight(Color::White)) => print!("[N]"),
                    Some(Piece::Bishop(Color::White)) => print!("[B]"),
                    Some(Piece::Rook(Color::White)) => print!("[R]"),
                    Some(Piece::Queen(Color::White)) => print!("[Q]"),
                    Some(Piece::King(Color::White)) => print!("[K]"),
                    None => print!("[ ]"),
                }
            }
            println!();
        }
    }
}
