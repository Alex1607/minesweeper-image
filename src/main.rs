use std::io;

use clap::Parser;

use crate::error::MinesweeperError;
use crate::parser::{
    parse_flag_data, parse_meta_data, parse_mine_data, parse_open_data, ApiData, ParsedData,
};
use crate::renderer::{RenderOptions, RenderType, Renderer};
use crate::textures::load_textures;

mod base62;
mod error;
mod minesweeper_logic;
mod parser;
mod renderer;
mod textures;

fn main() {
    let args = RenderOptions::parse();
    let data = fetch_data(&args);
    let option = data.split_once('=').expect("Unable to get Version");

    //Version 1 requires all data to exist, empty data has to be marked with an `++` but I might not be omitted
    //Only metadata and mine data might not be empty
    if option.0.eq("1") {
        let split: Vec<&str> = option.1.split('+').collect();
        let parsed_data = parse_v1(
            split[0].trim(),
            split[1].trim(),
            split[2].trim(),
            split[3].trim(),
        );
        render(parsed_data, args).expect("Error while rendering Image");
    } else {
        println!("Unknown / Unsupported version");
    }
}

fn fetch_data(args: &RenderOptions) -> String {
    let mut data = String::new();
    if args.custom_input {
        println!("Please enter the data:");

        let stdin = io::stdin();
        stdin.read_line(&mut data).unwrap();
        data
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
        v.game_data
    }
}

fn parse_v1(
    raw_meta: &str,
    raw_mine_data: &str,
    raw_open_data: &str,
    raw_flag_data: &str,
) -> ParsedData {
    let metadata = parse_meta_data(raw_meta);

    ParsedData {
        game_board: parse_mine_data(raw_mine_data, &metadata),
        open_data: parse_open_data(raw_open_data),
        flag_data: parse_flag_data(raw_flag_data),
        metadata,
    }
}

fn render(data: ParsedData, options: RenderOptions) -> Result<(), MinesweeperError> {
    let sprite = load_textures(&options, &data.metadata);

    let mut renderer = Renderer::new(
        data.metadata,
        data.game_board,
        data.open_data,
        data.flag_data,
        sprite.as_slice(),
        &options,
    );

    if let Some(t) = &options.force_type {
        match t {
            RenderType::Image => renderer.render_jpeg(),
            RenderType::Gif => renderer.render_gif(),
        }
    } else if renderer.metadata.y_size >= 32 || renderer.metadata.x_size >= 32 {
        renderer.render_jpeg()
    } else {
        renderer.render_gif()
    }
}
