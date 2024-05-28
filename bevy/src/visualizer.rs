use crate::observation::ObjectObservation;
use bevy::prelude::*;

///
/// InfoTimer is a timer resource which prompts a regular update of the attention state.
///
#[derive(Resource)]
struct InfoTimer {
    timer: Timer,
}

impl Default for InfoTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
}

pub struct DebugVisualizerPlugin;

impl Plugin for DebugVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InfoTimer>()
            .add_systems(Startup, setup)
            .add_systems(Update, print_state);
    }
}

fn setup() {
    info!("DebugVisualizerPlugin loaded!");
}

fn print_state(
    time: Res<Time>,
    mut info_timer: ResMut<InfoTimer>,
    observed_query: Query<(&ObjectObservation, Option<&Name>, Entity)>,
) {
    let mut print_string = String::new();
    if info_timer.timer.tick(time.delta()).just_finished() {
        for (index, (observation_component, name_component, entity)) in
            observed_query.iter().enumerate()
        {
            if index != 0 {
                print_string.push_str(", ");
            }

            let label = if let Some(name) = name_component {
                format!("'{}'", name.as_str())
            } else {
                entity.index().to_string()
            };

            print_string.push_str(&format!(
                "entity {}: {} attention",
                label,
                observation_component.get_attention()
            ));
        }
        info!("{}", print_string);
    }
}
