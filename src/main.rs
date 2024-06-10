pub mod chess;
use chess::Coordinate;


//#[macroquad::main("Chess")]
fn main() {
    let mut board = chess::Board::starting_positions();
    //let mut board = chess::Board::rook();
    board.display_as_text();
    println!();

    //println!("{:?}", board.move_piece(Coordinate::new(0, 7), Coordinate::new(0,0)));



    println!("{:?}", board.move_piece(Coordinate::new(4 , 1), Coordinate::new(4, 3)));
    println!("{:?}", board.move_piece(Coordinate::new(4 , 6), Coordinate::new(4, 4)));
    println!("{:?}", board.move_piece(Coordinate::new(4 , 3), Coordinate::new(4, 4)));
    println!("{:?}", board.move_piece(Coordinate::new(5 , 0), Coordinate::new(1, 4)));
    println!("{:?}", board.move_piece(Coordinate::new(1 , 7), Coordinate::new(2, 5)));
    println!("{:?}", board.move_piece(Coordinate::new(4 , 0), Coordinate::new(5, 0)));
    println!("{:?}", board.move_piece(Coordinate::new(0, 7), Coordinate::new(0, 5)));

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
