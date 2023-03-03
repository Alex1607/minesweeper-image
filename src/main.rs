use crate::parser::{parse_flag_data, parse_meta_data, parse_mine_data, parse_open_data};
use crate::renderer::Renderer;
use std::io;

mod base62;
mod minesweeper_logic;
mod parser;
mod renderer;

const SKIN_FULL: &[u8] = include_bytes!("../resources/skin_full.png");
const SKIN_GIF: &[u8] = include_bytes!("../resources/skin_20.png");

fn main() {
    println!("Please enter the data:");

    let stdin = io::stdin();
    let data = &mut String::new();
    stdin.read_line(data).unwrap();

    let option = data.split_once('=').unwrap();
    let version = option.0;

    let split: Vec<&str> = option.1.split('+').collect();

    //Version 1 requires all data to exist, empty data has to be marked with an `++` but I might not be omitted
    //Only metadata and mine data might not be empty
    if version.eq("1") {
        parse_v1(
            split[0].trim(),
            split[1].trim(),
            split[2].trim(),
            split[3].trim(),
        )
    }
}

fn parse_v1(raw_meta: &str, raw_mine_data: &str, raw_open_data: &str, raw_flag_data: &str) {
    let metadata = parse_meta_data(raw_meta).unwrap();
    let game_board = parse_mine_data(raw_mine_data, &metadata).unwrap();
    let open_data = parse_open_data(raw_open_data).unwrap();
    let flag_data = parse_flag_data(raw_flag_data).unwrap();

    let sprite = if metadata.x_size >= 32 || metadata.y_size >= 32 {
        SKIN_FULL
    } else {
        SKIN_GIF
    };

    let mut renderer = Renderer::new(metadata, game_board, open_data, flag_data, sprite);

    if renderer.metadata.y_size >= 32 || renderer.metadata.x_size >= 32 {
        renderer.render_jpeg();
    } else {
        renderer.render_gif();
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ActionType {
    Open,
    Flag,
}
