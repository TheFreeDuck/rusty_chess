#[derive(Copy, Clone, Debug)]
enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece{
    fn is_legal_move(&self, from: (usize, usize), to: (usize, usize), board: &Board) -> bool{
        match self {
            Piece::Pawn(color) => Piece::is_legal_pawn_move(from, to, board, *color),
            Piece::Knight(color) => Piece::is_legal_knight_move(from, to, board, *color),
            Piece::Bishop(color) => Piece::is_legal_bishop_move(from, to, board, *color),
            Piece::Rook(color) => Piece::is_legal_rook_move(from, to, board, *color),
            Piece::Queen(color) => Piece::is_legal_queen_move(from, to, board, *color),
            Piece::King(color) => Piece::is_legal_king_move(from, to, board, *color),
        }
    }

    fn is_legal_pawn_move(from: (usize, usize), to: (usize, usize), board: &Board, color: Color) -> bool {
        true
    }

    fn is_legal_knight_move(from: (usize, usize), to: (usize, usize), board: &Board, color: Color) -> bool {
        // Placeholder logic
        true
    }

    fn is_legal_bishop_move(from: (usize, usize), to: (usize, usize), board: &Board, color: Color) -> bool {
        // Placeholder logic
        true
    }

    fn is_legal_rook_move(from: (usize, usize), to: (usize, usize), board: &Board, color: Color) -> bool {
        // Placeholder logic
        true
    }

    fn is_legal_queen_move(from: (usize, usize), to: (usize, usize), board: &Board, color: Color) -> bool {
        // Placeholder logic
        true
    }

    fn is_legal_king_move(from: (usize, usize), to: (usize, usize), board: &Board, color: Color) -> bool {
        // Placeholder logic
        true
    }
}

#[derive(Copy, Clone, Debug)]
enum Color {
    Black,
    White,
}

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
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

        Board { squares }
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), &str> {
        if from == to {
            return Err("You didn't move the piece")
        }
        match self.squares[from.0][from.1] {
            Some(piece) => {
                if piece.is_legal_move(from, to, self) {
                    self.squares[to.0][to.1] = self.squares[from.0][from.1].take();
                    return Ok(())
                }
                Err("Illegal move!")
            },
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
