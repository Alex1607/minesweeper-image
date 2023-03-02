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
    // let data = "1=15x15+B14262B22393E334A44565864767B758593ADAABEB3CEC9DED5E7E+000;001;3325;342;255;3710;383;3A21;4922;482;474;464;361;3513;453;4425;6428;847;944;B39;B414;C46;A49;A53;B53;E619;E89;E94;CA10;DA5;EA5;EB5;EE36;AE15;9D5;8D6;6A29;5A4;698;5916;0B33;1C11;0C2;0D10;5848;5729;671;6631;5512;652;754+3216P;245P;2612P;3924P;4463P;436P;4411R;5425P;748P;A314P;C36P;E545P;E79P;D98P;BA15P;AD33P;BE6P;CE4P;DE3P;7B52P;5926P;5911R;4A9P;2B12P;1B5P;3E42P;6837P;576P;5722R;5618P;763P".to_string();
    // let data = "1=20x20+90H0J0D1G2135393G3J314D4E41565D5E5568696F6J61787D7D8F8G8J8197999D99AAADA0B1B4B9BAB2C7C9CDC3D4D6D5E1F6FFF4G5G6G8G1HGHJHBICIFI+110;111;6133;8113;A17;908;804;703;604;504;4011;304;4528;5513;6617;674;771;876;A717;B74;B62;B54;B318;B24;C39;C41;C55;C617;A042;C134;D217;D12;C06;D011;E212;D530;D725;B813;C83;D85;E84;E65;G752;H814;H73;H65;I818;G327;G24;G111;H09;G012;F326;F43;F212;E38;E43;F532;F021;ID53;IE4;JC12;JB5;JD13;JE4;CE31;CF4;GF22;HF6;HH25;HI4;IJ13;II4;IH5;IG5;JH12;JF24;BD30;8H43;BC33;BB5;A858;985;5756;584;8830;8992;7922;7A5;8A2;9A4;7B16;5948;494;297;194;6D58;3D15;2D5;0D13;1E14;2E2;3E6;0E12;0F4;2F7;3F4;4F22;4G27;3H18;4H1;5H4;5F8;7F72;8I31;7J16;5J10;4J4;3I22;2I15;2J6;1J9;1I4;1H5;2H2;0I35;6E44;7E8;8E24;AC32;AB21+3114P;419P;514P;7113P;916P;3562P;658P;9756P;B425P;B169P;B021P;C225P;D366P;D44P;D615P;C79P;F658P;G827P;G69P;G562P;G43P;H117P;E597P;F126P;IB49P;IC5P;CD68P;FF26P;HG27P;HJ18P;IF57P;AD36P;9D4P;BA88P;C95P;B914P;5674P;6829P;788P;A967P;997P;69115P;3917P;0910P;8D41P;7D4P;5D13P;4D5P;1D15P;4E72P;3G17P;2G4P;5E65P;6F6P;8G12P;8G21R;8G30P;8F5P;8J13P;6J8P;3I29P;3I10R;3J8P;0H52P;0J19P".to_string();
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

    // println!("{metadata:?}\n{game_board:?}\n{open_data:?}\n{flag_data:?}");

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
