use macroquad::prelude::*;

use super::ui_manager::*;

pub fn main_menu() -> UIManager {
    let mut main_menu = UIManager::new();
    main_menu.add_title(
        "Main Title",
        Title::new_center_width("Rusty Chess", 70.0, 0.1, BLACK),
    );
    main_menu.add_button(
        "Against yourself",
        Button::new_center_width(0.2, 0.3, 0.1, "Against yourself", BLUE, LIGHTGRAY),
    );
    main_menu.add_button(
        "Against bot",
        Button::new_center_width(0.35, 0.3, 0.1, "Against bot", BLUE, LIGHTGRAY),
    );
    main_menu.add_button(
        "Online",
        Button::new_center_width(0.5, 0.3, 0.1, "Online", BLUE, LIGHTGRAY),
    );
    main_menu.add_button(
        "Quit",
        Button::new(0.83, 0.0, 0.17, 0.08, "Quit Chess", BLUE, RED),
    );

    main_menu
}

pub fn against_yourself () -> UIManager{
    let back_button = Button::new(0.001, 0.001, 0.07, 0.04, "Back", GRAY, LIGHTGRAY);

    let mut against_yourself = UIManager::new();
    against_yourself.add_button("Back", back_button.clone());
    against_yourself.add_button(
        "Reset",
        Button::new(0.7, 0.2, 0.055, 0.05, "reset", BLUE, RED),
    );
    against_yourself.add_button(
        "Flip",
        Button::new(0.7, 0.1, 0.15, 0.05, "flip board", BLUE, GRAY),
    );

    against_yourself
}

pub fn against_bot () -> UIManager{
    let back_button = Button::new(0.001, 0.001, 0.07, 0.04, "Back", GRAY, LIGHTGRAY);
    let mut against_bot = UIManager::new();
    against_bot.add_button("Back", back_button.clone());
    against_bot.add_title(
        "Not Implmented",
        Title::new_center_width("Not Implmented", 90.0, 0.4, RED),
    );

    against_bot
}

pub fn online () -> UIManager{
    let back_button = Button::new(0.001, 0.001, 0.07, 0.04, "Back", GRAY, LIGHTGRAY);
    let mut online = UIManager::new();
    online.add_button("Back", back_button.clone());
    online.add_title(
        "Not Implmented",
        Title::new_center_width("Not Implmented", 90.0, 0.4, RED),
    );
    
    online
}