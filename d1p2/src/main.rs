use std::fs::File;
use std::io::Read;

struct Lift {
    floor: i64,
}

impl Lift {
    fn new() -> Self {
        Lift { floor: 0 }
    }

    fn down(&mut self) {
        println!("Going down!");
        self.floor -= 1;
    }

    fn up(&mut self) {
        println!("Going up!");
        self.floor += 1;
    }
}

fn main() {
    let mut fp = File::open("input.txt").expect("failed to read file");
    let mut buf = Vec::<u8>::new();

    fp.read_to_end(&mut buf);

    let mut lift = Lift::new();

    for idx in 0..buf.len() {
        match buf[idx] {
            b'(' => lift.up(),
            b')' => lift.down(),
            _ => (),
        }
        if lift.floor == -1 {
            println!("Basement instruction: {}", (idx + 1));
            break;
        }
    }
    println!("Floor: {}", lift.floor);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up_floor() {
        let mut l = Lift::new();
        l.up();
        assert_eq!(l.floor, 1);
    }

    #[test]
    fn down_floor() {
        let mut l = Lift::new();
        l.down();
        assert_eq!(l.floor, -1);
    }
}
