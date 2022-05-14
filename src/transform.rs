use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

pub trait TransformUtils {
    fn set_world_translation(&mut self, global_transform: &GlobalTransform, world_position: Vec3);
}

impl TransformUtils for Transform {
    fn set_world_translation(&mut self, global_transform: &GlobalTransform, world_position: Vec3) {
        /*println!(
            "before global: {}, local: {} to world_position: {}",
            global_transform.translation, self.translation, world_position
        );*/

        let parent_position = global_transform.translation - self.translation;
        //println!("parent: {}", parent_position);

        let local_position = world_position - parent_position;
        if self.translation.distance_squared(local_position) > f32::EPSILON * f32::EPSILON {
            self.translation = local_position;
        }
        //println!("after local: {}", self.translation);
    }
}

#[derive(WorldQuery)]
#[world_query(derive(Debug))]
pub struct TransformQuery<'w> {
    pub local: &'w Transform,
    pub global: &'w GlobalTransform,
}

#[derive(WorldQuery)]
#[world_query(mutable, derive(Debug))]
pub struct TransformQueryMut<'w> {
    pub local: &'w mut Transform,
    pub global: &'w GlobalTransform,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_translation_no_hierarchy() {
        let mut world = World::default();

        let start_position = Vec3::new(5.0, 6.0, 7.0);
        let position = Vec3::new(-10.0, -11.0, -12.0);

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(move |mut query: Query<TransformQueryMut>| {
            let mut transform = query.single_mut();
            transform
                .local
                .set_world_translation(transform.global, position);
        });

        let entity = world
            .spawn()
            .insert_bundle(TransformBundle {
                // have to sync the transforms manually
                local: Transform::from_translation(start_position),
                global: GlobalTransform::from_translation(start_position),
            })
            .id();

        update_stage.run(&mut world);

        assert_eq!(
            world.get::<Transform>(entity).unwrap().translation,
            position
        );
    }

    #[test]
    fn world_translation_child() {
        let mut world = World::default();

        let start_position = Vec3::new(5.0, 6.0, 7.0);
        let position = Vec3::new(-10.0, -11.0, -12.0);

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(move |mut query: Query<TransformQueryMut, With<Parent>>| {
            let mut transform = query.single_mut();
            transform
                .local
                .set_world_translation(transform.global, position);
        });

        let parent = world
            .spawn()
            .insert_bundle(TransformBundle {
                // have to sync the transforms manually
                local: Transform::from_translation(start_position),
                global: GlobalTransform::from_translation(start_position),
            })
            .id();

        let child = world
            .spawn()
            .insert_bundle(TransformBundle {
                // have to sync the transforms manually
                local: Transform::from_translation(start_position),
                global: GlobalTransform::from_translation(start_position * 2.0),
            })
            .id();

        world
            .get_entity_mut(parent)
            .unwrap()
            .push_children(&[child]);

        update_stage.run(&mut world);

        assert_eq!(
            world.get::<Transform>(child).unwrap().translation,
            position - start_position
        );
    }
}
