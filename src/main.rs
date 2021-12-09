use crate::part1_a::count_increasing_measurements;
use crate::part1_b::count_increasing_sliding_windows;
use crate::part2_a::compute_position;
use crate::part2_b::compute_position_precise;
use crate::part3_a::compute_power_consumption;
use crate::part3_b::compute_life_support;
use crate::part4_a::Game;
use crate::part4_b::Game as GameB;
use crate::part5_a::compute_overlapping_lines;
use crate::part5_b::compute_overlapping_lines as compute_overlapping_lines_all;
use crate::part6::model_population;

pub mod part1_a;
pub mod part1_b;
pub mod part2_a;
pub mod part2_b;
pub mod part3_a;
pub mod part3_b;
mod part4_a;
mod part4_b;
mod part5_a;
mod part5_b;
mod part6;
mod part7;

fn main() {
    // Part 1
    let input1: Vec<i32> = include_str!("part1_input.txt")
        .split(",")
        .map(|input| input.trim().parse().unwrap())
        .collect();

    println!(
        "Result 1.A: {}",
        count_increasing_measurements(input1.to_vec())
    );
    println!(
        "Result 1.B: {}",
        count_increasing_sliding_windows(input1.to_vec())
    );

    // Part 2
    let input2: Vec<&str> = include_str!("part2_input.txt")
        .split(",")
        .map(|input| input.trim())
        .collect();
    println!("Result 2.A: {}", compute_position(input2.to_vec()));
    println!("Result 2.B: {}", compute_position_precise(input2.to_vec()));

    // Part 3
    let input3: Vec<&str> = include_str!("part3_input.txt")
        .split(",")
        .map(|input| input.trim())
        .collect();
    println!("Result 3.A: {}", compute_power_consumption(input3.to_vec()));
    println!("Result 3.B: {}", compute_life_support(input3.to_vec()));

    // Part 4
    let input4 = vec![
        0, 56, 39, 4, 52, 7, 73, 57, 65, 13, 3, 72, 69, 96, 18, 9, 49, 83, 24, 31, 12, 64, 29, 21,
        80, 71, 66, 95, 2, 62, 68, 46, 11, 33, 74, 88, 17, 15, 5, 6, 98, 30, 51, 78, 76, 75, 28,
        53, 87, 48, 20, 22, 55, 86, 82, 90, 47, 19, 25, 1, 27, 60, 94, 38, 97, 58, 70, 10, 43, 40,
        89, 26, 34, 32, 23, 45, 50, 91, 61, 44, 35, 85, 63, 16, 99, 92, 8, 36, 81, 84, 79, 37, 93,
        67, 59, 54, 41, 77, 42, 14,
    ];
    let mut game = Game::new("src/part4_input.txt");
    println!("Result 4.A: {}", game.play(input4));

    let mut game_b = GameB::new("src/part4_input.txt");
    let input4b = vec![
        0, 56, 39, 4, 52, 7, 73, 57, 65, 13, 3, 72, 69, 96, 18, 9, 49, 83, 24, 31, 12, 64, 29, 21,
        80, 71, 66, 95, 2, 62, 68, 46, 11, 33, 74, 88, 17, 15, 5, 6, 98, 30, 51, 78, 76, 75, 28,
        53, 87, 48, 20, 22, 55, 86, 82, 90, 47, 19, 25, 1, 27, 60, 94, 38, 97, 58, 70, 10, 43, 40,
        89, 26, 34, 32, 23, 45, 50, 91, 61, 44, 35, 85, 63, 16, 99, 92, 8, 36, 81, 84, 79, 37, 93,
        67, 59, 54, 41, 77, 42, 14,
    ];
    println!("Result 4.B: {}", game_b.play(input4b));

    // Part 5
    println!(
        "Result 5.A: {}",
        compute_overlapping_lines("src/part5_input.txt")
    );
    println!(
        "Result 5.B: {}",
        compute_overlapping_lines_all("src/part5_input.txt")
    );

    // Part 6
    let input6 = vec![
        1, 1, 3, 5, 3, 1, 1, 4, 1, 1, 5, 2, 4, 3, 1, 1, 3, 1, 1, 5, 5, 1, 3, 2, 5, 4, 1, 1, 5, 1,
        4, 2, 1, 4, 2, 1, 4, 4, 1, 5, 1, 4, 4, 1, 1, 5, 1, 5, 1, 5, 1, 1, 1, 5, 1, 2, 5, 1, 1, 3,
        2, 2, 2, 1, 4, 1, 1, 2, 4, 1, 3, 1, 2, 1, 3, 5, 2, 3, 5, 1, 1, 4, 3, 3, 5, 1, 5, 3, 1, 2,
        3, 4, 1, 1, 5, 4, 1, 3, 4, 4, 1, 2, 4, 4, 1, 1, 3, 5, 3, 1, 2, 2, 5, 1, 4, 1, 3, 3, 3, 3,
        1, 1, 2, 1, 5, 3, 4, 5, 1, 5, 2, 5, 3, 2, 1, 4, 2, 1, 1, 1, 4, 1, 2, 1, 2, 2, 4, 5, 5, 5,
        4, 1, 4, 1, 4, 2, 3, 2, 3, 1, 1, 2, 3, 1, 1, 1, 5, 2, 2, 5, 3, 1, 4, 1, 2, 1, 1, 5, 3, 1,
        4, 5, 1, 4, 2, 1, 1, 5, 1, 5, 4, 1, 5, 5, 2, 3, 1, 3, 5, 1, 1, 1, 1, 3, 1, 1, 4, 1, 5, 2,
        1, 1, 3, 5, 1, 1, 4, 2, 1, 2, 5, 2, 5, 1, 1, 1, 2, 3, 5, 5, 1, 4, 3, 2, 2, 3, 2, 1, 1, 4,
        1, 3, 5, 2, 3, 1, 1, 5, 1, 3, 5, 1, 1, 5, 5, 3, 1, 3, 3, 1, 2, 3, 1, 5, 1, 3, 2, 1, 3, 1,
        1, 2, 3, 5, 3, 5, 5, 4, 3, 1, 5, 1, 1, 2, 3, 2, 2, 1, 1, 2, 1, 4, 1, 2, 3, 3, 3, 1, 3, 5,
    ];
    println!("Result 6.A: {}", model_population(&input6, 80));
    // println!("Result 6.B: {}", model_population(&input6, 256));

    // Part 7
}
