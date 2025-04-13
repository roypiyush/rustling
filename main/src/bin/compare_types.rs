use std::{fmt, mem::swap};

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: PartialEq> PartialEq for Pair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialOrd> PartialOrd for Pair<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let x_match: Option<std::cmp::Ordering> = self.x.partial_cmp(&other.x);
        match x_match {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

impl<T: fmt::Display> fmt::Display for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let ref mut p1 = Pair { x: 1, y: 2 };
    let ref mut p2 = Pair { x: 3, y: 4 };

    println!("{p1} {p2}");
    swap(p1, p2);
    println!("{p1} {p2}");

    println!("{}", p1 > p2);
}
