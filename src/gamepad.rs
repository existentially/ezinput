//! Full gamepad support for EZInput.
use bevy::prelude::*;

use crate::prelude::*;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct GamepadInputHandlingSystem;

// Marker responsible for allowing systems to listen to gamepad input.
#[derive(PartialEq, Eq, Debug, Component, Clone)]
pub struct GamepadMarker(pub Gamepad);

impl Default for GamepadMarker {
    fn default() -> Self {
        Self(Gamepad::new(0))
    }
}

impl GamepadMarker {
    /// Change the current button state for the given button and set the last input source to Gamepad.
    pub fn set_gamepad_button_state<Keys>(
        &mut self,
        view: &mut InputView<Keys>,
        button: GamepadButtonType,
        state: PressState,
        duration: f32,
    ) where
        Keys: BindingTypeView,
    {
        view.last_input_source = Some(InputSource::Gamepad);
        view.set_key_receiver_state(InputReceiver::GamepadButton(button), state);
        view.set_axis_value(InputReceiver::GamepadButton(button), duration, state);
    }

    /// Change the current axis state for the given axis and set the last input source to Gamepad.
    pub fn set_gamepad_axis_state<Keys>(
        &mut self,
        view: &mut InputView<Keys>,
        axis: GamepadAxisType,
        state: PressState,
        duration: f32,
    ) where
        Keys: BindingTypeView,
    {
        view.last_input_source = Some(InputSource::Gamepad);
        view.set_key_receiver_state(InputReceiver::GamepadAxis(axis), state);
        view.set_axis_value(InputReceiver::GamepadAxis(axis), duration, state);
    }
}

/// Input system responsible for handling gamepad input and setting the button state for each updated button and axis.
pub(crate) fn gamepad_input_system<Keys>(
    mut query: Query<(&mut InputView<Keys>, &mut GamepadMarker)>,
    mut rd: EventReader<GamepadEvent>,
) where
    Keys: BindingTypeView,
{
    for ev in rd.iter() {
        match ev.event_type {
            GamepadEventType::ButtonChanged(kind, duration) => {
                for (mut view, mut svc) in query.iter_mut() {
                    if ev.gamepad != svc.0 {
                        continue;
                    }
                    let state = if duration.abs() <= 0.1 {
                        PressState::Released
                    } else {
                        PressState::Pressed {
                            started_pressing_instant: None,
                        }
                    };
                    svc.set_gamepad_button_state::<Keys>(view.as_mut(), kind, state, duration);
                    break;
                }
            }
            GamepadEventType::AxisChanged(kind, value) => {
                for (mut view, mut svc) in query.iter_mut() {
                    if ev.gamepad != svc.0 {
                        continue;
                    }
                    let state = if value.abs() <= 0.1 {
                        PressState::Released
                    } else {
                        PressState::Pressed {
                            started_pressing_instant: None,
                        }
                    };
                    svc.set_gamepad_axis_state::<Keys>(view.as_mut(), kind, state, value);
                    break;
                }
            }
            _ => {}
        }
    }
}
