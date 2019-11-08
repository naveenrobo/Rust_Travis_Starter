enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Move UP"),
        Movement::Down => println!("Move DOWN"),
        Movement::Left => println!("Move LEFT"),
        Movement::Right => println!("Move RIGHT"),
    }
}

pub fn run() {
    let avatar1 = Movement::Up;

    move_avatar(avatar1);
}
