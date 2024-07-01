use super::{coordinate::Coordinate, ChessBoard};

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
    PawnCapture,
    OtherLegal,
    Illegal,
}

#[derive(PartialEq)]
pub enum CastleType {
    Short,
    Long,
}

impl Piece {
    pub(crate) fn is_legal_move(&self, from: Coordinate, to: Coordinate, board: &ChessBoard) -> MoveType {
        if let Some(destination_piece) = board.squares[to.x_i32()][to.y_i32()] {
            if self.get_color() == destination_piece.get_color() {
                return MoveType::Illegal;
            }
        }

        let mut board_after_move = board.clone();
        board_after_move.squares[to.x_i32()][to.y_i32()] = board_after_move.squares[from.x_i32()][from.y_i32()].take();
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

    fn is_legal_pawn_move(from: Coordinate, to: Coordinate, board: &ChessBoard, color: Color) -> MoveType {
        let is_capture = match board.squares[to.x_i32()][to.y_i32()] {
            Some(piece) => piece.get_color() != color,
            None => false,
        };

        if is_capture {
            if color == Color::White {
                if (to.x_i32()_i32() - from.x_i32()_i32() == 1 && to.y_i32()_i32() - from.y_i32()_i32() == 1) || (to.x_i32()_i32() - from.x_i32()_i32() == -1 && to.y_i32()_i32() - from.y_i32()_i32() == 1) {
                    return MoveType::PawnCapture;
                }
            }
            if color == Color::Black {
                if (to.x_i32() as i32 - from.x_i32() as i32 == 1 && to.y_i32() as i32 - from.y_i32() as i32 == -1) || (to.x_i32() as i32 - from.x_i32() as i32 == -1 && to.y_i32() as i32 - from.y_i32() as i32 == -1) {
                    return MoveType::PawnCapture;
                }
            }
            return MoveType::Illegal;
        }

        match color {
            Color::White => {
                if from.y_i32() == 4 && to.y_i32() == 5 && (to.x_i32() == from.x_i32() + 1 || to.x_i32() == from.x_i32() - 1) {
                    if let Some(Piece::Pawn { color: Color::Black, enpassantable_turn: Some(_) }) = board.squares[to.x_i32()][to.y_i32() - 1] {
                        return MoveType::EnPassant;
                    }
                }
            }
            Color::Black => {
                if from.y_i32() == 3 && to.y_i32() == 2 && (to.x_i32() == from.x_i32() + 1 || to.x_i32() == from.x_i32() - 1) {
                    if let Some(Piece::Pawn { color: Color::White, enpassantable_turn: Some(_) }) = board.squares[to.x_i32()][to.y_i32() + 1] {
                        return MoveType::EnPassant;
                    }
                }
            }
        }

        if from.x_i32() != to.x_i32() {
            return MoveType::Illegal;
        }

        if to.y_i32() < from.y_i32() && color == Color::White || to.y_i32() > from.y_i32() && color == Color::Black{
            return MoveType::Illegal;
        }

        let can_double_move = match color {
            Color::Black => from.y_i32() == 6 && !board.squares[from.x_i32()][from.y_i32() - 1].is_some(),
            Color::White => from.y_i32() == 1 && !board.squares[from.x_i32()][from.y_i32() + 1].is_some(),
        };

        if from.y_i32().abs_diff(to.y_i32()) <= 2 && can_double_move {
            if let Some(_) = Some(board.squares[from.y_i32() + 1][from.x_i32()]){

            }
            return MoveType::DoublePawn;
        }

        if from.y_i32().abs_diff(to.y_i32()) <= 1 {
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

    pub(crate) fn is_legal_bishop_move(from: Coordinate, to: Coordinate, board: &ChessBoard) -> bool {
        let difference = from.difference(to);
        if difference.x != difference.y {
            return false;
        }

        let movement_vector = to.subtract(from);

        let movement_direction = movement_vector.direction();

        let mut is_blocked = false;

        let x_end = to.x_i32();
        let y_end = to.y_i32();
        let mut x = from.x_i32() + movement_direction.x;
        let mut y = from.y_i32() + movement_direction.y;
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

    pub(crate) fn is_legal_rook_move(from: Coordinate, to: Coordinate, board: &ChessBoard) -> bool {
        let difference = from.difference(to);
        
        if difference.x != 0 && difference.y != 0 {
            return false;
        }
    
        let movement_vector = to.subtract(from);
        let movement_direction = movement_vector.direction();
        
    
        let mut x = from.x_i32() + movement_direction.x;
        let mut y = from.y_i32() + movement_direction.y;
        let x_end = to.x_i32();
        let y_end = to.y_i32();
    
        while x != x_end || y != y_end {
            if let Some(_) = board.squares[x as usize][y as usize] {
                return false;
            }
            x += movement_direction.x;
            y += movement_direction.y;
        }
    
        true
    }
    

    pub(crate) fn is_legal_queen_move(from: Coordinate, to: Coordinate, board: &ChessBoard) -> bool {
        Self::is_legal_rook_move(from, to, board) || Self::is_legal_bishop_move(from, to, board)
    }

    pub(crate) fn is_legal_king_move(&self, from: Coordinate, to: Coordinate, board: &ChessBoard, color: Color, has_moved: bool) -> MoveType {
        let difference = from.difference(to);
        if difference.x <= 1 && difference.y <= 1 {
            return MoveType::OtherLegal;
        }

        if has_moved {
            return MoveType::Illegal;
        }

        match color {
            Color::White => {
                if to == Coordinate::new(6, 0) {
                    // white short castle
                    if Piece::is_legal_castleing_move(color, board, 6, 7, 5) {
                        return MoveType::CastleShort;
                    }
                } else if to == Coordinate::new(2, 0) {
                    // white long castle
                    if Piece::is_legal_castleing_move(color, board, 2, 0, 3) {
                        return MoveType::CastleLong;
                    }
                }
            }
            Color::Black => {
                if to == Coordinate::new(6, 7) {
                    // Black short castle
                    if Piece::is_legal_castleing_move(color, board, 6, 7, 5) {
                        return MoveType::CastleShort;
                    }
                } else if to == Coordinate::new(2, 7) {
                    // Black long castle
                    if Piece::is_legal_castleing_move(color, board, 2, 0, 3) {
                        return MoveType::CastleLong;
                    }
                }
            }
        }

        MoveType::Illegal
    }

    fn is_legal_castleing_move(color: Color, board: &ChessBoard, to_x: usize, rook_x: usize, between_x: usize) -> bool {
        let rank_y = match color{
            Color::Black => 7,
            Color::White => 0,
        };
        if let Some(Piece::Rook { has_moved, .. }) = board.squares[rook_x][rank_y] {
            if has_moved {
                return false;
            }
        }

        if let Some(_piece) = board.squares[to_x][rank_y] {
            return false;
        }
        if let Some(_piece) = board.squares[between_x][rank_y] {
            return false;
        }

        for y in 0..8 {
            for x in 0..8 {
                let square = board.squares[x][y];
                if let Some(piece) = square {
                    if piece.get_color() != color {
                        if piece.is_legal_move(Coordinate::new(x, y), Coordinate::new(between_x, rank_y), board) != MoveType::Illegal || piece.is_legal_move(Coordinate::new(x, y), Coordinate::new(to_x, rank_y), board) != MoveType::Illegal || piece.is_legal_move(Coordinate::new(x, y), Coordinate::new(4, rank_y), board) != MoveType::Illegal {
                            dbg!(piece);
                            dbg!(x,y);
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
