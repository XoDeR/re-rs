use super::{
    context::Context,
};

pub struct GameLoop {
    pub setup: fn(context: &mut Context),
    pub update: fn(context: &mut Context),
}

impl GameLoop {
    pub fn new(setup: fn(context: &mut Context), update: fn(context: &mut Context)) -> Self {
        return Self {
            setup,
            update,
        };
    }
}