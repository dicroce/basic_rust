
#[derive(Copy, Clone)]
enum Movements {
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movements) {
    match m {
        Movements::Up => println!("Avatar: Up"),
        Movements::Down => println!("Avatar: Down"),
        Movements::Left => println!("Avatar: Left"),
        Movements::Right => println!("Avatar: Right")
    }
}

pub fn run() {
    let moves: [Movements; 4] = [Movements::Left, Movements::Down, Movements::Up, Movements::Right];
    for m in moves.iter() {
        move_avatar(*m);
    }

}
