use super::{
    piece::{CaptureType, MoveType, PromotionPiece},
    Color, Coordinate, Piece,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ChessPosition {
    pub squares: [[Option<Piece>; 8]; 8],
    side_to_move: Color,
}

impl ChessPosition {
    pub fn new(chess_board: &ChessBoard) -> Self {
        ChessPosition { squares: chess_board.squares, side_to_move: chess_board.side_to_move }
    }
}

#[derive(Clone, Debug)]
pub struct ChessBoard {
    pub squares: [[Option<Piece>; 8]; 8],
    side_to_move: Color,
    pub white_king_position: Coordinate,
    pub black_king_position: Coordinate,
    turn_number: u16,
    pub game_status: GameStatus,
    pub move_rule_counter: u8,
    previous_positions: Vec<ChessPosition>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawType {
    Stalemate,
    MoveRule,
    Repetion,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameStatus {
    Ongoing,
    Draw(DrawType),
    Win(Color),
}

pub enum MoveError {
    PieceMovedToSameSquare,
    OutOfBounds,
    NoPieceToMove,
    NotYourTurn,
    IllegalMove,
    GameHasEnded,
}

impl ChessBoard {
    pub fn check_game_status(&self) -> GameStatus {
        let moves = self.all_legal_moves();
        let mut side_to_move = self.side_to_move;

        if moves.len() == 0 {
            if self.is_in_check(self.side_to_move) {
                side_to_move.switch();
                return GameStatus::Win(side_to_move);
            }
            return GameStatus::Draw(DrawType::Stalemate);
        }
        if self.move_rule_counter >= 100 {
            return GameStatus::Draw(DrawType::MoveRule);
        }

        let mut all_positions = self.previous_positions.clone();
        all_positions.push(ChessPosition::new(self));
        for i in 0..all_positions.len() - 1 {
            let mut repetitions = 0;
            for j in 1..all_positions.len() {
                if all_positions.get(i) == all_positions.get(j) {
                    repetitions += 1;
                }
                if repetitions >= 3 {
                    return GameStatus::Draw(DrawType::Repetion);
                }
            }
        }
        GameStatus::Ongoing
    }

    pub fn get_legal_moves_for_piece_at(&self, piece_coordiante: Coordinate) -> Vec<Coordinate> {
        let mut legal_moves = Vec::<Coordinate>::new();
        if let Some(piece) = self.squares[piece_coordiante.x][piece_coordiante.y] {
            for to_x in 0..8 {
                for to_y in 0..8 {
                    let to = Coordinate::new(to_x, to_y);
                    if piece.is_legal_move(piece_coordiante, to, self, false) != MoveType::Illegal {
                        legal_moves.push(to);
                    }
                }
            }
        }

        legal_moves
    }

    pub fn all_legal_moves(&self) -> Vec<(Coordinate, Coordinate)> {
        let color: Color;
        if self.turn_number % 2 == 0 {
            color = Color::Black;
        } else {
            color = Color::White;
        }
        let mut legal_moves = Vec::<(Coordinate, Coordinate)>::new();
        for from_x in 0..8 {
            for to_x in 0..8 {
                for from_y in 0..8 {
                    for to_y in 0..8 {
                        if let Some(piece) = self.squares[from_x][from_y] {
                            if piece.get_color() == color {
                                let from = Coordinate::new(from_x, from_y);
                                let to = Coordinate::new(to_x, to_y);
                                if piece.is_legal_move(from, to, self, false) != MoveType::Illegal {
                                    legal_moves.push((from, to));
                                }
                            }
                        }
                    }
                }
            }
        }

        legal_moves
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let king_position = match color {
            Color::Black => self.black_king_position,
            Color::White => self.white_king_position,
        };

        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.squares[x][y] {
                    if piece.get_color() != color {
                        if piece.is_legal_move(Coordinate::new(x, y), king_position, self, true) != MoveType::Illegal {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn starting_positions() -> ChessBoard {
        let mut squares = [[None; 8]; 8];

        let positions = vec![
            (0, 0, Piece::Rook { color: Color::White, has_moved: false }),
            (1, 0, Piece::Knight { color: Color::White }),
            (2, 0, Piece::Bishop { color: Color::White }),
            (3, 0, Piece::Queen { color: Color::White }),
            (4, 0, Piece::King { color: Color::White, has_moved: false }),
            (5, 0, Piece::Bishop { color: Color::White }),
            (6, 0, Piece::Knight { color: Color::White }),
            (7, 0, Piece::Rook { color: Color::White, has_moved: false }),
            (0, 1, Piece::Pawn { color: Color::White, enpassantable_turn: None }),
            (1, 1, Piece::Pawn { color: Color::White, enpassantable_turn: None }),
            (2, 1, Piece::Pawn { color: Color::White, enpassantable_turn: None }),
            (3, 1, Piece::Pawn { color: Color::White, enpassantable_turn: None }),
            (4, 1, Piece::Pawn { color: Color::White, enpassantable_turn: None }),
            (5, 1, Piece::Pawn { color: Color::White, enpassantable_turn: None }),
            (6, 1, Piece::Pawn { color: Color::White, enpassantable_turn: None }),
            (7, 1, Piece::Pawn { color: Color::White, enpassantable_turn: None }),
            ////////////////////////////////////////////////////////////////
            (0, 7, Piece::Rook { color: Color::Black, has_moved: false }),
            (1, 7, Piece::Knight { color: Color::Black }),
            (2, 7, Piece::Bishop { color: Color::Black }),
            (3, 7, Piece::Queen { color: Color::Black }),
            (4, 7, Piece::King { color: Color::Black, has_moved: false }),
            (5, 7, Piece::Bishop { color: Color::Black }),
            (6, 7, Piece::Knight { color: Color::Black }),
            (7, 7, Piece::Rook { color: Color::Black, has_moved: false }),
            (0, 6, Piece::Pawn { color: Color::Black, enpassantable_turn: None }),
            (1, 6, Piece::Pawn { color: Color::Black, enpassantable_turn: None }),
            (2, 6, Piece::Pawn { color: Color::Black, enpassantable_turn: None }),
            (3, 6, Piece::Pawn { color: Color::Black, enpassantable_turn: None }),
            (4, 6, Piece::Pawn { color: Color::Black, enpassantable_turn: None }),
            (5, 6, Piece::Pawn { color: Color::Black, enpassantable_turn: None }),
            (6, 6, Piece::Pawn { color: Color::Black, enpassantable_turn: None }),
            (7, 6, Piece::Pawn { color: Color::Black, enpassantable_turn: None }),
        ];

        for (x, y, piece) in positions {
            squares[x][y] = Some(piece);
        }

        ChessBoard { squares, side_to_move: Color::White, white_king_position: Coordinate::new(4, 0), black_king_position: Coordinate::new(4, 7), turn_number: 1, game_status: GameStatus::Ongoing, move_rule_counter: 0, previous_positions: Vec::new() }
    }

    pub fn stalemate_start() -> ChessBoard {
        let mut squares = [[None; 8]; 8];

        let positions = vec![
            (3, 0, Piece::Queen { color: Color::White }),
            (4, 0, Piece::King { color: Color::White, has_moved: false }),
            ////////////////////////////////////////////////////////////////
            (6, 7, Piece::King { color: Color::Black, has_moved: false }),
        ];

        for (x, y, piece) in positions {
            squares[x][y] = Some(piece);
        }

        ChessBoard { squares, side_to_move: Color::White, white_king_position: Coordinate::new(4, 0), black_king_position: Coordinate::new(4, 7), turn_number: 1, game_status: GameStatus::Ongoing, move_rule_counter: 0, previous_positions: Vec::new() }
    }

    fn reset_enpassantable_flags(&mut self) {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(Piece::Pawn { ref mut enpassantable_turn, .. }) = self.squares[x][y] {
                    if let Some(turn) = *enpassantable_turn {
                        if turn != self.turn_number {
                            *enpassantable_turn = None;
                        }
                    }
                }
            }
        }
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize), promotion: Option<PromotionPiece>) -> Result<GameStatus, MoveError> {
        let from = Coordinate::from_tuple_usize(from);
        let to = Coordinate::from_tuple_usize(to);
        self.reset_enpassantable_flags();
        if from == to {
            return Err(MoveError::PieceMovedToSameSquare);
        }
        if to.x >= 8 || to.y >= 8 || from.y >= 8 || from.y >= 8 {
            return Err(MoveError::OutOfBounds);
        }
        if self.game_status != GameStatus::Ongoing {
            return Err(MoveError::GameHasEnded);
        }
        let position_before_move = ChessPosition::new(self);
        match self.squares[from.x][from.y] {
            None => Err(MoveError::NoPieceToMove),
            Some(piece) => {
                if piece.get_color() != self.side_to_move {
                    return Err(MoveError::NotYourTurn);
                }
                match piece.is_legal_move(from, to, self, false) {
                    MoveType::CastleShort => {
                        self.move_rule_counter += 1;
                        self.squares[to.x][to.y] = self.squares[from.x][from.y].take();
                        self.squares[5][to.y] = self.squares[7][to.y].take();
                    }
                    MoveType::CastleLong => {
                        self.move_rule_counter += 1;
                        self.squares[to.x][to.y] = self.squares[from.x][from.y].take();
                        self.squares[3][to.y] = self.squares[0][to.y].take();
                        if let Some(Piece::King { color, ref mut has_moved }) = self.squares[to.x][to.y] {
                            *has_moved = true;
                            match color {
                                Color::White => self.white_king_position = to,
                                Color::Black => self.black_king_position = to,
                            }
                        }
                    }
                    MoveType::Other => {
                        self.move_rule_counter += 1;
                        self.squares[to.x][to.y] = self.squares[from.x][from.y].take();
                    }
                    MoveType::DoublePawn => {
                        self.move_rule_counter = 0;
                        if let Some(Piece::Pawn { ref mut enpassantable_turn, .. }) = self.squares[from.x][from.y] {
                            *enpassantable_turn = Some(self.turn_number + 1);
                        }
                        self.squares[to.x][to.y] = self.squares[from.x][from.y].take();
                    }
                    MoveType::Promotion => {
                        self.move_rule_counter = 0;
                        self.squares[from.x][from.y].take();
                        match promotion {
                            Some(promotion_piece) => self.squares[to.x][to.y] = promotion_piece.as_piece(piece.get_color()),
                            None => (),
                        }
                    }
                    MoveType::SinglePawn => {
                        self.move_rule_counter = 0;
                        self.squares[to.x][to.y] = self.squares[from.x][from.y].take();
                    }
                    MoveType::Illegal => return Err(MoveError::IllegalMove),
                    MoveType::Capture(capture_type) => {
                        self.move_rule_counter = 0;
                        match capture_type {
                            CaptureType::EnPassant => {
                                self.squares[to.x][to.y] = self.squares[from.x][from.y].take();
                                self.squares[to.x][from.y].take();
                            }
                            CaptureType::Promotion => {
                                self.squares[from.x][from.y].take();
                                match promotion {
                                    Some(promotion_piece) => self.squares[to.x][to.y] = promotion_piece.as_piece(piece.get_color()),
                                    None => (),
                                }
                            }
                            CaptureType::Other => self.squares[to.x][to.y] = self.squares[from.x][from.y].take(),
                        }
                    }
                }

                if let Some(Piece::King { color, ref mut has_moved }) = self.squares[to.x][to.y] {
                    *has_moved = true;
                    match color {
                        Color::White => self.white_king_position = to,
                        Color::Black => self.black_king_position = to,
                    }
                }

                self.previous_positions.push(position_before_move);
                self.turn_number += 1;
                self.side_to_move.switch();

                let game_status = self.check_game_status();

                self.game_status = game_status;

                Ok(game_status)
            }
        }
    }

    pub fn display_as_text(&self) {
        for i in (0..8).rev() {
            for j in 0..8 {
                let piece = self.squares[j][i];
                match piece {
                    Some(Piece::Pawn { color: Color::Black, .. }) => print!("[p]"),
                    Some(Piece::Knight { color: Color::Black }) => print!("[n]"),
                    Some(Piece::Bishop { color: Color::Black }) => print!("[b]"),
                    Some(Piece::Rook { color: Color::Black, .. }) => print!("[r]"),
                    Some(Piece::Queen { color: Color::Black }) => print!("[q]"),
                    Some(Piece::King { color: Color::Black, .. }) => print!("[k]"),

                    Some(Piece::Pawn { color: Color::White, .. }) => print!("[P]"),
                    Some(Piece::Knight { color: Color::White }) => print!("[N]"),
                    Some(Piece::Bishop { color: Color::White }) => print!("[B]"),
                    Some(Piece::Rook { color: Color::White, .. }) => print!("[R]"),
                    Some(Piece::Queen { color: Color::White }) => print!("[Q]"),
                    Some(Piece::King { color: Color::White, .. }) => print!("[K]"),
                    None => print!("[ ]"),
                }
            }
            println!();
        }
    }
}
