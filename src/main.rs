use std::i64;
use std::str::FromStr;

const BASE: i64 = 62;
const CHARACTERS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn main() {
    let data = "1=15x15+8090B0B184C4D40565A5E5068696E627D7197999AB2C1D4D5DBD3E+000;7132;704;809;811;9016;A15;A06;B03;D332;D22;E533;E413;E223;D037;E011;E13;9867;9A14;AA4;8815;898;8A3;8B3;EB20;7930;784;5740;582;5914;5B22;5C4;3C18;2C4;1C16;1D6;5D21;6D1;7E27;4E25;0A38+500P;603P;725P;9133P;C226P;D510P;D47P;E365P;D139P;BA48P;9729P;9916P;DB58P;5649P;687P;695P;489P;5A40P;4C23P;1B17P;4D27P;7D13P;6E28P;5E16P;0B22P;0816P;094P".to_string();

    let option = data.split_once('=').unwrap();
    let version = option.0;

    let split: Vec<&str> = option.1.split('+').collect();

    //Version 1 requires all data to exist, empty data has to be marked with an `++` but I might not be omitted
    //Only metadata and mine data might not be empty
    if version.eq("1") {
        parse_v1(split[0], split[1], split[2], split[3])
    }
}

fn parse_v1(raw_meta: &str, raw_mine_data: &str, raw_open_data: &str, raw_flag_data: &str) {
    let metadata = parse_meta_data(raw_meta).unwrap();
    let game_board = parse_mine_data(raw_mine_data, &metadata).unwrap();
    let open_data = parse_open_data(raw_open_data).unwrap();
    let flag_data = parse_flag_data(raw_flag_data).unwrap();

    println!("{metadata:?}\n{game_board:?}\n{open_data:?}\n{flag_data:?}");
}

fn parse_mine_data(data: &str, metadata: &MetaData) -> Result<Board, ()> {
    let mut board = Board {
        fields: vec![vec![Field::new(); metadata.y_size as usize]; metadata.x_size as usize],
    };

    let mines = parse_mine_locations(data).unwrap();

    for cords in mines {
        let x = cords.0;
        let y = cords.1;
        let mut field = &mut board.fields[x as usize][y as usize];
        field.mine = true;
    }

    for x in 0..metadata.x_size {
        for y in 0..metadata.y_size {
            let field = &mut board.fields[x as usize][y as usize];

            if !field.mine {
                continue;
            }

            for xd in -1..=1_i32 {
                for zd in -1..=1_i32 {
                    let xx = x + xd;
                    let yy = y + zd;
                    if xx < 0 || xx >= metadata.x_size || yy < 0 || yy >= metadata.y_size || (zd == 0 && xd == 0) {
                        continue;
                    }

                    let checked_field = &mut board.fields[xx as usize][yy as usize];
                    if checked_field.mine {
                        continue;
                    }

                    checked_field.value += 1;
                }
            }
        }
    }

    Ok(board)
}

fn parse_mine_locations(data: &str) -> Result<Vec<(i32, i32)>, ()> {
    let mut return_data = Vec::new();

    if data.chars().count() == 0 {
        return Ok(return_data);
    }

    let raw_open_fields_data: Vec<&str> = data.split(';').collect();

    for raw_open_field in raw_open_fields_data {
        if raw_open_field.contains('|') {
            let part = raw_open_field.split_once('|').unwrap();

            return_data.push((
                decode(part.0) as i32,
                decode(part.1) as i32)
            );
        } else {
            raw_open_field.chars().collect::<Vec<char>>()
                .chunks(2)
                .map(|chunk| chunk.iter().collect::<String>())
                .for_each(|x| {
                    let mut chars = x.chars();
                    return_data.push((
                        decode(chars.next().unwrap().to_string().as_str()) as i32,
                        decode(chars.next().unwrap().to_string().as_str()) as i32,
                    ))
                });
        }
    }

    Ok(return_data)
}

