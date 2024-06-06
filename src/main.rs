pub mod chess;

//use chess::*;


//#[macroquad::main("BasicShapes")]
fn main() {
    let mut board = chess::Board::starting_positions();
    board.display_as_text();
    board.move_piece((4,1), (4,3));
    board.move_piece((4,6), (4,4));
    board.move_piece((5,0), (2,3));
    println!();
    board.display_as_text();

    /* loop {
        clear_background(RED);

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
    } */
}