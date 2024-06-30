use macroquad::{color::*, input::mouse_position, shapes::*, text::draw_text, window::*};

pub struct WindowParameters {
    pub target_aspect_ratio: (f32,f32),
    pub aspect_ratio_number: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub width: f32,
    pub height: f32,
}

impl WindowParameters {
    pub fn new(target_aspect_ratio: (f32,f32)) -> Self {
        let window_aspect_ratio = screen_width() / screen_height();
        let aspect_ratio_number = target_aspect_ratio.0/target_aspect_ratio.1;

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
        WindowParameters { target_aspect_ratio, aspect_ratio_number, x_offset: x, y_offset: y, width: width, height: height}
    }

    pub fn update(&mut self){
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

        self.x_offset =x;
        self.y_offset =y;
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
}
