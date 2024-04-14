extern crate sdl2;
use sdl2::controller::Button;  
use sdl2::keyboard::Keycode;
use sdl2::controller::Axis;
use sdl2::mouse::MouseButton;
use sdl2::event::Event;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use std::mem::transmute;

#[derive(Copy,Clone)]
pub enum InputTypeWrapper{
    KeyboardButton(Keycode),
    ControllerButton(Button),
    ControllerAxis(Axis,bool),
    MouseButton(MouseButton),
}
#[derive(Copy,Clone)]
pub enum AimInputTypeWrapper{
    MouseAim,
    AxisAim(Axis,Axis),
    FourButtonAim(InputTypeWrapper,InputTypeWrapper,InputTypeWrapper,InputTypeWrapper),
}

pub struct PlayerInputConfig
{
    pub up_button :InputTypeWrapper,
    pub down_button :InputTypeWrapper,
    pub left_button :InputTypeWrapper,
    pub right_button :InputTypeWrapper,
    pub launch_button :InputTypeWrapper,
    pub powerup_button :InputTypeWrapper,
    pub aim_setup : AimInputTypeWrapper,
    pub menu_button : InputTypeWrapper,
}


fn button_from_u8s(draw_from: &mut Vec<u8>) -> Result<InputTypeWrapper,&str> {
   // let but_type :PlayerInputConfig = unsafe {transmute(draw_from[0] as i32)};

    match draw_from[0]
    {
        0 => {
            let to_return = InputTypeWrapper::KeyboardButton(unsafe {transmute(draw_from[1] as i32)});
            draw_from.remove(0);
            draw_from.remove(0);
            return Ok(to_return)
        },
        1 => {
            let to_return = InputTypeWrapper::ControllerButton(unsafe {transmute(draw_from[1] as i32)});
            draw_from.remove(0);
            draw_from.remove(0);
            return Ok(to_return)           
        },
        2 => {
            let to_return = InputTypeWrapper::ControllerAxis(unsafe {transmute(draw_from[1] as i32)},unsafe {transmute(draw_from[2] as u8)});
            draw_from.remove(0);
            draw_from.remove(0);
            draw_from.remove(0);
            return Ok(to_return) 
        },
        3 => {
            let to_return = InputTypeWrapper::MouseButton(unsafe {transmute(draw_from[1] as u8)});
            draw_from.remove(0);
            draw_from.remove(0);
            return Ok(to_return)           
        },
        _ => {return Err("Tried to load unknow input type")},
    }

}

fn aim_setup_from_u8s(draw_from: &mut Vec<u8>) -> Result<AimInputTypeWrapper,&str> {

    match draw_from[0]
    {
        0 => {
            draw_from.remove(0);
            return Ok(AimInputTypeWrapper::MouseAim); 
        },
        1 => {
            let to_return = AimInputTypeWrapper::AxisAim(unsafe {transmute(draw_from[1] as i32)},unsafe {transmute(draw_from[2] as i32)});
            draw_from.remove(0);
            draw_from.remove(0);
            draw_from.remove(0);
            return Ok(to_return)
        },
        2 => {
            draw_from.remove(0);
            let to_return = AimInputTypeWrapper::FourButtonAim(button_from_u8s(draw_from).unwrap(),button_from_u8s(draw_from).unwrap(),button_from_u8s(draw_from).unwrap(),button_from_u8s(draw_from).unwrap());
            return Ok(to_return)
        },
        _ => {return Err("Tried to load unknow input type")},
    }


}

