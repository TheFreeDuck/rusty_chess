//#![windows_subsystem = "windows"]

pub mod chess;
pub mod ui;
use std::process::exit;

use chess::ChessBoard;
use draw::WindowParameters;
use macroquad::prelude::*;
use ui::{draw, layouts, ui_chess_board::UIChessBoard, ui_manager};
use ui_manager::Title;

fn window_conf() -> Conf {
    Conf { window_title: "Rusty Chess".to_owned(), window_width: 1600, window_height: 900, icon: None, window_resizable: true, fullscreen: true, ..Default::default() }
}

pub enum GameState {
    Menu,
    AgainstYourself,
    AgainstBot,
    Online,
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture = draw::load_texture_from_bytes(include_bytes!("../res/background.png")).await.unwrap();

    let mut game_state = GameState::Menu;

    let mut board = chess::chess_board::ChessBoard::stalemate_start();

    let mut window_parameters = WindowParameters::new((16.0, 9.0));

    let textures = ui::ui_chess_board::load_piece_textures().await;

    let mut ui_chess_board = UIChessBoard::new(0.05, 0.05555555555, 0.5, &board.squares, &window_parameters.aspect_ratio_number, chess::Color::White, textures);

    let mut main_menu = layouts::main_menu();

    let mut against_yourself = layouts::against_yourself();

    let mut against_bot = layouts::against_bot();

    let mut online = layouts::online();

    let mut is_fullscreen = true;

    loop {
        window_parameters.update();
        window_parameters.clear(BEIGE);
        window_parameters.render_texture(0.0, 0.0, 1.0, 1.0, &texture);

        if is_key_pressed(KeyCode::F11) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }

        match game_state {
            GameState::Menu => {
                main_menu.update(&window_parameters);
                main_menu.render(&window_parameters);

                if main_menu.was_button_clicked("Against yourself") {
                    game_state = GameState::AgainstYourself;
                }
                if main_menu.was_button_clicked("Against bot") {
                    game_state = GameState::AgainstBot;
                }
                if main_menu.was_button_clicked("Online") {
                    game_state = GameState::Online;
                }
                if main_menu.was_button_clicked("Quit") {
                    exit(0);
                }
            }
            GameState::AgainstYourself => {
                if against_yourself.was_button_clicked("Back") {
                    game_state = GameState::Menu;
                }
                if against_yourself.was_button_clicked("Reset") {
                    ui_chess_board.reset_board(&ChessBoard::starting_positions().squares);
                    board = ChessBoard::starting_positions();
                    ui_chess_board.update(&board.squares);
                    against_yourself.remove_title("Win");
                    against_yourself.remove_title("Draw");
                }
                if against_yourself.was_button_clicked("Flip") {
                    ui_chess_board.flip(&board.squares);
                }

                against_yourself.update(&window_parameters);
                ui_chess_board.update_assume_logic(&window_parameters);
                let movement_proposal = ui_chess_board.request_move(&window_parameters);
                if let Some(coord) = movement_proposal.0 {
                    let result = board.move_piece(coord.0, coord.1, movement_proposal.1);
                    ui_chess_board.check_result(result);
                    ui_chess_board.update(&board.squares);
                }

                match ui_chess_board.game_status {
                    chess::chess_board::GameStatus::Ongoing => {}
                    chess::chess_board::GameStatus::Draw(_) => against_yourself.add_title("Draw", Title::new("Draw", 130.0, 0.3, 0.4, BLACK)),
                    chess::chess_board::GameStatus::Win(color) => {
                        let win_string = match color {
                            chess::Color::White => "White won",
                            chess::Color::Black => "Black won",
                        };
                        against_yourself.add_title("Win", Title::new(win_string, 130.0, 0.3, 0.4, BLACK))
                    }
                }

                ui_chess_board.render(&window_parameters);
                against_yourself.render(&window_parameters);
            }
            GameState::AgainstBot => {
                against_bot.update(&window_parameters);
                against_bot.render(&window_parameters);
                if against_bot.was_button_clicked("Back") {
                    game_state = GameState::Menu;
                }
            }
            GameState::Online => {
                online.update(&window_parameters);
                online.render(&window_parameters);
                if online.was_button_clicked("Back") {
                    game_state = GameState::Menu;
                }
            }
        }
        window_parameters.clear_outside(BLACK);
        next_frame().await
    }
}
