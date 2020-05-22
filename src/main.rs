use raylib::prelude::*;
use specs::{
    prelude::*,
    saveload::{
        DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker,
        SimpleMarkerAllocator,
    },
};
use std::collections::HashMap;

extern crate strum; // 0.10.0
#[macro_use]
extern crate strum_macros; // 0.10.0


use strum::AsStaticRef;
use strum::IntoEnumIterator;

use crate::systems::{IconButtonsHandler, NetworkSync, Serialize, Deserialize, DrawSys, InputHandling, CollisionHandling, FallingBlockHandling, EditDragHandler, EditBtnHandle};
use crate::components::{EditState, Icon, Sprite, ToolPalette, Rect, EditBtn, Transform, Position, Hitbox, Triggerbox, Moveable, PlatformController, FallingBlock, DragBox};
use crate::utils::create_ent;

pub mod components;
pub mod systems;
pub mod utils;

pub const COLOUR: Color = Color::new(34, 32, 52, 255);
const WIDTH: i32 = 32 * 25;
const HEIGHT: i32 = 32 * 20;

fn window_should_close(world: &World) -> bool {
    let rl = world.read_resource::<RaylibHandle>();
    rl.window_should_close()
}

fn should_save(world: &World) {
    let mut edit_state = world.write_resource::<EditState>();
    if edit_state.should_save {
        Serialize.run_now(&world);
        edit_state.should_save = false;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Hello, World")
        .build();
    rl.set_target_fps(60);
    rl.set_mouse_scale(1., 1.);
    let mut world = World::new();
    let mut textures: HashMap<String, Texture2D> = {
        let mut tm = HashMap::new();

        for path in &["block", "fallingblock", "spikeblock"] {
            let texture = rl.load_texture(&thread, &format!("assets/{}.png", path)).unwrap();
            tm.insert(path.to_string(), texture);
        }
        tm
    };
    
    world.register::<Position>();
    world.register::<Transform>();
    world.register::<Hitbox>();
    world.register::<Triggerbox>();
    world.register::<PlatformController>();
    world.register::<Moveable>();
    world.register::<FallingBlock>();
    world.register::<EditBtn>();
    world.register::<Rect>();
    world.register::<DragBox>();
    world.register::<Sprite>();
    world.register::<Icon>();
    world.register::<SimpleMarker<NetworkSync>>();
    world.insert(SimpleMarkerAllocator::<NetworkSync>::new());
    world.insert(rl);
    world.insert(EditState::new());
    world
        .create_entity()
        .with(PlatformController::new())
        .with(Moveable::new())
        .with(Moveable::new().to_hitbox(Position {x: 150., y: 250. }))
        .build();
    
    world
        .create_entity()
        .with(Icon::new(raylib::consts::rIconDescription::RICON_EMPTYBOX, Position { x: 15., y: 15.})).build();
    world
        .create_entity()
        .with(Icon::new(raylib::consts::rIconDescription::RICON_BIN, Position { x: 15., y: 60.})).build();

    world
        .create_entity()
        .with(Icon::new(raylib::consts::rIconDescription::RICON_FILE_SAVE_CLASSIC, Position { x: 15., y: 105.})).build();

    Deserialize.run_now(&world);
        
        for (x, tool) in ToolPalette::iter().enumerate() {
            world
            .create_entity()
            .with(EditBtn {
                ty: tool,
                bounds: Rect {
                    width: 100.,
                    height: 50.,
                    position: Position {
                        x: 100. * x as f32 + 250.,
                        y: 100.
                    }
                },
                text: tool.as_static().to_string().to_lowercase()
            })
            .build();
        }
   
        
        
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(DrawSys { thread, textures })
        .with(InputHandling, "input_handling", &[])
        .with(CollisionHandling, "collision_handling", &["input_handling"])
        .with(FallingBlockHandling, "falling_block_handling", &[])
        .with(EditDragHandler, "edit_drag_handler", &[])
        .with(EditBtnHandle { selected_ent: None, selected_ty: None }, "edit_btn_handle", &[])
        .with(IconButtonsHandler, "icon_button", &[])
        .build();
    dispatcher.setup(&mut world);
    

    loop {
        dispatcher.dispatch(&world);
        {
            should_save(&world);
            if window_should_close(&world) {
                break;
            }
        }
        world.maintain();
    }
    
}
