use crate::base62::decode;
use crate::minesweeper_logic::{Board, Field};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub(crate) x_size: i32,
    pub(crate) y_size: i32,
}

#[derive(Debug)]
pub struct FlagAction {
    pub(crate) x: i32,
    pub(crate) y: i32,
    time: i64,
    pub(crate) action: Action,
    pub(crate) total_time: i64,
}

#[derive(Debug)]
pub enum Action {
    Place,
    Remove,
}

#[derive(Debug)]
pub struct OpenAction {
    pub(crate) x: i32,
    pub(crate) y: i32,
    time: i64,
    pub(crate) total_time: i64,
}

pub fn parse_mine_data(data: &str, metadata: &Metadata) -> Result<Board, ()> {
    let mines = parse_mine_locations(data).unwrap();

    let mut board = Board {
        fields: vec![vec![Field::new(); metadata.y_size as usize]; metadata.x_size as usize],
        metadata: metadata.clone(),
        mine_count: mines.len() as u32,
        open_fields: 0,
        total_fields: (metadata.y_size * metadata.x_size) as u32,
    };

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
                    if xx < 0
                        || xx >= metadata.x_size
                        || yy < 0
                        || yy >= metadata.y_size
                        || (zd == 0 && xd == 0)
                    {
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

pub fn parse_mine_locations(data: &str) -> Result<Vec<(i32, i32)>, ()> {
    let mut return_data = Vec::new();

    if data.chars().count() == 0 {
        return Ok(return_data);
    }

    let raw_open_fields_data: Vec<&str> = data.split(';').collect();

    for raw_open_field in raw_open_fields_data {
        if raw_open_field.contains('|') {
            let part = raw_open_field.split_once('|').unwrap();

            return_data.push((decode(part.0) as i32, decode(part.1) as i32));
        } else {
            raw_open_field
                .chars()
                .collect::<Vec<char>>()
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

pub fn parse_flag_data(data: &str) -> Result<Vec<FlagAction>, ()> {
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

            let time = part_two.1.parse::<i64>().unwrap();

            return_data.push(FlagAction {
                x: decode(part_one.0) as i32,
                y: decode(part_two.0) as i32,
                time,
                action: get_flag_type(action_type),
                total_time: time + return_data.iter().map(|x| x.time).sum::<i64>(),
            });
        } else {
            let mut chars = raw_open_field.chars();

            let x = decode(chars.next().unwrap().to_string().as_str()) as i32;
            let y = decode(chars.next().unwrap().to_string().as_str()) as i32;
            let action = get_flag_type(chars.next_back().unwrap());
            let time = chars.as_str().parse::<i64>().unwrap();

            return_data.push(FlagAction {
                x,
                y,
                action,
                time,
                total_time: time + return_data.iter().map(|x| x.time).sum::<i64>(),
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

pub fn parse_open_data(data: &str) -> Result<Vec<OpenAction>, ()> {
    let mut return_data = Vec::new();

    if data.chars().count() == 0 {
        return Ok(return_data);
    }

    let raw_open_fields_data: Vec<&str> = data.split(';').collect();

    for raw_open_field in raw_open_fields_data {
        if raw_open_field.contains('|') {
            let part_one = raw_open_field.split_once('|').unwrap();
            let part_two = part_one.1.split_once(':').unwrap();

            let time = part_two.1.parse::<i64>().unwrap();

            return_data.push(OpenAction {
                x: decode(part_one.0) as i32,
                y: decode(part_two.0) as i32,
                time,
                total_time: time + return_data.iter().map(|x| x.time).sum::<i64>(),
            });
        } else {
            let mut chars = raw_open_field.chars();

            let x = decode(chars.next().unwrap().to_string().as_str()) as i32;
            let y = decode(chars.next().unwrap().to_string().as_str()) as i32;
            let time = chars.as_str().parse::<i64>().unwrap();

            return_data.push(OpenAction {
                x,
                y,
                time,
                total_time: time + return_data.iter().map(|x| x.time).sum::<i64>(),
            });
        }
    }

    Ok(return_data)
}

pub fn parse_meta_data(data: &str) -> Result<Metadata, ()> {
    let data_split = data.split_once('x').unwrap();
    Ok(Metadata {
        x_size: i32::from_str(data_split.0).unwrap(),
        y_size: i32::from_str(data_split.1).unwrap(),
    })
}
