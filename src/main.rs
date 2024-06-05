use std::collections::btree_map::Range;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        let grid_width_x = 8;
        let grid_width_y = 8;

        let grid_item_width_x= screen_width()/grid_width_x as f32;
        let grid_item_width_y = screen_height()/grid_width_y as f32;

        let mut color = BLACK;
        for i in 0..grid_width_x{
            for j in 0..grid_width_y{
                draw_rectangle(i as f32 * grid_item_width_x,j as f32 * grid_item_width_y,grid_item_width_x,grid_item_width_y, color);
                if color == BLACK{
                    color = WHITE;
                }else{
                    color = BLACK
                }
            }
            if grid_width_y % 2 == 0 {
                if color == BLACK {
                    color = WHITE;
                } else {
                    color = BLACK;
                }
            }
        }
        

        next_frame().await
    }
}