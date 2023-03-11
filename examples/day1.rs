use itertools::Itertools;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut top_three = vec![u32::MIN; 3];
    let part_one_ans = fs::read_to_string("data/day1.txt")?
        .split("\n\n")
        .map(|items| {
            items
                .split("\n")
                .filter_map(|item| item.parse::<u32>().ok())
                .sum::<u32>()
        })
        .inspect(|item_sum| {
            top_three.iter().position_min().map(|min_pos| {
                if *item_sum > top_three[min_pos] {
                    top_three[min_pos] = *item_sum;
                }
            });
        })
        .max()
        .unwrap_or_default();
    println!("{}", part_one_ans);
    println!("{}", top_three.into_iter().sum::<u32>());
    Ok(())
}
