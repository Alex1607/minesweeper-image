use crate::parsers::parser::Metadata;
use crate::renderer::{RenderOptions, RenderType};
use std::fs;
use std::fs::File;
use std::io::Read;

pub fn load_textures(options: &RenderOptions, metadata: &Metadata) -> Vec<u8> {
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
