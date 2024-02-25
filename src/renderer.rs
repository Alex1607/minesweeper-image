use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use clap::Parser;
use gif::{Encoder, Frame as GifFrame, Repeat};
use image::{Delay, Frame, GenericImage, ImageBuffer, Rgba};

use crate::error::MinesweeperError;
use crate::minesweeper_logic::{Board, FieldState};
use crate::parsers::parser::{ActionType, FlagAction, Metadata, OpenAction};

const BAR_LENGTH: usize = 50;

pub struct Renderer<'a> {
    pub(crate) metadata: Metadata,
    game_board: Board,
    open_data: Vec<OpenAction>,
    flag_data: Vec<FlagAction>,
    image_data: Imagedata,
    encoder: Option<Encoder<File>>,
    options: &'a RenderOptions,
}

#[derive(Parser)]
#[command()]
pub struct RenderOptions {
    #[arg(
        short,
        long,
        value_enum,
        help = "Choose either 'image' or 'gif' to force that type to be generated. If not set, it will choose automatically based on the size."
    )]
    pub(crate) force_type: Option<RenderType>,
    #[arg(
        long,
        help = "To render the GIF or Image with a custom texture set the path relativ to the executable"
    )]
    pub(crate) custom_textures: Option<String>,
    #[arg(short, long, help = "Enable this if you want to insert data yourself.")]
    pub custom_input: bool,
    #[arg(short, long, help = "Should the GIF repeat?")]
    repeat: bool,
}

#[derive(Copy, Clone)]
pub enum RenderType {
    Image,
    Gif,
}

impl std::str::FromStr for RenderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "image" => Ok(RenderType::Image),
            "gif" => Ok(RenderType::Gif),
            _ => Err(format!("Unknown render type: {}", s)),
        }
    }
}

struct Imagedata {
    zero: ImageBuffer<Rgba<u8>, Vec<u8>>,
    one: ImageBuffer<Rgba<u8>, Vec<u8>>,
    two: ImageBuffer<Rgba<u8>, Vec<u8>>,
    three: ImageBuffer<Rgba<u8>, Vec<u8>>,
    four: ImageBuffer<Rgba<u8>, Vec<u8>>,
    five: ImageBuffer<Rgba<u8>, Vec<u8>>,
    six: ImageBuffer<Rgba<u8>, Vec<u8>>,
    seven: ImageBuffer<Rgba<u8>, Vec<u8>>,
    eight: ImageBuffer<Rgba<u8>, Vec<u8>>,
    tnt: ImageBuffer<Rgba<u8>, Vec<u8>>,
    empty: ImageBuffer<Rgba<u8>, Vec<u8>>,
    flag: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl Imagedata {
    pub fn new(sprite_data: &[u8]) -> Imagedata {
        let im = &mut image::load_from_memory(sprite_data).expect("Custom Textures file not found");

        let zero = im.sub_image(0, 0, 32, 32).to_image();
        let one = im.sub_image(32, 0, 32, 32).to_image();
        let two = im.sub_image(32 * 2, 0, 32, 32).to_image();
        let three = im.sub_image(32 * 3, 0, 32, 32).to_image();
        let four = im.sub_image(32 * 4, 0, 32, 32).to_image();
        let five = im.sub_image(32 * 5, 0, 32, 32).to_image();
        let six = im.sub_image(32 * 6, 0, 32, 32).to_image();
        let seven = im.sub_image(32 * 7, 0, 32, 32).to_image();
        let eight = im.sub_image(32 * 8, 0, 32, 32).to_image();
        let tnt = im.sub_image(32 * 9, 0, 32, 32).to_image();
        let empty = im.sub_image(32 * 10, 0, 32, 32).to_image();
        let flag = im.sub_image(32 * 11, 0, 32, 32).to_image();

        Imagedata {
            zero,
            one,
            two,
            three,
            four,
            five,
            six,
            seven,
            eight,
            tnt,
            empty,
            flag,
        }
    }
}

impl<'a> Renderer<'a> {
    pub fn new(
        metadata: Metadata,
        game_board: Board,
        open_data: Vec<OpenAction>,
        flag_data: Vec<FlagAction>,
        sprite_data: &[u8],
        options: &'a RenderOptions,
    ) -> Renderer<'a> {
        Renderer {
            metadata,
            game_board,
            open_data,
            flag_data,
            image_data: Imagedata::new(sprite_data),
            encoder: None,
            options,
        }
    }