impl PlayerInputConfig {
    pub fn load_from_file(name: String) -> PlayerInputConfig {
        let path = Path::new(".");
        let path = path.join("My_config").join(name);
        
        let mut file = File::open(path).expect("Failed to open file");
        let mut contents : Vec<u8> = Vec::new();      
        file.read_to_end(&mut contents);

        
        let to_return = PlayerInputConfig {
            up_button : button_from_u8s(&mut contents).expect("Failed to load input config"),
            down_button : button_from_u8s(&mut contents).expect("Failed to load input config"),
            left_button : button_from_u8s(&mut contents).expect("Failed to load input config"),
            right_button : button_from_u8s(&mut contents).expect("Failed to load input config"),
            launch_button : button_from_u8s(&mut contents).expect("Failed to load input config"),
            powerup_button : button_from_u8s(&mut contents).expect("Failed to load input config"),
            menu_button : button_from_u8s(&mut contents).expect("Failed to load input config"),
            aim_setup : aim_setup_from_u8s(&mut contents).expect("Failed to load input config"),

        };

        return to_return
    }

    pub fn save_to_file(&self,name: String){
        let path = Path::new(".");
        let path = path.join("My_config").join(name);

        let mut file = File::create(path).expect("Failed to open file");

        let mut to_save : Vec<u8> = Vec::new();

         
        push_input_type_wrapper(&mut to_save,self.up_button);
        push_input_type_wrapper(&mut to_save,self.down_button);
        push_input_type_wrapper(&mut to_save,self.left_button);
        push_input_type_wrapper(&mut to_save,self.right_button);
        push_input_type_wrapper(&mut to_save,self.launch_button);
        push_input_type_wrapper(&mut to_save,self.powerup_button);
        push_input_type_wrapper(&mut to_save,self.menu_button);
        push_aim_input_type_wrapper(&mut to_save,self.aim_setup);
        


        file.write(&to_save);
    }



}

fn push_aim_input_type_wrapper(to_push_to: &mut Vec<u8> ,to_push:AimInputTypeWrapper)
{
    match to_push 
    {
        AimInputTypeWrapper::MouseAim => {
            to_push_to.push(0);
        },
        AimInputTypeWrapper::AxisAim(axis_x,axis_y) => {
            to_push_to.push(1);
            to_push_to.push(axis_x as u8);
            to_push_to.push(axis_y as u8);            
        },
        AimInputTypeWrapper::FourButtonAim(p1,p2,p3,p4) => {
            to_push_to.push(2);
            push_input_type_wrapper(to_push_to,p1);
            push_input_type_wrapper(to_push_to,p2);
            push_input_type_wrapper(to_push_to,p3);
            push_input_type_wrapper(to_push_to,p4);
        },
    }
}

fn push_input_type_wrapper(to_push_to: &mut Vec<u8> ,to_push:InputTypeWrapper)
{
    match to_push 
    {
        InputTypeWrapper::KeyboardButton(key) => {
            to_push_to.push(0);
            to_push_to.push(key as u8);
        }, 
        InputTypeWrapper::ControllerButton(but) => {
            to_push_to.push(1);
            to_push_to.push(but as u8);
        },
        InputTypeWrapper::ControllerAxis(axis,dir) => {
            to_push_to.push(2);
            to_push_to.push(axis as u8);
            to_push_to.push(dir as u8)
        },
        InputTypeWrapper::MouseButton(mb) => {
            to_push_to.push(3);
            to_push_to.push(mb as u8);
        },         
    }
}



pub struct PlayerInput {
   pub config : PlayerInputConfig,
   pub up : i8,
   pub down : i8,
   pub left : i8,
   pub right : i8,
   pub launch : bool,
   pub powerup : bool,
   pub menu : bool,
   pub aimx : f32,
   pub aimy : f32,
   pub which : u32,
   pub mouse_aim : bool
}

impl PlayerInput {

    pub fn new(config:PlayerInputConfig) -> PlayerInput 
    {
        PlayerInput {
            config,
            up : 0,
            down : 0,
            left : 0,
            right : 0,
            launch : false,
            powerup : false,
            menu : false,
            aimx : 0.0,
            aimy : 0.0,
            which : 0,
            mouse_aim : false
        }
    }

}



