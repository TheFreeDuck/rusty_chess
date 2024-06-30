use crate::draw::WindowParameters;
use macroquad::{
    color::{Color, BLACK}, input::{is_mouse_button_pressed, MouseButton}, math::Vec2, text::get_text_center
};
use std::collections::HashMap;

#[derive(Clone)]
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
    pub fn new(x: f32, y: f32, width: f32, height: f32, label: &str, color: Color, hover_color: Color) -> Self {
        Button { x, y, width, height, label: label.to_string(), color, hover_color, is_hovered: false, is_clicked: false }
    }

    pub fn new_center_width(y: f32, width: f32, height: f32, label: &str, color: Color, hover_color: Color) -> Self {
        Button { x: 0.5 - width / 2.0, y, width, height, label: label.to_string(), color, hover_color, is_hovered: false, is_clicked: false }
    }

    pub fn render(&mut self, window_parameters: &WindowParameters) {
        let color = if self.is_hovered { self.hover_color } else { self.color };
        window_parameters.render_rectangle(self.x, self.y, self.width, self.height, color);

        let text_middle = get_text_center(&self.label, None, 40, 1.0, 0.0)/Vec2::new(window_parameters.width,window_parameters.height);

        window_parameters.render_text(&self.label, self.x + (self.width / 2.0) - text_middle.x, self.y + (self.height / 2.0) - text_middle.y, 40.0, BLACK);
    }

    pub fn update(&mut self, window_parameters: &WindowParameters) {
        let (mouse_x, mouse_y) = window_parameters.mouse_position();
        let x = self.x;
        let y = self.y;
        let width = self.width;
        let height = self.height;

        self.is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;
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
        Title { text: text.to_string(), size, x: 0.5, y, color }
    }

    pub fn render(&mut self, window_parameters: &WindowParameters) {
        let text_middle = get_text_center(&self.text, None, 40, 1.0, 0.0)/Vec2::new(window_parameters.width,window_parameters.height);
        window_parameters.render_text(&self.text, self.x - text_middle.x*2.0, self.y - text_middle.y*2.0, self.size, self.color);
    }
}

pub struct UIManager {
    buttons: HashMap<String, Button>,
    titles: HashMap<String, Title>,
}

impl UIManager {
    pub fn new() -> Self {
        UIManager { buttons: HashMap::new(), titles: HashMap::new()}
    }

    pub fn add_button(&mut self, id: &str, button: Button) {
        self.buttons.insert(id.to_string(), button);
    }

    pub fn add_title(&mut self, id: &str, title: Title) {
        self.titles.insert(id.to_string(), title);
    }


    pub fn render(&mut self, window_parameters: &WindowParameters) {
        for button in self.buttons.values_mut() {
            button.render(window_parameters);
        }
        for title in self.titles.values_mut() {
            title.render(window_parameters);
        }
    }

    pub fn update(&mut self, window_parameters: &WindowParameters) {
        for button in self.buttons.values_mut() {
            button.update(window_parameters);
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
