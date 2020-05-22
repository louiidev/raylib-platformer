use specs::{
    prelude::*,
    saveload::{
        DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker,
        SimpleMarkerAllocator,
    },
};
use crate::components::{DragBox, FallingBlock, Hitbox, Position, Sprite, ToolPalette, Triggerbox};
use crate::systems::{NetworkSync};
pub fn create_ent(world: &mut World, ty: ToolPalette, position: Position) {
    match ty {
        ToolPalette::Block => {
            world
                .create_entity()
                .with(Hitbox {
                    width: 32.,
                    height: 32.,
                    position,
                })
                .with(DragBox::default())
                .with(Sprite { name: "block".to_string() })
                .marked::<SimpleMarker<NetworkSync>>()
                .build();
        }
        ToolPalette::FallingBlock => {
            world
                .create_entity()
                .with(FallingBlock::default())
                .with(Sprite {
                    name: "fallingblock".to_string(),
                })
                .with(Triggerbox {
                    position: Position {
                        x: position.x,
                        y: position.y - 1.,
                    },
                    width: 32.,
                    height: 32.,
                })
                .with(DragBox::default())
                .marked::<SimpleMarker<NetworkSync>>()
                .with(Hitbox {
                    width: 32.,
                    height: 32.,
                    position,
                })
                .build();
        },
        _ => {

        }
    }
}
