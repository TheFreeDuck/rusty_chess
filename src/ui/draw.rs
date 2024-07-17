use std::io::Write;

use macroquad::{
    color::*,
    input::mouse_position,
    math::{vec2, Vec2},
    shapes::*,
    text::{draw_text, get_text_center},
    texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D},
    window::*,
};
use tempfile::NamedTempFile;

pub struct WindowParameters {
    pub target_aspect_ratio: (f32, f32),
    pub aspect_ratio_number: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub width: f32,
    pub height: f32,
}

impl WindowParameters {
    pub fn new(target_aspect_ratio: (f32, f32)) -> Self {
        let window_aspect_ratio = screen_width() / screen_height();
        let aspect_ratio_number = target_aspect_ratio.0 / target_aspect_ratio.1;

        let width: f32;
        let height: f32;
        let x: f32;
        let y: f32;

        if aspect_ratio_number < window_aspect_ratio {
            height = screen_height();
            width = (height / target_aspect_ratio.1) * target_aspect_ratio.0;

            x = (screen_width() - width) / 2.0;
            y = 0.0;
        } else if aspect_ratio_number > window_aspect_ratio {
            width = screen_width();
            height = (width / target_aspect_ratio.0) * target_aspect_ratio.1;

            x = 0.0;
            y = (screen_height() - height) / 2.0;
        } else {
            width = screen_width();
            height = screen_height();

            x = 0.0;
            y = 0.0;
        }
        WindowParameters { target_aspect_ratio, aspect_ratio_number, x_offset: x, y_offset: y, width: width, height: height }
    }

    pub fn update(&mut self) {
        let window_aspect_ratio = screen_width() / screen_height();

        let width: f32;
        let height: f32;
        let x: f32;
        let y: f32;

        if self.aspect_ratio_number < window_aspect_ratio {
            height = screen_height();
            width = (height / self.target_aspect_ratio.1) * self.target_aspect_ratio.0;

            x = (screen_width() - width) / 2.0;
            y = 0.0;
        } else if self.aspect_ratio_number > window_aspect_ratio {
            width = screen_width();
            height = (width / self.target_aspect_ratio.0) * self.target_aspect_ratio.1;

            x = 0.0;
            y = (screen_height() - height) / 2.0;
        } else {
            width = screen_width();
            height = screen_height();

            x = 0.0;
            y = 0.0;
        }

        self.x_offset = x;
        self.y_offset = y;
        self.width = width;
        self.height = height;
    }

    pub fn mouse_position(&self) -> (f32, f32) {
        let mouse_x = (mouse_position().0 - self.x_offset) / self.width;
        let mouse_y = (mouse_position().1 - self.y_offset) / self.height;
        (mouse_x, mouse_y)
    }

    pub fn clear_outside(&self, color: Color) {
        draw_rectangle(0.0, 0.0, self.x_offset, self.height, color);

        draw_rectangle(self.x_offset + self.width, 0.0, self.x_offset, self.height, color);

        draw_rectangle(0.0, 0.0, self.width, self.y_offset, color);

        draw_rectangle(0.0, self.y_offset + self.height, self.width, self.y_offset, color);
    }

    pub fn clear(&self, color: Color) {
        draw_rectangle(self.x_offset, self.y_offset, self.width, self.height, color);
    }

    pub fn get_text_center(&self, text: &str, font_size: u16) -> Vec2{
        get_text_center(text, None, font_size, 1.0, 0.0)/Vec2::new(self.width, self.height)
    }

    pub fn render_circle(&self, x: f32, y: f32, radius: f32, color: Color) {
        draw_circle(self.x_offset + x * self.width, self.y_offset + y * self.height, radius * self.width, color);
    }

    pub fn render_rectangle(&self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        draw_rectangle(self.x_offset + x * self.width, self.y_offset + y * self.height, width * self.width, height * self.height, color);
    }

    pub fn render_rectangle_line(&self, x: f32, y: f32, width: f32, height: f32, thickness: f32, color: Color) {
        draw_rectangle_lines(self.x_offset + x * self.width, self.y_offset + y * self.height, width * self.width, height * self.height, thickness * self.width, color);
    }

    pub fn render_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        draw_text(text, self.x_offset + x * self.width, self.y_offset + y * self.height, font_size, color);
    }

    pub fn render_texture(&self, x: f32, y: f32, width: f32, height: f32, texture: &Texture2D) {
        draw_texture_ex(texture, self.x_offset + x * self.width, self.y_offset + y * self.height, WHITE, DrawTextureParams { dest_size: Some(vec2(width * self.width, height * self.height)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None });
    }
}

pub async fn load_texture_from_bytes(bytes: &[u8]) -> Result<Texture2D, Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;

    temp_file.write_all(bytes)?;

    let file_path = temp_file.path().to_str().ok_or("Failed to convert path to str")?;

    let texture = load_texture(file_path).await?;
    Ok(texture)
}
