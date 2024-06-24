pub mod chess;
pub mod draw;
pub mod ui;
use draw::WindowParameters;
use macroquad::prelude::*;
use ui::{Button, ChessBoard, Title, UIManager};

fn window_conf() -> Conf {
    Conf { window_title: "Rusty Chess".to_owned(), window_width: 1600, window_height: 900, window_resizable: true, ..Default::default() }
}

pub enum GameState {
    Menu,
    AgainstYourself,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut fullscreen = false;
    let texture: Texture2D = load_texture("background.png").await.unwrap();
    let mut game_state = GameState::Menu;

    let mut against_yourself = UIManager::new();

    let mut board = chess::board::Board::starting_positions();

    against_yourself.add_chess_board("chessBoard", ChessBoard::new(0.05,0.05, 0.5, board.squares).await);

    let mut main_menu = UIManager::new();
    main_menu.add_title("Main Title", Title::new_center_width("Rusty Chess", 200.0, 0.1, BLACK));
    main_menu.add_button("Against yourself", Button::new_center_width(0.2, 0.5, 0.15, "Against yourself", BLUE, LIGHTGRAY));
    main_menu.add_button("Against bot", Button::new_center_width(0.5, 0.5, 0.15, "Against bot", BLUE, LIGHTGRAY));
    main_menu.add_button("Online", Button::new_center_width(0.8, 0.5, 0.15, "Online", BLUE, LIGHTGRAY));
    loop {
        let window_parameters = WindowParameters::new(16.0 / 9.0);
        window_parameters.clear(WHITE);
        draw_texture_ex(&texture, window_parameters.x_offset, window_parameters.y_offset, WHITE, DrawTextureParams{ dest_size: Some(vec2(window_parameters.width, window_parameters.height)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None });
        match game_state {
            GameState::Menu => {
                main_menu.update(&window_parameters);
                main_menu.render(&window_parameters);

                if main_menu.was_button_clicked("Against yourself") {
                    game_state = GameState::AgainstYourself;
                }
                if main_menu.was_button_clicked("Against bot") {
                    println!("Against bot button clicked!");
                }
                if main_menu.was_button_clicked("Online") {
                    println!("Online button clicked!");
                }
            }
            GameState::AgainstYourself => {
                against_yourself.render(&window_parameters);
                against_yourself.update(&window_parameters);
            }
        }
        if is_key_pressed(KeyCode::F){
            set_fullscreen(!fullscreen);
            fullscreen = !fullscreen;
        }

        

        window_parameters.clear_outside(BLACK);
        next_frame().await
    }
}
