use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn deliver(grid: &mut HashMap<(i32, i32), i32>, x: &i32, y: &i32) {
    if let Some(v) = grid.get_mut(&(*x, *y)) {
        *v += 1;
        return;
    }
    grid.insert((*x, *y), 1);
}

fn main() {
    let mut fp = File::open("input.txt").expect("failed to read file");
    let mut buf = Vec::<u8>::new();

    fp.read_to_end(&mut buf);

    let mut x = 0_i32;
    let mut y = 0_i32;

    let mut grid: HashMap<(i32, i32), i32> = HashMap::with_capacity(256);

    let _ = grid.insert((0, 0), 1);

    for c in buf.drain(..) {
        match c {
            b'^' => {
                y += 1;
            }
            b'>' => {
                x += 1;
            }
            b'<' => {
                x -= 1;
            }
            b'v' => {
                y -= 1;
            }
            _ => (),
        }
        println!("Delivering at position ({},{})", &x, &y);
        deliver(&mut grid, &x, &y);
    }

    let mut gt_one = 0_u32;

    for (k, v) in grid.iter() {
        if *v >= 1 {
            gt_one += 1;
        }
    }
    println!("House {}", &gt_one);
}

#[cfg(test)]
mod tests {
    use super::*;

}
