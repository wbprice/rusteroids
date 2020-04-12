use amethyst::{
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};

use crate::state::MyState;

pub struct EndState;

impl SimpleState for EndState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        dbg!("new state!");
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::R) {
                return Trans::Replace(Box::new(MyState));
            }
        }

        Trans::None
    }
}
