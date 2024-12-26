use bevy::prelude::*;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum CollisionPoint {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Left,
    Top,
    Right,
    Bottom,
}

fn get_collision_point_position(
    transform: &Transform,
    collision_point: CollisionPoint,
) -> (f32, f32) {
    let half_width = transform.scale.x / 2.0;
    let half_height = transform.scale.y / 2.0;

    let center_x = transform.translation.x;
    let center_y = transform.translation.y;

    match collision_point {
        CollisionPoint::TopLeft => (center_x - half_width, center_y + half_height),
        CollisionPoint::TopRight => (center_x + half_width, center_y + half_height),
        CollisionPoint::BottomLeft => (center_x - half_width, center_y - half_height),
        CollisionPoint::BottomRight => (center_x + half_width, center_y - half_height),
        CollisionPoint::Left => (center_x - half_width, center_y),
        CollisionPoint::Top => (center_x, center_y + half_height),
        CollisionPoint::Right => (center_x + half_width, center_y),
        CollisionPoint::Bottom => (center_x, center_y - half_height),
    }
}

fn is_collision_point_colliding(
    collision_point: CollisionPoint,
    colliding_transform: &Transform,
    collided_transform: &Transform,
) -> bool {
    let collided_translation = collided_transform.translation;
    let collided_scale = collided_transform.scale;

    let half_width = collided_scale.x / 2.0;
    let half_height = collided_scale.y / 2.0;

    let left = collided_translation.x - half_width;
    let right = collided_translation.x + half_width;
    let bottom = collided_translation.y - half_height;
    let top = collided_translation.y + half_height;

    let (x, y) = get_collision_point_position(colliding_transform, collision_point);

    x > left && x < right && y > bottom && y < top
}

pub fn are_collision_points_colliding(
    colliding_transform: &Transform,
    collided_transform: &Transform,
) -> bool {
    is_collision_point_colliding(
        CollisionPoint::TopLeft,
        colliding_transform,
        collided_transform,
    ) || is_collision_point_colliding(
        CollisionPoint::TopRight,
        colliding_transform,
        collided_transform,
    ) || is_collision_point_colliding(
        CollisionPoint::BottomRight,
        colliding_transform,
        collided_transform,
    ) || is_collision_point_colliding(
        CollisionPoint::BottomLeft,
        colliding_transform,
        collided_transform,
    ) || is_collision_point_colliding(
        CollisionPoint::Left,
        colliding_transform,
        collided_transform,
    ) || is_collision_point_colliding(CollisionPoint::Top, colliding_transform, collided_transform)
        || is_collision_point_colliding(
            CollisionPoint::Right,
            colliding_transform,
            collided_transform,
        )
        || is_collision_point_colliding(
            CollisionPoint::Bottom,
            colliding_transform,
            collided_transform,
        )
}
