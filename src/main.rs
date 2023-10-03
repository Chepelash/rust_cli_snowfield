use console_engine::{ConsoleEngine, KeyCode, pixel};

const SNOW_FLAKE_SYMB: char = '❄';
const NUMBER_OF_SNOWFLAKES: u8 = 5;
const FALLING_SPEED: u8 = 1;

/// custom function for generating a random u32 bound into [0;max]
fn random(max: u32) -> u32 {
    rand::random::<u32>() % max
}

struct Point {
    pub x: u32,
    pub y: u32
}

struct SnowField {
    snowflakes: Vec<Point>,
    falling_speed: u8,
    _terminal_width: u32,
    _terminal_height: u32
}

impl SnowField {
    fn init(height: u32, width: u32) -> SnowField {
        let snowflakes: Vec<Point> = (0..NUMBER_OF_SNOWFLAKES).into_iter()
        .map(|_| Point{x: random(width), y: 0 as u32}).collect();
        SnowField { snowflakes: (snowflakes), falling_speed: (FALLING_SPEED), _terminal_height: height, _terminal_width: (width) }
    }

    fn draw(&mut self, engine: &mut ConsoleEngine) -> () {
        self.snowflakes.iter_mut().for_each(|snow_flake| {
            snow_flake.y += self.falling_speed as u32;
            if snow_flake.y >= self._terminal_height {
                snow_flake.y = 0;
            }
            engine.set_pxl(snow_flake.x as i32, snow_flake.y as i32, pixel::pxl(SNOW_FLAKE_SYMB));
        });        
    }

    
}

fn main() {
    // initializes a screen filling the terminal of at least 10x10 of size with a target of 4 frame per second
    let mut engine = ConsoleEngine::init_fill_require(10, 10, 30).expect("Terminal screen is too small");
    let mut snowfield = SnowField::init(engine.get_height(), engine.get_width());
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
                             // engine.check_resize(); here we do not want to resize the terminal because it could break the boundaries of the game

        // exit check
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        engine.clear_screen(); // reset the screen


        snowfield.draw(&mut engine);
        engine.draw();
    }
} 