use crate::{chess, draw::WindowParameters};
use chess::piece::Piece;
use macroquad::{
    color::{Color, BLACK, GRAY, ORANGE, RED, WHITE},
    input::{is_mouse_button_down, is_mouse_button_pressed, mouse_position, MouseButton},
    texture::{load_texture, Texture2D},
};
use std::{collections::HashMap, usize};

#[derive(Clone)]
pub enum GraphicsPiece {
    Pawn {
        texture: Texture2D,
        color: Color,
        x: f32,
        y: f32,
    },
    Knight {
        texture: Texture2D,
        color: Color,
        x: f32,
        y: f32,
    },
    Bishop {
        texture: Texture2D,
        color: Color,
        x: f32,
        y: f32,
    },
    Rook {
        texture: Texture2D,
        color: Color,
        x: f32,
        y: f32,
    },
    Queen {
        texture: Texture2D,
        color: Color,
        x: f32,
        y: f32,
    },
    King {
        texture: Texture2D,
        color: Color,
        x: f32,
        y: f32,
    },
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
pub struct ChessBoard {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub squares: HashMap<(usize, usize), Square>,
}

impl ChessBoard {
    pub async fn new(x: f32, y: f32, width: f32, position: [[Option<Piece>; 8]; 8]) -> Self {
        let texture: Texture2D = load_texture("background.png").await.unwrap();
        let mut squares: HashMap<(usize, usize), Square> = HashMap::new();

        let grid_width_x = 8;
        let grid_width_y = 8;

        let square_width = width / grid_width_x as f32;
        let square_height = (width * 16.0 / 9.0) / grid_width_y as f32;

        let mut color = BLACK;
        for i in 0..grid_width_x {
            for j in 0..grid_width_y {
                let graphics_piece = match position[i][j] {
                    Some(Piece::Pawn { color, .. }) => Some(GraphicsPiece::Pawn {
                        texture: texture.clone(),
                        color: match color {
                            chess::Color::Black => BLACK,
                            chess::Color::White => WHITE,
                        },
                        x: (i as f32 * square_width + x) + square_width / 2.0,
                        y: (j as f32 * square_height + y) + square_height / 2.0,
                    }),
                    Some(Piece::Knight { color }) => Some(GraphicsPiece::Knight {
                        texture: texture.clone(),
                        color: match color {
                            chess::Color::Black => BLACK,
                            chess::Color::White => WHITE,
                        },
                        x: (i as f32 * square_width + x) + square_width / 2.0,
                        y: (j as f32 * square_height + y) + square_height / 2.0,
                    }),
                    Some(Piece::Bishop { color, .. }) => Some(GraphicsPiece::Bishop {
                        texture: texture.clone(),
                        color: match color {
                            chess::Color::Black => BLACK,
                            chess::Color::White => WHITE,
                        },
                        x: (i as f32 * square_width + x) + square_width / 2.0,
                        y: (j as f32 * square_height + y) + square_height / 2.0,
                    }),
                    Some(Piece::Rook { color, .. }) => Some(GraphicsPiece::Rook {
                        texture: texture.clone(),
                        color: match color {
                            chess::Color::Black => BLACK,
                            chess::Color::White => WHITE,
                        },
                        x: (i as f32 * square_width + x) + square_width / 2.0,
                        y: (j as f32 * square_height + y) + square_height / 2.0,
                    }),
                    Some(Piece::Queen { color }) => Some(GraphicsPiece::Queen {
                        texture: texture.clone(),
                        color: match color {
                            chess::Color::Black => BLACK,
                            chess::Color::White => WHITE,
                        },
                        x: (i as f32 * square_width + x) + square_width / 2.0,
                        y: (j as f32 * square_height + y) + square_height / 2.0,
                    }),
                    Some(Piece::King { color, .. }) => Some(GraphicsPiece::King {
                        texture: texture.clone(),
                        color: match color {
                            chess::Color::Black => BLACK,
                            chess::Color::White => WHITE,
                        },
                        x: (i as f32 * square_width + x) + square_width / 2.0,
                        y: (j as f32 * square_height + y) + square_height / 2.0,
                    }),
                    None => None,
                };
                squares.insert(
                    (i, j),
                    Square {
                        x: i as f32 * square_width + x,
                        y: j as f32 * square_height + y,
                        width: square_width,
                        height: square_height,
                        color,
                        graphics_piece,
                    },
                );
                color = if color == BLACK { WHITE } else { BLACK };
            }
            if grid_width_y % 2 == 0 {
                color = if color == BLACK { WHITE } else { BLACK };
            }
        }
        ChessBoard {
            x,
            y,
            width,
            squares,
        }
    }

    pub fn render(&mut self, window_parameter: &WindowParameters) {
        for ((_i, _j), square) in &self.squares {
            window_parameter.render_rectangle(
                square.x,
                square.y,
                square.width,
                square.height,
                square.color,
            );
        }

        for ((_i, _j), square) in &self.squares {
            if let Some(ref piece) = square.graphics_piece {
                let piece_info = match piece {
                    GraphicsPiece::Pawn { color, x, y, .. } => (color, x, y, "P"),
                    GraphicsPiece::Knight { color, x, y, .. } => (color, x, y, "Kn"),
                    GraphicsPiece::Bishop { color, x, y, .. } => (color, x, y, "B"),
                    GraphicsPiece::Rook { color, x, y, .. } => (color, x, y, "R"),
                    GraphicsPiece::Queen { color, x, y, .. } => (color, x, y, "Q"),
                    GraphicsPiece::King { color, x, y, .. } => (color, x, y, "K"),
                };

                window_parameter.render_rectangle(
                    *piece_info.1 - square.width / 4.0,
                    *piece_info.2 - square.width / 2.0,
                    square.width / 2.0,
                    square.height / 2.0,
                    GRAY,
                );

                window_parameter.render_text(
                    piece_info.3,
                    *piece_info.1,
                    *piece_info.2,
                    20.0,
                    *piece_info.0,
                );
            }
        }
    }

