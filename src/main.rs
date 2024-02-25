use std::io;

use clap::Parser;

use crate::error::MinesweeperError;
use crate::parsers::parser::{ApiData, Iparser, ParsedData};
use crate::renderer::{RenderOptions, RenderType, Renderer};
use crate::textures::load_textures;

mod base62;
mod error;
mod minesweeper_logic;
mod parsers;
mod renderer;
mod textures;

fn main() {
    let args = RenderOptions::parse();
    let data = fetch_data(&args);
    let option = data.split_once('=').expect("Unable to get Version");
    let possible_parsers: Vec<dyn Iparser> = vec![parsers::v1::parser::ParserV1, parsers::v2::parser::ParserV2];

    let found_parser = possible_parsers.iter().filter(|p| p.supported_versions().contains(&option.0))
        .next()
        .expect("Unknown / Unsupported version");

    let split: Vec<&str> = option.1.split('+').collect();
    
    let metadata = found_parser.parse_meta_data(split[0].trim());

    let parsed_data = ParsedData {
        game_board: found_parser.parse_mine_data(split[1].trim(), &metadata),
        open_data: found_parser.parse_open_data(split[2].trim()),
        flag_data: found_parser.parse_flag_data(split[3].trim()),
        metadata,
    };
    
    render(parsed_data, args).expect("Unable to render");
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
