mod constants;
use constants::*;
use ggez::{conf::FullscreenType::True, graphics::{Color, DrawParam}, input::mouse, audio::Source, audio::SoundSource};

use samolss_chess::{move_piece, board,board::Board};
use std::{io::SeekFrom::Current, path::PathBuf};


// #[derive(Clone)]
struct State {
    dt: std::time::Duration, //this gets the dt
    chess_board: graphics::Mesh,
    backend_board:Board,
    highlight_mesh: Option<graphics::Mesh>,
    active_position: Option<(usize, usize)>,

    //audio
    current_song_i: usize,
    playlist: Vec<Source>,

    //images
    white_pawn: graphics::Image,
    white_bishop:graphics::Image,
    white_knight:graphics::Image,
    white_rook:graphics::Image,
    white_king:graphics::Image,
    white_queen:graphics::Image,
    
    black_pawn:graphics::Image,
    black_bishop:graphics::Image,
    black_knight:graphics::Image,
    black_rook:graphics::Image,
    black_king:graphics::Image,
    black_queen:graphics::Image,

    white_lost:graphics::Image,
    black_lost:graphics::Image,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let chess_board = making_board_using_coords(ctx, SQUARE_COORDS)?;//? checks for errors, makes it safer
        let backend_board = board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
        let mut playlist = vec![
            Source::new(ctx, "/a_brand_new_day.ogg")?, 
            Source::new(ctx, "/airplane_pt_2.ogg")?,
            Source::new(ctx, "/boy_with_luv.ogg")?,
            Source::new(ctx, "/idol.ogg")?,
            ];

        playlist[0].play(ctx)?;

