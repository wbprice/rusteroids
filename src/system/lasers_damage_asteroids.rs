use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::{
    component::{Asteroid, Collidable, Laser, SmallAsteroid, Velocity},
    resource::SpriteResource,
};

pub struct LasersDamageAsteroids;

impl<'a> System<'a> for LasersDamageAsteroids {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, SpriteResource>,
        WriteStorage<'a, SpriteRender>,
        ReadStorage<'a, Asteroid>,
        WriteStorage<'a, SmallAsteroid>,
        ReadStorage<'a, Laser>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Collidable>,
    );

    fn run(
        &mut self,
        (
            entities,
            sprite_resources,
            mut sprites,
            asteroids,
            mut small_asteroids,
            lasers,
            mut transforms,
            mut velocities,
            mut collidables,
        ): Self::SystemData,
    ) {
        let mut new_small_asteroids: Vec<Transform> = vec![];

        for (asteroid_entity, _asteroid, asteroid_local, asteroid_collidable) in
            (&entities, &asteroids, &transforms, &collidables).join()
        {
            for (_laser, laser_local, laser_collidable) in
                (&lasers, &transforms, &collidables).join()
            {
                let laser_x = laser_local.translation().x;
                let laser_y = laser_local.translation().y;
                let asteroid_x = asteroid_local.translation().x;
                let asteroid_y = asteroid_local.translation().y;

                let dx = asteroid_x - laser_x;
                let dy = asteroid_y - laser_y;
                let distance = (dx.powi(2) + dy.powi(2)).sqrt();

                if distance < laser_collidable.radius + asteroid_collidable.radius {
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

                entities
                    .build_entity()
                    .with(
                        SpriteRender {
                            sprite_sheet: sprite_resources.sprite_sheet.clone(),
                            sprite_number: 3,
                        },
                        &mut sprites,
                    )
                    .with(local, &mut transforms)
                    .with(SmallAsteroid {}, &mut small_asteroids)
                    .with(Collidable { radius: 12.0 }, &mut collidables)
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
