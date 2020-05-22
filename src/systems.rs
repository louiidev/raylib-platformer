use crate::components::{EditState, Icon, ToolPalette, Sprite, CollisionsPoint, FallingBlock, Hitbox, Moveable, PlatformController, Triggerbox, EditBtn, DragBox, Position};
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use specs::{
    prelude::*,
    saveload::{
        DeserializeComponents, MarkedBuilder, MarkerAllocator, SerializeComponents, SimpleMarker,
        SimpleMarkerAllocator,
    },
};
use std::{convert::Infallible, fmt};
use strum::AsStaticRef;
use std::collections::HashMap;

const DEBUG: bool = false;

// System is not thread safe
pub struct DrawSys {
    pub thread: RaylibThread,
    pub textures: HashMap<String, Texture2D>
}
impl<'a> System<'a> for DrawSys {
    type SystemData = (
        WriteExpect<'a, EditState>,
        WriteExpect<'a, RaylibHandle>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Hitbox>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, EditBtn>,
        ReadStorage<'a, Icon>
    );

    fn run(&mut self, (mut edit_state, mut rl, sprites, hitboxs, positions, edit_btns, icons): Self::SystemData) {
        let width = rl.get_screen_width();
        let height = rl.get_screen_height();
        let cols = width / 32;
        let rows = height / 32;
        if rl.is_key_pressed(KEY_P) {
            edit_state.editting = !edit_state.editting;
        }
        let mut d = rl.begin_drawing(&self.thread);
        d.clear_background(crate::COLOUR);
        for (sprite, hitbox, position) in (&sprites, (&hitboxs).maybe(), (&positions).maybe()).join() {
            if hitbox.is_some() || position.is_some() {
                
                let texture = self.textures.get(&sprite.name).unwrap_or_else(|| panic!("cannot find sprite for {}", sprite.name));
                let v_pos: Vector2 = if let Some(hitbox) = hitbox {
                    hitbox.position.into()
                } else {
                    let p = *position.unwrap();
                    p.into()
                };
                d.draw_texture_ex(texture, v_pos, 0., 1., Color::WHITE);
            }
           
        }
        if DEBUG {
            for h in hitboxs.join() {
                let rec: Rectangle = h.clone().into();
                d.draw_rectangle_lines_ex(rec, 2, Color::RED);
            }
        }
        if edit_state.editting {
            for edit_btn in edit_btns.join() {
                let rec: Rectangle = edit_btn.bounds.clone().into();
                let texture_name = edit_btn.text.to_string().to_lowercase();
                d.draw_rectangle(rec.x as i32, rec.y as i32, 70, 70, Color::BLUE);
                // d.draw_text(&edit_btn.text, rec.x as i32 + 10, rec.y as i32 + 10, 20, Color::WHITE);
                d.draw_texture_ex(self.textures.get(&*texture_name).unwrap(), Vector2::new(rec.x + 2., rec.y + 2.),0., 1.5, Color::WHITE)
            }

            let border_color = Color::new(Color::GRAY.r, Color::GRAY.g, Color::GRAY.b, 50);
            for col in 0..cols {
                d.draw_line(col * 32, 0, col * 32, height,border_color);  
            }

            for row in 0..rows {   
                d.draw_line(0, row * 32, width, row * 32, border_color);
            }

            for icon in icons.join() {
                let pos: Vector2 = icon.position.into();
                d.draw_icon(
                    icon.icon.0,
                    pos,
                    2,
                    Color::WHITE,
                );
            }
           
        }
    }
}

const MAX_COYOTE_TIME: f32 = 15.;
const TIME_TO_JUMP_HEIGHT: f32 = 0.55;
const JUMP_HEIGHT: f32 = 66.0;
const HOZ_SPEED: f32 = 200.0;
const GRAVITY: f32 = (JUMP_HEIGHT * 2.) / (TIME_TO_JUMP_HEIGHT * TIME_TO_JUMP_HEIGHT);
const JUMP_VELOCITY: f32 = GRAVITY * TIME_TO_JUMP_HEIGHT;
const PADDING: f32 = 0.05;
pub struct InputHandling;
impl<'a> System<'a> for InputHandling {
    type SystemData = (
        WriteExpect<'a, EditState>,
        ReadExpect<'a, RaylibHandle>,
        WriteStorage<'a, PlatformController>,
        WriteStorage<'a, Moveable>,
    );

    fn run(&mut self, (edit_state, rl, mut platform_controller, mut moveable): Self::SystemData) {
        if edit_state.editting { return; }
        for (controller, m) in (&mut platform_controller, &mut moveable).join() {
            m.velocity.x = if rl.is_key_down(KEY_LEFT) {
                -HOZ_SPEED
            } else if rl.is_key_down(KEY_RIGHT) {
                HOZ_SPEED
            } else {
                0.0
            };

            if rl.is_key_down(KEY_SPACE)
                && (controller.can_jump || controller.coyote_time < MAX_COYOTE_TIME)
            {
                m.velocity.y = -JUMP_VELOCITY;
                controller.can_jump = false;
                controller.coyote_time = MAX_COYOTE_TIME;
            }
        }
    }
}