        Ok(State {
            dt: std::time::Duration::new(0,0),
            chess_board,
            backend_board, //instance of a class
            highlight_mesh: None,
            active_position: None,
            current_song_i: 0,
            playlist:playlist,

            white_pawn: graphics::Image::from_path(ctx, "/white_pawn1.png")?,
            white_bishop:graphics::Image::from_path(ctx, "/white_bishop1.png")?,
            white_knight:graphics::Image::from_path(ctx, "/white_knight1.png")?,
            white_rook:graphics::Image::from_path(ctx, "/white_rook1.png")?,
            white_king:graphics::Image::from_path(ctx, "/white_king1.png")?,
            white_queen:graphics::Image::from_path(ctx, "/white_queen1.png")?,
            
            black_pawn:graphics::Image::from_path(ctx, "/black_pawn1.png")?,
            black_bishop:graphics::Image::from_path(ctx, "/black_bishop1.png")?,
            black_knight:graphics::Image::from_path(ctx, "/black_knight1.png")?,
            black_rook:graphics::Image::from_path(ctx, "/black_rook1.png")?,
            black_king:graphics::Image::from_path(ctx, "/black_king1.png")?,
            black_queen:graphics::Image::from_path(ctx, "/black_queen1.png")?,

            white_lost:graphics::Image::from_path(ctx, "/black_wins_banner.png")?,
            black_lost:graphics::Image::from_path(ctx, "/white_wins_banner.png")?,

        })
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult { //update and draw are required?
        self.dt = ctx.time.delta(); //gets the system time.
        
        if let Some(current_index) = self.playlist.get(self.current_song_i) {
            if current_index.stopped() {
                self.current_song_i = (self.current_song_i+1) % self.playlist.len();
                self.playlist[self.current_song_i].play(ctx)?;
            }
        }

        if self.active_position == None {
            if ctx.mouse.button_just_pressed(event::MouseButton::Left) {
                let mouse_pos = ctx.mouse.position(); //get pos of mouse
                //mouse_pos.x and mouse_pos.y
                'outer: for x in 0..8 {
                    for y in 0..8 {
                        let (posx, posy) = SQUARE_COORDS[x][y]; //check every square.
                        
                        if !(posx+SQUARE_LENGTH > mouse_pos.x) || !(posx < mouse_pos.x) 
                        || !(posy+SQUARE_LENGTH > mouse_pos.y) || !(posy < mouse_pos.y) {
                            //you found the correct square!
                            //viss x and viss y
                            continue
                        }


                        let piece:board::Piece  = self.backend_board.squares[y][x];
                        if piece.is_white() != Some(self.backend_board.white_turn) {
                            //you can pick this character.
                            //show legal moves, create a mesh of them.
                            continue
                        }
                        self.active_position = Some((x, y)); //to bo an option you have to be a Some() or a None
                        let highlighted_squares: graphics::Mesh = highlight_legal_moves(ctx, &self.backend_board, (x, y))?;
                        //then i want to draw the thing. how do i go from update to draw?
                        // println!("legal moves for ({x},{y}): {:?}", move_piece::gen_moves_for_piece(&self.backend_board, (x, y)));
                        self.highlight_mesh = Some(highlighted_squares);
                        break 'outer;
                    }
                }
            }
            //check what you are pressing.
            //for square in all_squares {}
        } else {
            //Your position is up. You have an active_position
            if ctx.mouse.button_just_pressed(event::MouseButton::Left) {
                let mouse_pos = ctx.mouse.position(); //get pos of mouse
                'outer: for x in 0..8 {
                    for y in 0..8 {
                        let (posx, posy) = SQUARE_COORDS[x][y]; //check every square.
                        
                        if !(posx+SQUARE_LENGTH > mouse_pos.x) || !(posx < mouse_pos.x) 
                        || !(posy+SQUARE_LENGTH > mouse_pos.y) || !(posy < mouse_pos.y) {
                            //you found the incorrect square!
                            //viss x and viss y
                            continue
                        }
                        //now you've found the correct square.
                        let coords = (x, y);

                        if let Some(active_position) = self.active_position {
                            match move_piece::move_piece(self.backend_board.clone(), active_position, coords, 'q') {
                                Ok(new_board) => {
                                    self.backend_board = new_board;
                                    self.active_position = None; //back to none
                                    self.highlight_mesh = None;
                                    // println!("Sucess in moving piece");
                                    break 'outer;
                                },
                                Err(_) => {
                                    self.highlight_mesh = None;
                                    self.active_position = None;
                                    // println!("not sucessfull moving piece");
                                    break 'outer;
                                }
                            }
                        }
                    }
                }                   
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        // println!("Hello ggez! dt = {}ns", self.dt.as_nanos());
        //Draw the board
        canvas.draw(&self.chess_board, graphics::DrawParam::default());

        for x in 0..8{
            for y in 0..8 {
                let piece:board::Piece = self.backend_board.squares[y][x]; //this is the piece, for some reason its y -> x ...
                let (posx, posy) = SQUARE_COORDS[x][y];
                let param = graphics::DrawParam::default().dest([posx,posy]);
                match piece {
                    board::Piece::Pawn { is_white:true, .. } => {
                        canvas.draw(&self.white_pawn, param);
                    }
                    board::Piece::Pawn { is_white:false, .. } => {
                        canvas.draw(&self.black_pawn, param);
                    }
                    board::Piece::Bishop { is_white:true, .. } => {
                        canvas.draw(&self.white_bishop, param);
                    }
                    board::Piece::Bishop { is_white:false, .. } => {
                        canvas.draw(&self.black_bishop, param);                        
                    }
                    board::Piece::Rook { is_white:true, .. } => {
                        canvas.draw(&self.white_rook, param);
                    }
                    board::Piece::Rook { is_white:false, .. } => {
                        canvas.draw(&self.black_rook, param);
                    }
                    board::Piece::Knight { is_white:true, ..} => {
                        canvas.draw(&self.white_knight, param);
                    }
                    board::Piece::Knight { is_white:false, .. } => {
                        canvas.draw(&self.black_knight, param)
                    }
                    board::Piece::Queen { is_white:true, .. } => {
                        canvas.draw(&self.white_queen, param);
                    }
                    board::Piece::Queen { is_white:false, .. } => {
                        canvas.draw(&self.black_queen, param);
                    }
                    board::Piece::King { is_white:true, .. } => {
                        canvas.draw(&self.white_king, param);
                    }
                    board::Piece::King { is_white:false, .. } => {
                        canvas.draw(&self.black_king, param);
                    }
                    board::Piece::Empty => {
                        continue;
                    }

                }
            }
        }

        if let Some(ref mesh) = self.highlight_mesh { //ref mesh islike somewhere i store self.highlight_mesh
            canvas.draw(mesh, graphics::DrawParam::default());
        }

        let mut param = graphics::DrawParam::default();
        param = param.dest([400.0, 600.0]);
        param = param.color(graphics::Color::new(1.0, 1.0, 1.0, 0.8)); //to make it transparent
        // canvas.draw(&self.white_lost, graphics::DrawParam::default().dest([10.0, 500.0])).color(graphics::Color::new(1.0, 1.0, 1.0, 0.5));
       
       
        if self.backend_board.white_lost == true {
            canvas.draw(&self.white_lost, param);
        } else if self.backend_board.black_lost == true {
            canvas.draw(&self.black_lost, param)
        }

        canvas.finish(ctx);//.map_err(|e| { eprintln!("finish failed: {e:?}"); e })?; //if something breaks -> 
        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = PathBuf::from("./resources");
    let c = conf::Conf::new();

    let (mut ctx, event_loop) = ContextBuilder::new("hanna_chess_GUI", "Hanna")
    .default_conf(c)
    .add_resource_path(resource_dir)
    .build()
    .unwrap();

    let mut state = State::new(&mut ctx)?;//initialising state


    event::run(ctx, event_loop, state); //update and draw is the thing that will be constantly called
}


pub fn highlight_legal_moves(ctx:&mut Context, board:&Board, square: (usize, usize)) -> GameResult<graphics::Mesh> {

    let mut grouped_mesh: graphics::MeshBuilder = graphics::MeshBuilder::new();
    let all_legal_moves: Vec<((usize, usize), (usize, usize), char)> = move_piece::gen_moves_for_piece(board, square);
    
    let color = Color::from_rgba(223, 220, 136, 128);
    for (_, (x, y), _) in all_legal_moves {
        let (posx, posy) = SQUARE_COORDS[x][y]; //translates to the board position
        let square =  graphics::Rect::new(posx, posy, SQUARE_LENGTH, SQUARE_LENGTH);

        grouped_mesh.rectangle(graphics::DrawMode::fill(), square, color)?;
    }
    Ok(graphics::Mesh::from_data(ctx, grouped_mesh.build()))
}


#[test]
fn test_show_moves() {
    let mut board =
        board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    // This will generate all LEGAL moves, each move consisting of the old square, the new square
    // and a char denoting what the promotion piece should be.
    println!("{:?}", move_piece::gen_all_moves(&board));
    let square = (0, 6);
    println!("{:?}", move_piece::gen_moves_for_piece(&board, square)); //vänster top = (0, 0), höger bottom = (7,7)

    // Good to know is that when castling the old square should be the kings square and the new
    // square should be the rooks square.


}

//have a plan:
// coordinates in a vector maybe. like 0,0 represents a1
// each type is connected to a picture.
//draw you own board.

