use std::path::PrefixComponent;

enum Movement {
    Up, 
    Down,
    Left,
    Right
}

fn move_avatar(m: Vec<&Movement>) {
    for movement in &m {
        match movement {
            Movement::Up => println!("Move Up"),
            Movement::Down => println!("Move Down"),
            Movement::Left => println!("Move Left"),
            Movement::Right => println!("Move Right"),
        }
    };
}

pub fn run() {
    let jump = Movement::Up;
    let jump_off = Movement::Down;
    let move_left = Movement::Left;
    let move_right = Movement::Right;
    move_avatar(vec![&jump_off, &jump, &move_left, &jump, &jump_off, &move_right]);
}