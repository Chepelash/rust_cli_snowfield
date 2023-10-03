use std::{thread, time::Duration};

use console_engine::Color;
use console_engine::screen::Screen;
use console_engine::pixel::{self, Pixel};

const SNOW_FLAKE: char = '❄';

struct Point {
    pub x: i32,
    pub y: i32
}

fn main() {
    // create a screen of 20x11 characters
    let mut scr = Screen::new(20,11);
    let mut snow_flakes = vec![Point{x: 3, y: 0}, Point{x: 6, y: 0}];
    loop {

        scr.clear();
        scr.rect(0,0, 19,10,pixel::pxl('#'));
        for snow_flake in snow_flakes.iter_mut() {            
            snow_flake.y = (snow_flake.y + 1) % 11;
            scr.set_pxl(snow_flake.x, snow_flake.y, Pixel { bg: Color::Reset,
                fg: Color::White, chr: SNOW_FLAKE });
        }
        thread::sleep(Duration::from_secs(2));
        scr.draw();        

    }    
} 