use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        transform::Transform,
        ArcThreadPool
    },
    ecs::prelude::{
        Entity,
        DispatcherBuilder,
        Dispatcher
    },
    prelude::*,
    renderer::{
        debug_drawing::{DebugLines, DebugLinesParams},
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,

    },
    ui::{Anchor, FontAsset, TtfFormat, UiText, UiTransform},
    window::ScreenDimensions,
    input::{VirtualKeyCode, is_key_down},
    utils::removal::{
        Removal,
        exec_removal
    }
};

use crate::{
    state::{
        MyState,
        RemovalId
    },
    system::{
        MoveObjects,
        DebugBoxes,
        LasersDamageAsteroids
    },
    entity::init_asteroid, 
    resource::SpriteResource,
};

#[derive(Default)]
pub struct TitleState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>
}

impl<'a, 'b> SimpleState for TitleState<'a, 'b> {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let world = &mut data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let mut dispatcher_builder = DispatcherBuilder::new();
        dispatcher_builder.add(MoveObjects,"move_objects", &[]);
        dispatcher_builder.add(DebugBoxes,"debug_boxes", &[]);
        dispatcher_builder.add(LasersDamageAsteroids,"lasers_damage_asteroids", &[]);

        // Setup dispatcher
        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        world.insert(DebugLines::new());
        world.insert(DebugLinesParams { line_width: 2.0 });
        world.register::<Removal<RemovalId>>();

        load_sprites(world);
        init_camera(world, &dimensions);
        init_title(world);
        init_instruction(world);

        let sprites = load_sprites(world);

        for _ in 0..64 {
            init_asteroid(world, &sprites, &dimensions)
        }

        self.dispatcher = Some(dispatcher);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                let world = data.world;
                // Clean up text from this page
                exec_removal(&world.entities(), &world.read_storage(), RemovalId::TitleText);
                world.maintain();

                return Trans::Switch(Box::new(MyState::default()));
            }
        }

        Trans::None
    }

    fn update(
        &mut self,
        data: &mut StateData<'_, GameData<'_, '_>>
    ) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}

pub struct Label {
    pub entity: Entity,
}

fn init_title(world: &mut World) {
    let font: Handle<FontAsset> = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let label_transform = UiTransform::new(
        "title".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        0.,
        1.,
        550.,
        100.,
    );

    let label_entity = world
        .create_entity()
        .with(label_transform)
        .with(UiText::new(
            font.clone(),
            "RUSTEROIDS".to_string(),
            [1., 1., 1., 1.],
            100.,
        ))
        .with(Removal::new(RemovalId::TitleText))
        .build();

    world.insert(Label {
        entity: label_entity,
    });
}

fn init_instruction(world: &mut World) {
    let font: Handle<FontAsset> = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let label_transform = UiTransform::new(
        "title".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        -70.,
        1.,
        550.,
        100.,
    );

    let label_entity = world
        .create_entity()
        .with(label_transform)
        .with(UiText::new(
            font.clone(),
            "PRESS SPACEBAR".to_string(),
            [1., 1., 1., 1.],
            36.,
        ))
        .with(Removal::new(RemovalId::TitleText))
        .build();

    world.insert(Label {
        entity: label_entity,
    });
}

pub fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

pub fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/spritesheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Mutate the world with a sheet handle
    world.insert(SpriteResource {
        sprite_sheet: sheet_handle.clone(),
    });

    (0..2)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}
