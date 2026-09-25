use ggez::graphics::Color;
pub use ggez::*;
// use samolss_chess::board::Piece;
// use samolss_chess::move_piece;


// pub const SCREEN_WIDTH:f32 = 2000.0;
// pub const SCREEN_HEIGHT:f32 = 2000.0;
pub const SCREEN_X:f32 = 650.0;
pub const SCREEN_Y:f32 = 100.0;
// pub const BOARD_SIZE: graphics::Rect = graphics::Rect{
//     x: SCREEN_X, 
//     y: SCREEN_Y, 
//     w: SCREEN_WIDTH, 
//     h: SCREEN_HEIGHT
// }; //x, y, width, height

pub const SQUARE_LENGTH:f32 = 200.0;
// pub const SQUARE_SIZE: graphics::Rect = graphics::Rect{
//     x: SCREEN_X, //hmmmm these will be different, thonk
//     y: SCREEN_Y, 
//     w: SCREEN_WIDTH, 
//     h: SCREEN_HEIGHT
// }; //x, y, width, height

pub const SQUARE_COORDS:[[(f32, f32); 8]; 8] = setting_coords_for_board();
//just make one for the squares...


//this creates the board coordinates
const fn setting_coords_for_board() -> [[(f32, f32); 8]; 8] {
    let start_x:f32 = SCREEN_X;
    let start_y:f32 = SCREEN_Y;

    let mut square_coords = [[(0.0, 0.0); 8]; 8]; //placeholders (0.0, 0.0)

    let mut x  = 0;
    let mut x_countdown = 7;
    while x < 8 { //for doesnt work becaus eit like works differently than while
        let mut y = 0;
        let mut y_countdown = 7;
        while y < 8 {
            square_coords[x][y] = (start_x + x_countdown as f32 * SQUARE_LENGTH, start_y + y_countdown as f32 * SQUARE_LENGTH);
            y += 1; //countdown is so that the board flips hahahah.
            y_countdown -= 1
        }
        x += 1;
        x_countdown -= 1
    }
    // for x in 0..8 {
    //     for y in 0..8 {
    //         square_coords[x][y] = (start_x + x as f32 * SQUARE_LENGTH, start_y + y as f32 * SQUARE_LENGTH);
    //     }
    // }
    square_coords
}



//this creates the group of the board squares
pub fn making_board_using_coords(ctx:&mut Context, square_coords: [[(f32, f32); 8]; 8]) -> GameResult<graphics::Mesh> {

    //mesh is like a group of a bucnh of stuff (such as rects)
    let mut grouped_mesh: graphics::MeshBuilder = graphics::MeshBuilder::new();

    let dark_color = Color::from_rgb(80, 100, 80);
    let light_color = Color::from_rgb(240, 240, 240);

    let mut x = 0;
    while x < 8 { //for doesnt work becaus eit like works differently than while
        let mut y = 0;
        while y < 8 {
            let (posx, posy) = square_coords[x][y];
            // grouped_mesh.
            let square = graphics::Rect::new(posx, posy, SQUARE_LENGTH, SQUARE_LENGTH);

            let color:Color;
            if (x+y)%2 == 0 { //both odd or both even
                color = dark_color;
            } else {
                color = light_color;
            }
            grouped_mesh.rectangle(graphics::DrawMode::fill(), square, color)?;
            y += 1
        }
        x += 1
    }
    //return a mesh from the objects i collected
    return Ok(graphics::Mesh::from_data(ctx, grouped_mesh.build())) //ok turns it into gameresult.
}