    pub fn render_jpeg(&mut self) -> Result<(), MinesweeperError> {
        self.flag_data
            .iter()
            .for_each(|action| action.perform_action(&mut self.game_board));

        self.open_data.iter().for_each(|action| {
            self.game_board
                .open_field(action.x as usize, action.y as usize);
        });

        let percentage_done = self.game_board.calculate_done_percentage();
        let frame = self.generate_image(percentage_done)?;

        println!("[{}] 100%", "#".repeat(BAR_LENGTH));

        frame
            .save("output.webp")
            .map_err(|_| MinesweeperError::ImageSave)
    }

    pub fn render_gif(&mut self) -> Result<(), MinesweeperError> {
        let tick_map: BTreeMap<i64, Vec<ActionType>> = self.create_tick_map();
        let mut current_image = 0;

        self.encoder =
            Some(self.create_encoder(self.metadata.x_size as u32, self.metadata.y_size as u32)?);

        let frame = self.generate_image(0)?;
        self.encode_frame_to_gif(
            Frame::from_parts(
                frame,
                0,
                0,
                Delay::from_saturating_duration(Duration::from_secs(1)),
            ),
            current_image,
            tick_map.len(),
        )?;
        current_image += 1;

        for (id, tick) in tick_map.iter().enumerate() {
            let next_tick = tick_map.keys().nth(id + 1);

            let duration = if let Some(next) = next_tick {
                Duration::from_millis(((next - tick.0) * 50) as u64)
            } else {
                Duration::from_secs(15)
            };

            if tick.1.contains(&ActionType::Flag) {
                self.flag_data
                    .iter()
                    .filter(|flag| flag.total_time.eq(tick.0))
                    .for_each(|flag| flag.perform_action(&mut self.game_board));
                //Remove all elements which are less than tick.0
                self.flag_data.retain(|flag| flag.total_time.gt(tick.0))
            }

            if tick.1.contains(&ActionType::Open) {
                self.open_data
                    .iter()
                    .filter(|flag| flag.total_time.eq(tick.0))
                    .for_each(|action| {
                        self.game_board
                            .open_field(action.x as usize, action.y as usize);
                    });

                //Remove all elements which are less than tick.0
                self.open_data.retain(|open| open.total_time.gt(tick.0))
            }

            let frame = self.generate_image(if id == (tick_map.len() - 1) {
                100
            } else {
                ((id as f32 / tick_map.len() as f32) * 100.0) as u32
            })?;

            self.encode_frame_to_gif(
                Frame::from_parts(frame, 0, 0, Delay::from_saturating_duration(duration)),
                current_image,
                tick_map.len(),
            )?;
            current_image += 1;
        }

        print!("\r[{}] 100%", "#".repeat(BAR_LENGTH));

        Ok(())
    }

    fn encode_frame_to_gif(
        &mut self,
        image: Frame,
        current_image_id: usize,
        total_frames: usize,
    ) -> Result<(), MinesweeperError> {
        let (width, height) = image.buffer().dimensions();

        let percent_complete = (current_image_id as f32 / total_frames as f32 * 100.0) as usize;
        let num_hashes = percent_complete * BAR_LENGTH / 100;
        print!(
            "\r[{}{}] {percent_complete:}%",
            "#".repeat(num_hashes),
            " ".repeat(BAR_LENGTH - num_hashes)
        );
        std::io::stdout().flush().unwrap_or_default();

        let frame_delay = image.delay().numer_denom_ms().0 / 10;
        let rbga_frame = &mut image.into_buffer();
        let mut frame = GifFrame::from_rgba_speed(width as u16, height as u16, rbga_frame, 1);
        frame.delay = frame_delay as u16;
        frame.dispose = gif::DisposalMethod::Keep;

        self.encoder
            .as_mut()
            .unwrap()
            .write_frame(&frame)
            .map_err(|_| MinesweeperError::GifEncoding)?;

        Ok(())
    }

