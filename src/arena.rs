use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use std::mem::transmute;
use std::f64::consts::PI;
use crate::input::PlayerInput;



#[derive(Copy,Clone)]
enum TileDrawType {
    UpCorner,
    HorisontalSide,
    VerticalSide,
    Floor,
    DownCorner,
    RiverSide,
    RiverInterior,
    Spwan,
    VerticalSideLit
}

#[derive(Copy,Clone)]
#[derive(PartialEq)]
enum TileEffect {
    None,
    BlockPlayersNotRedDisk,
    BlockPlayersNotBlueDisk,
    BlockPlayers    
}

struct Tile {
    tile_draw_type : TileDrawType,
    tile_effect : TileEffect,
    effect_count : u32,
    w : u32,
    h : u32,
    x : i32,
    y : i32
}

#[derive(PartialEq)]
pub enum PovTeams {
    Blue,
    Red
}

pub enum CollosionType
{
    Stop,
    Bounce,
    DeadStop,
    Check
}

#[derive(Copy,Clone)]
pub struct PhysicsObject 
{
    x : f32,
    y : f32,
    velx : f32,
    vely : f32,
    w: f32,
    h: f32,
    speed : f32
}

impl PhysicsObject 
{
    pub fn move_final(&mut self,elapsedtime:f32)
    {
        self.x += self.velx*elapsedtime;
        self.y += self.vely*elapsedtime;
    }
    pub fn collision_with_tile_map(&mut self,tilemap:&Vec<Tile>,elapsedtime:f32,tile_effect:TileEffect,collision_type:CollosionType) -> bool
    {
        for tile in tilemap
        {
            if tile_effect == tile.tile_effect
            {
                let tile_x = tile.x as f32;
                let tile_y = tile.y as f32;
                let tile_w = tile.w as f32;
                let tile_h = tile.h as f32;
                let check_x = self.x +self.velx*elapsedtime;
                let check_y = self.y +self.vely*elapsedtime;
               
                if tile_x <= check_x +self.w && tile_x+tile_w >= check_x && tile_y <= check_y +self.h && tile_y+tile_h >= check_y
                {     
                    let xoffset;
                    let xoffset_1 = check_x - (tile_x+tile_w);  
                    let xoffset_2 = (check_x+self.w) - (tile_x); 
                    if xoffset_1.abs() < xoffset_2.abs()
                    {
                        xoffset =xoffset_1;
                    }  else {xoffset =xoffset_2;}               
                    let xcorrection = self.velx*elapsedtime -xoffset;
                
                    let yoffset;
                    let yoffset_1 = check_y - (tile_y+tile_h);  
                    let yoffset_2 = (check_y+self.h) - (tile_y); 
                    if yoffset_1.abs() < yoffset_2.abs()
                    {
                        yoffset =yoffset_1;
                    }  else {yoffset =yoffset_2;}               
                    let ycorrection = self.vely*elapsedtime -yoffset;
          
                    match collision_type 
                    {
           
                        CollosionType::Stop => {collision_stop_behaviour(xcorrection,ycorrection,self,elapsedtime);},
                        CollosionType::DeadStop => {collision_stop_behaviour(xcorrection,ycorrection,self,elapsedtime);
                        self.velx = 0.0;
                        self.vely  =0.0; },
                        CollosionType::Bounce => {collision_bounce_behaviour(xcorrection,ycorrection,self); }
                        CollosionType::Check => {return true}
                    }
                }
            }


        }
        return false

       
    }
    pub fn collision_with_physics_object(&mut self,other: PhysicsObject,elapsedtime:f32,collision_type:CollosionType) -> bool
    {
        let check_x = self.x +self.velx*elapsedtime;
        let check_y = self.y +self.vely*elapsedtime;
       
        if other.x <= check_x +self.w && other.x+other.w >= check_x && other.y <= check_y +self.h && other.y+other.h >= check_y
        {     
            let xoffset;
            let xoffset_1 = check_x - (other.x+other.w);  
            let xoffset_2 = (check_x+self.w) - (other.x); 
            if xoffset_1.abs() < xoffset_2.abs()
            {
                xoffset =xoffset_1;
            }  else {xoffset =xoffset_2;}               
            let xcorrection = self.velx*elapsedtime -xoffset;
        
            let yoffset;
            let yoffset_1 = check_y - (other.y+other.h);  
            let yoffset_2 = (check_y+self.h) - (other.y); 
            if yoffset_1.abs() < yoffset_2.abs()
            {
                yoffset =yoffset_1;
            }  else {yoffset =yoffset_2;}               
            let ycorrection = self.vely*elapsedtime -yoffset;
            match collision_type 
            {
   
                CollosionType::Stop => {collision_stop_behaviour(xcorrection,ycorrection,self,elapsedtime);},
                CollosionType::DeadStop => {collision_stop_behaviour(xcorrection,ycorrection,self,elapsedtime);
                self.velx = 0.0;
                self.vely  =0.0; },
                CollosionType::Bounce => {collision_bounce_behaviour(xcorrection,ycorrection,self); }
                CollosionType::Check => {return true}
            }
        }
        return false
    }

}

