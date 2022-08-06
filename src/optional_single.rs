use bevy::ecs::query::{QueryItem, QuerySingleError, ROQueryItem, WorldQuery};
use bevy::prelude::*;

pub trait OptionalSingle<'s, Q>
where
    Q: WorldQuery,
{
    fn optional_single(&self) -> Option<ROQueryItem<'_, Q>>;
    fn optional_single_mut(&mut self) -> Option<QueryItem<'_, Q>>;
}

impl<'w, 's, Q, F> OptionalSingle<'s, Q> for Query<'w, 's, Q, F>
where
    Q: WorldQuery,
    F: WorldQuery,
{
    fn optional_single(&self) -> Option<ROQueryItem<'_, Q>> {
        match self.get_single() {
            Ok(item) => Some(item),
            Err(QuerySingleError::NoEntities(_)) => None,
            Err(QuerySingleError::MultipleEntities(_)) => {
                panic!("multiple items from optional single query")
            }
        }
    }

    fn optional_single_mut(&mut self) -> Option<QueryItem<'_, Q>> {
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
