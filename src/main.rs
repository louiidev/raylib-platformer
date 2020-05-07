use raylib::prelude::*;
use specs::prelude::*;

use crate::systems::DrawSys;
use crate::components::{Transform, Position};

pub mod components;
pub mod systems;

pub const COLOUR: Color = Color::new(34, 32, 52, 255);
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 800;

fn window_should_close(world: &World) -> bool {
    let rl = world.read_resource::<RaylibHandle>();
    rl.window_should_close()
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello, World")
        .build();
    rl.set_target_fps(60);
    rl.set_mouse_scale(1., 1.);
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Transform>();
    world.insert(rl);
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(DrawSys { thread })
        .build();
    dispatcher.setup(&mut world);

    loop {
        dispatcher.dispatch(&world);
        {
            if window_should_close(&world) {
                break;
            }
        }
        world.maintain();
    }
}
