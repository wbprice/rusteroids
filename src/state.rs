use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        debug_drawing::{DebugLines, DebugLinesParams},
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
    },
    ui::{Anchor, FontAsset, TtfFormat, UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::{
    component::Laser,
    end_state::EndState,
    entity::{init_asteroid, init_player_ship},
    resource::SpriteResource,
};

// Rough State Transition Stuff, Doesn't Have To Live Here Forever
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentState {
    Gameplay,
    GameOver,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Gameplay
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserAction {
    Restart,
    EndGame,
}

pub struct GameState {
    pub user_action: Option<UserAction>,
    current_state: CurrentState,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            user_action: None,
            current_state: CurrentState::default(),
        }
    }
}

pub struct MyState;

impl SimpleState for MyState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        // Do prework for setting up lasers
        world.register::<Laser>();
        world.insert(GameState::default());
        world.insert(DebugLines::new());
        world.insert(DebugLinesParams { line_width: 2.0 });

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprites(world);

        init_player_ship(world, &sprites, &dimensions);
        init_lives_remaining(world);
        // Initialize 12 asteroids
        for _ in 0..12 {
            init_asteroid(world, &sprites, &dimensions);
        }
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        // Keep going
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let mut game = data.world.write_resource::<GameState>();

        if let Some(UserAction::EndGame) = game.user_action.take() {
            return Trans::Push(Box::new(EndState));
        }

        Trans::None
    }
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

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
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

pub struct LivesLeft {
    pub lives: i8,
}

impl Default for LivesLeft {
    fn default() -> Self {
        LivesLeft { lives: 3 }
    }
}

pub struct RespawnTimer {
    pub time_remaining: f32,
}

impl Default for RespawnTimer {
    fn default() -> Self {
        RespawnTimer {
            time_remaining: 3.0,
        }
    }
}

pub struct LivesLeftText {
    pub text: Entity,
}

fn init_lives_remaining(world: &mut World) {
    let font: Handle<FontAsset> = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let lives_transform = UiTransform::new(
        "Lives".to_string(),
        Anchor::TopRight,
        Anchor::TopRight,
        -50.,
        -50.,
        1.,
        200.,
        50.,
    );

    let lives_left = world
        .create_entity()
        .with(lives_transform)
        .with(UiText::new(
            font.clone(),
            "3 LIVES".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.insert(LivesLeftText { text: lives_left });
}
