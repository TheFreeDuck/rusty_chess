use crate::{
    chess::{
        self,
        chess_board::{GameStatus, MoveError},
        piece::PromotionPiece, Color as ChessColor,
    },
    draw::WindowParameters,
};
use chess::piece::Piece;
use macroquad::{
    color::{Color, BROWN, GRAY},
    input::{is_mouse_button_down, is_mouse_button_pressed, MouseButton},
    texture::Texture2D,
};
use std::{collections::HashMap, usize};

use super::draw::load_texture_from_bytes;

const WHITE_SQUARE_COLOR: Color = Color::new(0.860, 0.767, 0.64, 1.0);
const BLACK_SQUARE_COLOR: Color = BROWN;

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
pub struct GraphicsPiece {
    piece_type: PieceType,
    x: f32,
    y: f32,
    texture: Texture2D,
}

impl GraphicsPiece {
    fn from_chess_piece(piece: Option<Piece>, square_x: f32, square_y: f32, square_width: f32, square_height: f32, textures: HashMap<PieceType,Texture2D>) -> Option<GraphicsPiece> {
        if let Some(piece) = piece {
            let piece_type = match piece {
                Piece::Pawn { color, .. } => PieceType::Pawn(color),
                Piece::Knight { color, .. } => PieceType::Knight(color),
                Piece::Bishop { color, .. } => PieceType::Bishop(color),
                Piece::Rook { color, .. } => PieceType::Rook(color),
                Piece::Queen { color, .. } => PieceType::Queen(color),
                Piece::King { color, .. } => PieceType::King(color),
            };
            match textures.get(&piece_type){
                Some(texture) => return Some(GraphicsPiece { piece_type, x: square_x + square_width/2.0, y: square_y + square_height/2.0, texture: texture.to_owned() }),
                None => return None,
            }
        }
        None
    }

