extern crate sdl2;
use sdl2::controller::Button;  
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::controller::Axis;
use std::time::{Duration,Instant};
use std::path::Path;
use std::env;
use crate::sdl2::image::LoadTexture;



mod arena;
mod input;
mod explore;



fn main() {


    let (screen_width,screen_height) = (1280,720);

    let sdl_context = sdl2::init().expect("Failed to intialise sdl2 context");
    let video_subsystem = sdl_context.video().expect("Failed to create video subsystem");
    let window = video_subsystem.window("Can close yay",screen_width,screen_height).position_centered().build().expect("Failed to create window");
 

    let mut event_pump = sdl_context.event_pump().expect("failed to create event pump");
    let mut canvas = window.into_canvas().accelerated().build().expect("Failed to create canvas");
   
    let game_controller_subsystem = sdl_context.game_controller().expect("Failed to create game controller subsystem");

    let joy_count =  game_controller_subsystem.num_joysticks().unwrap();
    let mut connected_controllers = 0;
    let mut last_joycount = joy_count;

    let mut controllers : Vec<sdl2::controller::GameController> = Vec::new();

    let mut players : Vec<input::PlayerInput> = Vec::new();

    let mut config = input::PlayerInputConfig::load_from_file("toets_config.ccon".to_string());
    let config1 = input::PlayerInputConfig::load_from_file("p2.ccon".to_string());


    players.push(input::PlayerInput::new(config)); 
    players.push(input::PlayerInput::new(config1)); 
    
    
    let mut start = Instant::now();
    let mut elapsed;

    let path = Path::new(".");
    let path = path.join("sprite").join("Texture_Atlas.png");
    let texture_creator = canvas.texture_creator();
    let texture_atlas = texture_creator.load_texture(path).expect("Failed to load texure");
 
    let mut arena_game = arena::Game::new(screen_width,screen_height);


   // let exe_path = env::current_exe().expect("Failed to get exe path");
   // let exe_path = exe_path.as_path();

    loop {
        elapsed = start.elapsed();
        start = Instant::now();      
        let elapsedtime =  elapsed.as_micros() as f32/1_000_000.0;
        //println!("{}",elapsedtime);



        
        let joy_count =  game_controller_subsystem.num_joysticks().unwrap();

        if joy_count < last_joycount
        {
            connected_controllers -= 1;
            println!("A controller disconnected");
        }
        last_joycount = joy_count;
        if joy_count-connected_controllers > 0 
        {
            let mut i = 0;
            while i < joy_count
            {
                if game_controller_subsystem.is_game_controller(i)
                {
                    println!("A controller connected");                
                    controllers.push(game_controller_subsystem.open(i).expect("and was denied"));
                    println!("and was accepted");
                    connected_controllers+=1;
                }
                i+=1;
            }
        }

        input::update_inputs(&mut players,&mut event_pump);

      
        arena_game.run(elapsedtime,&mut canvas,&texture_atlas,&players);
        
        



        if players[0].menu
        {           
            break;
        }
       
    }

}