pub fn update_inputs(players:&mut Vec<PlayerInput>, e_pump: &mut sdl2::EventPump)
{
   
    for event in e_pump.poll_iter()
    {
        match event {
            Event::Quit {..} => { players[0].menu = true;},
            Event::KeyDown {timestamp:_,window_id:_,keycode,scancode:_,keymod:_,repeat} => {                
                if !repeat
                {
                    let mut key_to_toets = Keycode::Greater;
                    match keycode {
                        Some(a) => {
                            key_to_toets =a;
                        },
                        None => {},
                        
                    }

                   let mut i =0;
                   while i < players.len()
                   {                        
                        if let InputTypeWrapper::KeyboardButton(key) = players[i].config.menu_button {
                            if  key == key_to_toets{
                                println!("PLAYER {} : MENU PRESSED",(i+1));
                                players[i].menu = true;
                            }
                        } 
                        if let InputTypeWrapper::KeyboardButton(key) = players[i].config.up_button {
                           if  key == key_to_toets{
                               println!("PLAYER {} : UP PRESSED",(i+1));
                               players[i].up = 1;
                           }
                        }
                        if let InputTypeWrapper::KeyboardButton(key) = players[i].config.down_button {
                            if  key == key_to_toets {
                                println!("PLAYER {} : DOWN PRESSED",(i+1));
                                players[i].down = 1;
                            }
                        }
                        if let InputTypeWrapper::KeyboardButton(key) = players[i].config.left_button {
                            if  key == key_to_toets{
                                println!("PLAYER {} : LEFT PRESSED",(i+1));
                                players[i].left = 1;
                            }
                        }
                        if let InputTypeWrapper::KeyboardButton(key) = players[i].config.right_button {
                            if  key == key_to_toets{
                                 println!("PLAYER {} : RIGHT PRESSED",(i+1));
                                 players[i].right = 1;
                            }
                        }
                        if let InputTypeWrapper::KeyboardButton(key) = players[i].config.launch_button {
                            if  key == key_to_toets {
                                 println!("PLAYER {} : LAUNCH PRESSED",(i+1));
                                 players[i].launch = true;
                            }
                        }
                        if let InputTypeWrapper::KeyboardButton(key) = players[i].config.powerup_button {
                            if  key == key_to_toets {
                                 println!("PLAYER {} : LAUNCH PRESSED",(i+1));
                                 players[i].powerup = true;
                            }
                        }

                        if let AimInputTypeWrapper::FourButtonAim(key_1,key_2,key_3,key_4) = players[i].config.aim_setup {
                            if let InputTypeWrapper::KeyboardButton(key) = key_1 {
                                if  key == key_to_toets {
                                    players[i].aimx -= 1.0;
                                }
                            }
                            if let InputTypeWrapper::KeyboardButton(key) = key_2 {
                                if  key == key_to_toets {
                                    players[i].aimy += 1.0;
                                }                      
                            }
                            if let InputTypeWrapper::KeyboardButton(key) = key_3 {
                                if  key == key_to_toets {
                                    players[i].aimx += 1.0;
                                }                    
                            }
                            if let InputTypeWrapper::KeyboardButton(key) = key_4 {
                                if  key== key_to_toets{
                                    players[i].aimy -= 1.0;
                                }                  
                            }                        
                        }
                        i+=1;
                   }                
                }
            },
            Event::KeyUp {timestamp:_,window_id:_,keycode,scancode:_,keymod:_,repeat:_} => {
                let mut i =0;
                while i < players.len()
                {
                    if let InputTypeWrapper::KeyboardButton(key) = players[i].config.menu_button {
                        if  key == keycode.unwrap() {
                            println!("PLAYER {} : MENU RELEASED",(i+1));
                            players[i].menu = false;
                        }
                    } 
                    if let InputTypeWrapper::KeyboardButton(key) = players[i].config.up_button {
                        if  key == keycode.unwrap() {
                            println!("PLAYER {} : UP RELEASED",(i+1));
                            players[i].up = 0;
                        }
                    }
                    if let InputTypeWrapper::KeyboardButton(key) = players[i].config.down_button {
                         if  key == keycode.unwrap() {
                            println!("PLAYER {} : DOWN RELEASED",(i+1));
                            players[i].down = 0;
                        }
                    }
                    if let InputTypeWrapper::KeyboardButton(key) = players[i].config.left_button {
                        if  key == keycode.unwrap() {
                            println!("PLAYER {} : LEFT RELEASED",(i+1));
                            players[i].left = 0;
                        }
                    }
                    if let InputTypeWrapper::KeyboardButton(key) = players[i].config.right_button {
                        if  key == keycode.unwrap() {
                            println!("PLAYER {} : RIGHT RELEASED",(i+1));
                            players[i].right = 0;
                        }
                    }
                    if let InputTypeWrapper::KeyboardButton(key) = players[i].config.launch_button {
                        if  key == keycode.unwrap() {
                            println!("PLAYER {} : LAUNCH RELEASED",(i+1));
                            players[i].launch = false;
                        }
                    }
                    if let InputTypeWrapper::KeyboardButton(key) = players[i].config.powerup_button {
                        if  key == keycode.unwrap() {
                            println!("PLAYER {} : POWERUP RELEASED",(i+1));
                            players[i].powerup = false;
                        }
                    }
                    if let AimInputTypeWrapper::FourButtonAim(key_1,key_2,key_3,key_4) = players[i].config.aim_setup {
                        if let InputTypeWrapper::KeyboardButton(key) = key_1 {
                            if  key == keycode.unwrap() {
                                players[i].aimx = 0.0;
                            }
                        }
                        if let InputTypeWrapper::KeyboardButton(key) = key_2 {
                            if  key == keycode.unwrap() {
                                players[i].aimy = 0.0;
                            }                      
                        }
                        if let InputTypeWrapper::KeyboardButton(key) = key_3 {
                            if  key == keycode.unwrap() {
                                players[i].aimx = 0.0;
                            }                    
                        }
                        if let InputTypeWrapper::KeyboardButton(key) = key_4 {
                            if  key== keycode.unwrap() {
                                players[i].aimy = 0.0;
                            }                  
                        }                        
                    }
 

               
                 i+=1;
                }        
            },
            Event::ControllerAxisMotion {timestamp:_,which,axis,value} => {
                let mut i = 0;
                while i < players.len()
                {
                    if players[i].which == which
                    {
                        if let InputTypeWrapper::ControllerAxis(player_axis,axis_positive) = players[i].config.up_button {
                            if player_axis == axis 
                            {
                                if !axis_positive && value < -8000 
                                {
                                    println!("PLAYER {} : UP PRESSED",(i+1));
                                    players[i].up = 1;
                                }
                                if axis_positive && value > 8000 
                                {
                                    println!("PLAYER {} : UP PRESSED",(i+1));
                                    players[i].up = 1;
                                }
    
                                if value > -8000 && value < 8000
                                {
                                    println!("PLAYER {} : UP RELEASED",(i+1));
                                    players[i].up = 0;
                                }
                            }                        
                        }
                        
                        if let InputTypeWrapper::ControllerAxis(player_axis,axis_positive) = players[i].config.down_button {
                            if player_axis == axis 
                            {
                            if !axis_positive && value < -8000 
                            {
                                println!("PLAYER {} : DOWN PRESSED",(i+1));
                                players[i].down = 1;
                            }
                            if axis_positive && value > 8000 
                            {
                                println!("PLAYER {} : DOWN PRESSED",(i+1));
                                players[i].down = 1;
                            }

                            if value > -8000 && value < 8000
                            {
                                println!("PLAYER {} : DOWN RELEASED",(i+1));
                                players[i].down = 0;
                            }
                            }
                        }

                        if let InputTypeWrapper::ControllerAxis(player_axis,axis_positive) = players[i].config.left_button {
                            if player_axis == axis 
                            {
                            if !axis_positive && value < -8000 
                            {
                                println!("PLAYER {} : LEFT PRESSED",(i+1));
                                players[i].left = 1;
                            }
                            if axis_positive && value > 8000 
                            {
                                println!("PLAYER {} : LEFT PRESSED",(i+1));
                                players[i].left = 1;
                            }

                            if value > -8000 && value < 8000
                            {
                                println!("PLAYER {} : LEFT RELEASED",(i+1));
                                players[i].left = 0;
                            }
                            }
                        }

                        if let InputTypeWrapper::ControllerAxis(player_axis,axis_positive) = players[i].config.right_button {
                            if player_axis == axis 
                            {
                            if !axis_positive && value < -8000 
                            {
                                println!("PLAYER {} : RIGHT PRESSED",(i+1));
                                players[i].right = 1;
                            }
                            if axis_positive && value > 8000 
                            {
                                println!("PLAYER {} : RIGHT PRESSED",(i+1));
                                players[i].right = 1;
                            }

                            if value > -8000 && value < 8000
                            {
                                println!("PLAYER {} : RIGHT RELEASED",(i+1));
                                players[i].right = 0;
                            }
                            }
                        }

                        if let InputTypeWrapper::ControllerAxis(player_axis,axis_positive) = players[i].config.launch_button {
                            if player_axis == axis 
                            {
                            if !axis_positive && value < -8000 
                            {
                                println!("PLAYER {} : LAUNCH PRESSED",(i+1));
                                players[i].launch = true;
                            }
                            if axis_positive && value > 8000 
                            {
                                println!("PLAYER {} : LAUNCH PRESSED",(i+1));
                                players[i].launch = true;
                            }

                            if value > -8000 && value < 8000
                            {
                                println!("PLAYER {} : LAUNCH RELEASED",(i+1));
                                players[i].launch = false;
                            }
                            }
                        }

                        if let InputTypeWrapper::ControllerAxis(player_axis,axis_positive) = players[i].config.powerup_button {
                            if player_axis == axis 
                            {
                            if !axis_positive && value < -8000 
                            {
                                println!("PLAYER {} : POWERUP PRESSED",(i+1));
                                players[i].powerup = true;
                            }
                            if axis_positive && value > 8000 
                            {
                                println!("PLAYER {} : POWERUP PRESSED",(i+1));
                                players[i].powerup = true;
                            }

                            if value > -8000 && value < 8000
                            {
                                println!("PLAYER {} : POWERUP RELEASED",(i+1));
                                players[i].powerup = false;
                            }
                            }
                        }

                        if let InputTypeWrapper::ControllerAxis(player_axis,axis_positive) = players[i].config.menu_button {
                            if player_axis == axis 
                            {
                            if !axis_positive && value < -8000 
                            {
                                println!("PLAYER {} : MENU PRESSED",(i+1));
                                players[i].menu = true;
                            }
                            if axis_positive && value > 8000 
                            {
                                println!("PLAYER {} : MENU PRESSED",(i+1));
                                players[i].menu = true;
                            }

                            if value > -8000 && value < 8000
                            {
                                println!("PLAYER {} : MENU RELEASED",(i+1));
                                players[i].menu = false;
                            }
                            }
                        }

                        if let AimInputTypeWrapper::AxisAim(x_axis,y_axis) = players[i].config.aim_setup {
                            if x_axis == axis 
                            {
                                players[i].aimx = value as f32;
                            }
                            if y_axis == axis 
                            {
                                players[i].aimy = value as f32;
                            }
                        }



                    }

                    
                    i+=1;
                }
            }, 
            Event::ControllerButtonDown {timestamp:_,which,button} => {
                let mut i =0;
                while i < players.len()
                {
                    if which == players[i].which
                    {
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.menu_button {
                            if  but == button {
                                println!("PLAYER {} : MENU PRESSED",(i+1));
                                players[i].menu = true;
                            }
                        }                          
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.up_button {
                            if  but == button {
                                println!("PLAYER {} : UP PRESSED",(i+1));
                                players[i].up = 1;
                            }
                        }
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.down_button {
                            if  but == button {
                                println!("PLAYER {} : DOWN PRESSED",(i+1));
                                players[i].down = 1;
                            }
                        }
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.left_button {
                            if  but == button {
                                println!("PLAYER {} : LEFT PRESSED",(i+1));
                                players[i].left = 1;
                            }
                        }
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.right_button {
                            if  but == button {
                                println!("PLAYER {} : RIGHT PRESSED",(i+1));
                                players[i].right = 1;
                            }
                        }
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.launch_button {
                            if  but == button {
                                println!("PLAYER {} : LAUNCH PRESSED",(i+1));
                                players[i].launch = true;
                            }
                        } 
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.powerup_button {
                            if  but == button {
                                println!("PLAYER {} : POWERUP PRESSED",(i+1));
                                players[i].powerup = true;
                            }
                        }

                        if let AimInputTypeWrapper::FourButtonAim(but_1,but_2,but_3,but_4) = players[i].config.aim_setup {
                            if let InputTypeWrapper::ControllerButton(but) = but_1 {
                                if  but == button{
                                    players[i].aimx -= 1.0;
                                }
                            }
                            if let InputTypeWrapper::ControllerButton(but) = but_2 {
                                if  but == button{
                                    players[i].aimy += 1.0;
                                }                      
                            }
                            if let InputTypeWrapper::ControllerButton(but) = but_3 {
                                if  but == button {
                                    players[i].aimx += 1.0;
                                }                    
                            }
                            if let InputTypeWrapper::ControllerButton(but) = but_4 {
                                if  but== button{
                                    players[i].aimy -= 1.0;
                                }                  
                            }                        
                        }   
                    }                
                    i+=1;
                }  
            }, 
            Event::ControllerButtonUp {timestamp:_,which,button} => {      


                let mut i =0;
                while i < players.len()
                {
                    if which == players[i].which
                    {
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.menu_button {
                            if  but == button {
                                println!("PLAYER {} : MENU RELEASED",(i+1));
                                players[i].menu = false;
                            }
                        }  
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.up_button {
                            if  but == button {
                                println!("PLAYER {} : UP RELEASED",(i+1));
                                players[i].up = 1;
                            }
                        }
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.down_button {
                            if  but == button {
                                println!("PLAYER {} : DOWN RELEASED",(i+1));
                                players[i].down = 1;
                            }
                        }
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.left_button {
                            if  but == button {
                                println!("PLAYER {} : LEFT RELEASED",(i+1));
                                players[i].left = 1;
                            }
                        }
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.right_button {
                            if  but == button {
                                println!("PLAYER {} : RIGHT RELEASED",(i+1));
                                players[i].right = 1;
                            }
                        } 
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.launch_button {
                            if  but == button {
                                println!("PLAYER {} : LAUNCH RELEASED",(i+1));
                                players[i].launch = false;
                            }
                        } 
                        if let InputTypeWrapper::ControllerButton(but) = players[i].config.powerup_button {
                            if  but == button {
                                println!("PLAYER {} : POWERUP RELEASED",(i+1));
                                players[i].powerup = false;
                            }
                        }   
                        
                        if let AimInputTypeWrapper::FourButtonAim(but_1,but_2,but_3,but_4) = players[i].config.aim_setup {
                            if let InputTypeWrapper::ControllerButton(but) = but_1 {
                                if  but == button{
                                    players[i].aimx = 0.0;
                                }
                            }
                            if let InputTypeWrapper::ControllerButton(but) = but_2 {
                                if  but == button{
                                    players[i].aimy = 0.0;
                                }                      
                            }
                            if let InputTypeWrapper::ControllerButton(but) = but_3 {
                                if  but == button {
                                    players[i].aimx = 0.0;
                                }                    
                            }
                            if let InputTypeWrapper::ControllerButton(but) = but_4 {
                                if  but== button{
                                    players[i].aimy = 0.0;
                                }                  
                            }                        
                        }
                    
                    } 
                    i+=1;
                }  


            },    
            Event::MouseButtonDown {timestamp:_,window_id:_,which:_,mouse_btn,clicks:_,x:_,y:_} =>  {


                let mut i =0;
                while i < players.len()
                {
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.menu_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : MENU PRESSED",(i+1));
                            players[i].menu = true;
                        }
                    } 
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.up_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : UP PRESSED",(i+1));
                            players[i].up = 1;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.down_button {
                         if  button == mouse_btn {
                            println!("PLAYER {} : DOWN PRESSED",(i+1));
                            players[i].down = 1;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.left_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : LEFT PRESSED",(i+1));
                            players[i].left = 1;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.right_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : RIGHT PRESSED",(i+1));
                            players[i].right = 1;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.launch_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : LAUNCH PRESSED",(i+1));
                            players[i].launch = true;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.powerup_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : POWERUP PRESSED",(i+1));
                            players[i].powerup = true;
                        }
                    }

                    if let AimInputTypeWrapper::FourButtonAim(but_1,but_2,but_3,but_4) = players[i].config.aim_setup {
                        if let InputTypeWrapper::MouseButton(button)= but_1 {
                            if  mouse_btn == button{
                                players[i].aimx -= 1.0;
                            }
                        }
                        if let InputTypeWrapper::MouseButton(button) = but_2 {
                            if  mouse_btn == button{
                                players[i].aimy += 1.0;
                            }                      
                        }
                        if let InputTypeWrapper::MouseButton(button) = but_3 {
                            if  mouse_btn == button {
                                players[i].aimx += 1.0;
                            }                    
                        }
                        if let InputTypeWrapper::MouseButton(button) = but_4 {
                            if  mouse_btn == button{
                                players[i].aimy -= 1.0;
                            }                  
                        }                        
                    }   
   

                 i+=1;
                }     
            },
            Event::MouseButtonUp {timestamp:_,window_id:_,which:_,mouse_btn,clicks:_,x:_,y:_} =>  {
                let mut i =0;
                while i < players.len()
                {
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.menu_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : MENU RELEASED",(i+1));
                            players[i].menu = false;
                        }
                    } 
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.up_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : UP RELEASED",(i+1));
                            players[i].up = 0;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.down_button {
                         if  button == mouse_btn {
                            println!("PLAYER {} : DOWN RELEASED",(i+1));
                            players[i].down = 0;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.left_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : LEFT RELEASED",(i+1));
                            players[i].left = 0;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.right_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : RIGHT RELEASED",(i+1));
                            players[i].right = 0;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.launch_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : LAUNCH RELEASED",(i+1));
                            players[i].launch = false;
                        }
                    }
                    if let InputTypeWrapper::MouseButton(button) = players[i].config.powerup_button {
                        if  button == mouse_btn {
                            println!("PLAYER {} : POWERUP RELEASED",(i+1));
                            players[i].powerup = false;
                        }
                    }

                    if let AimInputTypeWrapper::FourButtonAim(but_1,but_2,but_3,but_4) = players[i].config.aim_setup {
                        if let InputTypeWrapper::MouseButton(button) = but_1 {
                            if  mouse_btn == button{
                                players[i].aimx = 0.0;
                            }
                        }
                        if let InputTypeWrapper::MouseButton(button) = but_2 {
                            if  mouse_btn == button{
                                players[i].aimy = 0.0;
                            }                      
                        }
                        if let InputTypeWrapper::MouseButton(button) = but_3 {
                            if  mouse_btn == button {
                                players[i].aimx = 0.0;
                            }                    
                        }
                        if let InputTypeWrapper::MouseButton(button)= but_4 {
                            if  mouse_btn== button{
                                players[i].aimy = 0.0;
                            }                  
                        }                        
                    }

                 i+=1;
                }     
            },
            Event::MouseMotion {timestamp:_,window_id:_,which:_,mousestate:_,x,y,xrel:_,yrel:_} => {
                let mut i = 0;
                while i < players.len()
                {
                    if let AimInputTypeWrapper::MouseAim = players[i].config.aim_setup {
                        players[i].aimx = x as f32; 
                        players[i].aimy = y as f32; 
                        if !players[i].mouse_aim { players[i].mouse_aim = true}
                    }
                    i += 1;
                }              
            },
            _ => {},
        }
    }
}