    pub fn update(&mut self, window_parameters: &WindowParameters) {
        let mouse_x = mouse_position().0 / window_parameters.width;
        let mouse_y = mouse_position().1 / window_parameters.height;
        
        if let Some(square) = self.squares.get_mut(&(5, 0)) {
            if !is_mouse_button_down(MouseButton::Left) {
                return;
            }
            if let Some(ref mut piece) = square.graphics_piece {
                match piece {
                    GraphicsPiece::Pawn { x, y, .. }
                    | GraphicsPiece::Knight { x, y, .. }
                    | GraphicsPiece::Bishop { x, y, .. }
                    | GraphicsPiece::Rook { x, y, .. }
                    | GraphicsPiece::Queen { x, y, .. }
                    | GraphicsPiece::King { x, y, .. } => {
                        *x = mouse_x;
                        *y = mouse_y;
                    }
                }
            }
        }
    }
}

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub label: String,
    pub color: Color,
    pub hover_color: Color,
    pub is_hovered: bool,
    pub is_clicked: bool,
}

impl Button {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        label: &str,
        color: Color,
        hover_color: Color,
    ) -> Self {
        Button {
            x,
            y,
            width,
            height,
            label: label.to_string(),
            color,
            hover_color,
            is_hovered: false,
            is_clicked: false,
        }
    }

    pub fn new_center_width(
        y: f32,
        width: f32,
        height: f32,
        label: &str,
        color: Color,
        hover_color: Color,
    ) -> Self {
        Button {
            x: 0.5 - width / 2.0,
            y,
            width,
            height,
            label: label.to_string(),
            color,
            hover_color,
            is_hovered: false,
            is_clicked: false,
        }
    }

    pub fn render(&mut self, window_parameters: &WindowParameters) {
        let color = if self.is_hovered {
            self.hover_color
        } else {
            self.color
        };
        window_parameters.render_rectangle(self.x, self.y, self.width, self.height, color);

        //let text_middle = get_text_center(&self.label, None, (window_parameters.width / 20.0) as u16, 1.0, 0.0);

        window_parameters.render_text(
            &self.label,
            self.x + (self.width / 2.0) - 1.0 / window_parameters.width,
            self.y + (self.height / 2.0) + 1.0 / window_parameters.height,
            window_parameters.width / 20.0,
            BLACK,
        );
    }

    pub fn update(&mut self, window_parameters: &WindowParameters) {
        let (mouse_x, mouse_y) = mouse_position();
        let x = window_parameters.x_offset + self.x * window_parameters.width;
        let y = window_parameters.y_offset + self.y * window_parameters.height;
        let width = self.width * window_parameters.width;
        let height = self.height * window_parameters.height;

        self.is_hovered =
            mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
        self.is_clicked = self.is_hovered && is_mouse_button_pressed(MouseButton::Left);
    }
}

pub struct Title {
    text: String,
    size: f32,
    x: f32,
    y: f32,
    color: Color,
}

impl Title {
    pub fn new_center_width(text: &str, size: f32, y: f32, color: Color) -> Self {
        Title {
            text: text.to_string(),
            size,
            x: 0.5,
            y,
            color,
        }
    }

    pub fn render(&mut self, window_parameters: &WindowParameters) {
        //let text_size = measure_text(&self.text, None, (window_parameters.width / 20.0* self.size) as u16, 1.0);
        //window_parameters.render_text(&self.text, self.x - (text_size.width/2.0)/window_parameters.width, self.y, window_parameters.width / 20.0* self.size, self.color);
        window_parameters.render_text(&self.text, 0.1, 0.1, self.size, self.color);
    }
}

pub struct UIManager {
    buttons: HashMap<String, Button>,
    titles: HashMap<String, Title>,
    chess_boards: HashMap<String, ChessBoard>,
}

impl UIManager {
    pub fn new() -> Self {
        UIManager {
            buttons: HashMap::new(),
            titles: HashMap::new(),
            chess_boards: HashMap::new(),
        }
    }

    pub fn add_button(&mut self, id: &str, button: Button) {
        self.buttons.insert(id.to_string(), button);
    }

    pub fn add_title(&mut self, id: &str, title: Title) {
        self.titles.insert(id.to_string(), title);
    }

    pub fn add_chess_board(&mut self, id: &str, chess_board: ChessBoard) {
        self.chess_boards.insert(id.to_string(), chess_board);
    }

    pub fn render(&mut self, window_parameters: &WindowParameters) {
        for button in self.buttons.values_mut() {
            button.render(window_parameters);
        }
        for title in self.titles.values_mut() {
            title.render(window_parameters);
        }
        for chess_board in self.chess_boards.values_mut() {
            chess_board.render(window_parameters);
        }
    }

    pub fn update(&mut self, window_parameters: &WindowParameters) {
        for button in self.buttons.values_mut() {
            button.update(window_parameters);
        }

        for chess_board in self.chess_boards.values_mut() {
            chess_board.update(window_parameters);
        }
    }

    pub fn was_button_clicked(&self, id: &str) -> bool {
        if let Some(button) = self.buttons.get(id) {
            button.is_clicked
        } else {
            false
        }
    }
}
