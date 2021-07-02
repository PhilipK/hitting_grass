use components::Blade;
use components::Position;
use components::Robot;
use core::time;
use legion::systems::CommandBuffer;
use legion::*;
use legion::{Resources, World};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::thread::{self};
use vector2d::Vector2D;

mod components;
mod systems;

pub struct Game {
    pub resources: Resources,
    pub schedule: Schedule,
    pub world: World,
    pub sdl_context: sdl2::Sdl,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

pub enum GlProfile {
    _Core43,
    ES3,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let mut resources = Resources::default();
        let world = initialize_world(&mut resources);

        #[cfg(not(debug_assertions))]
        let bounds = video_subsystem.display_bounds(0)?;
        #[cfg(not(debug_assertions))]
        let window_x = bounds.width();
        #[cfg(debug_assertions)]
        let window_x = 1920;
        #[cfg(not(debug_assertions))]
        let window_y = bounds.height();
        #[cfg(debug_assertions)]
        let window_y = 1080;

        let profile = GlProfile::ES3;
        let context_params = match profile {
            GlProfile::_Core43 => (sdl2::video::GLProfile::Core, 4, 3),
            GlProfile::ES3 => (sdl2::video::GLProfile::GLES, 3, 0),
        };

        video_subsystem
            .gl_attr()
            .set_context_profile(context_params.0);
        video_subsystem
            .gl_attr()
            .set_context_major_version(context_params.1);
        video_subsystem
            .gl_attr()
            .set_context_minor_version(context_params.2);

        let mut window = video_subsystem
            .window("Robot Cards", window_x, window_y)
            .resizable()
            .borderless()
            .opengl()
            .build()
            .expect("Could not initialize vido system");
        window
            .set_size(window_x, window_y)
            .expect("Should be able to set window size");

        let canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .expect("Could not make a canvas");

        let schedule = systems::setup_systems();

        Ok(Game {
            world,
            canvas,
            resources,
            sdl_context,
            schedule,
        })
    }

    pub fn tick(&mut self) -> ConitueToken {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return ConitueToken::Terminate;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return ConitueToken::Terminate,
                _ => {}
            };
        }

        self.schedule.execute(&mut self.world, &mut self.resources);

        return ConitueToken::Continue;
    }
}

fn initialize_world(resources: &mut Resources) -> World {
    let mut world = World::default();
    let mut buffer = CommandBuffer::new(&world);

    buffer.push((
        Robot {
            radius: 1.,
            cutting_height: 0.4,
        },
        Position {
            position: Vector2D { x: 50., y: 50. },
        },
    ));

    for x in 0..10 {
        for y in 0..10 {
            buffer.push((
                Blade { height: 1. },
                Position {
                    position: Vector2D {
                        x: x as f64,
                        y: y as f64,
                    },
                },
            ));
        }
    }

    buffer.flush(&mut world, resources);

    world
}

pub enum ConitueToken {
    Continue,
    Terminate,
}

impl ConitueToken {
    pub fn should_terminate(&self) -> bool {
        matches!(self, &ConitueToken::Terminate)
    }
}

fn main() {
    let mut game = Game::new().unwrap();

    loop {
        let res = game.tick();
        if res.should_terminate() {
            return;
        }
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
        let color = Color::RGB(0, 0, 0);
        game.canvas.set_draw_color(color);
        game.canvas.clear();
        let color = Color::RGB(125, 12, 44);
        game.canvas.set_draw_color(color);
        game.canvas.fill_rect(Rect::new(10, 10, 50, 580)).unwrap();
        game.canvas.present();
    }
}
