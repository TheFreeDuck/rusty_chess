pub mod chess;
pub mod ui;
use chess::Coordinate;
use draw::WindowParameters;
use macroquad::prelude::*;
use ui::{draw, ui_chess_board::UIChessBoard, ui_manager};
use ui_manager::{Button, Title, UIManager};

fn window_conf() -> Conf {
    Conf { window_title: "Rusty Chess".to_owned(), window_width: 1600, window_height: 900, window_resizable: true, ..Default::default() }
}

pub enum GameState {
    Menu,
    AgainstYourself,
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture: Texture2D = load_texture("background.png").await.unwrap();
    let mut game_state = GameState::Menu;

    let mut against_yourself = UIManager::new();

    let mut board = chess::chess_board::ChessBoard::starting_positions();

    let mut window_parameters = WindowParameters::new((16.0, 9.0));

    let mut ui_chess_board = UIChessBoard::new_square_board(0.05, 0.05555555555, 0.5, &board.squares, &window_parameters.aspect_ratio_number, chess::Color::White,&texture);

    let mut main_menu = UIManager::new();
    main_menu.add_title("Main Title", Title::new_center_width("Rusty Chess", 70.0, 0.1, BLACK));
    main_menu.add_button("Against yourself", Button::new_center_width(0.2, 0.5, 0.15, "Against yourself", BLUE, LIGHTGRAY));
    main_menu.add_button("Against bot", Button::new_center_width(0.5, 0.5, 0.15, "Against bot", BLUE, LIGHTGRAY));
    main_menu.add_button("Online", Button::new_center_width(0.8, 0.5, 0.15, "Online", BLUE, LIGHTGRAY));
    loop {
        window_parameters.update();
        window_parameters.clear(WHITE);
        draw_texture_ex(&texture, window_parameters.x_offset, window_parameters.y_offset, WHITE, DrawTextureParams { dest_size: Some(vec2(window_parameters.width, window_parameters.height)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None });

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
                against_yourself.update(&window_parameters);
                ui_chess_board.update_assume_logic(&window_parameters);
                match ui_chess_board.request_move(&window_parameters){
                    Some(movement_proposal) => {
                        dbg!(board.move_piece(Coordinate::new(movement_proposal.0.0 as i32, movement_proposal.0.1 as i32), Coordinate::new(movement_proposal.1.0 as i32, movement_proposal.1.1 as i32)));
                        ui_chess_board = UIChessBoard::new_square_board(0.05, 0.05, 0.5, &board.squares, &window_parameters.aspect_ratio_number,chess::Color::White, &texture);
                        board.display_as_text();
                    },
                    None => (),
                }
                
                against_yourself.render(&window_parameters);
                ui_chess_board.render(&window_parameters);
                
            }
        }

        window_parameters.clear_outside(BLACK);
        next_frame().await
    }
}
