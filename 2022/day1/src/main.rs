use std::collections::BinaryHeap;

fn main() {
    println!("part one: {}", get_top_n(1));
    println!("part two: {}", get_top_n(3));
}

// get the top n sums from input file
fn get_top_n(n: i32) -> i32 {
    let res = get_ordered_calories("./input.txt");
    let mut ordered_calories = match res {
        Err(err) => panic!("failed to get ordered heap of calories: {}", err),
        Ok(heap) => heap,
    };

    let mut top_n = 0;
    for _i in 0..n {
        if ordered_calories.len() == 0 {
            break;
        }

        let next_calories = match ordered_calories.pop() {
            None => panic!("tried to get more calories than there are elves"),
            Some(elem) => elem,
        };
        top_n += next_calories;
    }
    top_n
}

// build a max heap of sums from input file
fn get_ordered_calories(filename: &str) -> Result<BinaryHeap<i32>, std::io::Error> {
    let file_data = std::fs::read_to_string(filename)?;

    let mut ordered_calories: BinaryHeap<i32> = BinaryHeap::new();
    let mut curr_calories = 0;
    for line in file_data.split("\n") {
        match line {
            "" => {
                ordered_calories.push(curr_calories);
                curr_calories = 0;
            }
            _ => {
                let num = line.parse::<i32>().unwrap();
                curr_calories += num;
            }
        }
    }

    Ok(ordered_calories)
}
