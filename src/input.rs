use std::hash::Hash;

use bevy::prelude::*;

pub trait Pressed<T>: Sized
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    fn controls() -> &'static [T];

    fn pressed(input: &Res<ButtonInput<T>>) -> bool {
        pressed::<Self, T>(input)
    }

    fn pressed_condition(input: Res<ButtonInput<T>>) -> bool {
        Self::pressed(&input)
    }
}

pub fn pressed<P, T>(input: &Res<ButtonInput<T>>) -> bool
where
    P: Pressed<T>,
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    for &keycode in P::controls() {
        if input.pressed(keycode) {
            return true;
        }
    }
    false
}

pub trait JustPressed<T>: Sized
where
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    fn controls() -> &'static [T];

    fn just_pressed(input: &Res<ButtonInput<T>>) -> bool {
        just_pressed::<Self, T>(input)
    }

    fn just_pressed_condition(input: Res<ButtonInput<T>>) -> bool {
        Self::just_pressed(&input)
    }
}

pub fn just_pressed<J, T>(input: &Res<ButtonInput<T>>) -> bool
where
    J: JustPressed<T>,
    T: Copy + Eq + Hash + Send + Sync + 'static,
{
    for &keycode in J::controls() {
        if input.just_pressed(keycode) {
            return true;
        }
    }
    false
}
