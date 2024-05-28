use crate::observation::ObjectObservation;
use bevy::prelude::*;

///
/// The ProximityObserver component marks a game entity at as a proximity observer.
/// Proximity observers will provide attention to all regions and objects that are
/// within a set distance.
///
#[derive(Component)]
pub struct ProximityObserver {
    square_radius: f32,
}

impl ProximityObserver {
    pub fn new(radius: f32) -> Self {
        Self {
            square_radius: radius.powi(2),
        }
    }

    fn within_distance(&self, first: &Transform, second: &Transform) -> bool {
        first.translation.distance_squared(second.translation) <= self.square_radius
    }
}

pub(crate) fn proximity_object_observation(
    observer_query: Query<(&Transform, &ProximityObserver)>,
    mut observation_query: Query<(&Transform, &mut ObjectObservation)>,
) {
    for (observer_transform, observer_component) in observer_query.iter() {
        for (observation_transform, mut observation_component) in observation_query.iter_mut() {
            if observer_component.within_distance(observer_transform, observation_transform) {
                observation_component.increment_attention();
            }
        }
    }
}

macro_rules! systems {
    () => {
        (crate::observer::proximity_object_observation)
    };
}

pub(crate) use systems;
