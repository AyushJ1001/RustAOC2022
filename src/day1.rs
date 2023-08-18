pub fn run() {
    let file = std::fs::read_to_string("input/day1").unwrap();
    let mut max: i32 = 0;
    let mut second: i32 = 0;
    let mut third: i32 = 0;
    let mut total = 0;

    file.lines().for_each(|line| {
        let number = line.parse::<i32>().unwrap_or(-1);

        if number == -1 {
            if total > third {
                third = total;
            }

            if third > second {
                (second, third) = (third, second);
            }

            if second > max {
                (max, second) = (second, max);
            }

            total = 0;
        } else {
            total += number;
        }
    });

    println!("Day 1 Part 1: {}", max);
    println!("Day 1 Part 2: {}", max + second + third);
}
