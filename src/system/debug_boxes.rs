use amethyst::{
    core::Transform,
    ecs::{Entities, Entity, Join, ReadStorage, System, WriteStorage},
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba},
};

use crate::component::{Asteroid, Player};

pub struct DebugBoxes;

impl<'a> System<'a> for DebugBoxes {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Asteroid>,
        WriteStorage<'a, DebugLinesComponent>,
        ReadStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (entities, players, asteroids, mut debug_lines, transforms): Self::SystemData,
    ) {
        let mut debug_lines_to_update: Vec<(Entity, DebugLinesComponent)> = vec![];

        // Update debug components for the Player
        for (entity, _player, _debug_line, local) in
            (&entities, &players, &debug_lines, &transforms).join()
        {
            let pos_x = local.translation().x;
            let pos_y = local.translation().y;
            let mut debug_component = DebugLinesComponent::new();
            debug_component.add_circle_2d(
                [pos_x, pos_y, 0.0].into(),
                24.0,
                12,
                Srgba::new(0.3, 0.3, 1.0, 1.0),
            );

            debug_lines_to_update.push((entity, debug_component));
        }

        // Update debug components for the asteroids
        for (entity, _asteroid, _debug_line, local) in
            (&entities, &asteroids, &debug_lines, &transforms).join()
        {
            let pos_x = local.translation().x;
            let pos_y = local.translation().y;
            let mut debug_component = DebugLinesComponent::new();
            debug_component.add_circle_2d(
                [pos_x, pos_y, 0.0].into(),
                32.0,
                12,
                Srgba::new(0.3, 0.3, 1.0, 1.0),
            );

            debug_lines_to_update.push((entity, debug_component));
        }

        for (entity, debug_component) in debug_lines_to_update {
            debug_lines.insert(entity, debug_component).unwrap();
        }
    }
}
