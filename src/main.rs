
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use std::time::Duration;


pub mod display;
pub mod files;
pub mod game;



const GRID_X: u32 = 80;
const GRID_Y: u32 = 60;

const BLOCK_SIZE: u32 = 10;

pub struct Renderer {

    pub canvas: WindowCanvas,
}

impl Renderer {

    pub fn new(window: Window) -> Result<Renderer, String>{

        
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok( Renderer { canvas })
    }

    pub fn draw_block(&mut self, pos: &game::Point) -> Result<(), String> {
        
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        self.canvas.fill_rect(Rect::new(
            pos.x * BLOCK_SIZE as i32, 
            pos.y * BLOCK_SIZE as i32,  
            BLOCK_SIZE, 
            BLOCK_SIZE,
        ))?; 

        self.canvas.present();
        
        Ok(())
    }


    pub fn setup(&mut self) -> Result<(), String> {

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();

        Ok(())
    }

}

fn main() -> Result<(), String> {


    

    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Game of life", GRID_X * BLOCK_SIZE, GRID_Y * BLOCK_SIZE)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;


        let mut event_pump = sdl_context.event_pump()?;

    
    let mut render = Renderer::new(window)?;

    render.setup()?;

    render.draw_block(&game::Point { x: 10, y: 10 })?; 




    files::clear();


    let mut blocks: Vec<Vec<bool>> = Vec::new();

    for i in 0..20 {
        blocks.push(Vec::new());

        for _j in 0..20 {
            blocks[i as usize].push(false);
        }
    }

    let mut alive: Vec<game::Block> = Vec::new();


    
    
    let mut rounds = 0;
    

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // canvas.clear();
        // canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}

fn start_setup(mut alive: Vec<game::Block>, mut blocks: Vec<Vec<bool>>) {

    let (mut alive, mut blocks) = game::place_block(game::Point{x:25,y:25}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:25,y:26}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:25,y:27}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:27,y:24}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:27,y:28}, alive, blocks);
    
    let (mut alive, mut blocks) = game::place_block(game::Point{x:28,y:24}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:28,y:28}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:30,y:25}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:30,y:26}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:30,y:27}, alive, blocks);

    let (mut alive, mut blocks) = game::place_block(game::Point{x:33,y:25}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:33,y:26}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:33,y:27}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:35,y:24}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:35,y:28}, alive, blocks);
    
    let (mut alive, mut blocks) = game::place_block(game::Point{x:36,y:24}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:36,y:28}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:38,y:25}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:38,y:26}, alive, blocks);
    let (mut alive, mut blocks) = game::place_block(game::Point{x:38,y:27}, alive, blocks);
 


}


