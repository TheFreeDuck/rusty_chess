use crate::{
    chess::{self, ChessBoard, Color as ChessColor, Coordinate},
    draw::WindowParameters,
};
use chess::piece::Piece;
use macroquad::{
    color::{Color, BLACK, GRAY, WHITE},
    input::{is_mouse_button_down, is_mouse_button_pressed, MouseButton},
    texture::Texture2D,
};
use std::{collections::HashMap, usize};

#[derive(Clone)]
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
            GraphicsPiece::Pawn { color, x, y, .. } => (color, x, y, "P"),
            GraphicsPiece::Knight { color, x, y, .. } => (color, x, y, "Kn"),
            GraphicsPiece::Bishop { color, x, y, .. } => (color, x, y, "B"),
            GraphicsPiece::Rook { color, x, y, .. } => (color, x, y, "R"),
            GraphicsPiece::Queen { color, x, y, .. } => (color, x, y, "Q"),
            GraphicsPiece::King { color, x, y, .. } => (color, x, y, "K"),
        };

        window_parameters.render_rectangle(*piece_info.1 - parent_square.width / 4.0, *piece_info.2 - parent_square.height / 4.0, parent_square.width / 2.0, parent_square.height / 2.0, GRAY);

        window_parameters.render_text(piece_info.3, *piece_info.1, *piece_info.2, 20.0, *piece_info.0);
    }

    fn create_graphics_piece(piece: Option<Piece>, texture: &Texture2D, square_x: f32, square_y: f32, square_width: f32, square_height: f32) -> Option<GraphicsPiece> {
        piece.map(|piece| {
            let color = match piece.get_color() {
                chess::Color::Black => BLACK,
                chess::Color::White => WHITE,
            };
            let (x, y) = (square_x + square_width / 2.0, square_y + square_height / 2.0);
            match piece {
                Piece::Pawn { .. } => GraphicsPiece::Pawn { texture: texture.clone(), color, x, y },
                Piece::Knight { .. } => GraphicsPiece::Knight { texture: texture.clone(), color, x, y },
                Piece::Bishop { .. } => GraphicsPiece::Bishop { texture: texture.clone(), color, x, y },
                Piece::Rook { .. } => GraphicsPiece::Rook { texture: texture.clone(), color, x, y },
                Piece::Queen { .. } => GraphicsPiece::Queen { texture: texture.clone(), color, x, y },
                Piece::King { .. } => GraphicsPiece::King { texture: texture.clone(), color, x, y },
            }
        })
    }
}

#[derive(Clone)]
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
}

impl UIChessBoard {
    pub fn new_square_board(x: f32, y: f32, size: f32, position: &[[Option<Piece>; 8]; 8], aspect_ratio: &f32, play_as: ChessColor, texture: &Texture2D) -> Self {
        let width = size;
        let height = size * aspect_ratio;

        let mut squares: HashMap<(usize, usize), Square> = HashMap::new();

        let grid_width_x = 8;
        let grid_width_y = 8;

        let square_width = width / grid_width_x as f32;
        let square_height = height / grid_width_y as f32;

        let mut color = BLACK;

        for i in 0..grid_width_x {
            for j in 0..grid_width_y {
                let square_x;
                let square_y;
                match play_as {
                    ChessColor::Black => {
                        square_x = width - i as f32 * square_width + - square_width + x;
                        square_y = j as f32 * square_height + y;
                    }
                    ChessColor::White => {
                        square_x = i as f32 * square_width + x;
                        square_y = height - j as f32 * square_height - square_height + y;
                    }
                }

                let graphics_piece = GraphicsPiece::create_graphics_piece(position[i][j], texture, square_x, square_y, square_width, square_height);
                squares.insert((i, j), Square { x: square_x, y: square_y, width: square_width, height: square_height, color, graphics_piece });
                color = if color == BLACK { WHITE } else { BLACK };
            }
            if grid_width_y % 2 == 0 {
                color = if color == BLACK { WHITE } else { BLACK };
            }
        }
        UIChessBoard { x, y, width, height, squares, held_piece: None, play_as }
    }

    /*pub fn update(&mut self, position: &[[Option<Piece>; 8]; 8], play_as: ChessColor){

        let mut squares: HashMap<(usize, usize), Square> = HashMap::new();

        let grid_width_x = 8;
        let grid_width_y = 8;

        let square_width = self.width / grid_width_x as f32;
        let square_height = self.height / grid_width_y as f32;

        let mut color = BLACK;

        for i in 0..grid_width_x {
            for j in 0..grid_width_y {
                let square_x;
                let square_y;
                match play_as {
                    ChessColor::Black => {
                        square_x = self.width - i as f32 * square_width + - square_width + x;
                        square_y = j as f32 * square_height + y;
                    }
                    ChessColor::White => {
                        square_x = i as f32 * square_width + x;
                        square_y = self.height - j as f32 * square_height - square_height + y;
                    }
                }

                let graphics_piece = GraphicsPiece::create_graphics_piece(position[i][j], texture, square_x, square_y, square_width, square_height);
                squares.insert((i, j), Square { x: square_x, y: square_y, width: square_width, height: square_height, color, graphics_piece });
                color = if color == BLACK { WHITE } else { BLACK };
            }
            if grid_width_y % 2 == 0 {
                color = if color == BLACK { WHITE } else { BLACK };
            }
        }
    }*/

    pub fn render(&mut self, window_parameters: &WindowParameters) {
        for ((_i, _j), square) in &self.squares {
            window_parameters.render_rectangle(square.x, square.y, square.width, square.height, square.color);
        }

        for ((_i, _j), square) in &self.squares {
            if let Some(ref piece) = square.graphics_piece {
                piece.render(window_parameters, square);
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

    pub fn request_move(&mut self, window_parameters: &WindowParameters) -> Option<((usize, usize), (usize, usize))> {
        let (mouse_x, mouse_y) = window_parameters.mouse_position();
        if let Some((i, j)) = self.held_piece {
            if let Some(_) = self.squares.get_mut(&(i, j)) {
                if !is_mouse_button_down(MouseButton::Left) {
                    let new_square = self.squares.iter().find_map(|((new_i, new_j), new_square)| {
                        let is_hovered = mouse_x >= new_square.x && mouse_x <= new_square.x + new_square.width && mouse_y >= new_square.y && mouse_y <= new_square.y + new_square.height;
                        if is_hovered {
                            Some((*new_i, *new_j))
                        } else {
                            None
                        }
                    });

                    return new_square.map(|new_square| ((i, j), new_square));
                }
            }
        }
        None
    }

    pub fn update_assume_logic(&mut self, window_parameters: &WindowParameters) {
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
                                match chess_position.move_piece(Coordinate::new(i as i32, j as i32), Coordinate::new(*new_i as i32, *new_j as i32)) {
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
