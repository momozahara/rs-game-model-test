use std::{ sync::Mutex, collections::HashMap };

use pathfinding::prelude::astar;

static MAP: Mutex<Option<HashMap<Pos, u32>>> = Mutex::new(None);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;

        let mut results = Vec::new();

        let mut map_instance = MAP.lock().unwrap();
        let map = map_instance.as_mut().unwrap();

        let top = map.get(&Pos(x, y + 1));
        if let Some(top) = top {
            results.push((Pos(x, y + 1), *top));
        }

        let right = map.get(&Pos(x + 1, y));
        if let Some(right) = right {
            results.push((Pos(x + 1, y), *right));
        }

        let bottom = map.get(&Pos(x, y - 1));
        if let Some(bottom) = bottom {
            results.push((Pos(x, y - 1), *bottom));
        }

        let left = map.get(&Pos(x - 1, y));
        if let Some(left) = left {
            results.push((Pos(x - 1, y), *left));
        }

        results
    }
}

fn main() {
    let mut map_instance = MAP.lock().unwrap();
    *map_instance = Some(HashMap::new());

    let map = map_instance.as_mut().unwrap();

    for x in 0..=10 {
        for y in 0..=10 {
            map.insert(Pos(x, y), 1);
        }
    }

    drop(map);
    drop(map_instance);

    let results = astar(
        &Pos(0, 0),
        |p| p.successors(),
        |p| p.distance(&Pos(5, 5)) / 3,
        |p| *p == Pos(5, 5)
    );

    println!("{:?}", results);
}