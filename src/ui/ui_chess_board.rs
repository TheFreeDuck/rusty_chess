use crate::{
    chess::{
        self,
        chess_board::{GameStatus, MoveError},
        piece::PromotionPiece,
        ChessBoard, Color as ChessColor,
    },
    draw::WindowParameters,
};
use chess::piece::Piece;
use macroquad::{
    color::{Color, BLACK, DARKBROWN, GRAY, WHITE},
    input::{is_mouse_button_down, is_mouse_button_pressed, MouseButton},
    texture::{self, Texture2D},
};
use std::{collections::HashMap, usize};

use super::draw::load_texture_from_bytes;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum PieceType {
    Pawn(ChessColor),
    Knight(ChessColor),
    Bishop(ChessColor),
    Rook(ChessColor),
    Queen(ChessColor),
    King(ChessColor),
}

#[derive(Clone, PartialEq, Debug)]
pub struct GraphicsPiecee{
    piece_type: PieceType,
    x: f32, 
    y: f32,
    texture: Texture2D, 
    color: Color, 
}

#[derive(Clone, PartialEq, Debug)]
pub enum GraphicsPiece {
    Pawn { texture: Texture2D, color: Color, x: f32, y: f32 },
    Knight { texture: Texture2D, color: Color, x: f32, y: f32 },
    Bishop { texture: Texture2D, color: Color, x: f32, y: f32 },
    Rook { texture: Texture2D, color: Color, x: f32, y: f32 },
    Queen { texture: Texture2D, color: Color, x: f32, y: f32 },
    King { texture: Texture2D, color: Color, x: f32, y: f32 },
}

impl GraphicsPiece {
    pub fn render(&self, window_parameters: &WindowParameters, parent_square: &Square) {
        let piece_info = match self {
            GraphicsPiece::Pawn { color, x, y, texture } => (color, x, y, "P", texture),
            GraphicsPiece::Knight { color, x, y, texture } => (color, x, y, "Kn", texture),
            GraphicsPiece::Bishop { color, x, y, texture } => (color, x, y, "B", texture),
            GraphicsPiece::Rook { color, x, y, texture } => (color, x, y, "R", texture),
            GraphicsPiece::Queen { color, x, y, texture } => (color, x, y, "Q", texture),
            GraphicsPiece::King { color, x, y, texture } => (color, x, y, "K", texture),
        };

        window_parameters.render_rectangle(*piece_info.1 - parent_square.width / 4.0, *piece_info.2 - parent_square.height / 4.0, parent_square.width / 2.0, parent_square.height / 2.0, GRAY);
        window_parameters.render_text(piece_info.3, *piece_info.1, *piece_info.2, 20.0, *piece_info.0);
        window_parameters.render_texture(*piece_info.1 - parent_square.width / 4.0, *piece_info.2 - parent_square.height / 4.0, parent_square.width / 2.0, parent_square.height / 2.0, piece_info.4);
        
    }

    fn from_chess_piece(piece: Option<Piece>, square_x: f32, square_y: f32, square_width: f32, square_height: f32, texture: Texture2D) -> Option<GraphicsPiece> {
        piece.map(|piece| {
            let color = match piece.get_color() {
                chess::Color::Black => BLACK,
                chess::Color::White => WHITE,
            };
            let (x, y) = (square_x + square_width / 2.0, square_y + square_height / 2.0);
            match piece {
                Piece::Pawn { .. } => GraphicsPiece::Pawn { texture, color, x, y },
                Piece::Knight { .. } => GraphicsPiece::Knight { texture: Texture2D::empty(), color, x, y },
                Piece::Bishop { .. } => GraphicsPiece::Bishop { texture: Texture2D::empty(), color, x, y },
                Piece::Rook { .. } => GraphicsPiece::Rook { texture: Texture2D::empty(), color, x, y },
                Piece::Queen { .. } => GraphicsPiece::Queen { texture: Texture2D::empty(), color, x, y },
                Piece::King { .. } => GraphicsPiece::King { texture: Texture2D::empty(), color, x, y },
            }
        })
    }

