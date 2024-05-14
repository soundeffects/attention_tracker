use bevy::prelude::*;

#[derive(Component)]
pub struct Attent;

pub struct AttentionTracker;

impl Plugin for AttentionTracker {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, attend);
    }
}

fn setup() {
    info!("loaded!");
}

fn attend(mut commands: Commands, attent_query: Query<&Transform, With<Attent>>) {
    for transform in attent_query.iter() {
        //commands.spawn()
        todo!();
    }
}