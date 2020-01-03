use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba, SpriteRender},
};

use crate::{
    component::{Asteroid, Laser, SmallAsteroid, Velocity},
    resource::SpriteResource,
};

pub struct LasersDamageAsteroids;

const LASER_RADIUS: f32 = 3.0;
const ASTEROID_RADIUS: f32 = 36.0;

impl<'a> System<'a> for LasersDamageAsteroids {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, SpriteResource>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, DebugLinesComponent>,
        ReadStorage<'a, Asteroid>,
        WriteStorage<'a, SmallAsteroid>,
        ReadStorage<'a, Laser>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            sprite_resources,
            mut sprites,
            mut debug_lines,
            asteroids,
            mut small_asteroids,
            lasers,
            mut transforms,
            mut velocities,
        ): Self::SystemData,
    ) {
        let mut new_small_asteroids: Vec<Transform> = vec![];

        for (asteroid_entity, _asteroid, asteroid_local) in
            (&entities, &asteroids, &transforms).join()
        {
            for (_laser, laser_local) in (&lasers, &transforms).join() {
                let laser_x = laser_local.translation().x;
                let laser_y = laser_local.translation().y;
                let asteroid_x = asteroid_local.translation().x;
                let asteroid_y = asteroid_local.translation().y;

                let dx = asteroid_x - laser_x;
                let dy = asteroid_y - laser_y;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                if distance < LASER_RADIUS + ASTEROID_RADIUS {
                    entities.delete(asteroid_entity).unwrap();
                    new_small_asteroids.push(asteroid_local.clone());
                }
            }
        }

        for local in new_small_asteroids {
            let magnitudes: Vec<f32> = vec![0.0, 2.0, -2.0];
            for i in magnitudes.into_iter() {
                let local = local.clone();
                let asteroid_displacement = 75.0;
                let velocity_x = i.cos() * asteroid_displacement;
                let velocity_y = i.sin() * asteroid_displacement;

                let pos_x = local.translation().x;
                let pos_y = local.translation().x;

                let mut debug_component = DebugLinesComponent::new();
                debug_component.add_circle_2d(
                    [pos_x, pos_y, 0.0].into(),
                    6.0,
                    12,
                    Srgba::new(0.3, 0.3, 1.0, 1.0),
                );

                entities
                    .build_entity()
                    .with(
                        SpriteRender {
                            sprite_sheet: sprite_resources.sprite_sheet.clone(),
                            sprite_number: 3,
                        },
                        &mut sprites,
                    )
                    .with(debug_component, &mut debug_lines)
                    .with(local, &mut transforms)
                    .with(SmallAsteroid {}, &mut small_asteroids)
                    .with(
                        Velocity {
                            x: velocity_x,
                            y: velocity_y,
                            a: 0.,
                        },
                        &mut velocities,
                    )
                    .build();
            }
        }
    }
}