    pub fn get_color(&self) -> Color {
        match self {
            GraphicsPiece::Pawn { color, .. } => *color,
            GraphicsPiece::Knight { color, .. } => *color,
            GraphicsPiece::Bishop { color, .. } => *color,
            GraphicsPiece::Rook { color, .. } => *color,
            GraphicsPiece::Queen { color, .. } => *color,
            GraphicsPiece::King { color, .. } => *color,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Square {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    graphics_piece: Option<GraphicsPiece>,
}

#[derive(Clone)]
pub struct UIChessBoard {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub squares: HashMap<(usize, usize), Square>,
    pub held_piece: Option<(usize, usize)>,
    pub play_as: ChessColor,
    promotion: Option<PromotionPiecesPopup>,
    pub game_status: GameStatus,
    pub textures: HashMap<PieceType, Texture2D>,
}

#[derive(Clone, Debug)]
struct PromotionPiecesPopup {
    pub squares: Vec<Square>,
    pub from: (usize, usize),
    pub to: (usize, usize),
    is_hoverd: (bool, usize),
}

pub async fn load_piece_textures() -> HashMap<PieceType, Texture2D> {
    let mut textures: HashMap<PieceType, Texture2D> = HashMap::new();
    let white_pawn = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();
    let white_knight = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();
    let white_bishop = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();
    let white_queen = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();
    let white_king = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();

    let black_pawn = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();
    let black_knight = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();
    let black_bishop = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();
    let black_queen = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();
    let black_king = load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap();
    textures.insert(PieceType::Pawn(ChessColor::White), load_texture_from_bytes(include_bytes!("../../res/pawn.png")).await.unwrap());
    textures
}

impl UIChessBoard {
    pub fn new(x: f32, y: f32, size: f32, chess_position: &[[Option<Piece>; 8]; 8], aspect_ratio: &f32, play_as: ChessColor, textures: HashMap<PieceType, Texture2D>) -> Self {
        let width = size;
        let height = size * aspect_ratio;

        let mut squares: HashMap<(usize, usize), Square> = HashMap::new();

        let grid_width_x = 8;
        let grid_width_y = 8;

        let square_width = width / grid_width_x as f32;
        let square_height = height / grid_width_y as f32;

        let mut color = DARKBROWN;

        for i in 0..grid_width_x {
            for j in 0..grid_width_y {
                let square_x;
                let square_y;
                match play_as {
                    ChessColor::Black => {
                        square_x = width - i as f32 * square_width + -square_width + x;
                        square_y = j as f32 * square_height + y;
                    }
                    ChessColor::White => {
                        square_x = i as f32 * square_width + x;
                        square_y = height - j as f32 * square_height - square_height + y;
                    }
                }

                let graphics_piece = GraphicsPiece::from_chess_piece(chess_position[i][j], square_x, square_y, square_width, square_height, textures.get(&0).unwrap().clone());
                squares.insert((i, j), Square { x: square_x, y: square_y, width: square_width, height: square_height, color, graphics_piece });
                color = if color == DARKBROWN { WHITE } else { DARKBROWN };
            }
            if grid_width_y % 2 == 0 {
                color = if color == DARKBROWN { WHITE } else { DARKBROWN };
            }
        }
        UIChessBoard { x, y, width, height, squares, held_piece: None, play_as, promotion: None, game_status: GameStatus::Ongoing, textures }
    }

    pub fn update(&mut self, chess_position: &[[Option<Piece>; 8]; 8]) {
        for i in 0..8 {
            for j in 0..8 {
                let square = self.squares.get_mut(&(i, j)).unwrap();
                let current_graphics_piece = &square.graphics_piece;
                let new_piece = &chess_position[i][j];
                let new_graphics_piece = GraphicsPiece::from_chess_piece(new_piece.clone(), square.x, square.y, square.width, square.height, self.textures.get(&0).unwrap().clone());

                if current_graphics_piece != &new_graphics_piece {
                    square.graphics_piece = new_graphics_piece;
                }
            }
        }
    }

    pub fn reset_board(&mut self, chess_position: &[[Option<Piece>; 8]; 8]) {
        *self = UIChessBoard::new(self.x, self.y, self.width, chess_position, &(16.0 / 9.0), self.play_as, self.textures.clone());
    }

    pub fn flip(&mut self, chess_position: &[[Option<Piece>; 8]; 8]) {
        match self.play_as {
            ChessColor::White => {
                *self = UIChessBoard::new(self.x, self.y, self.width, chess_position, &(16.0 / 9.0), ChessColor::Black, self.textures.clone());
            }
            ChessColor::Black => {
                *self = UIChessBoard::new(self.x, self.y, self.width, chess_position, &(16.0 / 9.0), ChessColor::White, self.textures.clone());
            }
        }
    }

    pub fn render(&mut self, window_parameters: &WindowParameters) {
        for ((_i, _j), square) in &self.squares {
            window_parameters.render_rectangle(square.x, square.y, square.width, square.height, square.color);
        }

        for ((_i, _j), square) in &self.squares {
            if let Some(ref piece) = square.graphics_piece {
                piece.render(window_parameters, square);
            }
        }
        if let Some(promotion) = self.promotion.clone() {
            window_parameters.render_rectangle(self.x, self.y, self.width, self.height, Color::new(0.0, 0.0, 0.0, 0.6));
            let mut i = 0;
            for square in promotion.squares.iter() {
                if promotion.is_hoverd.0 && i == promotion.is_hoverd.1 {
                    window_parameters.render_rectangle(square.x, square.y, square.width, square.height, Color::new(0.6, 0.3, 0.1, 1.0));
                } else {
                    window_parameters.render_rectangle(square.x, square.y, square.width, square.height, square.color);
                }

                i += 1;
            }
            for square in promotion.squares {
                if let Some(ref piece) = square.graphics_piece {
                    piece.render(window_parameters, &square);
                }
            }
        }

        window_parameters.render_rectangle_line(self.x, self.y, self.width, self.height, 0.003, GRAY);

        match self.held_piece {
            Some(held_piece) => match self.squares.get(&held_piece) {
                Some(square) => {
                    if let Some(ref piece) = square.graphics_piece {
                        piece.render(window_parameters, square);
                    }
                }
                None => (),
            },
            None => (),
        }
    }

    fn create_promotion_pieces_popup(&self, promotion_x: usize, to_move_is_white: bool, from: (usize, usize), to: (usize, usize)) -> PromotionPiecesPopup {
        let color_to_move = if to_move_is_white { ChessColor::White } else { ChessColor::Black };
        let (increment, mut square_y) = if to_move_is_white { (-1, 7) } else { (1, 0) };
        let mut squares = Vec::new();
        for i in 0..4 {
            let piece = match i {
                0 => Piece::Queen { color: color_to_move },
                1 => Piece::Knight { color: color_to_move },
                2 => Piece::Rook { color: color_to_move, has_moved: true },
                _ => Piece::Bishop { color: color_to_move },
            };
            let mut promotion_square = self.squares.get(&(promotion_x, square_y as usize)).unwrap().clone();
            promotion_square.color = Color::new(0.2, 0.2, 0.2, 1.0);
            promotion_square.graphics_piece = GraphicsPiece::from_chess_piece(Some(piece), promotion_square.x, promotion_square.y, promotion_square.width, promotion_square.height, self.textures.get(&0).unwrap().clone());
            squares.push(promotion_square);

            square_y += increment;
        }

        PromotionPiecesPopup { squares, from, to, is_hoverd: (false, 0) }
    }

    pub fn check_result(&mut self, result: Result<GameStatus, MoveError>) {
        match result {
            Ok(game_status) => {
                self.game_status = game_status;
            }
            Err(_) => (),
        }
    }

    fn handle_promotion_click(&mut self, window_parameters: &WindowParameters) -> Option<PromotionPiece> {
        let mut promotion_squares = self.promotion.clone().unwrap().squares;
        let (mouse_x, mouse_y) = window_parameters.mouse_position();
        let mut i = 4;
        for square in promotion_squares.iter_mut() {
            if is_mouse_button_pressed(MouseButton::Left) {
                let is_hovered = mouse_x >= square.x && mouse_x <= square.x + square.width && mouse_y >= square.y && mouse_y <= square.y + square.height;
                if is_hovered {
                    if let Some(ref piece) = square.graphics_piece {
                        let piece = match piece {
                            GraphicsPiece::Queen { .. } => Some(PromotionPiece::Queen),
                            GraphicsPiece::Knight { .. } => Some(PromotionPiece::Knight),
                            GraphicsPiece::Rook { .. } => Some(PromotionPiece::Rook),
                            GraphicsPiece::Bishop { .. } => Some(PromotionPiece::Bishop),
                            _ => None,
                        };
                        self.promotion = None;
                        return piece;
                    }
                } else {
                    i -= 1;
                }
                if i == 0 {
                    self.promotion = None;
                    return None;
                }
            }
        }

        None
    }

    pub fn request_move(&mut self, window_parameters: &WindowParameters) -> (Option<((usize, usize), (usize, usize))>, Option<PromotionPiece>) {
        if let Some(promotion) = self.promotion.clone() {
            if let Some(piece_to_promote_to) = self.handle_promotion_click(window_parameters) {
                return (Some((promotion.from, promotion.to)), Some(piece_to_promote_to));
            }
            if self.promotion.is_none() {
                return (Some((promotion.from, promotion.from)), None);
            }
        }
        let (mouse_x, mouse_y) = window_parameters.mouse_position();
        if let Some((i, j)) = self.held_piece {
            if let Some(_) = self.squares.get_mut(&(i, j)) {
                if !is_mouse_button_down(MouseButton::Left) {
                    self.held_piece = None;
                    let new_square = self.squares.iter().find_map(|((new_i, new_j), new_square)| {
                        let is_hovered = mouse_x >= new_square.x && mouse_x <= new_square.x + new_square.width && mouse_y >= new_square.y && mouse_y <= new_square.y + new_square.height;
                        if mouse_x > self.x + self.width || mouse_x < self.x || mouse_y > self.y + self.height || mouse_y < self.y {
                            return Some((i, j));
                        }
                        if is_hovered {
                            Some((*new_i, *new_j))
                        } else {
                            None
                        }
                    });

                    if let Some(new_square) = new_square {
                        if let Some(ref piece) = self.squares.get(&(i, j)).unwrap().graphics_piece {
                            if let GraphicsPiece::Pawn { .. } = piece {
                                if piece.get_color() == WHITE {
                                    if new_square.1 == 7 {
                                        let is_square_occupied = self.squares.get(&new_square).unwrap().graphics_piece.is_some();
                                        let is_capture_move = (new_square.0 as i32 - i as i32).abs() == 1;

                                        if !is_square_occupied || is_capture_move {
                                            self.promotion = Some(self.create_promotion_pieces_popup(new_square.0, true, (i, j), new_square));
                                            return (None, None);
                                        }
                                    }
                                } else {
                                    if new_square.1 == 0 {
                                        let is_square_occupied = self.squares.get(&new_square).unwrap().graphics_piece.is_some();
                                        let is_capture_move = (new_square.0 as i32 - i as i32).abs() == 1;

                                        if !is_square_occupied || is_capture_move {
                                            self.promotion = Some(self.create_promotion_pieces_popup(new_square.0, false, (i, j), new_square));
                                            return (None, None);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    return (new_square.map(|new_square| ((i, j), new_square)), None);
                }
            }
        }
        (None, None)
    }

    pub fn update_assume_logic(&mut self, window_parameters: &WindowParameters) {
        if self.promotion.is_some() {
            let promotion_squares = self.promotion.clone().unwrap().squares;
            let (mouse_x, mouse_y) = window_parameters.mouse_position();
            for i in 0..4 {
                let square = promotion_squares.get(i).unwrap();
                let is_hovered = mouse_x >= square.x && mouse_x <= square.x + square.width && mouse_y >= square.y && mouse_y <= square.y + square.height;
                if is_hovered {
                    self.promotion.as_mut().unwrap().is_hoverd = (true, i);
                }
            }
            return;
        }
        let (mouse_x, mouse_y) = window_parameters.mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            for ((i, j), square) in &self.squares {
                let is_hovered = mouse_x >= square.x && mouse_x <= square.x + square.width && mouse_y >= square.y && mouse_y <= square.y + square.height;

                if is_hovered {
                    self.held_piece = Some((*i, *j));
                    break;
                }
            }
        }

        if let Some((i, j)) = self.held_piece {
            if let Some(square) = self.squares.get_mut(&(i, j)) {
                if is_mouse_button_down(MouseButton::Left) {
                    if let Some(ref mut piece) = square.graphics_piece {
                        match piece {
                            GraphicsPiece::Pawn { x, y, .. } | GraphicsPiece::Knight { x, y, .. } | GraphicsPiece::Bishop { x, y, .. } | GraphicsPiece::Rook { x, y, .. } | GraphicsPiece::Queen { x, y, .. } | GraphicsPiece::King { x, y, .. } => {
                                *x = mouse_x;
                                *y = mouse_y;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update_as_free_movement(&mut self, window_parameters: &WindowParameters, chess_position: &mut ChessBoard) {
        let (mouse_x, mouse_y) = window_parameters.mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            for ((i, j), square) in &self.squares {
                let is_hovered = mouse_x >= square.x && mouse_x <= square.x + square.width && mouse_y >= square.y && mouse_y <= square.y + square.height;

                if is_hovered {
                    self.held_piece = Some((*i, *j));
                    break;
                }
            }
        }

        if let Some((i, j)) = self.held_piece {
            if let Some(square) = self.squares.get_mut(&(i, j)) {
                if is_mouse_button_down(MouseButton::Left) {
                    if let Some(ref mut piece) = square.graphics_piece {
                        match piece {
                            GraphicsPiece::Pawn { x, y, .. } | GraphicsPiece::Knight { x, y, .. } | GraphicsPiece::Bishop { x, y, .. } | GraphicsPiece::Rook { x, y, .. } | GraphicsPiece::Queen { x, y, .. } | GraphicsPiece::King { x, y, .. } => {
                                *x = mouse_x;
                                *y = mouse_y;
                            }
                        }
                    }
                } else {
                    let new_square_pos = self
                        .squares
                        .iter()
                        .find_map(|((new_i, new_j), new_square)| {
                            let is_hovered = mouse_x >= new_square.x && mouse_x <= new_square.x + new_square.width && mouse_y >= new_square.y && mouse_y <= new_square.y + new_square.height;
                            if is_hovered {
                                match chess_position.move_piece((i, j), (*new_i, *new_j), None) {
                                    Ok(_) => Some((*new_i, *new_j)),
                                    Err(_) => None,
                                }
                            } else {
                                None
                            }
                        })
                        .or(Some((i, j)));

                    if let Some((new_i, new_j)) = new_square_pos {
                        if let Some(mut piece) = self.squares.get_mut(&(i, j)).and_then(|square| square.graphics_piece.take()) {
                            if let Some(new_square) = self.squares.get_mut(&(new_i, new_j)) {
                                let new_x = new_square.x + new_square.width / 2.0;
                                let new_y = new_square.y + new_square.height / 2.0;

                                match &mut piece {
                                    GraphicsPiece::Pawn { x, y, .. } | GraphicsPiece::Knight { x, y, .. } | GraphicsPiece::Bishop { x, y, .. } | GraphicsPiece::Rook { x, y, .. } | GraphicsPiece::Queen { x, y, .. } | GraphicsPiece::King { x, y, .. } => {
                                        *x = new_x;
                                        *y = new_y;
                                    }
                                }

                                new_square.graphics_piece = Some(piece);
                            }
                        }
                    }

                    self.held_piece = None;
                }
            }
        }
    }
}
