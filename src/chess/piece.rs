use super::{coordinate::Coordinate, Board};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Pawn { color: Color, enpassantable_turn: Option<u16> },
    Knight { color: Color },
    Bishop { color: Color },
    Rook { color: Color, has_moved: bool },
    Queen { color: Color },
    King { color: Color, has_moved: bool },
}

#[derive(PartialEq)]
pub enum MoveType {
    CastleShort,
    CastleLong,
    EnPassant,
    DoublePawn,
    OtherLegal,
    Illegal,
}

#[derive(PartialEq)]
pub enum CastleType{
    Short,
    Long
}

impl Piece {
    pub(crate) fn is_legal_move(&self, from: Coordinate, to: Coordinate, board: &Board) -> MoveType {
        if let Some(destination_piece) = board.squares[to.x_usize()][to.y_usize()] {
            if self.get_color() == destination_piece.get_color() {
                return MoveType::Illegal;
            }
        }

        let mut board_after_move = board.clone();
        board_after_move.squares[to.x_usize()][to.y_usize()] = board_after_move.squares[from.x_usize()][from.y_usize()].take();
        if board_after_move.is_in_check(self.get_color()) {
            return MoveType::Illegal;
        }

        match self {
            Piece::Pawn { color, .. } => Piece::is_legal_pawn_move(from, to, board, *color),
            Piece::Knight { .. } => {
                if Piece::is_legal_knight_move(from, to) {
                    return MoveType::OtherLegal;
                }
                return MoveType::Illegal;
            }
            Piece::Bishop { .. } => {
                if Piece::is_legal_bishop_move(from, to, board) {
                    return MoveType::OtherLegal;
                }
                return MoveType::Illegal;
            }
            Piece::Rook { .. } => {
                if Piece::is_legal_rook_move(from, to, board) {
                    MoveType::OtherLegal
                } else {
                    MoveType::Illegal
                }
            }
            Piece::Queen { .. } => {
                if Piece::is_legal_queen_move(from, to, board) {
                    MoveType::OtherLegal
                } else {
                    MoveType::Illegal
                }
            }
            Piece::King { color, has_moved } => Piece::is_legal_king_move(self, from, to, board, *color, *has_moved),
        }
    }

    fn is_legal_pawn_move(from: Coordinate, to: Coordinate, board: &Board, color: Color) -> MoveType {
        let is_capture = match board.squares[to.x_usize()][to.y_usize()] {
            Some(_) => board.squares[to.x_usize()][to.y_usize()].unwrap().get_color() != color,
            None => false,
        };

        if is_capture {
            if color == Color::White {
                if (to.x - from.x == 1 && to.y - from.y == 1) || to.y - from.y == -1 {
                    return MoveType::OtherLegal;
                }
            }
            if color == Color::Black {
                if (to.x - from.x == -1 && to.y - from.y == 1) || to.y - from.y == -1 {
                    return MoveType::OtherLegal;
                }
            }
            return MoveType::Illegal;
        }

        match color {
            Color::White => {
                
                if from.y == 4 && to.y == 5 && (to.x == from.x + 1 || to.x == from.x - 1) {
                    if let Some(Piece::Pawn { color: Color::Black, enpassantable_turn: Some(_)}) = board.squares[to.x_usize()][to.y_usize() - 1] {
                        return MoveType::EnPassant;
                    }
                }
            }
            Color::Black => {
                if from.y == 3 && to.y == 2 && (to.x == from.x + 1 || to.x == from.x - 1) {
                    if let Some(Piece::Pawn { color: Color::White, enpassantable_turn: Some(_) }) = board.squares[to.x_usize()][to.y_usize() + 1] {
                        return MoveType::EnPassant;
                    }
                }
            }
        }

        if from.x != to.x {
            return MoveType::Illegal;
        }

        let can_double_move = (from.y == 1 && color == Color::White) || (from.y == 6 && color == Color::Black);
        if from.y.abs_diff(to.y) <= 2 && can_double_move{
            return MoveType::DoublePawn;
        }

        if from.y.abs_diff(to.y) <= 1{
            return MoveType::OtherLegal;
        }

        MoveType::Illegal
    }

    pub(crate) fn is_legal_knight_move(from: Coordinate, to: Coordinate) -> bool {
        let difference = from.difference(to);
        if difference == Coordinate::new(2, 1) || difference == Coordinate::new(1, 2) {
            return true;
        }

        false
    }

