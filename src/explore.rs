use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub fn gameloop(elapsedtime:f32,screen_width:u32,screen_height:u32,canvas: &mut Canvas<Window>)
{


    canvas.set_draw_color(Color::RGB(0,0,255));
    canvas.clear();
    canvas.present();

}