pub struct CollisionHandling;
impl<'a> System<'a> for CollisionHandling {
    type SystemData = (
        WriteExpect<'a, EditState>,
        ReadExpect<'a, RaylibHandle>,
        WriteStorage<'a, Moveable>,
        WriteStorage<'a, Hitbox>,
        Entities<'a>,
        WriteStorage<'a, PlatformController>,
    );
    fn run(&mut self, (edit_state, rl, mut moveable, mut hitboxs, entities, mut controller): Self::SystemData) {
        if edit_state.editting { return; }
        let delta = rl.get_frame_time();
        for (m, entity, mut control) in (&mut moveable, &entities, (&mut controller).maybe()).join()
        {
            let hbs: Vec<Hitbox> = (&entities, &hitboxs)
                .join()
                .filter(|(e, _)| *e != entity)
                .map(|(_, h)| *h)
                .collect();
            // check vert movement
            let hitbox = hitboxs
                .get_mut(entity)
                .expect("Moveable component needs hitbox");
            let mut pot_rec_x: Rectangle = hitbox.clone().into();
            pot_rec_x.x += m.velocity.x * delta;
            let mut pot_rec_y: Rectangle = hitbox.clone().into();
            pot_rec_y.y += m.velocity.y * delta;
            let mut collision_x = false;
            let mut collision_y = false;
            for h in hbs {
                let rect = h.clone().into();
                if pot_rec_x.check_collision_recs(&rect) {
                    collision_x = true;
                    m.velocity.x = 0.0;
                }
                if pot_rec_y.check_collision_recs(&rect) {
                    collision_y = true;
                    m.velocity.y = 0.0;
                    // collision ground
                    if pot_rec_y.y > hitbox.position.y {
                        hitbox.position.y = rect.y - hitbox.height - PADDING;
                        if let Some(control) = &mut control {
                            control.coyote_time = 0.;
                            control.can_jump = true;
                        }
                    } else {
                        hitbox.position.y = rect.y + rect.height + PADDING;
                    }
                }
            }
            if !collision_x {
                hitbox.position.x = pot_rec_x.x;
            }

            if !collision_y {
                if let Some(control) = &mut control {
                    control.coyote_time += 1.;
                    control.can_jump = false;
                }
                hitbox.position.y += m.velocity.y * delta;
                m.velocity.y += GRAVITY * delta;
            }
        }
    }
}

const MAX_FALL_COUNT: u32 = 80;
pub struct FallingBlockHandling;
impl<'a> System<'a> for FallingBlockHandling {
    type SystemData = (
        ReadExpect<'a, EditState>,
        WriteStorage<'a, Moveable>,
        WriteStorage<'a, Hitbox>,
        WriteStorage<'a, Triggerbox>,
        WriteStorage<'a, PlatformController>,
        WriteStorage<'a, FallingBlock>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (edit_state, mut moveable, hitboxs, triggers, controller, mut falling_blocks, entities): Self::SystemData,
    ) {
        if edit_state.editting { return; }
        for (entity, trigger, fb) in (&entities, &triggers, &mut falling_blocks).join() {
            let has_movable = moveable.get(entity);
            if has_movable.is_none() {
                if !fb.should_fall {
                    let trigger_rect: Rectangle = trigger.clone().into();
                    for (_, hitbox) in (&controller, &hitboxs).join() {
                        let rect: Rectangle = hitbox.clone().into();
                        if rect.check_collision_recs(&trigger_rect) {
                            fb.should_fall = true;
                        }
                    }
                } else if fb.count <= MAX_FALL_COUNT {
                    fb.count += 1;
                } else {
                    let _res = moveable.insert(entity, Moveable::new()); 
                }
            }
        }
    }
}


pub struct EditDragHandler;
impl<'a> System<'a> for EditDragHandler {
    type SystemData = (
        ReadExpect<'a, EditState>,
        ReadExpect<'a, RaylibHandle>,
        WriteStorage<'a, DragBox>,
        WriteStorage<'a, Hitbox>
    );

    fn run(&mut self, (edit_state, rl, mut drag_boxes, mut hitboxes): Self::SystemData) {
        if edit_state.editting {
            let mouse_pouse = Position::from(rl.get_mouse_position());
            for (drag_box, hitbox) in (&mut drag_boxes, &mut hitboxes).join() {
                if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) && hitbox.point_inside_rec(mouse_pouse) {
                    drag_box.drag_offset = mouse_pouse - hitbox.position;
                    drag_box.dragging = true;
                }
                if drag_box.dragging {
                    let mut pos = (mouse_pouse - drag_box.drag_offset) / 32.;
                    pos.x = pos.x.round();
                    pos.y = pos.y.round();
                    hitbox.position = pos * 32.;
                }
    
                if rl.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
                    drag_box.dragging = false;
                }
            }
        }
    }
}

