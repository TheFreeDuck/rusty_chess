use super::{piece::MoveType, Color, Coordinate, Piece};

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    side_to_move: Color,
    pub white_king_position: Coordinate,
    pub black_king_position: Coordinate,
    turn_number: u16,
}

impl Board {
    pub fn is_in_check(&self, color: Color) -> bool {
        let king_position = match color {
            Color::Black => self.black_king_position,
            Color::White => self.white_king_position,
        };

        for y in 0..8 {
            for x in 0..8 {
                let square = self.squares[x][y];
                if Coordinate::new(x as i32, y as i32) != king_position {
                    if let Some(piece) = square {
                        if piece.get_color() != color {
                            if piece.is_legal_move(Coordinate::new(x as i32, y as i32), king_position, self) == MoveType::OtherLegal {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn starting_positions() -> Board {
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

        Board { squares, side_to_move: Color::White, white_king_position: Coordinate::new(4, 0), black_king_position: Coordinate::new(4, 7), turn_number: 1 }
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
    

    pub fn move_piece(&mut self, from: Coordinate, to: Coordinate) -> Result<(), &str> {
        self.reset_enpassantable_flags();
        if from == to {
            return Err("Piece did not move");
        }
        if to.x >= 8 && to.y >= 8 {
            return Err("Out of bounds");
        }
        match self.squares[from.x_usize()][from.y_usize()] {
            None => Err("There is not a piece here!"),
            Some(piece) => {
                if piece.get_color() != self.side_to_move {
                    return Err("Not your turn");
                }

                match piece.is_legal_move(from, to, self) {
                    MoveType::CastleShort => {
                        self.squares[to.x_usize()][to.y_usize()] = self.squares[from.x_usize()][from.y_usize()].take();
                        self.squares[5][to.y_usize()] = self.squares[7][to.y_usize()].take();
                    }
                    MoveType::CastleLong => {
                        self.squares[to.x_usize()][to.y_usize()] = self.squares[from.x_usize()][from.y_usize()].take();
                        self.squares[0][to.y_usize()] = self.squares[3][to.y_usize()].take();
                    }
                    MoveType::EnPassant => {
                        self.squares[to.x_usize()][to.y_usize()] = self.squares[from.x_usize()][from.y_usize()].take();
                        self.squares[to.x_usize()][from.y_usize()].take();
                    }
                    MoveType::OtherLegal => {
                        self.squares[to.x_usize()][to.y_usize()] = self.squares[from.x_usize()][from.y_usize()].take();
                    }
                    MoveType::Illegal => return Err("illegal move"),
                    MoveType::DoublePawn => {
                        if let Some(Piece::Pawn { ref mut enpassantable_turn, .. }) = self.squares[from.x_usize()][from.y_usize()] {
                            *enpassantable_turn = Some(self.turn_number + 1);
                        }
                        self.squares[to.x_usize()][to.y_usize()] = self.squares[from.x_usize()][from.y_usize()].take();
                    }
                }
                self.turn_number += 1;

                if let Piece::King { color, has_moved: _ } = piece {
                    match color {
                        Color::White => self.white_king_position = to,
                        Color::Black => self.black_king_position = to,
                    }
                }
                self.side_to_move.switch();
                Ok(())
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
