//! The press state for a button or axis. Also useful methods for checking the elapsed time.
use std::fmt::{Debug, Display};
#[allow(unused_imports)]
use std::ops::Add;
use std::slice::Iter;

use bevy::input::ButtonState;
use bevy::utils::{Duration, Instant};

/// The current state of a specific axis or button. By default, calls return [`PressState::Released`].
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum PressState {
    /// The button or axis is pressed, along with the initial instant for the press.
    /// This need to be set as none if is the moment the button is just pressed, since it will
    /// let the input view know that the button is just pressed. The pressing instant is set
    /// in the next tick to allow users to know the pressing duration.
    Pressed {
        started_pressing_instant: Option<Instant>,
    },

    /// The button or axis is released.
    Released,
}
impl PressState {
    /// Returns whether if the current press state is released or not.
    #[inline]
    pub fn released(&self) -> bool {
        *self == PressState::Released
    }

    /// Returns whether if the current press state is pressed for more than a specific duration.
    #[inline]
    pub fn is_pressed_for(&self, duration: Duration) -> bool {
        if let PressState::Pressed {
            started_pressing_instant,
        } = *self
        {
            started_pressing_instant.is_some()
                && started_pressing_instant.unwrap().elapsed() >= duration
        } else {
            false
        }
    }

    /// Returns whether the button or axis was just pressed or moved in this exact tick or not.
    #[inline]
    pub fn just_pressed(&self) -> bool {
        if let PressState::Pressed {
            started_pressing_instant,
        } = *self
        {
            if let Some(instant) = started_pressing_instant {
                instant.elapsed().as_millis() <= 1
            } else {
                true
            }
        } else {
            false
        }
    }

    /// Returns whether the button or axis is currently pressed or moving.
    #[inline]
    pub fn pressed(&self) -> bool {
        matches!(*self, PressState::Pressed { .. })
    }

    /// Returns the elapsed time since the action was pressed.
    #[inline]
    pub fn elapsed(&self) -> Option<Duration> {
        match self {
            PressState::Pressed {
                started_pressing_instant,
            } => started_pressing_instant
                .as_ref()
                .map(|started_pressing_instant| started_pressing_instant.elapsed())
                .or(Some(Duration::ZERO)),
            _ => None,
        }
    }
}

/// Implement partial comparision between press states.
impl PartialOrd for PressState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            PressState::Pressed {
                started_pressing_instant: a,
            } => match other {
                PressState::Pressed {
                    started_pressing_instant: b,
                } => Some(a.cmp(b)),
                PressState::Released => Some(std::cmp::Ordering::Greater),
            },
            PressState::Released => match other {
                PressState::Pressed { .. } => Some(std::cmp::Ordering::Less),
                PressState::Released => Some(std::cmp::Ordering::Equal),
            },
        }
    }
}

/// Implement comparison between press states.
impl Ord for PressState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Implementation responsible for translating Bevy element states to EZInput press states.
/// By default, the default pressing instant is the None.
impl From<ButtonState> for PressState {
    fn from(value: ButtonState) -> PressState {
        match value {
            ButtonState::Pressed => PressState::Pressed {
                started_pressing_instant: None,
            },
            ButtonState::Released => PressState::Released,
        }
    }
}

/// Implementation responsible for allowing the input source to be displayed as a string.
impl Display for PressState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            PressState::Pressed { .. } => {
                if self.just_pressed() {
                    write!(f, "Pressing since Now")
                } else {
                    write!(f, "Pressing for {:?}", self.elapsed())
                }
            }

            PressState::Released => write!(f, "Released"),
        }
    }
}

// Test to compare if `PartialOrd` is implemented correctly.
#[test]
fn partial_ord_press_state_test() {
    let a = PressState::Pressed {
        started_pressing_instant: Some(Instant::now()),
    };
    let b = PressState::Pressed {
        started_pressing_instant: Some(Instant::now().add(Duration::from_secs(342534))),
    };
    let value = a.cmp(&b);
    assert_eq!(value, std::cmp::Ordering::Less);
}

/// The current axis state. In other words, the strength (how much the axis is moved) and press state.
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct AxisState {
    pub value: f32,
    pub press: PressState,
}

impl AxisState {
    pub const ZERO: Self = Self {
        value: 0.0,
        press: PressState::Released,
    };

    pub fn new(value: f32, press: PressState) -> Self {
        Self { value, press }
    }

    pub fn set(&mut self, value: f32, press: PressState) {
        self.value = value;
        self.press = press;
    }
}

pub trait AxisStateVecExt {
    fn is_all_pressed(&mut self) -> bool;
    
    fn is_all_just_pressed(&mut self) -> bool;

    fn is_all_released(&mut self) -> bool;
}

impl AxisStateVecExt for Vec<AxisState> {
    fn is_all_pressed(&mut self) -> bool {
        self.iter().all(|s| s.press.pressed())
    }

    fn is_all_just_pressed(&mut self) -> bool {
        self.iter().all(|s| s.press.just_pressed())
    }

    fn is_all_released(&mut self) -> bool {
        self.iter().all(|s| s.press.released())
    }
}

impl AxisStateVecExt for Iter<'_, AxisState> {
    fn is_all_pressed(&mut self) -> bool {
        self.all(|s| s.press.pressed())
    }

    fn is_all_just_pressed(&mut self) -> bool {
        self.all(|s| s.press.just_pressed())
    }

    fn is_all_released(&mut self) -> bool {
        self.all(|s| s.press.released())
    }
}