#[derive(Debug)]
enum Combined {
    Ron(ron::ser::Error),
}

// Implementing the required `Display`-trait, by matching the `Combined` enum,
// allowing different error types to be displayed.
impl fmt::Display for Combined {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Combined::Ron(ref e) => write!(f, "{}", e),
        }
    }
}

// This returns the `ron::ser:Error` in form of the `Combined` enum, which can
// then be matched and displayed accordingly.
impl From<ron::ser::Error> for Combined {
    fn from(x: ron::ser::Error) -> Self {
        Combined::Ron(x)
    }
}

// This cannot be called.
impl From<Infallible> for Combined {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}


pub struct NetworkSync;

pub struct Serialize;

impl<'a> System<'a> for Serialize {
    // This SystemData contains the entity-resource, as well as all components that
    // shall be serialized, plus the marker component storage.
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Hitbox>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, SimpleMarker<NetworkSync>>,
    );

    fn run(&mut self, (ents, hitboxs, positions, sprites, markers): Self::SystemData) {
        // First we need a serializer for the format of choice, in this case the
        // `.ron`-format.
        let mut serializer = ron::ser::Serializer::new(Some(Default::default()), true);
        SerializeComponents::<specs::error::NoError, SimpleMarker<NetworkSync>>::serialize(
            &(&hitboxs, &positions, &sprites),
            &ents,
            &markers,
            &mut serializer,
        )
        .unwrap_or_else(|e| eprintln!("Error: {}", e));
        //println!("{}", serializer.into_output_string());
        let file_contents = serializer.into_output_string();

        use ::std::fs::File;
        use ::std::io::Write;

        let mut file = File::create("storage.ron")
            .expect("Could not create save file.");
        file.write_all(file_contents.as_bytes())
            .expect("Could not write save file.");
    }
}

pub struct Deserialize;

    impl<'a> System<'a> for Deserialize {
        // This requires all the component storages our serialized entities have,
        // mutably, plus a `MarkerAllocator` resource to write the deserialized
        // ids into, so that we can later serialize again.
        type SystemData = (
            Entities<'a>,
            Write<'a, SimpleMarkerAllocator<NetworkSync>>,
            WriteStorage<'a, Hitbox>,
            WriteStorage<'a, Position>,
            WriteStorage<'a, Sprite>,
            WriteStorage<'a, SimpleMarker<NetworkSync>>,
        );

        fn run(&mut self, (ent, mut alloc, hitbox, positions, sprites, mut markers): Self::SystemData) {
            // The `const ENTITIES: &str` at the top of this file was formatted according to
            // the `.ron`-specs, therefore we need a `.ron`-deserializer.
            // Others can be used, as long as they implement the
            // `serde::de::Deserializer`-trait.
            use ron::de::Deserializer;
            use ::std::fs::File;
            use ::std::io::Read;
            let file_contents = {
                // FIXME: Replace panic! and expect! with actual error handling/recovery
                let mut file = match File::open("storage.ron") {
                    Ok(file) => file,
                    Err(error) => {
                        if error.kind() == ::std::io::ErrorKind::NotFound {
                            // eprintln!("Save file '{}' not found, loading from '{}' instead.",
                            //           self.file_name, self.default_storage);
                            File::open("storage.ron")
                                .expect("Could not open file.")
                        } else {
                            panic!("Could not open save file: {} ({})", "storage.ron", error);
                        }
                    },
                };
                let mut file_contents = Vec::new();
                file.read_to_end(&mut file_contents)
                    .expect("Could not read file.");
                file_contents
            };

            // Typical file operations are omitted in this example, since we do not have a
            // seperate file, but a `const &str`. We use a convencience function
            // of the `ron`-crate: `from_str`, to convert our data form the top of the file.
            if let Ok(mut de) = Deserializer::from_bytes(&file_contents) {
                // Again, we need to pass in a type implementing the `Display`-trait,
                // as well as a type implementing the `Marker`-trait.
                // However, from the function parameter `&mut markers`, which refers to the
                // `SimpleMarker`-storage, the necessary type of marker can be
                // inferred, hence the `, _>´.
                DeserializeComponents::<specs::error::NoError, _>::deserialize(
                    &mut (hitbox, positions, sprites),
                    &ent,
                    &mut markers,
                    &mut alloc,
                    &mut de,
                )
                .unwrap_or_else(|e| eprintln!("Error: {}", e));
            }
        }
    }


