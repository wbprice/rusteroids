use amethyst::{
    core::Transform,
    ecs::{Entities, Entity, Join, ReadStorage, System, WriteStorage},
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba},
};

use crate::component::Collidable;

pub struct DebugBoxes;

impl<'a> System<'a> for DebugBoxes {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, DebugLinesComponent>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Collidable>,
    );

    fn run(&mut self, (entities, mut debug_lines, transforms, collidables): Self::SystemData) {
        let mut debug_lines_to_update: Vec<(Entity, DebugLinesComponent)> = vec![];

        // Find collidable components that don't yet have debug components
        for (entity, collidable, _debug_line, local) in
            (&entities, &collidables, !&debug_lines, &transforms).join()
        {
            let pos_x = local.translation().x;
            let pos_y = local.translation().y;
            let mut debug_component = DebugLinesComponent::new();
            debug_component.add_circle_2d(
                [pos_x, pos_y, 0.0].into(),
                collidable.radius,
                12,
                Srgba::new(0.3, 0.3, 1.0, 1.0),
            );

            debug_lines_to_update.push((entity, debug_component));
        }

        // Queue up existing debug components for update
        for (entity, collidable, _debug_line, local) in
            (&entities, &collidables, &debug_lines, &transforms).join()
        {
            let pos_x = local.translation().x;
            let pos_y = local.translation().y;
            let mut debug_component = DebugLinesComponent::new();
            debug_component.add_circle_2d(
                [pos_x, pos_y, 0.0].into(),
                collidable.radius,
                12,
                Srgba::new(0.3, 0.3, 1.0, 1.0),
            );

            debug_lines_to_update.push((entity, debug_component));
        }

        // Update debug components for collidable entities
        for (entity, debug_component) in debug_lines_to_update {
            debug_lines.insert(entity, debug_component).unwrap();
        }
    }
}
