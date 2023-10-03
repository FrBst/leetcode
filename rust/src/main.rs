use std::{io, fs, error::Error};

use solutions::remove_colored_pieces_if_both_neighbors_are_the_same_color_2038;

mod common;
mod solutions;

fn main() {
    remove_colored_pieces_if_both_neighbors_are_the_same_color_2038::Solution::winner_of_game(String::from("AAAABBBB"));
}
