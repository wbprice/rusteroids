use amethyst::{
    assets::{Handle, Loader},
    ecs::prelude::Entity,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{Anchor, FontAsset, TtfFormat, UiText, UiTransform},
};

pub struct EndState;

impl SimpleState for EndState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        init_game_over_text(world);
    }

    fn handle_event(
        &mut self,
        mut data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let world = data.world;
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::R) {
                let mut game_over_text = world.try_fetch::<GameOverText>();
                if let Some(text) = game_over_text {
                }
                return Trans::Pop;
            }
        }

        Trans::None
    }
}

struct GameOverText {
    pub text: Option<Entity>,
}

fn init_game_over_text(world: &mut World) {
    let font: Handle<FontAsset> = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let lives_transform = UiTransform::new(
        "Lives".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        0.,
        1.,
        250.,
        50.,
    );

    let lives_left = world
        .create_entity()
        .with(lives_transform)
        .with(UiText::new(
            font.clone(),
            "Game Over".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.insert(GameOverText { text: Some(lives_left) });
}