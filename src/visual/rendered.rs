use crate::generation::Generator;
use crate::fixed_size_pipe::FixedSizePipe;
use ggez::{ContextBuilder, conf, Context, GameError};
use ggez::conf::{WindowMode, FullscreenType};
use ggez::event::{run, EventHandler, KeyMods};
use ggez::graphics::{Mesh, BLACK, draw, DrawParam, present, clear, WHITE};
use mint::Point2;
use std::time::{Instant, Duration};
use std::thread::sleep;
use ggez::input::keyboard::KeyCode;

const PIXEL_SCALE: usize = 10;

use Going::*;

#[derive(Eq, PartialEq)]
enum Going { Left, Right, Not }

const MAX: isize = std::f32::MAX as isize;

pub struct RendVisual<G: Generator<f32>> {
    width: usize,
    height: usize,
    pipe: FixedSizePipe<f32>,
    generator: G,
    at: isize,
    going: Going,

    ms_per_frame: u128,
    last_update: Instant,
}

impl<G: Generator<f32>> EventHandler for RendVisual<G> {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        match self.going {
            Left => {
                self.at -= 1;
                self.pipe.push_left(self.generator.at(self.at % MAX));
            },
            Right => {
                self.at += 1;
                self.pipe.push_right(self.generator.at(
                    self.at % MAX + self.pipe.len() as isize));
            },
            Not => (),
        }

        // fps enforcement
        let elapsed = self.last_update.elapsed().as_millis();
        if elapsed < self.ms_per_frame {
            sleep(Duration::from_millis((self.ms_per_frame - elapsed) as u64))
        }
        self.last_update = Instant::now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        clear(ctx, WHITE);

        if self.pipe.len() >= 3 {
            let line = Mesh::new_line(
                ctx,
                &self.pipe.as_vec().iter()
                    .zip(0..)
                    .map(|(&&y, x)| Point2 {
                        x: (x * PIXEL_SCALE) as f32,
                        y: (1. - y) * (self.height * PIXEL_SCALE) as f32,
                    })
                    .collect::<Vec<_>>(),
                1.,
                BLACK,
            )?;
            draw(ctx, &line, DrawParam::default())?;
        }

        present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _: bool) {
        match key {
            KeyCode::Up => self.set_fps(self.fps() + 5.),
            KeyCode::Down => self.set_fps(self.fps() - 5.),
            KeyCode::Left => self.going = Left,
            KeyCode::Right => self.going = Right,
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods) {
        match key {
            KeyCode::Left if self.going == Left => self.going = Not,
            KeyCode::Right if self.going == Right => self.going = Not,
            _ => (),
        }
    }
}

fn fps2ms_per_frame(fps: u16) -> u128 {
    1000 / fps as u128
}

impl<G: Generator<f32>> RendVisual<G> {
    pub fn new(generator: G, height: usize, width: usize) -> Self {
        let mut it = Self {
            width,
            height,
            pipe: FixedSizePipe::new(width),
            generator,
            at: 0,
            going: Not,

            ms_per_frame: fps2ms_per_frame(2),
            last_update: Instant::now(),
        };
        it.set_fps(60.);
        it
    }

    fn fps(&self) -> f64 {
        (1000 / self.ms_per_frame) as f64
    }

    fn set_fps(&mut self, fps: f64) {
        let new_fps = match fps {
            f if f < 10. => 10.,
            f if f > 60. => 60.,
            f => f,
        };
        self.ms_per_frame = (1000. / new_fps) as u128;
    }

    pub fn run(&mut self) {
        let wm = WindowMode {
            width: (self.width * PIXEL_SCALE) as f32,
            height: (self.height * PIXEL_SCALE) as f32,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.,
            min_height: 0.,
            max_width: 0.,
            max_height: 0.,
            resizable: false
        };

        let (ref mut ctx, ref mut event_loop)
            = ContextBuilder::new("game", "author")
            .conf(conf::Conf::new())
            .window_mode(wm)
            .build()
            .unwrap();

        run(ctx, event_loop, self).expect("crashed")
    }
}