    pub fn render(&self, window_parameters: &WindowParameters, parent_square: &Square) {
        window_parameters.render_texture(self.x - parent_square.width/2.0, self.y - parent_square.height/2.0, parent_square.width, parent_square.height, &self.texture);
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
    window_aspect_ratio: f32,
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
    let white_pawn = load_texture_from_bytes(include_bytes!("../../res/white_pawn.png")).await.unwrap();
    let white_knight = load_texture_from_bytes(include_bytes!("../../res/white_knight.png")).await.unwrap();
    let white_bishop = load_texture_from_bytes(include_bytes!("../../res/white_bishop.png")).await.unwrap();
    let white_rook = load_texture_from_bytes(include_bytes!("../../res/white_rook.png")).await.unwrap();
    let white_queen = load_texture_from_bytes(include_bytes!("../../res/white_queen.png")).await.unwrap();
    let white_king = load_texture_from_bytes(include_bytes!("../../res/white_king.png")).await.unwrap();

    let black_pawn = load_texture_from_bytes(include_bytes!("../../res/black_pawn.png")).await.unwrap();
    let black_knight = load_texture_from_bytes(include_bytes!("../../res/black_knight.png")).await.unwrap();
    let black_bishop = load_texture_from_bytes(include_bytes!("../../res/black_bishop.png")).await.unwrap();
    let black_rook = load_texture_from_bytes(include_bytes!("../../res/black_rook.png")).await.unwrap();
    let black_queen = load_texture_from_bytes(include_bytes!("../../res/black_queen.png")).await.unwrap();
    let black_king = load_texture_from_bytes(include_bytes!("../../res/black_king.png")).await.unwrap();
    textures.insert(PieceType::Pawn(ChessColor::White), white_pawn);
    textures.insert(PieceType::Knight(ChessColor::White), white_knight);
    textures.insert(PieceType::Bishop(ChessColor::White), white_bishop);
    textures.insert(PieceType::Rook(ChessColor::White), white_rook);
    textures.insert(PieceType::Queen(ChessColor::White), white_queen);
    textures.insert(PieceType::King(ChessColor::White), white_king);

    textures.insert(PieceType::Pawn(ChessColor::Black), black_pawn);
    textures.insert(PieceType::Knight(ChessColor::Black), black_knight);
    textures.insert(PieceType::Bishop(ChessColor::Black), black_bishop);
    textures.insert(PieceType::Rook(ChessColor::Black), black_rook);
    textures.insert(PieceType::Queen(ChessColor::Black), black_queen);
    textures.insert(PieceType::King(ChessColor::Black), black_king);

    textures
}

impl UIChessBoard {
    pub fn new(x: f32, y: f32, size: f32, chess_position: &[[Option<Piece>; 8]; 8], window_aspect_ratio: &f32, play_as: ChessColor, textures: HashMap<PieceType, Texture2D>) -> Self {
        let width = size;
        let height = size * window_aspect_ratio;

        let mut squares: HashMap<(usize, usize), Square> = HashMap::new();

        let grid_width_x = 8;
        let grid_width_y = 8;

        let square_width = width / grid_width_x as f32;
        let square_height = height / grid_width_y as f32;



        let mut is_square_white = true;
        let mut color = BLACK_SQUARE_COLOR;

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
                let graphics_piece = GraphicsPiece::from_chess_piece(chess_position[i][j], square_x, square_y, square_width, square_height, textures.clone());
                squares.insert((i, j), Square { x: square_x, y: square_y, width: square_width, height: square_height, color, graphics_piece });
                color = if is_square_white { WHITE_SQUARE_COLOR } else { BLACK_SQUARE_COLOR };
                is_square_white = !is_square_white;
            }
            if grid_width_y % 2 == 0 {
                color = if is_square_white { WHITE_SQUARE_COLOR } else { BLACK_SQUARE_COLOR };
                is_square_white = !is_square_white;
            }
        }
        UIChessBoard { x, y, width, height, squares, held_piece: None, play_as, promotion: None, game_status: GameStatus::Ongoing, textures, window_aspect_ratio: window_aspect_ratio.to_owned() }
    }

    pub fn update(&mut self, chess_position: &[[Option<Piece>; 8]; 8]) {
        for i in 0..8 {
            for j in 0..8 {
                let square = self.squares.get_mut(&(i, j)).unwrap();
                let current_graphics_piece = &square.graphics_piece;
                let new_piece = &chess_position[i][j];
                let new_graphics_piece = GraphicsPiece::from_chess_piece(new_piece.clone(), square.x, square.y, square.width, square.height, self.textures.clone());

                if current_graphics_piece != &new_graphics_piece {
                    square.graphics_piece = new_graphics_piece;
                }
            }
        }
    }

    pub fn reset_board(&mut self, chess_position: &[[Option<Piece>; 8]; 8]) {
        *self = UIChessBoard::new(self.x, self.y, self.width, chess_position, &self.window_aspect_ratio, self.play_as, self.textures.clone());
    }

    pub fn flip(&mut self, chess_position: &[[Option<Piece>; 8]; 8]) {
        match self.play_as {
            ChessColor::White => {
                *self = UIChessBoard::new(self.x, self.y, self.width, chess_position, &self.window_aspect_ratio, ChessColor::Black, self.textures.clone());
            }
            ChessColor::Black => {
                *self = UIChessBoard::new(self.x, self.y, self.width, chess_position, &self.window_aspect_ratio, ChessColor::White, self.textures.clone());
            }
        }
    }

    pub fn render(&mut self, window_parameters: &WindowParameters) {
        window_parameters.render_rectangle_line(self.x - 0.001, self.y - 0.002, self.width + 0.002, self.height + 0.004, 0.003, GRAY);
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
            promotion_square.graphics_piece = GraphicsPiece::from_chess_piece(Some(piece), promotion_square.x, promotion_square.y, promotion_square.width, promotion_square.height, self.textures.clone());
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
                        let piece = match piece.piece_type {
                            PieceType::Queen { .. } => Some(PromotionPiece::Queen),
                            PieceType::Knight { .. } => Some(PromotionPiece::Knight),
                            PieceType::Rook { .. } => Some(PromotionPiece::Rook),
                            PieceType::Bishop { .. } => Some(PromotionPiece::Bishop),
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
                            if let PieceType::Pawn(color) = piece.piece_type {
                                if color == ChessColor::White {
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
                        piece.x = mouse_x;
                        piece.y = mouse_y;
                    }
                }
            }
        }
    }
}
