use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;
use std::time::Duration;

use image::{Delay, Frame, GenericImage, ImageBuffer, Rgba};
use image::codecs::gif::GifEncoder;
use image::codecs::gif::Repeat::Infinite;

use crate::ActionType;
use crate::minesweeper_logic::{Board, FieldState};
use crate::parser::{Action, FlagAction, Metadata, OpenAction};

pub struct Renderer {
    pub(crate) metadata: Metadata,
    pub(crate) game_board: Board,
    pub(crate) open_data: Vec<OpenAction>,
    pub(crate) flag_data: Vec<FlagAction>,
}



impl Renderer {
    pub fn render_jpeg(&mut self) {
        if self.metadata.x_size >= 32 || self.metadata.y_size >= 32 {
            self.open_data.iter().for_each(|action| {
                self.game_board.open_field(action.x as usize, action.y as usize);
            });
            self.flag_data.iter().for_each(|action| match action.action {
                Action::Place => {
                    self.game_board.fields[action.y as usize][action.x as usize].field_state =
                        FieldState::Flagged
                }
                Action::Remove => {
                    self.game_board.fields[action.y as usize][action.x as usize].field_state =
                        FieldState::Closed
                }
            });
            let percentage_done = self.game_board.calculate_done_percentage();
            let frame = self.generate_image(percentage_done, "skin_full.png");
            frame.save("output.jpeg").unwrap();
        }
    }

    pub fn render_gif(&mut self) {
        let mut frames = Vec::new();

        let mut tick_map: BTreeMap<i64, Vec<ActionType>> = BTreeMap::new();
        for x in self.open_data.iter() {
            if let std::collections::btree_map::Entry::Vacant(e) = tick_map.entry(x.total_time) {
                e.insert(vec![ActionType::Open]);
            } else {
                tick_map
                    .get_mut(&x.total_time)
                    .unwrap()
                    .push(ActionType::Open);
            }
        }

        for x in self.flag_data.iter() {
            if let std::collections::btree_map::Entry::Vacant(e) = tick_map.entry(x.total_time) {
                e.insert(vec![ActionType::Flag]);
            } else {
                tick_map
                    .get_mut(&x.total_time)
                    .unwrap()
                    .push(ActionType::Flag);
            }
        }

        let frame = self.generate_image(0, "skin_20.png");
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
                self.open_data
                    .iter()
                    .filter(|flag| flag.total_time.eq(tick.0))
                    .for_each(|action| {
                        self.game_board.open_field(action.x as usize, action.y as usize);
                    });

                //Remove all elements which are less than tick.0
                self.open_data.retain(|open| open.total_time.gt(tick.0))
            }

            if tick.1.contains(&ActionType::Flag) {
                self.flag_data
                    .iter()
                    .filter(|flag| flag.total_time.eq(tick.0))
                    .for_each(|flag| match flag.action {
                        Action::Place => {
                            self.game_board.fields[flag.y as usize][flag.x as usize].field_state =
                                FieldState::Flagged
                        }
                        Action::Remove => {
                            self.game_board.fields[flag.y as usize][flag.x as usize].field_state =
                                FieldState::Closed
                        }
                    });
                //Remove all elements which are less than tick.0
                self.flag_data.retain(|flag| flag.total_time.gt(tick.0))
            }

            let frame = self.generate_image(
                if id == (tick_map.len() - 1) {
                    100
                } else {
                    ((id as f32 / tick_map.len() as f32) * 100.0) as u32
                },
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
        &mut self,
        percentage: u32,
        image_path: &str,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let progressbar_height = 4;
        let imgx = (self.metadata.x_size * 32) as u32;
        let imgy = ((self.metadata.y_size * 32) as u32) + progressbar_height;

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

        for x in 0..self.metadata.x_size as u32 {
            for y in 0..self.metadata.y_size as u32 {
                let field = &self.game_board.fields[x as usize][y as usize];
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
}
