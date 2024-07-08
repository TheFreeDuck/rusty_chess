use super::{
    coordinate::{Coordinate, Vector},
    ChessBoard,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Pawn { color: Color, enpassantable_turn: Option<u16> },
    Knight { color: Color },
    Bishop { color: Color },
    Rook { color: Color, has_moved: bool },
    Queen { color: Color },
    King { color: Color, has_moved: bool },
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl PromotionPiece {
    pub fn as_piece(&self, color: Color) -> Option<Piece> {
        match self {
            PromotionPiece::Knight => Some(Piece::Knight { color: color }),
            PromotionPiece::Bishop => Some(Piece::Bishop { color: color }),
            PromotionPiece::Rook => Some(Piece::Rook { color: color, has_moved: true }),
            PromotionPiece::Queen => Some(Piece::Queen { color: color }),
        }
    }
}

#[derive(PartialEq)]
pub enum MoveType {
    CastleShort,
    CastleLong,
    SinglePawn,
    DoublePawn,
    Promotion,
    Capture(CaptureType),
    Illegal,
    Other,
}
#[derive(PartialEq)]
pub enum CaptureType {
    EnPassant,
    Promotion,
    Other,
}

#[derive(PartialEq)]
pub enum CastleType {
    Short,
    Long,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn switch(&mut self) {
        *self = match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }
}

impl Piece {
    pub fn is_legal_move(&self, from: Coordinate, to: Coordinate, board: &ChessBoard, ignore_checks: bool) -> MoveType {
        if let Some(destination_piece) = board.squares[to.x][to.y] {
            if self.get_color() == destination_piece.get_color() {
                return MoveType::Illegal;
            }
        }

        if !ignore_checks {
            let mut board_after_move = board.clone();
            if let Some(Piece::King { color, .. }) = board.squares[from.x][from.y] {
                match color {
                    Color::Black => board_after_move.black_king_position = to,
                    Color::White => board_after_move.white_king_position = to,
                }
            }
            board_after_move.squares[to.x][to.y] = board_after_move.squares[from.x][from.y].take();
            if board_after_move.is_in_check(self.get_color()) {
                return MoveType::Illegal;
            }
        }

        match self {
            Piece::Pawn { color, .. } => Piece::is_legal_pawn_move(from, to, board, *color),
            Piece::Knight { color } => Piece::is_legal_knight_move(from, to, board, *color),
            Piece::Bishop { color } => Piece::is_legal_bishop_move(from, to, board, *color),
            Piece::Rook { color, .. } => Piece::is_legal_rook_move(from, to, board, *color),
            Piece::Queen { color } => Piece::is_legal_queen_move(from, to, board, *color),
            Piece::King { color, has_moved } => Piece::is_legal_king_move(from, to, board, *color, *has_moved),
        }
    }

    fn is_legal_pawn_move(from: Coordinate, to: Coordinate, board: &ChessBoard, color: Color) -> MoveType {
        let is_capture = match board.squares[to.x][to.y] {
            Some(piece) => piece.get_color() != color,
            None => false,
        };

        let from_vector = from.vector();
        let to_vector = to.vector();

        if is_capture {
            if color == Color::White {
                if (to_vector.x - from_vector.x == 1 && to_vector.y - from_vector.y == 1) || (to_vector.x - from_vector.x == -1 && to_vector.y - from_vector.y == 1) {
                    if to.y == 7 && color == Color::White || to.y == 0 && color == Color::Black {
                        return MoveType::Capture(CaptureType::Promotion);
                    }
                    return MoveType::Capture(CaptureType::Other);
                }
            }
            if color == Color::Black {
                if (to_vector.x - from_vector.x == 1 && to_vector.y - from_vector.y == -1) || (to_vector.x - from_vector.x == -1 && to_vector.y - from_vector.y == -1) {
                    if to.y == 7 && color == Color::White || to.y == 0 && color == Color::Black {
                        return MoveType::Capture(CaptureType::Promotion);
                    }
                    return MoveType::Capture(CaptureType::Other);
                }
            }
            return MoveType::Illegal;
        }

        match color {
            Color::White => {
                if from_vector.y == 4 && to_vector.y == 5 && (to_vector.x == from_vector.x + 1 || to_vector.x == from_vector.x - 1) {
                    if let Some(Piece::Pawn { color: Color::Black, enpassantable_turn: Some(_) }) = board.squares[to.x][to.y - 1] {
                        return MoveType::Capture(CaptureType::EnPassant);
                    }
                }
            }
            Color::Black => {
                if from_vector.y == 3 && to_vector.y == 2 && (to_vector.x == from_vector.x + 1 || to_vector.x == from_vector.x - 1) {
                    if let Some(Piece::Pawn { color: Color::White, enpassantable_turn: Some(_) }) = board.squares[to.x][to.y + 1] {
                        return MoveType::Capture(CaptureType::EnPassant);
                    }
                }
            }
        }

        if from.x != to.x {
            return MoveType::Illegal;
        }

        if to.y < from.y && color == Color::White || to.y > from.y && color == Color::Black {
            return MoveType::Illegal;
        }

        let can_double_move = match color {
            Color::Black => from.y == 6 && !board.squares[from.x][from.y - 1].is_some(),
            Color::White => from.y == 1 && !board.squares[from.x][from.y + 1].is_some(),
        };

        if from.y.abs_diff(to.y) == 2 && can_double_move {
            if let Some(_) = Some(board.squares[from.y + 1][from.x]) {}
            return MoveType::DoublePawn;
        }

        if from.y.abs_diff(to.y) == 1 {
            if to.y == 7 && color == Color::White || to.y == 0 && color == Color::Black {
                return MoveType::Promotion;
            }
            return MoveType::Other;
        }

        MoveType::Illegal
    }

    pub fn is_legal_knight_move(from: Coordinate, to: Coordinate, board: &ChessBoard, color: Color) -> MoveType {
        let from_vector = from.vector();
        let to_vector = to.vector();
        let difference = from_vector.difference(to_vector);
        if difference == Vector::new(2, 1) || difference == Vector::new(1, 2) {
            if let Some(piece) = board.squares[to.x][to.y] {
                if piece.get_color() != color {
                    return MoveType::Capture(CaptureType::Other);
                }
            } else {
                return MoveType::Other;
            }
        }

        MoveType::Illegal
    }

    pub fn is_legal_bishop_move(from: Coordinate, to: Coordinate, board: &ChessBoard, color: Color) -> MoveType {
        let from_vector = from.vector();
        let to_vector = to.vector();

        let difference = from_vector.difference(to_vector);
        if difference.x != difference.y {
            return MoveType::Illegal;
        }

        let movement_vector = to_vector.subtract(from_vector);

        let movement_direction = movement_vector.direction();

        let mut is_blocked = false;

        let x_end = to_vector.x;
        let y_end = to_vector.y;
        let mut x = from_vector.x + movement_direction.x;
        let mut y = from_vector.y + movement_direction.y;
        while x != x_end && y != y_end {
            is_blocked = match board.squares[x as usize][y as usize] {
                Some(_) => {
                    return MoveType::Illegal;
                }
                None => false,
            };
            x += movement_direction.x;
            y += movement_direction.y;
        }

        if let Some(piece) = board.squares[to.x][to.y] {
            if piece.get_color() != color && !is_blocked {
                return MoveType::Capture(CaptureType::Other);
            }
        } else {
            return MoveType::Other;
        }
        return MoveType::Illegal;
    }

    pub fn is_legal_rook_move(from: Coordinate, to: Coordinate, board: &ChessBoard, color: Color) -> MoveType {
        let from_vector = from.vector();
        let to_vector = to.vector();

        let difference = from_vector.difference(to_vector);

        if difference.x != 0 && difference.y != 0 {
            return MoveType::Illegal;
        }

        let movement_vector = to_vector.subtract(from_vector);
        let movement_direction = movement_vector.direction();

        let mut x = from_vector.x + movement_direction.x;
        let mut y = from_vector.y + movement_direction.y;
        let x_end = to_vector.x;
        let y_end = to_vector.y;

        while x != x_end || y != y_end {
            if let Some(_) = board.squares[x as usize][y as usize] {
                return MoveType::Illegal;
            }
            x += movement_direction.x;
            y += movement_direction.y;
        }


        if let Some(piece) = board.squares[to.x][to.y] {
            if piece.get_color() != color{
                return MoveType::Capture(CaptureType::Other);
            }
        } else {
            return MoveType::Other;
        }
        return MoveType::Illegal;
    }

    pub fn is_legal_queen_move(from: Coordinate, to: Coordinate, board: &ChessBoard, color: Color) -> MoveType {
        let legal_rook = Self::is_legal_rook_move(from, to, board, color);
        let legal_bishop = Self::is_legal_bishop_move(from, to, board, color);

        if legal_bishop != MoveType::Illegal {
            return legal_bishop;
        }else if legal_rook != MoveType::Illegal{
            return legal_rook;
        }
        return MoveType::Illegal;
    }

    pub fn is_legal_king_move(from: Coordinate, to: Coordinate, board: &ChessBoard, color: Color, has_moved: bool) -> MoveType {
        let from_vector = from.vector();
        let to_vector = to.vector();

        let difference = from_vector.difference(to_vector);
        if difference.x <= 1 && difference.y <= 1 {
            if let Some(piece) = board.squares[to.x][to.y] {
                if piece.get_color() != color{
                    return MoveType::Capture(CaptureType::Other);
                }
            } else {
                return MoveType::Other;
            }
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
        let rank_y = match color {
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
                        if piece.is_legal_move(Coordinate::new(x, y), Coordinate::new(between_x, rank_y), board, false) != MoveType::Illegal || piece.is_legal_move(Coordinate::new(x, y), Coordinate::new(to_x, rank_y), board, false) != MoveType::Illegal || piece.is_legal_move(Coordinate::new(x, y), Coordinate::new(4, rank_y), board, false) != MoveType::Illegal {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn get_color(&self) -> Color {
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
