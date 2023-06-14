fn main() {
    // read file input.txt
    let file = match std::fs::read_to_string("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // part one
    let pos = part_one(&file);
    println!("Part one: {}", pos.0.abs() + pos.1.abs());
}

fn part_one(input: &str) -> (i32, i32) {
    // taxicab geometry
    // https://en.wikipedia.org/wiki/Taxicab_geometry
    // T = | y2 - y1 | + | x2 - x1 |
    // split str by ',' and trim
    let input = input.split(",").map(|x| x.trim());
    let mut pos: (i32, i32) = (0, 0);
    let mut direction: (i32, i32) = (0, 1);
    for i in input {
        // 1. get direction (R or L)
        if i.starts_with("R") {
            direction = (direction.1, -direction.0);
        } else if i.starts_with("L") {
            direction = (-direction.1, direction.0);
        } else {
            panic!("Invalid direction: {}", i);
        }

        // 2. get number of steps
        let steps: i32 = i[1..].parse().unwrap();

        // 3. move in direction
        for _ in 0..steps {
            pos.0 += direction.0;
            pos.1 += direction.1;
        }
    }
    pos
}
