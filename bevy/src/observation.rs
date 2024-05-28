use bevy::prelude::*;

///
/// The ObjectObservation component marks any object that receives attention.
///
#[derive(Component)]
pub struct ObjectObservation {
    attention: u32,
}

impl ObjectObservation {
    pub fn new() -> Self {
        Self { attention: 0 }
    }

    pub fn increment_attention(&mut self) {
        self.attention += 1;
    }

    pub fn get_attention(&self) -> u32 {
        self.attention
    }
}

impl Default for ObjectObservation {
    fn default() -> Self {
        Self::new()
    }
}