fn collision_bounce_behaviour(xcorrection: f32,ycorrection :f32,object: &mut PhysicsObject)
{
   
    
    if ycorrection.abs()< xcorrection.abs(){
        object.vely = -object.vely;
    }
    else{
        object.velx = -object.velx;
    }
}

fn collision_stop_behaviour(xcorrection: f32,ycorrection :f32,object: &mut PhysicsObject,elapsedtime:f32)
{

    

    if ycorrection.abs()< xcorrection.abs(){
       

        if object.vely > 0.0 
        {
            object.y += ycorrection.abs();
        }
        else
        {
            object.y -= ycorrection.abs(); 
        }
        object.vely =0.0;
        
    }
    else
    {                        
        if object.velx > 0.0 
        {
            object.x += xcorrection.abs();
        }
        else
        {
            object.x -= xcorrection.abs(); 
        }
        object.velx =0.0;
    }
} 


pub struct Pov {
    hitbox : PhysicsObject,
    disk : PhysicsObject,
    team : PovTeams,
    hasdisk : bool,
    diskangle : f32,
    gems : u8
}

impl Pov {
    pub fn new(x:f32,y:f32,team:PovTeams)->Pov{
        Pov {
            hitbox  : PhysicsObject {x,y,velx:0.0,vely:0.0,w:64.0,h:64.0,speed : 500.0},
            disk  : PhysicsObject {x:0.0,y:0.0,velx:0.0,vely:0.0,w:32.0,h:32.0,speed:1000.0},
            team,
            hasdisk : true,
            diskangle : 0.0,
            gems : 3
        }
    }
}


pub struct Game {
    screen_width : u32,
    screen_height : u32,  
    tile_map : Vec<Tile>, 
    povs : Vec<Pov>,
    accumelated_time : f32,
    b_score : u8,
    r_score : u8
}


fn save_tile_map(name:String,tilemap_to_save:&Vec<Tile>)
{
   
    let path = Path::new(".");
    let path = path.join("Maps").join(name);

    let mut file = File::create(path).expect("Failed to open file");

    let mut to_save : Vec<u8> = Vec::new();

    for tile in tilemap_to_save
    {
        to_save.push(tile.tile_draw_type as u8);
        to_save.push(tile.tile_effect as u8);
        to_save.push((tile.x/32)as u8);
        to_save.push((tile.y/32)as u8);
    } 
        


    file.write(&to_save);
    
}

fn load_tile_map(name:String) -> Vec<Tile>
{
    let path = Path::new(".");
    let path = path.join("Maps").join(name);
        
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents : Vec<u8> = Vec::new();      
    file.read_to_end(&mut contents);

    let mut to_return = Vec::new();
    let mut i =0;
    while i < contents.len()
    {
      
        to_return.push(Tile{
            tile_draw_type: unsafe {transmute(contents[i] as u8)},
            tile_effect: unsafe {transmute(contents[i+1] as u8)},
            x : contents[i+2]as i32*32,
            y : contents[i+3]as i32*32,
            w : 32,
            h : 32,
            effect_count :0
        });
        i+=4
    }
    return to_return

}

fn distanice_between_points_f32(x1:f32,x2:f32,y1:f32,y2:f32) -> f32
{
    return ((x1-x2)*(x1-x2)+(y1-y2)*(y1-y2)).sqrt()
}


impl Game {
    pub fn new(screen_width : u32,screen_height :u32) -> Game 
    {
        let povs = vec![Pov::new(33.0,33.0,PovTeams::Blue),Pov::new(800.0,33.0,PovTeams::Red)];
        let mut tile_map=load_tile_map("Map_1.map".to_string());


        
       // save_tile_map("Map_1.map".to_string(),&tile_map);
        

        Game {screen_width,screen_height,tile_map,povs,accumelated_time:0.0,b_score:0,r_score:0}
    }