    pub(crate) fn is_legal_bishop_move(from: Coordinate, to: Coordinate, board: &Board) -> bool {
        let difference = from.difference(to);
        if difference.x != difference.y {
            return false;
        }

        let movement_vector = to.subtract(from);

        let movement_direction = movement_vector.direction();

        let mut is_blocked = false;

        let x_end = to.x;
        let y_end = to.y;
        let mut x = from.x + movement_direction.x;
        let mut y = from.y + movement_direction.y;
        while x != x_end && y != y_end {
            is_blocked = match board.squares[x as usize][y as usize] {
                Some(_) => {
                    return false;
                }
                None => false,
            };
            x += movement_direction.x;
            y += movement_direction.y;
        }

        !is_blocked
    }

    pub(crate) fn is_legal_rook_move(from: Coordinate, to: Coordinate, board: &Board) -> bool {
        let movement_vector = to.subtract(from);
        if movement_vector.x == 0 {
            let mut is_blocked = false;
            if movement_vector.y > 0 {
                for y in from.y + 1..to.y {
                    is_blocked = match board.squares[from.x_usize()][y as usize] {
                        Some(_) => {
                            return false;
                        }
                        None => false,
                    };
                }
            } else {
                for y in (to.y + 1..from.y).rev() {
                    is_blocked = match board.squares[from.x_usize()][y as usize] {
                        Some(_) => {
                            return false;
                        }
                        None => false,
                    };
                }
            }

            return !is_blocked;
        }

        if movement_vector.y == 0 {
            let mut is_blocked = false;
            if movement_vector.x > 0 {
                for y in from.x + 1..to.x {
                    is_blocked = match board.squares[y as usize][from.y_usize()] {
                        Some(_) => {
                            return false;
                        }
                        None => false,
                    };
                }
            } else {
                for y in (to.y + 1..from.x).rev() {
                    is_blocked = match board.squares[y as usize][from.y_usize()] {
                        Some(_) => {
                            return false;
                        }
                        None => false,
                    };
                }
            }

            return !is_blocked;
        }
        false
    }

    pub(crate) fn is_legal_queen_move(from: Coordinate, to: Coordinate, board: &Board) -> bool {
        Self::is_legal_rook_move(from, to, board) || Self::is_legal_bishop_move(from, to, board)
    }

    pub(crate) fn is_legal_king_move(&self, from: Coordinate, to: Coordinate, board: &Board, color: Color, has_moved: bool) -> MoveType {
        let difference = from.difference(to);
        if difference.x == 1 || difference.y == 1 {
            return MoveType::OtherLegal;
        }

        if has_moved {
            return MoveType::Illegal;
        }

        match color {
            Color::White => {
                if to == Coordinate::new(6, 0) {
                    // white short castle
                    if Piece::is_legal_castleing_move(color, board, 6, 7, 5, 0) {
                        return MoveType::CastleShort;
                    }
                } else if to == Coordinate::new(1, 0) {
                    // white long castle
                    if Piece::is_legal_castleing_move(color, board, 1, 0, 2, 0) {
                        return MoveType::CastleLong;
                    }
                }
            }
            Color::Black => {
                if to == Coordinate::new(6, 0) {
                    // Black short castle
                    if Piece::is_legal_castleing_move(color, board, 6, 7, 5, 7) {
                        return MoveType::CastleShort;
                    }
                } else if to == Coordinate::new(1, 0) {
                    // Black long castle
                    if Piece::is_legal_castleing_move(color, board, 1, 0, 2, 7) {
                        return MoveType::CastleLong;
                    }
                }
            }
        }

        MoveType::Illegal
    }

    fn is_legal_castleing_move(color: Color, board: &Board, to_x: usize, rook_x: usize, between_x: usize, y: usize) -> bool {
        if let Some(Piece::Rook { has_moved, .. }) = board.squares[rook_x][y] {
            if has_moved {
                return false;
            }
        }

        if let Some(_piece) = board.squares[to_x][y] {
            return false;
        }
        if let Some(_piece) = board.squares[between_x][y] {
            return false;
        }

        for y in 0..8 {
            for x in 0..8 {
                let square = board.squares[x][y];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        if piece.is_legal_move(Coordinate::new(x as i32, y as i32), Coordinate::new(between_x as i32, y as i32), board) == MoveType::OtherLegal || piece.is_legal_move(Coordinate::new(x as i32, y as i32), Coordinate::new(to_x as i32, y as i32), board) == MoveType::OtherLegal || piece.is_legal_move(Coordinate::new(x as i32, y as i32), Coordinate::new(4, y as i32), board) == MoveType::OtherLegal {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub(crate) fn get_color(&self) -> Color {
        match self {
            Piece::Pawn { color, .. } => *color,
            Piece::Knight { color } => *color,
            Piece::Bishop { color } => *color,
            Piece::Rook { color, .. } => *color,
            Piece::Queen { color } => *color,
            Piece::King { color, .. } => *color,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn switch(&mut self) {
        *self = match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }
}
