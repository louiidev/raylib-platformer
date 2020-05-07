use raylib::prelude::*;
use specs::prelude::*;
use crate::components::{Transform, Position};

// System is not thread safe
pub struct DrawSys {
    pub thread: RaylibThread
}
impl<'a> System<'a> for DrawSys {
    type SystemData = (
        WriteExpect<'a, RaylibHandle>
    );

    fn run(&mut self, mut rl: Self::SystemData) {
        let mut d = rl.begin_drawing(&self.thread);
        d.clear_background(crate::COLOUR);
        
    }
}