    fn create_encoder(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<Encoder<File>, MinesweeperError> {
        let mut encoder = Encoder::new(
            File::create("output.gif").unwrap(),
            width as u16,
            height as u16,
            &[],
        )
        .unwrap();

        encoder
            .set_repeat(if self.options.repeat {
                Repeat::Infinite
            } else {
                Repeat::Finite(0)
            })
            .map_err(|_| MinesweeperError::GifEncoding)?;
        Ok(encoder)
    }

    fn create_tick_map(&mut self) -> BTreeMap<i64, Vec<ActionType>> {
        let mut tick_map = BTreeMap::new();

        for x in self.open_data.iter() {
            Self::insert_action(&mut tick_map, x.total_time, ActionType::Open)
        }

        for x in self.flag_data.iter() {
            Self::insert_action(&mut tick_map, x.total_time, ActionType::Flag)
        }

        tick_map
    }

    fn insert_action(
        tick_map: &mut BTreeMap<i64, Vec<ActionType>>,
        total_time: i64,
        action: ActionType,
    ) {
        if let std::collections::btree_map::Entry::Vacant(e) = tick_map.entry(total_time) {
            e.insert(vec![action]);
        } else {
            tick_map.get_mut(&total_time).unwrap().push(action);
        }
    }

    fn generate_image(
        &mut self,
        percentage: u32,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, MinesweeperError> {
        let progressbar_height = 4;
        let imgx = (self.metadata.x_size * 32) as u32;
        let imgy = ((self.metadata.y_size * 32) as u32) + progressbar_height;

        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

        for x in 0..self.metadata.x_size as u32 {
            for y in 0..self.metadata.y_size as u32 {
                let field = &self.game_board.fields[y as usize][x as usize];

                // Only render fields that got changed in the last iteration
                if !self.game_board.changed_fields[y as usize][x as usize] {
                    continue;
                }

                let xx = x * 32;
                let yy = y * 32;
                if field.field_state == FieldState::Closed {
                    imgbuf
                        .copy_from(&self.image_data.empty, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?;
                    continue;
                }
                if field.field_state == FieldState::Flagged {
                    imgbuf
                        .copy_from(&self.image_data.flag, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?;
                    continue;
                }
                if field.mine {
                    imgbuf
                        .copy_from(&self.image_data.tnt, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?;
                    continue;
                }
                match field.value {
                    0 => imgbuf
                        .copy_from(&self.image_data.zero, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?,
                    1 => imgbuf
                        .copy_from(&self.image_data.one, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?,
                    2 => imgbuf
                        .copy_from(&self.image_data.two, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?,
                    3 => imgbuf
                        .copy_from(&self.image_data.three, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?,
                    4 => imgbuf
                        .copy_from(&self.image_data.four, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?,
                    5 => imgbuf
                        .copy_from(&self.image_data.five, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?,
                    6 => imgbuf
                        .copy_from(&self.image_data.six, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?,
                    7 => imgbuf
                        .copy_from(&self.image_data.seven, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?,
                    8 => imgbuf
                        .copy_from(&self.image_data.eight, xx, yy)
                        .map_err(|_| MinesweeperError::ImageInsertion)?,
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

        //Reset the changed fields after they got rendered
        self.game_board
            .changed_fields
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|field| *field = false));

        Ok(imgbuf)
    }
}
