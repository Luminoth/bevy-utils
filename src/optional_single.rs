use bevy::ecs::query::{Fetch, FilterFetch, WorldQuery};
use bevy::ecs::system::QuerySingleError;
use bevy::prelude::*;

pub trait OptionalSingle<'s, Q>
where
    Q: WorldQuery,
{
    fn optional_single(&self) -> Option<<Q::ReadOnlyFetch as Fetch<'_, 's>>::Item>;
    fn optional_single_mut(&mut self) -> Option<<Q::Fetch as Fetch<'_, '_>>::Item>;
}

impl<'w, 's, Q, F> OptionalSingle<'s, Q> for Query<'w, 's, Q, F>
where
    Q: WorldQuery,
    F: WorldQuery,
    F::Fetch: FilterFetch,
{
    fn optional_single(&self) -> Option<<Q::ReadOnlyFetch as Fetch<'_, 's>>::Item> {
        match self.get_single() {
            Ok(item) => Some(item),
            Err(QuerySingleError::NoEntities(_)) => None,
            Err(QuerySingleError::MultipleEntities(_)) => {
                panic!("multiple items from optional single query")
            }
        }
    }

    fn optional_single_mut(&mut self) -> Option<<Q::Fetch as Fetch<'_, '_>>::Item> {
        match self.get_single_mut() {
            Ok(item) => Some(item),
            Err(QuerySingleError::NoEntities(_)) => None,
            Err(QuerySingleError::MultipleEntities(_)) => {
                panic!("multiple items from optional single query")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component, Default)]
    struct OptionalSingleComponent;

    #[test]
    fn optional_single_exists_success() {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(|query: Query<&OptionalSingleComponent>| {
            query.optional_single();
        });

        world.spawn().insert(OptionalSingleComponent);

        update_stage.run(&mut world);
    }

    #[test]
    fn optional_single_not_exists_success() {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(|query: Query<&OptionalSingleComponent>| {
            query.optional_single();
        });

        update_stage.run(&mut world);
    }

    #[test]
    #[should_panic]
    fn optional_single_many_failure() {
        let mut world = World::default();

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(|query: Query<&OptionalSingleComponent>| {
            query.optional_single();
        });

        world.spawn().insert(OptionalSingleComponent);
        world.spawn().insert(OptionalSingleComponent);

        update_stage.run(&mut world);
    }
}
