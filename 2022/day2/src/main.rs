fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

fn part2() -> i32 {
    let str = match std::fs::read_to_string("./input.txt") {
        Err(err) => panic!("failed to read file: {}", err),
        Ok(str) => str,
    };

    let mut score = 0;
    for line in str.split("\n") {
        if line.trim() == "" {
            break;
        }

        let mut characters = line.chars();
        let outcome = characters.nth(0).unwrap();
        // iterator nth method consumes elements as we travel the iterator
        let my_choice = characters.nth(1).unwrap();
        score += income_with_outcome(my_choice, outcome);
    }

    score
}

fn part1() -> i32 {
    let str = match std::fs::read_to_string("./input.txt") {
        Err(err) => panic!("failed to read file: {}", err),
        Ok(str) => str,
    };

    let mut score = 0;
    for line in str.split("\n") {
        if line.trim() == "" {
            break;
        }

        let mut characters = line.chars();
        let opponent_choice = characters.nth(0).unwrap();
        // iterator nth method consumes elements as we travel the iterator
        let my_choice = characters.nth(1).unwrap();
        score += income_with_move(my_choice, opponent_choice);
    }

    score
}

// enum Move {
//     Rock = 1,
//     Paper = 2,
//     Scissors = 3,
// }

// enum Outcome {
//     Loss = 0,
//     Draw = 3,
//     Win = 6,
// }

fn income_with_outcome(me: char, them: char) -> i32 {
    let points_rock = 1;
    let points_paper = 2;
    let points_scissors = 3;
    let points_draw = 3;
    let points_win = 6;

    if them == 'A' && me == 'X' {
        // lose with scissors
        return points_scissors;
    }
    if them == 'A' && me == 'Y' {
        // draw with rock
        return points_rock + points_draw;
    }
    if them == 'A' && me == 'Z' {
        // win with paper
        return points_paper + points_win;
    }
    if them == 'B' && me == 'X' {
        // lose with rock
        return points_rock;
    }
    if them == 'B' && me == 'Y' {
        // draw with paper
        return points_paper + points_draw;
    }
    if them == 'B' && me == 'Z' {
        // win with scissors
        return points_scissors + points_win;
    }
    if them == 'C' && me == 'X' {
        // lose with paper
        return points_paper;
    }
    if them == 'C' && me == 'Y' {
        // draw with scissors
        return points_scissors + points_draw;
    }
    // win with rock
    points_rock + points_win
}

fn income_with_move(me: char, them: char) -> i32 {
    let points_rock = 1;
    let points_paper = 2;
    let points_scissors = 3;
    let points_draw = 3;
    let points_win = 6;

    // let mut points_won = 0;

    if them == 'A' && me == 'X' {
        // draw
        return points_rock + points_draw;
    }
    if them == 'A' && me == 'Y' {
        // win
        return points_paper + points_win;
    }
    if them == 'A' && me == 'Z' {
        // loss
        return points_scissors;
    }
    if them == 'B' && me == 'X' {
        // loss
        return points_rock;
    }
    if them == 'B' && me == 'Y' {
        // draw
        return points_paper + points_draw;
    }
    if them == 'B' && me == 'Z' {
        // win
        return points_scissors + points_win;
    }
    if them == 'C' && me == 'X' {
        // win
        return points_rock + points_win;
    }
    if them == 'C' && me == 'Y' {
        // loss
        return points_paper;
    }
    // draw
    points_scissors + points_draw
}
