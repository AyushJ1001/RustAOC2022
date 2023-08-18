pub fn run1() {
    let file = std::fs::read_to_string("input/day2").unwrap();

    let mut total = 0;

    file.lines().for_each(|line| {
        let (opponent, player) = line.split_once(" ").unwrap_or(("", ""));

        let shape = match player {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        let outcome = match (player, opponent) {
            ("X", "A") => 3,
            ("X", "B") => 0,
            ("X", "C") => 6,
            ("Y", "A") => 6,
            ("Y", "B") => 3,
            ("Y", "C") => 0,
            ("Z", "A") => 0,
            ("Z", "B") => 6,
            ("Z", "C") => 3,
            _ => 0,
        };

        total += shape + outcome;
    });

    println!("Total: {}", total);
}

pub fn run2() {
    let mut total = 0;
    let file = std::fs::read_to_string("input/day2").unwrap();

    file.lines().for_each(|line| {
        let (opponent, outcome) = line.split_once(" ").unwrap_or(("", ""));

        let player = match (opponent, outcome) {
            ("A", "X") => "Z",
            ("A", "Y") => "X",
            ("A", "Z") => "Y",
            ("B", "X") => "X",
            ("B", "Y") => "Y",
            ("B", "Z") => "Z",
            ("C", "X") => "Y",
            ("C", "Y") => "Z",
            ("C", "Z") => "X",
            _ => "",
        };

        let shape = match player {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        let outcome = match (player, opponent) {
            ("X", "A") => 3,
            ("X", "B") => 0,
            ("X", "C") => 6,
            ("Y", "A") => 6,
            ("Y", "B") => 3,
            ("Y", "C") => 0,
            ("Z", "A") => 0,
            ("Z", "B") => 6,
            ("Z", "C") => 3,
            _ => 0,
        };

        total += shape + outcome;
    });

    println!("Total: {}", total);
}