    pub fn run(&mut self,elapsedtime:f32,canvas:&mut Canvas<Window>,texture_atlas:&Texture,inputs:&Vec<PlayerInput>)
    {
        let gui_space =2.5*32.0;
        //Set input
        let mut i = 0;
        for pov in &mut self.povs
        {
            let tempx = (inputs[i].right-inputs[i].left) as f32;
            let tempy = (inputs[i].down-inputs[i].up)as f32;
            
            if tempx != 0.0 || tempy != 0.0 
            {
                pov.hitbox.velx = (tempx/distanice_between_points_f32(0.0,tempx,0.0,tempy))*pov.hitbox.speed;
                pov.hitbox.vely = (tempy/distanice_between_points_f32(0.0,tempx,0.0,tempy))*pov.hitbox.speed;
            }
            else 
            {
                pov.hitbox.velx =0.0; 
                pov.hitbox.vely =0.0; 
                
            }

           

            if pov.hasdisk 
            {
                let mut temp_dir_x = 0.0;
                let mut temp_dir_y = 0.0; 
                
                
                if !inputs[i].mouse_aim
                {
                    if (inputs[i].aimx != 0.0 || inputs[i].aimy != 0.0) {
                        temp_dir_x = inputs[i].aimx/distanice_between_points_f32(0.0,inputs[i].aimx,0.0,inputs[i].aimy);
                        temp_dir_y = inputs[i].aimy/distanice_between_points_f32(0.0,inputs[i].aimx,0.0,inputs[i].aimy);  
                    }  
                }
                else
                {
                    let tempx = inputs[i].aimx-(pov.hitbox.x+pov.hitbox.w/2.0);
                    let tempy = inputs[i].aimy-(pov.hitbox.y+ pov.hitbox.h/2.0+gui_space);


                    temp_dir_x = tempx/distanice_between_points_f32(0.0,tempx,0.0,tempy);
                    temp_dir_y = tempy/distanice_between_points_f32(0.0,tempx,0.0,tempy);  

                   
                }
                 
                 

                if temp_dir_x > 0.0 && temp_dir_y > 0.0
                {
                    pov.diskangle = (temp_dir_y/temp_dir_x).abs().atan();
                }
                else if temp_dir_x < 0.0 && temp_dir_y < 0.0 
                {
                    pov.diskangle = PI as f32+(temp_dir_y/temp_dir_x).abs().atan();
                }
                else if temp_dir_x < 0.0 && temp_dir_y > 0.0 
                {
                    pov.diskangle = PI as f32-(temp_dir_y/temp_dir_x).abs().atan();
                }
                else if temp_dir_x > 0.0 && temp_dir_y < 0.0 
                {
                    pov.diskangle = -(temp_dir_y/temp_dir_x).abs().atan();
                }
                else if temp_dir_x == 1.0
                {
                    pov.diskangle = 0.0;
                }
                else if temp_dir_x == -1.0
                {
                    pov.diskangle = PI as f32;
                }
                else if temp_dir_y == 1.0
                {
                    pov.diskangle = PI as f32/2.0;
                }
                else if temp_dir_y == -1.0
                {
                    pov.diskangle = -PI as f32/2.0;
                }

 

                pov.disk.velx = temp_dir_x*pov.disk.speed;
                pov.disk.vely = temp_dir_y*pov.disk.speed;
            }
       




            if inputs[i].launch && pov.hasdisk 
            {
                pov.hasdisk = false;
            }
            


            i+=1;     
        }


        //Process
        let mut i = 0;
        
        for pov in &mut self.povs
        {
            if pov.gems ==0 
            {
                pov.gems = 3;
                match pov.team {
                    PovTeams::Blue => {pov.hitbox.x = 32.0},
                    PovTeams::Red => {pov.hitbox.x = 37.0*32.0},
                }

            }
            

            if !pov.hitbox.collision_with_tile_map(&self.tile_map,elapsedtime,TileEffect::BlockPlayersNotRedDisk,CollosionType::Stop) 
            && !pov.hitbox.collision_with_tile_map(&self.tile_map,elapsedtime,TileEffect::BlockPlayersNotBlueDisk,CollosionType::Stop)
            && !pov.hitbox.collision_with_tile_map(&self.tile_map,elapsedtime,TileEffect::BlockPlayers,CollosionType::Stop)
            {
                pov.hitbox.move_final(elapsedtime);
            }

            if pov.hasdisk
            {
                if pov.team == PovTeams::Blue
                {
                    pov.disk.x = pov.hitbox.x+pov.hitbox.w;
                    pov.disk.y = pov.hitbox.y+pov.hitbox.h/4.0;
                }
                else{
                    pov.disk.x = pov.hitbox.x-pov.disk.w;
                    pov.disk.y = pov.hitbox.y+pov.hitbox.h/4.0;
                }
            }
            else
            {
                if pov.team == PovTeams::Blue
                {
                    if !pov.disk.collision_with_tile_map(&self.tile_map,elapsedtime,TileEffect::BlockPlayersNotRedDisk,CollosionType::DeadStop) 
                    && !pov.disk.collision_with_tile_map(&self.tile_map,elapsedtime,TileEffect::BlockPlayersNotBlueDisk,CollosionType::Bounce)
                    {
                        pov.disk.move_final(elapsedtime);
                    }
                   
                }
                else{
                    if !pov.disk.collision_with_tile_map(&self.tile_map,elapsedtime,TileEffect::BlockPlayersNotRedDisk,CollosionType::Bounce) 
                    && !pov.disk.collision_with_tile_map(&self.tile_map,elapsedtime,TileEffect::BlockPlayersNotBlueDisk,CollosionType::DeadStop)
                    {
                        pov.disk.move_final(elapsedtime);
                    }
                }
                
                if pov.hitbox.collision_with_physics_object(pov.disk,elapsedtime,CollosionType::Check) &&pov.disk.velx == 0.0 && pov.disk.vely ==0.0
                {
                    pov.hasdisk = true;
                }               
            }
        }
      
        let mut  i = 0;
        let mut k;
        while i < self.povs.len()
        {
            k =0;
            while k < self.povs.len()
            {                
                    let temp =self.povs[k].hitbox.clone();
                    if self.povs[i].disk.collision_with_physics_object(temp,elapsedtime,CollosionType::Check) 
                    {
                      
                        if self.povs[i].team != self.povs[k].team
                        {
                            self.povs[i].hasdisk = true;
                            self.povs[k].hasdisk = true;
                            self.povs[i].disk.x = self.povs[i].hitbox.x;
    
                            self.povs[k].gems -= 1;
                            
                            if self.povs[i].team ==PovTeams::Red
                            {
                                self.r_score +=1;
                            }
                            else
                            {
                                self.b_score +=1; 
                            }
                        }

                      


                    }
                            
                k+=1;
            }
            i+=1;
        }

        


        //Render
        let drawscale = 1280.0/self.screen_width as f32;
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        
        for mut tile in &mut self.tile_map 
        {
            let mut i =0;
            while i < self.r_score
            {
                if tile.x == 39*32 && tile.y == (self.r_score as i32)*32
                {
                    tile.tile_draw_type = TileDrawType::VerticalSideLit;
                }
                i+=1;
            }
            let mut i =0;
            while i < self.b_score
            {
                if tile.x == 0 && tile.y == (self.b_score as i32)*32
                {
                    tile.tile_draw_type = TileDrawType::VerticalSideLit;
                }
                i+=1;
            }



            let w = (tile.w as f32)*drawscale;
            let h = (tile.h as f32)*drawscale;
            let x = (tile.x as f32)*drawscale;
            let y = (gui_space+(tile.y as f32))*drawscale;
            let draw_offset = (tile.tile_draw_type as i32)*8;
            let mut hflip = false;
            let mut vflip = false;
            if tile.x >= 640 
            {
                hflip = true;
            }        
            canvas.copy_ex(&texture_atlas,Rect::new(0,draw_offset,8,8),Rect::new(x as i32,y as i32,w as u32,h as u32),0.0,None,hflip,vflip).unwrap();  
               
        }

       

        for pov in &self.povs
        {
            let w = (pov.hitbox.w as f32)*drawscale;
            let h = (pov.hitbox.h as f32)*drawscale;
            let x = (pov.hitbox.x as f32)*drawscale;
            let y = (gui_space+(pov.hitbox.y as f32))*drawscale;
            let mut hflip =  pov.team == PovTeams::Blue ;
            
            canvas.copy_ex(&texture_atlas,Rect::new(0,712,8,8),Rect::new(x as i32,y as i32,w as u32,h as u32),0.0,None,hflip,false).unwrap();


            let w = (pov.disk.w as f32)*drawscale;
            let h = (pov.disk.h as f32)*drawscale;
            let x = (pov.disk.x as f32)*drawscale;
            let y = (gui_space+(pov.disk.y as f32))*drawscale;
            let angle = (pov.diskangle as f64)*(180.0/PI);

            canvas.copy_ex(&texture_atlas,Rect::new(0,696,8,8),Rect::new(x as i32,y as i32,w as u32,h as u32),45.0+angle,None,false,false).unwrap();


            let w = (16 as f32)*drawscale;
            let h = (16 as f32)*drawscale;
            let x = (pov.hitbox.x +pov.hitbox.w/2.0)*drawscale;
            let y = (gui_space+(pov.hitbox.y  +pov.hitbox.h/2.0))*drawscale;
            self.accumelated_time += elapsedtime;
            let mut i = 0;
            while i < pov.gems 
            {
                let offset = (2.0*PI as f32)/pov.gems as f32;

                let x = x + (self.accumelated_time as f32 +offset*i as f32).sin()*64.0;
                let y = y + (self.accumelated_time as f32 +offset*i as f32).cos()*64.0;
                canvas.copy_ex(&texture_atlas,Rect::new(0,688,8,8),Rect::new(x as i32,y as i32,w as u32,h as u32),0.0,None,false,false).unwrap();
                i+=1;
            }


        }

        canvas.present();
    }
}

