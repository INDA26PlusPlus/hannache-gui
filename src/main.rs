mod constants;
use constants::*;
use ggez::{audio::{SoundSource, Source}, graphics::{Color}};

use samolss_chess::{board::{self, Board, Piece}, move_piece};
use std::path::PathBuf;

use std::net::{TcpListener, TcpStream};
use std::env;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

// use std::collections::HashMap;


// use dict::Dict; WHy is dict not inbyggt I'm crying

struct State {
    square_coords:[[(f32, f32); 8]; 8],
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

    //networking
    stream:TcpStream,
    // is_host:bool,
    is_white:bool, //wether you are white or not
    network_buffer:String,
    new_board:Option<Board>,
    waiting_for_input:bool,

}

impl State {
    fn new(ctx: &mut Context, stream: TcpStream, is_white: bool) -> GameResult<State> {
        let square_coords = setting_coords_for_board(&is_white);
        let chess_board = making_board_using_coords(ctx, square_coords)?;//? checks for errors, makes it safer
        let backend_board = board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
        //music
        let mut playlist = vec![
            Source::new(ctx, "/a_brand_new_day.ogg")?,
            Source::new(ctx, "/airplane_pt_2.ogg")?,
            Source::new(ctx, "/boy_with_luv.ogg")?,
            Source::new(ctx, "/idol.ogg")?,
            ];

        playlist[0].play(ctx)?;

        

        Ok(State {
            square_coords,
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

            // ip_adress:SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            stream,
            // is_host: is_host,
            is_white,
            network_buffer: "".to_string(),

            //the things i'll need to update board.
            new_board:None,
            //if black then its true if white then its false
            waiting_for_input:if is_white {false} else {true},

        })
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult { //update and draw are required?
        self.dt = ctx.time.delta(); //gets the system time.
        
        //--taking in the network
        let mut buffer: [u8; 128] = [0; 128];

        match self.stream.read(&mut buffer){ //use the internet buffer
            Ok(0) => {} //they disconnected
            Ok(bytes_read) => {self.network_buffer.push_str(std::str::from_utf8(&buffer[..bytes_read]).unwrap_or(""));}
            Err(_) => {}//uh error u didnt get anything
        }
        //what happens if theres nothing?
        // let text = std::str::from_utf8(&buffer[..bytes_read]) //in simple terms this turns the buf into &str
        // .unwrap_or(""); //i get empty string if theres no input
        // //.trim(); //gets rid of space and \n
        
        // //this is how i can do network_buffer + text
        // self.network_buffer.push_str(&text.to_string());

        //to get rid: .drain(..end_index)

        //if you are waiting for input
        if self.waiting_for_input {
            if let Some(newline_index) = self.network_buffer.find("\n") {
                //you got a message!
                let message: &str = &self.network_buffer[..newline_index];
                match message {
                    "OK" => {
                        self.waiting_for_input = true; //you sucessfully sent your message, now wait for theirs.
                        self.highlight_mesh = None;
                        self.active_position = None;

                        if let Some(the_new_board) = self.new_board.take() { //what does take() do?
                            self.backend_board = the_new_board;
                            self.new_board = None
                        } else {
                            //you dont have a board... why  did you get this message even.. What is they oking to
                        }
                        println!("Move aceppted");
                    }
                    "REJECT" => {
                        self.highlight_mesh = None;
                        self.active_position = None;
                        self.new_board = None;
                        self.waiting_for_input = false; //leme send you a new move before u say something
                        println!("Move rejected");
                    }
                    "CHECKMATE" => {
                        if self.is_white {
                            self.backend_board.black_lost = true;
                        } else {
                            self.backend_board.white_lost = true;
                        }
                    }
                    "STALEMATE" => {
                        self.backend_board.is_draw = true;
                    }
                    _ => {
                        println!("Something else input happned");
                        //it could be the message... lets make like a waiting for move variable.
                        // if !self.waiting_for_input {
                        //     //not ok. i dont want this
                        //     //send reject
                        //     let _ = self.stream.write_all(b"REJECT\n");
                        // }
                        //its probably an input
                        //responds for you and changes waiting for input
                        simulate_changes(message.to_string(), self);

                    }
                }
                self.network_buffer.drain(..=newline_index);
            }   
        }     

        //-music
        if let Some(current_index) = self.playlist.get(self.current_song_i) {
            if current_index.stopped() {
                self.current_song_i = (self.current_song_i+1) % self.playlist.len();
                self.playlist[self.current_song_i].play(ctx)?;
            }
        }

        let game_over = self.backend_board.black_lost || self.backend_board.white_lost || self.backend_board.is_draw;
        if game_over {
            return Ok(()); 
        }


        if self.is_white == self.backend_board.white_turn && !self.waiting_for_input {
            //your turn and you arent waiting for input
            if self.active_position == None {  // You havent chosen a piece yet.
            if ctx.mouse.button_just_pressed(event::MouseButton::Left) { 
                let mouse_pos = ctx.mouse.position(); //get pos of mouse
                //mouse_pos.x and mouse_pos.y
                'outer: for x in 0..8 {
                    for y in 0..8 {
                        let (posx, posy) = self.square_coords[x][y]; //check every square.
                        
                        if !(posx+SQUARE_LENGTH > mouse_pos.x) || !(posx < mouse_pos.x) 
                        || !(posy+SQUARE_LENGTH > mouse_pos.y) || !(posy < mouse_pos.y) {
                            //you found the correct square!
                            continue
                        }

                        let piece:board::Piece  = self.backend_board.squares[y][x];
                        if piece.is_white() != Some(self.backend_board.white_turn) {
                            //you can't pick this character.
                            continue
                        }
                        //you can chose this character
                        self.active_position = Some((x, y)); //to bo an option you have to be a Some() or a None
                        let highlighted_squares: graphics::Mesh = highlight_legal_moves(ctx, (x, y), self)?;
                        //then i want to draw the thing. how do i go from update to draw?
                        self.highlight_mesh = Some(highlighted_squares);
                        break 'outer;
                    }
                }
            }
            //check what you are pressing.
            //for square in all_squares {}
        } 
        //maybe if else will get skipped but if its two if statements it wont
        else {
            //Your position is up <=> You have an active_position
            if ctx.mouse.button_just_pressed(event::MouseButton::Left) {
                let mouse_pos = ctx.mouse.position(); //get pos of mouse
                'outer: for x in 0..8 {
                    for y in 0..8 {
                        let (posx, posy) = self.square_coords[x][y]; //check every square.
                        
                        if !(posx+SQUARE_LENGTH > mouse_pos.x) || !(posx < mouse_pos.x) 
                        || !(posy+SQUARE_LENGTH > mouse_pos.y) || !(posy < mouse_pos.y) {
                            //you found the incorrect square!
                            //viss x and viss y
                            continue
                        }
                        //now you've found the correct square.
                        let coords = (x, y);
                        //this is the new coords

                        //store old map, old coords. maybe...
                        //is it enough to just restore the old map?
                        // let old_board = self.backend_board.clone();
                        
                        if let Some(active_position) = self.active_position {
                            match move_piece::move_piece(self.backend_board.clone(), active_position, coords, 'q') {
                                Ok(new_board) => { 
                                    //i'm only doing this on a cloned board actual board i untouched
                                    
                                    //its my turn:
                                    //this whole thing is jsut sending my move to the opponent
                                    let from = coord_transformer(active_position.clone());
                                    let to = coord_transformer(coords.clone());
                                    let to_piece:String;
                                    if let Piece::Pawn { .. } = self.backend_board.squares[active_position.1][active_position.0] {
                                        to_piece = "Q".to_string(); //Temporary: queen
                                    } else {
                                        to_piece = "-".to_string();
                                    }
                                    let formatted_new_board = board_transformer(new_board.clone());//from: pub squares: [[Piece; WIDTH]; HEIGHT], to rnbqkbnrpppppppp PPPPPPPPRNBQKBNR
                                    let send_string = format!("{from}{to}{to_piece}{formatted_new_board}\n");
                                    let _ = self.stream.write_all(send_string.as_bytes()); //A1B2P[char; 64]\n
                                    //--

                                    self.waiting_for_input = true;
                                    self.new_board = Some(new_board);
                                    // self.backend_board = new_board;
                                    // self.active_position = None; //back to none
                                    // self.highlight_mesh = None;
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
        } else {
            //wait for input hahahahah
            return Ok(()); //dont do stuff I wanna highlight them thoughh...
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        canvas.draw(&self.chess_board, graphics::DrawParam::default());


        for x in 0..8{
            for y in 0..8 {
                let piece:board::Piece = self.backend_board.squares[y][x]; //this is the piece, for some reason its y -> x ...
                let (posx, posy) = self.square_coords[x][y];
                //drawing the board, matching picture to name
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

        //if i have a highlight then i highlight it.
        if let Some(ref mesh) = self.highlight_mesh { //ref mesh islike somewhere i store self.highlight_mesh
            canvas.draw(mesh, graphics::DrawParam::default());
        }

        //drawing out winning our losing
        let mut param = graphics::DrawParam::default();
        param = param.dest([400.0, 600.0]);
        param = param.color(graphics::Color::new(1.0, 1.0, 1.0, 0.8)); //to make it transparent
        // canvas.draw(&self.white_lost, graphics::DrawParam::default().dest([10.0, 500.0])).color(graphics::Color::new(1.0, 1.0, 1.0, 0.5));
        if self.backend_board.white_lost == true {
            canvas.draw(&self.white_lost, param);
        } else if self.backend_board.black_lost == true {
            canvas.draw(&self.black_lost, param);
        } else if self.backend_board.is_draw == true {
            canvas.draw(&self.black_lost, param);
            canvas.draw(&self.white_lost, param);
        } //types dont make lots of sense.

        let _ = canvas.finish(ctx);//.map_err(|e| { eprintln!("finish failed: {e:?}"); e })?; //if something breaks -> 
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

    // let (a, b) = initial_handling().expect("hi");
    // let mut state = State::new(&mut ctx)?; //initialising state
    let (stream, is_white) = initial_handling().expect("error!");
    let _ = stream.set_nonblocking(true); //returns error maybe sometimes.

    let state = State::new(&mut ctx, stream, is_white)?; //initialising state

    event::run(ctx, event_loop, state); //update and draw is the thing that will be constantly called
}

fn highlight_legal_moves(ctx:&mut Context, square: (usize, usize), state:&mut State) -> GameResult<graphics::Mesh> {
    let board = &state.backend_board;
    let mut grouped_mesh: graphics::MeshBuilder = graphics::MeshBuilder::new();
    let all_legal_moves: Vec<((usize, usize), (usize, usize), char)> = move_piece::gen_moves_for_piece(board, square);
    
    let color = Color::from_rgba(223, 220, 136, 128);
    for (_, (x, y), _) in all_legal_moves {
        let (posx, posy) = state.square_coords[x][y]; //translates to the board position
        let square =  graphics::Rect::new(posx, posy, SQUARE_LENGTH, SQUARE_LENGTH);

        grouped_mesh.rectangle(graphics::DrawMode::fill(), square, color)?;
    }
    Ok(graphics::Mesh::from_data(ctx, grouped_mesh.build()))
}

fn coord_transformer(coords:(usize, usize)) -> String {
    let mut return_value = "".to_string();
    let lst = ["A", "B", "C", "D", "E", "F", "G", "H"];
    return_value.push_str(lst[coords.0]);

    let lst = ["1", "2", "3", "4", "5", "6", "7", "8"];
    return_value.push_str(lst[coords.1]);
    return_value
}

fn decoder(input:String) -> Option<((usize,usize), (usize, usize))> {//, [[Piece; WIDTH]; HEIGHT]) {

    // let mut from:(usize,usize) = (0, 0);
    if input.len() == 69 {
    let crypt_from = &input[0..2];
    let crypt_to = &input[2..4];
    // let crypt_promotion = &input[4..5];
    // let crypt_new_map = &input[5..69];

    let lst_letter = ["A", "B", "C", "D", "E", "F", "G", "H"];
    let lst_number = ["1", "2", "3", "4", "5", "6", "7", "8"];

    let from = (lst_letter.iter().position(|&x| x == &crypt_from[0..1]).unwrap_or(0), 
        lst_number.iter().position(|&x| x == &crypt_from[1..2]).unwrap_or(0));
    let to = (lst_letter.iter().position(|&x| x == &crypt_to[0..1]).unwrap_or(0), 
        lst_number.iter().position(|&x| x == &crypt_to[1..2]).unwrap_or(0));

    // let (a, b) = crypt_new_map.as_bytes().chunks(8);
            
    // for y in (0..8).rev() {
    //     for x in 0..8 {
    //         match row[x] {
    //             new_map[y][x]

    //         }
    //     }
    // }
        return Some((from, to))
    }
    else {
        return None
    }

    //, promotion, new_map);
}

fn simulate_changes(input:String, state:&mut State) {

    //remember this is for the other party

    //check if the move is available on my board?
    //simulate the thing on the clone board then check if its the same as the inputted board
    //if yes -> send OK\n and put the cloens board as the actual board
    //if no -> send REJECT\n and dont do anything

    //extras:
    //check promotion type i guess..
    if let Some((from_coords, to_coords)) = decoder(input.to_string()) {
        
        let board_i_got = &input[5..69];

        let copy_board = state.backend_board.clone();
        println!("{:?}", copy_board.white_turn);
        println!("{:?}", state.is_white);
        println!("{:?}", copy_board.legal_moves);
        
        // let mut to_piece: String;
        let new_board = move_piece::move_piece(state.backend_board.clone(), from_coords, to_coords, 'q');
        match new_board {
            Ok(new_board) => {
                //this means that its sucessful and i put it into a board.
                

                //compare it to the board you got from the message..
                let string_board = board_transformer(new_board.clone());
                println!("{:?}", string_board);
                if board_i_got == string_board { //they are the sameee
                    if new_board.black_lost||new_board.white_lost {
                        let _ = state.stream.write_all(b"CHECKMATE\n");
                    } else if new_board.is_draw {
                        let _ = state.stream.write_all(b"STALEMATE\n");
                    }
                    state.backend_board = new_board;
                    let _ = state.stream.write_all(b"OK\n");
                    state.waiting_for_input = false //no longer waiting i send now
                
                } else {
                    //its not the same
                    //send reject
                    let _ = state.stream.write_all(b"REJECT\n");
                }
            },
            Err(_) => {
                let _ = state.stream.write_all(b"REJECT\n");
                println!("Not OK(())");

            }
        }
    } else {
        let _ = state.stream.write_all(b"REJECT\n");
        println!("What outside of the match even");
    }
}

fn board_transformer(board:Board) -> String {
    let mut return_value = "".to_string();
    //you have to flip the y axis
    for y in (0..8).rev() { //this should be reveresrd btw, 8 -> 0
        for x in 0..8 {
            let piece = board.squares[y][x];
            match piece { //this is board::Piece
                    board::Piece::Pawn { is_white:true, .. } => {
                        return_value.push_str("P");
                        // canvas.draw(piece.white_pawn, param);
                    }
                    board::Piece::Pawn { is_white:false, .. } => {
                        return_value.push_str("p");
                    }
                    board::Piece::Bishop { is_white:true, .. } => {
                        return_value.push_str("B");
                    }
                    board::Piece::Bishop { is_white:false, .. } => {
                        return_value.push_str("b");
                    }
                    board::Piece::Rook { is_white:true, .. } => {
                        return_value.push_str("R");
                    }
                    board::Piece::Rook { is_white:false, .. } => {
                        return_value.push_str("r");
                    }
                    board::Piece::Knight { is_white:true, ..} => {
                        return_value.push_str("N");
                    }
                    board::Piece::Knight { is_white:false, .. } => {
                        return_value.push_str("n");
                    }
                    board::Piece::Queen { is_white:true, .. } => {
                        return_value.push_str("Q");
                    }
                    board::Piece::Queen { is_white:false, .. } => {
                        return_value.push_str("q");
                    }
                    board::Piece::King { is_white:true, .. } => {
                        return_value.push_str("K");
                    }
                    board::Piece::King { is_white:false, .. } => {
                        return_value.push_str("k");
                    }
                    board::Piece::Empty => {
                        return_value.push_str(" ");
                    }
            }
            
        }
        // let piece:board::Piece  = state.backend_board.squares[y][x];
        // if piece.is_white() != Some(self.backend_board.white_turn) {
                        
    }
    return_value
}


//fn initialising_connection() ... uhh
// #[test]
fn initial_handling() -> std::io::Result<(TcpStream, bool)> {//-> std::io::Result {
    let args: Vec<String> = env::args().collect();

    let mut is_white= true; //defult to you are white
    // let network_buffer: &str = "";

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--connect" => {
                //if you are --connect -> you are not host.
                // is_host = false;
                let stream1 = TcpStream::connect(&args[i+1])?; 
                let mut reader = BufReader::new(&stream1); //how i can use .read_line() on the reader that is based on stream1
                let mut buffer = String::new(); //allocated space for you to fill what input you get.
                
                //use buffer to read so i dont get excess info
                let _ = reader.read_line(&mut buffer).expect("noo it failed"); //split with the \n
                let color = buffer.as_str();

                if color == "W\n" {
                    is_white = true

                } else if color == "B\n" {
                    is_white = false
                }

                return Ok((stream1, is_white))
                
            }
            "--color" => {
                //you are the host! But you dont have to send color!

                if args[i+1] == "W" { //you are host and chose 
                    is_white = true;
                } else if args[i+1] == "B" {
                    is_white = false;
                }
            }
            _ => {}
        }
        i+=1
    }

    //there is no connect and there is maybe a color. ->
    //you are host -> listen for a connection when they connect. host sends color
    println!("Waiting for opponent...");
    let listener = TcpListener::bind(IP_ADRESS)?; //you chose adress
    // let _ = listener.set_nonblocking(true); //uh ig listner also blocks my code..?
    let (mut stream, _) = listener.accept()?; //Second is their ip adress that they're connecting from i think?

    
    if is_white {
        //they are black
        stream.write_all(b"B\n")?; //b turns it into bytes
    } else {
        //they are white
        stream.write_all(b"W\n")?;
    }

    return Ok((stream, is_white));
}



//this creates the board coordinates
pub fn setting_coords_for_board(is_white:&bool) -> [[(f32, f32); 8]; 8] {
    let start_x:f32 = SCREEN_X;
    let start_y:f32 = SCREEN_Y;

    let mut square_coords = [[(0.0, 0.0); 8]; 8]; //placeholders (0.0, 0.0)


    // let mut x  = 0;
    // let mut x_countdown = 7;
    for x in 0..8 { //for doesnt work becaus eit like works differently than while
        // let mut y = 0;
        // let mut y_countdown = 7;
        for y in 0..8 {
            
            let (col, row) = if *is_white {
                (x, 7-y)
            } else {
                (7-x, y)
            };
            square_coords[x][y] = (start_x + col as f32 * SQUARE_LENGTH, start_y + row as f32 * SQUARE_LENGTH);
            
        }
        // x_countdown -= 1
    }
    // for x in 0..8 {
    //     for y in 0..8 {
    //         square_coords[x][y] = (start_x + x as f32 * SQUARE_LENGTH, start_y + y as f32 * SQUARE_LENGTH);
    //     }
    // }
    square_coords
}



#[test]
fn test_show_moves() {
    // let mut board =
    let board =
        board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    // This will generate all LEGAL moves, each move consisting of the old square, the new square
    // and a char denoting what the promotion piece should be.
    println!("{:?}", move_piece::gen_all_moves(&board));
    let square = (0, 6);
    println!("{:?}", move_piece::gen_moves_for_piece(&board, square)); //vänster top = (0, 0), höger bottom = (7,7)
    // Good to know is that when castling the old square should be the kings square and the new
    // square should be the rooks square.
}

#[test]
fn testing_move_piece() {
    let mut board =
        board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    // The e is the 5th letter, hence the index will be 5 - 1 = 4 for the x coordinate
    // 2 will similarily give us 1 as 2 - 1 = 1
    let e2_square = (4, 1);
    let e4_square = (4, 3);
    println!("{}", board.white_turn);

    // To move a piece, we need a board, the square the current piece is on, the square we want to
    // move to, and what we want to promote our piece to (will only be used when a pawn is actually
    // being promoted, but it needs to be supplied for now).
    board = move_piece::move_piece(board, e2_square, e4_square, 'q').expect("Move was invalid");
    println!("{}", board.white_turn);

    board.white_turn = !board.white_turn; //flipping turns
    println!("{}", board.white_turn);

    board::print_board(&board);

    println!("Hurray!!");
}


#[test]
fn testing_sytaxes() {
    let args: Vec<String> = vec!["hello,".to_string(), "hi".to_string(), "--connect".to_string()];

    if let Some(pos) = args.iter().position(|arg| arg == "--connect") {
        // println!("{}", Some(args));
        println!("{}", pos);
    }
}

#[test]
fn testing_string() {
    let a = "Hello";
    let b = " World";
    // let c = a+bs;
    println!("{:?}{}", a, b);
}

#[test]
fn teeestt() {
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks(2);
    // println!("{}", iter);
    assert_eq!(iter.next(), Some(&['l', 'o'][..]));
    assert_eq!(iter.next(), Some(&['r', 'e'][..]));
    assert_eq!(iter.next(), Some(&['m'][..]));
    assert_eq!(iter.next(), None);
}

#[test]
fn testing_find() {
    let text = "01234567\n1".to_string();
    if let Some(newline_index) = text.find("\n") {
                
        //you got a message!
        println!("{}", newline_index);
        println!("{}", &text[..=newline_index]);
        println!("{}", &text[..newline_index]);
        
        let mut test1 = text.clone();
        let mut test2 = text.clone();
        test1.drain(..newline_index);
        test2.drain(..=newline_index); //this i want
        println!("{}", test1);
        println!("{}", test2);
    }
}

//have a plan:
// coordinates in a vector maybe. like 0,0 represents a1
// each type is connected to a picture.
//draw you own board.

