use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::i64;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use image::{Delay, Frame, GenericImage, ImageBuffer, Rgba};
use image::codecs::gif::GifEncoder;
use image::codecs::gif::Repeat::Infinite;

const BASE: i64 = 62;
const CHARACTERS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

//TODO: Add percentage bar at the bottom
//      Show win or loose info at the end?
fn main() {
    // let data = "1=15x15+B14262B22393E334A44565864767B758593ADAABEB3CEC9DED5E7E+000;001;3325;342;255;3710;383;3A21;4922;482;474;464;361;3513;453;4425;6428;847;944;B39;B414;C46;A49;A53;B53;E619;E89;E94;CA10;DA5;EA5;EB5;EE36;AE15;9D5;8D6;6A29;5A4;698;5916;0B33;1C11;0C2;0D10;5848;5729;671;6631;5512;652;754+3216P;245P;2612P;3924P;4463P;436P;4411R;5425P;748P;A314P;C36P;E545P;E79P;D98P;BA15P;AD33P;BE6P;CE4P;DE3P;7B52P;5926P;5911R;4A9P;2B12P;1B5P;3E42P;6837P;576P;5722R;5618P;763P".to_string();
    let data = "1=15x15+60B0A1E1024262821384C566A627E71868697A5BDB4CDC5DED2E9E+000;001;3439;332;325;2515;352;454;3019;7218+2021P;2410P;3149P".to_string();
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
    let mut game_board = parse_mine_data(raw_mine_data, &metadata).unwrap();
    let open_data = &mut parse_open_data(raw_open_data).unwrap();
    let flag_data = &mut parse_flag_data(raw_flag_data).unwrap();

    // println!("{metadata:?}\n{game_board:?}\n{open_data:?}\n{flag_data:?}");

    if metadata.x_size >= 32 || metadata.y_size >= 32 {
        open_data.iter().for_each(|action| {
            game_board.open_field(action.x as usize, action.y as usize);
        });
        flag_data.iter().for_each(|action| match action.action {
            Action::Place => {
                game_board.fields[action.y as usize][action.x as usize].field_state =
                    FieldState::Flagged
            }
            Action::Remove => {
                game_board.fields[action.y as usize][action.x as usize].field_state =
                    FieldState::Closed
            }
        });
        let percentage_done = game_board.calculate_done_percentage();
        let frame = generate_image(&mut game_board, &metadata, percentage_done, "skin_full.png");
        frame.save("output.jpeg").unwrap();
        return;
    }

    let mut frames = Vec::new();

    let mut tick_map: BTreeMap<i64, Vec<ActionType>> = BTreeMap::new();
    for x in open_data.iter() {
        if let std::collections::btree_map::Entry::Vacant(e) = tick_map.entry(x.total_time) {
            e.insert(vec![ActionType::Open]);
        } else {
            tick_map
                .get_mut(&x.total_time)
                .unwrap()
                .push(ActionType::Open);
        }
    }

    for x in flag_data.iter() {
        if let std::collections::btree_map::Entry::Vacant(e) = tick_map.entry(x.total_time) {
            e.insert(vec![ActionType::Flag]);
        } else {
            tick_map
                .get_mut(&x.total_time)
                .unwrap()
                .push(ActionType::Flag);
        }
    }

    let frame = generate_image(&mut game_board, &metadata, 0, "skin_20.png");
    frames.push(Frame::from_parts(
        frame,
        0,
        0,
        Delay::from_saturating_duration(Duration::from_secs(1)),
    ));

    for (id, tick) in tick_map.iter().enumerate() {
        let next_tick = tick_map.keys().nth(id + 1);
        let duration = if let Some(next) = next_tick {
            Duration::from_millis(((next - tick.0) * 50) as u64)
        } else {
            Duration::from_secs(15)
        };

        if tick.1.contains(&ActionType::Open) {
            open_data
                .iter()
                .filter(|flag| flag.total_time.eq(tick.0))
                .for_each(|action| {
                    game_board.open_field(action.x as usize, action.y as usize);
                });

            //Remove all elements which are less than tick.0
            open_data.retain(|open| open.total_time.gt(tick.0))
        }

        if tick.1.contains(&ActionType::Flag) {
            flag_data
                .iter()
                .filter(|flag| flag.total_time.eq(tick.0))
                .for_each(|flag| match flag.action {
                    Action::Place => {
                        game_board.fields[flag.y as usize][flag.x as usize].field_state =
                            FieldState::Flagged
                    }
                    Action::Remove => {
                        game_board.fields[flag.y as usize][flag.x as usize].field_state =
                            FieldState::Closed
                    }
                });
            //Remove all elements which are less than tick.0
            flag_data.retain(|flag| flag.total_time.gt(tick.0))
        }

        let frame = generate_image(
            &mut game_board,
            &metadata,
            if id == (tick_map.len() - 1) { 100 } else { ((id as f32 / tick_map.len() as f32) * 100.0) as u32 },
            "skin_20.png",
        );
        frames.push(Frame::from_parts(
            frame,
            0,
            0,
            Delay::from_saturating_duration(duration),
        ));
    }

    let mut gif_encoder = GifEncoder::new(File::create("output.gif").unwrap());
    gif_encoder.set_repeat(Infinite).unwrap();
    let le = frames.len();
    for (i, frame) in frames.into_iter().enumerate() {
        println!("Encoding frame {} of {}", i, le);
        gif_encoder.encode_frame(frame).unwrap();
    }
}

