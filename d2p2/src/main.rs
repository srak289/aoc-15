use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::min;

use regex::Regex;

struct Rect3D {
    l: u32,
    w: u32,
    h: u32,
    _b: u32,
    _f: u32,
    _s: u32,
}

impl Rect3D {
    fn new(l: u32, w: u32, h: u32) -> Self {
        Self { l, w, h, _b: 0, _f: 0, _s: 0 }
    }

    fn _base(&mut self) -> u32 {
        if self._b != 0 {
            return self._b;
        }
        self._b = self.w * self.l;
        return self._b;
    }

    fn sp(&self) -> u32 {
        return self.w * 2 + self.h * 2;
    }

    fn fp(&self) -> u32 {
        return self.l * 2 + self.h * 2;
    }

    fn bp(&self) -> u32 {
        return self.w * 2 + self.l * 2;
    }

    fn _face(&mut self) -> u32 {
        if self._f != 0 {
            return self._f;
        }
        self._f = self.l * self.h;
        return self._f;
    }

    fn _side(&mut self) -> u32 {
        if self._s != 0 {
            return self._s;
        }
        self._s = self.w * self.h;
        return self._s;
    }

    fn ribbon(&mut self) -> u32 {
        // shortest distance around sides or smallest perimeter of one face
        // bow is cubic feet volume of present
        let l = min(min(self.bp(), self.fp()), self.sp());
        println!("Ribbon for package: {}", &l);
        let b = self.l * self.w * self.h;
        return l + b;
    }

    fn area(&mut self) -> u32 {
        return 2 * self._base()
            + 2 * self._side()
            + 2 * self._face();
    }

    fn slack(&mut self) -> u32 {
        return min(min(self._base(), self._face()), self._side())
    }

}

fn main() {
    let mut fp = BufReader::new(File::open("input.txt").expect("failed to read file"));
    let mut line = String::new();

    let dig_re = Regex::new(r"[0-9]+").expect("Invalid regex");

    let mut packages: Vec<Rect3D> = Vec::with_capacity(256);

    loop {
        match fp.read_line(&mut line) {
            Ok(x) => {
                if x == 0 {
                    println!("EOF");
                    break;
                }
                let lwh: Vec<u32> = dig_re.find_iter(&line).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
                println!("{:?}", lwh);
                packages.push(Rect3D::new(lwh[0], lwh[1], lwh[2]));
                line.clear();
            }
            Err(_) => panic!("Error reading line"),
        }
    }

    let mut ribbon = 0_u32;

    for p in &mut packages {
        ribbon += p.ribbon();
        println!("Ribbon is {}", &ribbon);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_area() {
        let mut r = Rect3D::new(2, 2, 2);
        assert_eq!(r.l, 2);
        assert_eq!(r.w, 2);
        assert_eq!(r.h, 2);
        assert_eq!(r._side(), 4);
        assert_eq!(r._face(), 4);
        assert_eq!(r._base(), 4);
    }

    #[test]
    fn test_slack() {
        let mut r = Rect3D::new(2, 3, 4);
        assert_eq!(r.slack(), 6);
    }
}