fn parse_flag_data(data: &str) -> Result<Vec<FlagAction>, ()> {
    let mut return_data = Vec::new();

    if data.chars().count() == 0 {
        return Ok(return_data);
    }

    let raw_open_fields_data: Vec<&str> = data.split(';').collect();

    for raw_open_field in raw_open_fields_data {
        if raw_open_field.contains('|') {
            let mut chars = raw_open_field.chars();

            let action_type = chars.next_back().unwrap();
            let part_one = chars.as_str().split_once('|').unwrap();
            let part_two = part_one.1.split_once(':').unwrap();

            return_data.push(FlagAction {
                x: decode(part_one.0) as i32,
                y: decode(part_two.0) as i32,
                time: part_two.1.parse::<i64>().unwrap(),
                action: get_flag_type(action_type),
            });
        } else {
            let mut chars = raw_open_field.chars();
            return_data.push(FlagAction {
                x: decode(chars.next().unwrap().to_string().as_str()) as i32,
                y: decode(chars.next().unwrap().to_string().as_str()) as i32,
                action: get_flag_type(chars.next_back().unwrap()),
                time: chars.as_str().parse::<i64>().unwrap(),
            });
        }
    }

    Ok(return_data)
}

fn get_flag_type(raw_flag_type: char) -> Action {
    match raw_flag_type {
        'P' => Action::Place,
        'R' => Action::Remove,
        _ => unreachable!(),
    }
}

fn parse_open_data(data: &str) -> Result<Vec<OpenAction>, ()> {
    let mut return_data = Vec::new();

    if data.chars().count() == 0 {
        return Ok(return_data);
    }

    let raw_open_fields_data: Vec<&str> = data.split(';').collect();

    for raw_open_field in raw_open_fields_data {
        if raw_open_field.contains('|') {
            let part_one = raw_open_field.split_once('|').unwrap();
            let part_two = part_one.1.split_once(':').unwrap();

            return_data.push(OpenAction {
                x: decode(part_one.0) as i32,
                y: decode(part_two.0) as i32,
                time: part_two.1.parse::<i64>().unwrap(),
            });
        } else {
            let mut chars = raw_open_field.chars();
            return_data.push(OpenAction {
                x: decode(chars.next().unwrap().to_string().as_str()) as i32,
                y: decode(chars.next().unwrap().to_string().as_str()) as i32,
                time: chars.as_str().parse::<i64>().unwrap(),
            });
        }
    }

    Ok(return_data)
}

fn parse_meta_data(data: &str) -> Result<MetaData, ()> {
    let data_split = data.split_once('x').unwrap();
    Ok(MetaData {
        x_size: i32::from_str(data_split.0).unwrap(),
        y_size: i32::from_str(data_split.1).unwrap(),
    })
}

fn encode(number: i64) -> String {
    let mut result = String::with_capacity(1);
    let mut num = number;

    while num > 0 {
        let digit = num % BASE;
        num /= BASE;
        result.insert(0, CHARACTERS.chars().nth(digit as usize).unwrap());
    }

    result
}

fn decode(number: &str) -> i64 {
    let mut result: i64 = 0;
    let length = number.len();
    let chars: Vec<char> = CHARACTERS.chars().collect();

    for i in 0..length {
        let digit = chars
            .iter()
            .position(|&c| c == number.chars().nth(length - i - 1).unwrap())
            .unwrap() as i64;
        result += BASE.pow(i as u32) * digit;
    }

    result
}

#[derive(Debug)]
struct MetaData {
    x_size: i32,
    y_size: i32,
}

#[derive(Debug)]
struct FlagAction {
    x: i32,
    y: i32,
    time: i64,
    action: Action,
}

#[derive(Debug)]
enum Action {
    Place,
    Remove,
}

#[derive(Debug)]
struct OpenAction {
    x: i32,
    y: i32,
    time: i64,
}

#[derive(Debug)]
struct Board {
    fields: Vec<Vec<Field>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub value: u8,
    pub field_state: FieldState,
    pub mine: bool,
    pub x: usize,
    pub z: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FieldState {
    Open,
    Closed,
    Flagged,
}

impl Field {
    pub(crate) fn new() -> Self {
        Field {
            value: 0,
            field_state: FieldState::Closed,
            mine: false,
            x: 0,
            z: 0,
        }
    }
}
