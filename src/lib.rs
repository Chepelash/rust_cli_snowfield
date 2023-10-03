pub struct Point {
    pub x: i32,
    pub y: i32
}

struct SnowFlake {
    position: Point,
    speed: u32
}

struct SnowField {
    field: Vec<u32>
}