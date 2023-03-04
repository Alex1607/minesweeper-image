use std::{fs, io};
use std::fs::File;
use std::io::Read;

use clap::{Parser};

use crate::parser::{ApiData, Metadata, parse_flag_data, parse_meta_data, parse_mine_data, parse_open_data};
use crate::renderer::{Renderer, RenderOptions, RenderType};

mod base62;
mod minesweeper_logic;
mod parser;
mod renderer;

fn main() {
    let args = RenderOptions::parse();

    let mut data = String::new();

    if args.custom_input {
        println!("Please enter the data:");

        let stdin = io::stdin();
        stdin.read_line(&mut data).unwrap();
    } else {
        println!("Please enter the GameID:");

        let gameid = &mut String::new();
        let stdin = io::stdin();
        stdin.read_line(gameid).unwrap();

        let request_data =
            ureq::get(format!("https://api.greev.eu/v2/stats/minesweeper/game/{gameid}").as_ref())
                .call()
                .expect("Unable to fetch Data")
                .into_string()
                .expect("Unable to parse Data");

        let v: ApiData = serde_json::from_str(request_data.as_ref()).expect("Unable to parse Data");
        data = v.game_data;
    }

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
            args,
        )
    } else {
        println!("Unknown / Unsupported version");
    }
}

fn parse_v1(
    raw_meta: &str,
    raw_mine_data: &str,
    raw_open_data: &str,
    raw_flag_data: &str,
    options: RenderOptions,
) {
    let metadata = parse_meta_data(raw_meta).unwrap();
    let game_board = parse_mine_data(raw_mine_data, &metadata).unwrap();
    let open_data = parse_open_data(raw_open_data).unwrap();
    let flag_data = parse_flag_data(raw_flag_data).unwrap();

    let sprite = load_textures(&options, &metadata);

    let mut renderer = Renderer::new(metadata, game_board, open_data, flag_data, sprite.as_slice(), &options);

    if let Some(t) = &options.force_type {
        match t {
            RenderType::Image => renderer.render_jpeg(),
            RenderType::Gif => renderer.render_gif(),
        }
    } else if renderer.metadata.y_size >= 32 || renderer.metadata.x_size >= 32 {
        renderer.render_jpeg();
    } else {
        renderer.render_gif();
    }
}

fn load_textures(options: &RenderOptions, metadata: &Metadata) -> Vec<u8> {
    let skin_full: Vec<u8> = include_bytes!("../resources/skin_full.png").to_vec();
    let skin_gif: Vec<u8> = include_bytes!("../resources/skin_20.png").to_vec();

    if let Some(path) = &options.custom_textures {
        load_file_data(path)
    } else if let Some(t) = &options.force_type {
        match t {
            RenderType::Image => skin_full,
            RenderType::Gif => skin_gif,
        }
    } else if metadata.x_size >= 32 || metadata.y_size >= 32 {
        skin_full
    } else {
        skin_gif
    }
}

fn load_file_data(custom_texture_sprite: &str) -> Vec<u8> {
    let mut file = File::open(custom_texture_sprite).expect("no file found");
    let metadata = fs::metadata(custom_texture_sprite).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read_exact(&mut buffer).expect("buffer overflow");
    buffer
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ActionType {
    Open,
    Flag,
}
