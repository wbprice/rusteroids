use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::math::geometry::Point2,
    core::transform::Transform,
    ecs::prelude::Entity,
    prelude::*,
    renderer::{
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
    },
    ui::{Anchor, FontAsset, TtfFormat, UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::{entity::init_asteroid, resource::SpriteResource};

pub struct TitleState;

impl SimpleState for TitleState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        world.insert(DebugLines::new());
        world.insert(DebugLinesParams { line_width: 2.0 });

        load_sprites(world);
        init_camera(world, &dimensions);
        init_title(world);
        init_instruction(world);

        let sprites = load_sprites(world);

        for _ in 0..64 {
            init_asteroid(world, &sprites, &dimensions)
        }
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
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
        .build();

    let mut debug_component = DebugLinesComponent::new();
    debug_component.add_rectangle_2d(
        Point2::new(0., 100.),
        Point2::new(0., 100.),
        1.,
        Srgba::new(0.3, 0.3, 1.0, 1.0),
    );

    let rectangle_entity = world
        .create_entity()
        .with(debug_component)
        .build();

    world.insert(Label {
        entity: label_entity,
    });
    world.insert(rectangle_entity);
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
