use console_engine::{ConsoleEngine, KeyCode, pixel};

const SNOW_FLAKE_SYMB: char = '❄';
const SCREEN_SPACE_AMOUNT: u32 = 50;

/// custom function for generating a random u32 bound into [0;max]
fn random(max: u32) -> u32 {
    rand::random::<u32>() % max
}

#[derive(Copy, Clone)]
struct Point {
    pub x: i32,
    pub y: u32,
    pub speed_x: i32,
    pub speed_y: u32
}

impl Point {
    fn new(x: i32, y: u32) -> Self {
        Point { x, y, speed_x: 1, speed_y: random(2)}
    }
}

struct SnowField {
    snowflakes: Vec<Point>,
    w_bound: u32,
    h_bound: u32,
    snowflakes_num: u32
}

impl SnowField {
    fn init(height: u32, width: u32) -> SnowField {
        let snowflakes_num = height * width / SCREEN_SPACE_AMOUNT;
        SnowField { snowflakes: (vec![]), h_bound: height, w_bound: width, snowflakes_num }
    }

    fn draw(&mut self, engine: &mut ConsoleEngine) -> () {
        self.snowflakes.iter().for_each(|snowflake| {
            engine.set_pxl(snowflake.x as i32, snowflake.y as i32,
                 pixel::pxl(SNOW_FLAKE_SYMB));
        });
    }

    fn update(&mut self) {
        let mut updated_positions = vec![];
        for i in 0..self.snowflakes.len() {      
            let mut snowflake = self.snowflakes[i];      
            // move snowflakes
            snowflake.speed_x = random(3) as i32 - random(3) as i32;
            snowflake.speed_y = random(3);
            snowflake.x += snowflake.speed_x;
            if snowflake.x < 0 {
                snowflake.x = self.w_bound as i32 - snowflake.x;
            }
            snowflake.y += snowflake.speed_y;
            // remove out of bounds
            if snowflake.x <= self.w_bound as i32 && snowflake.y <= self.h_bound {
                updated_positions.push(snowflake);
            }
        }
        for _ in 0..random(self.snowflakes_num - updated_positions.len() as u32) {
            updated_positions.push(Point::new(random(self.w_bound) as i32, 0));
        }
        self.snowflakes = updated_positions;
    }
}

fn main() {
    // initializes a screen filling the terminal of at least 10x10 of size with a target of 4 frame per second
    let mut engine = ConsoleEngine::init_fill_require(10, 10, 5).expect("Terminal screen is too small");
    let mut snowfield = SnowField::init(engine.get_height(), engine.get_width());
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
                             // engine.check_resize(); here we do not want to resize the terminal because it could break the boundaries of the game

        // exit check
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        engine.clear_screen(); // reset the screen


        snowfield.update();
        snowfield.draw(&mut engine);
        engine.draw();
    }
} 