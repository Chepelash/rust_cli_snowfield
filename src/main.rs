use console_engine::{pixel, ConsoleEngine, KeyCode};

const SNOW_FLAKE_SYMB: char = '❄';
const SCREEN_SPACE_AMOUNT: u32 = 50;

/// custom function for generating a random u32 bound into [0;max]
fn random(max: u32) -> u32 {
    rand::random::<u32>() % max
}

struct Point {
    pub x: i32,
    pub y: u32,
    pub speed_x: i32,
    pub speed_y: u32,
}

impl Point {
    fn new(x: i32, y: u32) -> Self {
        Point {
            x,
            y,
            speed_x: 1,
            speed_y: random(2),
        }
    }
}

struct SnowField {
    snowflakes: Vec<Option<Point>>,
    w_bound: u32,
    h_bound: u32,
    snowflakes_num: u32,
    max_snowflakes: u32,
}

impl SnowField {
    fn init(height: u32, width: u32) -> SnowField {
        let snowflakes_num = height * width / SCREEN_SPACE_AMOUNT;
        SnowField {
            snowflakes: ((0..snowflakes_num).map(|_| None).collect()),
            h_bound: height,
            w_bound: width,
            snowflakes_num: 0,
            max_snowflakes: snowflakes_num,
        }
    }

    fn draw(&mut self, engine: &mut ConsoleEngine) -> () {
        self.snowflakes.iter().flatten().for_each(|snowflake| {
            engine.set_pxl(
                snowflake.x as i32,
                snowflake.y as i32,
                pixel::pxl(SNOW_FLAKE_SYMB),
            );
        });
    }

    fn update(&mut self) {
        // update some points
        for snowflake in self.snowflakes.iter_mut().flatten() {
            snowflake.speed_x = random(3) as i32 - random(3) as i32;
            snowflake.speed_y = random(3);
            snowflake.x += snowflake.speed_x;
            if snowflake.x < 0 {
                snowflake.x = self.w_bound as i32 - snowflake.x;
            }
            snowflake.y += snowflake.speed_y;
        }
        // none out of bounds some
        for snowflake in self.snowflakes.iter_mut().filter(|c| c.is_some()) {
            if let Some(point) = snowflake {
                if point.y > self.h_bound {
                    *snowflake = None;
                    self.snowflakes_num = self.snowflakes_num.wrapping_sub(1);
                }
            }
        }
        // populate with some
        let mut new_snowflakes_num = random(self.w_bound / 100 * 10);
        if self
            .max_snowflakes
            .checked_sub(self.snowflakes_num + new_snowflakes_num)
            .is_none()
        {
            new_snowflakes_num = self.max_snowflakes - self.snowflakes_num;
        }

        let skip_num = (self.max_snowflakes - self.snowflakes_num - new_snowflakes_num) as usize;
        self.snowflakes_num += new_snowflakes_num;
        self.snowflakes
            .iter_mut()
            .filter(|c| c.is_none())
            .skip(skip_num)
            .for_each(|c| *c = Some(Point::new(random(self.w_bound) as i32, 0)));
    }
}

fn main() {
    // initializes a screen filling the terminal of at least 10x10 of size with a target of 4 frame per second
    let mut engine =
        ConsoleEngine::init_fill_require(10, 10, 5).expect("Terminal screen is too small");
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
