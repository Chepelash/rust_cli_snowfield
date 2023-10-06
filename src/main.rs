use console_engine::{pixel, Color, ConsoleEngine, KeyCode};
use cli_snowfield::SnowField;

const FPS: u32 = 3;

fn main() {
    // initializes a screen filling the terminal of at least 10x10 of size with a target frames per second
    let mut engine =
        ConsoleEngine::init_fill_require(10, 10, FPS).expect("Terminal screen is too small");
    let mut snowfield = SnowField::init(engine.get_height(), engine.get_width());
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
                             // engine.check_resize(); here we do not want to resize the terminal because it could break the boundaries of the game

        // exit check
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        engine.clear_screen(); // reset the screen
        engine.fill(pixel::pxl_fbg(' ', Color::Black, Color::Black));

        snowfield.update();
        snowfield.draw(&mut engine);
        engine.draw();        
    }
}
