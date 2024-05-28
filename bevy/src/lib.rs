use bevy::prelude::*;

pub mod observation;
pub mod observer;
pub mod visualizer;

pub struct AttentionTrackerPlugin;
impl Plugin for AttentionTrackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, observer::systems!());
    }
}

fn setup() {
    info!("AttentionTrackerPlugin loaded!");
}