fn generate_image(
    board: &mut Board,
    metadata: &Metadata,
    percentage: u32,
    image_path: &str,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let progressbar_height = 4;
    let imgx = (metadata.x_size * 32) as u32;
    let imgy = ((metadata.y_size * 32) as u32) + progressbar_height;

    let im = &mut image::open(Path::new(image_path)).unwrap();
    let zero = &im.sub_image(0, 0, 32, 32).to_image();
    let one = &im.sub_image(32, 0, 32, 32).to_image();
    let two = &im.sub_image(32 * 2, 0, 32, 32).to_image();
    let three = &im.sub_image(32 * 3, 0, 32, 32).to_image();
    let four = &im.sub_image(32 * 4, 0, 32, 32).to_image();
    let five = &im.sub_image(32 * 5, 0, 32, 32).to_image();
    let six = &im.sub_image(32 * 6, 0, 32, 32).to_image();
    let seven = &im.sub_image(32 * 7, 0, 32, 32).to_image();
    let eight = &im.sub_image(32 * 8, 0, 32, 32).to_image();
    let tnt = &im.sub_image(32 * 9, 0, 32, 32).to_image();
    let closed = &im.sub_image(32 * 10, 0, 32, 32).to_image();
    let flag = &im.sub_image(32 * 11, 0, 32, 32).to_image();

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for x in 0..metadata.x_size as u32 {
        for y in 0..metadata.y_size as u32 {
            let field = &mut board.fields[x as usize][y as usize];
            let xx = x * 32;
            let yy = y * 32;
            if field.field_state == FieldState::Closed {
                imgbuf
                    .copy_from(closed, xx, yy)
                    .expect("TODO: panic message");
                continue;
            }
            if field.field_state == FieldState::Flagged {
                imgbuf.copy_from(flag, xx, yy).expect("TODO: panic message");
                continue;
            }
            if field.mine {
                imgbuf.copy_from(tnt, xx, yy).expect("TODO: panic message");
                continue;
            }
            match field.value {
                0 => imgbuf.copy_from(zero, xx, yy).expect("TODO: panic message"),
                1 => imgbuf.copy_from(one, xx, yy).expect("TODO: panic message"),
                2 => imgbuf.copy_from(two, xx, yy).expect("TODO: panic message"),
                3 => imgbuf
                    .copy_from(three, xx, yy)
                    .expect("TODO: panic message"),
                4 => imgbuf.copy_from(four, xx, yy).expect("TODO: panic message"),
                5 => imgbuf.copy_from(five, xx, yy).expect("TODO: panic message"),
                6 => imgbuf.copy_from(six, xx, yy).expect("TODO: panic message"),
                7 => imgbuf
                    .copy_from(seven, xx, yy)
                    .expect("TODO: panic message"),
                8 => imgbuf
                    .copy_from(eight, xx, yy)
                    .expect("TODO: panic message"),
                _ => unreachable!(),
            }
        }
        let pixel_coloring = (percentage * imgx) / 100;

        for x in 0..imgx {
            for y in (imgy - progressbar_height)..imgy {
                let pixel = imgbuf.get_pixel_mut(x, y);
                if x <= pixel_coloring {
                    *pixel = Rgba([103, 149, 60, 255]);
                } else {
                    *pixel = Rgba([0, 0, 0, 255]);
                }
            }
        }
    }

    imgbuf
}

fn parse_mine_data(data: &str, metadata: &Metadata) -> Result<Board, ()> {
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

fn parse_mine_locations(data: &str) -> Result<Vec<(i32, i32)>, ()> {
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

fn parse_meta_data(data: &str) -> Result<Metadata, ()> {
    let data_split = data.split_once('x').unwrap();
    Ok(Metadata {
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

impl Board {
    fn open_field(&mut self, x: usize, y: usize) {
        let field = &mut self.fields[y][x];

        //If flagged or already open return
        if field.field_state != FieldState::Closed {
            return;
        }

        field.field_state = FieldState::Open;
        self.open_fields += 1;

        if field.mine {
            return;
        }

        if field.value == 0 {
            for xd in -1..=1_i32 {
                for yd in -1..=1_i32 {
                    let xx = xd + x as i32;
                    let yy = yd + y as i32;
                    if xx < 0
                        || xx >= self.metadata.x_size
                        || yy < 0
                        || yy >= self.metadata.y_size
                        || xd == 0 && yd == 0
                    {
                        continue;
                    }
                    self.open_field(xx as usize, yy as usize)
                }
            }
        }
    }

    fn calculate_done_percentage(&self) -> u32 {
        ((self.open_fields as f32 / (self.total_fields - self.mine_count) as f32) * 100_f32) as u32
    }
}

#[derive(Debug, Clone)]
struct Metadata {
    x_size: i32,
    y_size: i32,
}

#[derive(Debug)]
struct FlagAction {
    x: i32,
    y: i32,
    time: i64,
    action: Action,
    total_time: i64,
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
    total_time: i64,
}

#[derive(Debug)]
struct Board {
    fields: Vec<Vec<Field>>,
    metadata: Metadata,
    open_fields: u32,
    mine_count: u32,
    total_fields: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub value: u8,
    pub field_state: FieldState,
    pub mine: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FieldState {
    Open,
    Closed,
    Flagged,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ActionType {
    Open,
    Flag,
}

impl Field {
    pub(crate) fn new() -> Self {
        Field {
            value: 0,
            field_state: FieldState::Closed,
            mine: false,
        }
    }
}
