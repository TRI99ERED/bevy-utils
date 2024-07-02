#![allow(unused)]

use bevy::{
    ecs::query::{QueryData, QueryFilter},
    prelude::*,
};

pub fn switch_to_state<S>(state: &'static S) -> impl Fn(ResMut<NextState<S>>) + 'static
where
    S: States + Clone,
{
    let f: _ = |mut next_state: ResMut<NextState<S>>| {
        next_state.set(state.clone());
    };

    f
}

pub fn entity_exists_df<D, F>() -> impl Fn(Query<D, F>) -> bool
where
    D: QueryData,
    F: QueryFilter,
{
    let f: _ = |q_start: Query<D, F>| !q_start.is_empty();

    f
}

pub fn entity_exists_d<D>() -> impl Fn(Query<D, ()>) -> bool
where
    D: QueryData,
{
    let f: _ = |q_start: Query<D>| !q_start.is_empty();

    f
}