pub struct EditBtnHandle {
    pub selected_ent: Option<Entity>,
    pub selected_ty: Option<ToolPalette>
}
use crate::components::CollisionsRec;
impl<'a> System<'a> for EditBtnHandle {
    type SystemData = (
        ReadExpect<'a, EditState>,
        ReadExpect <'a, RaylibHandle>,
        WriteStorage<'a, EditBtn>,
        WriteStorage<'a, Hitbox>,
        WriteStorage<'a, DragBox>,
        WriteStorage<'a, FallingBlock>,
        WriteStorage<'a, Sprite>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, SimpleMarkerAllocator<NetworkSync>>,
        WriteStorage<'a, SimpleMarker<NetworkSync>>,
        Entities<'a>
    );

    fn run(&mut self, (edit_state, rl, edit_btns, mut hitboxes, mut drag_boxes, mut falling_blocks, mut sprites, mut positions, mut marker_alloc, mut markets, entities): Self::SystemData) {
        if edit_state.editting {
            let mouse_pouse = Position::from(rl.get_mouse_position());
            let mut button_pressed = false;
            for edit_btn in edit_btns.join() {
                
                if rl.is_mouse_button_released(MOUSE_LEFT_BUTTON) && edit_btn.point_inside_rec(mouse_pouse) {       
                    
                    if self.selected_ent.is_none() {
                        let ent = entities.create();
                        let _ = sprites.insert(ent, Sprite { name: edit_btn.text.clone() });
                        let _ = positions.insert(ent, mouse_pouse);
                        self.selected_ent = Option::from(ent);
                    } else if self.selected_ent.is_some() {
                        if self.selected_ty.unwrap() == edit_btn.ty {
                            let ent = self.selected_ent.take().unwrap();
                            self.selected_ty = None;
                            let _ = entities.delete(ent);
                        } else {
                            let sprite = sprites.get_mut(self.selected_ent.unwrap()).unwrap();
                            sprite.name = edit_btn.text.clone();
                        }
                    }

                    self.selected_ty = Option::from(edit_btn.ty);
                    button_pressed = true;
                    println!("Pressed");
                }
            }

            if let Some(ent) = self.selected_ent {
                let mut pos = mouse_pouse / 32.;
                pos.x = pos.x.floor();
                pos.y = pos.y.floor();
                pos *= 32.;
                *positions.get_mut(ent).unwrap() = pos;
                if !button_pressed && rl.is_mouse_button_down(MOUSE_LEFT_BUTTON) {

                    let hit = (&entities, &hitboxes, &sprites).join().find(|(_, h, _)| h.collision_rec(Hitbox::new(pos.x, pos.y)));
                    let ty = self.selected_ty.unwrap();
                    let ty_str = ty.as_static().to_string().to_lowercase();
                    let mut can_place = true;
                    if let Some((e, _, s)) = hit {
                        if s.name == ty_str {
                            can_place = false;
                        } else {
                            let _ = entities.delete(e);
                        }
                    }
                    if can_place {
                        let ent = entities.create();
                        let _ = drag_boxes.insert(ent, DragBox::default());
                        let _ = sprites.insert(ent, Sprite { name: ty.as_static().to_string().to_lowercase() });
                        let _ = hitboxes.insert(ent, Hitbox::new(pos.x, pos.y));
                        let m = marker_alloc.allocate(ent, None);
                        let _ = markets.insert(ent, m);
                        match ty {
                            ToolPalette::FallingBlock => {},
                            ToolPalette::SpikeBlock => {}
                            _ => {}
                        }
                    }    
                }
               
            }
        }
    }
}


pub struct IconButtonsHandler;


impl<'a> System<'a> for IconButtonsHandler {

    type SystemData = (
        WriteExpect<'a, EditState>,
        ReadExpect<'a, RaylibHandle>,
        ReadStorage<'a, Icon>,
        Entities<'a>,
        WriteStorage<'a, SimpleMarker<NetworkSync>> 
    );

    fn run(&mut self, (mut edit_state, rl, icons, entities, markers):Self::SystemData) {
        let mouse_pos = Position::from(rl.get_mouse_position());
        for icon in icons.join() {
            let rect = Hitbox::new(icon.position.x, icon.position.y);
            if rl.is_mouse_button_released(MOUSE_LEFT_BUTTON) && rect.point_inside_rec(mouse_pos) {
                match icon.icon.0 {
                    raylib::consts::rIconDescription::RICON_EMPTYBOX => {
                        for (e, _) in (&entities, &markers).join() {
                            let _ = entities.delete(e);
                        }
                    },
                    raylib::consts::rIconDescription::RICON_FILE_SAVE_CLASSIC => {
                        edit_state.should_save = true;
                    }
                    _ => {}
                }
            }
        }
    }

}