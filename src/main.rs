use bevy::prelude::*;
use aabbcollision::*;

mod util;
mod aabbcollision;

const PADDLE_SIZE: Vec2 = Vec2::new(30.0, 150.0);
const PADDLE_CENTER_OFFSET: f32 = (WORLD_SIZE.x - PADDLE_SIZE.x) / 2.0;
const PADDLE_COLOR: Color = Color::WHITE;
const PADDLE_SPEED: f32 = 300.0;
const PADDLE_RIGHT_UP: KeyCode = KeyCode::ArrowUp;
const PADDLE_RIGHT_DOWN: KeyCode = KeyCode::ArrowDown;
const PADDLE_LEFT_UP: KeyCode = KeyCode::KeyW;
const PADDLE_LEFT_DOWN: KeyCode = KeyCode::KeyS;

const WORLD_SIZE: Vec2 = Vec2::new(1100.0, 600.0);

const BALL_START_VELOCITY: Vec2 = Vec2::new(150.0, 150.0);
const BALL_START_POS: Vec2 = Vec2::new(-WORLD_SIZE.x / 2.0 + 20.0, -150.0);
const BALL_COLOR: Color = Color::WHITE;
const BALL_SIZE: Vec2 = Vec2::splat(10.0);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PongGamePlugin)
        .run();
}


struct PongGamePlugin;

impl Plugin for PongGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_up)
            .add_systems(FixedUpdate, (
                (
                    TransformInputMover::fixedupdate_move_system,
                    Ball::fixedupdate_move_system,
                ),
                TransformBoundsBox::after_moves_system,
                AABBCollider::fixedupdate_collisiondetect_system,
                (
                    AABBCollisionAvoider::fixedupdate_system,
                    Ball::fixedupdate_bounce_system,
                ),
                // AABBCollisionEvent::debug_system,
            ).chain())
            .add_event::<AABBCollisionEvent>();
    }
}


fn start_up(mut commands: Commands) {

    commands.spawn(Camera2d);

    // World background
    commands.spawn((
        Sprite::from_color(Color::BLACK, WORLD_SIZE),
        Transform::from_translation(Vec3::default().with_z(-1.0))
    ));

    spawn_aabb_col_box(&mut commands,
        Rect::from_center_size(Vec2::ZERO, WORLD_SIZE));

    // Left
    spawn_paddle(&mut commands, PADDLE_LEFT_UP, PADDLE_LEFT_DOWN,
        Vec3::new(-PADDLE_CENTER_OFFSET, 0.0, 0.0));

    // Right
    spawn_paddle(&mut commands, PADDLE_RIGHT_UP, PADDLE_RIGHT_DOWN,
        Vec3::new(PADDLE_CENTER_OFFSET, 0.0, 0.0));

    // Ball
    {
        let ball = Ball { velocity: BALL_START_VELOCITY };
        let sprite = Sprite::from_color(BALL_COLOR, Vec2::ONE);
        let trs = Transform::from_scale(BALL_SIZE.extend(1.0))
            .with_translation(BALL_START_POS.extend(0.0));
        let collider = AABBCollider::from_size(Vec2::ONE);
        commands.spawn((trs, ball, sprite, collider, AABBCollisionAvoider));
    }
}

fn spawn_aabb_col_box(commands: &mut Commands, in_rect: Rect) {
    const COL_WIDTH: f32 = 100.0;
    for r in util::rect_box_in_with_rects(in_rect, COL_WIDTH) {
        commands.spawn(AABBCollider { bounds: r });
        // commands.spawn((Transform::from_translation(r.center().extend(0.0)), Sprite::from_color(Color::WHITE, r.size())));
    }
}

fn spawn_paddle(commands: &mut Commands, key_up: KeyCode, key_down: KeyCode, center: Vec3) {

    let trs = Transform::from_scale(PADDLE_SIZE.extend(1.0))
        .with_translation(center);

    let sprite = Sprite::from_color(PADDLE_COLOR, Vec2::ONE);

    let mover = TransformInputMover {
        speed: Vec2::default().with_y(PADDLE_SPEED),
        key_down: Some(key_down),
        key_up: Some(key_up),
        ..Default::default()
    };

    let bounds = TransformBoundsBox {
        upper_bounds: Some((WORLD_SIZE.y - PADDLE_SIZE.y) / 2.0),
        lower_bounds: Some(-(WORLD_SIZE.y - PADDLE_SIZE.y) / 2.0),
        ..Default::default()
    };

    let collider = AABBCollider::from_size(Vec2::ONE);

    commands.spawn((trs, sprite, mover, bounds, collider));
}


#[derive(Component, Debug, Default)]
struct TransformInputMover {
    pub key_up: Option<KeyCode>,
    pub key_down: Option<KeyCode>,
    pub key_left: Option<KeyCode>,
    pub key_right: Option<KeyCode>,
    pub speed: Vec2,
}

impl TransformInputMover {
    pub fn fixedupdate_move_system(query: Query<(&TransformInputMover, &mut Transform)>,
        time: Res<Time<Fixed>>,
        input: Res<ButtonInput<KeyCode>>) {

        for (mover, mut trs) in query {

            if let Some(key) = mover.key_up {
                if input.pressed(key) {
                    trs.translation.y += mover.speed.y * time.delta_secs();
                }
            }

            if let Some(key) = mover.key_down {
                if input.pressed(key) {
                    trs.translation.y -= mover.speed.y * time.delta_secs();
                }
            }

            if let Some(key) = mover.key_right {
                if input.pressed(key) {
                    trs.translation.x += mover.speed.x * time.delta_secs();
                }
            }

            if let Some(key) = mover.key_left {
                if input.pressed(key) {
                    trs.translation.x -= mover.speed.x * time.delta_secs();
                }
            }
        }
    }
}


#[derive(Default, Component, Debug)]
struct TransformBoundsBox {
    pub upper_bounds: Option<f32>,
    pub lower_bounds: Option<f32>,
    pub right_bounds: Option<f32>,
    pub left_bounds: Option<f32>,
}

impl TransformBoundsBox {
    /// Must be called after all changes on Transforms have been made
    pub fn after_moves_system(query: Query<(&TransformBoundsBox, &mut Transform)>) {
        for (bounds, mut trs) in query {

            if let Some(v) = bounds.upper_bounds {
                trs.translation.y = v.min(trs.translation.y);
            }

            if let Some(v) = bounds.lower_bounds {
                trs.translation.y = v.max(trs.translation.y);
            }

            if let Some(v) = bounds.right_bounds {
                trs.translation.x = v.min(trs.translation.x);
            }

            if let Some(v) = bounds.left_bounds {
                trs.translation.x = v.max(trs.translation.x);
            }
        }
    }
}


#[derive(Component, Debug, Default)]
struct Ball {
    pub velocity: Vec2,
}

impl Ball {
    pub fn fixedupdate_move_system(query: Query<(&Ball, &mut Transform)>, time: Res<Time<Fixed>>) {
        for (ball, mut trs) in query {
            trs.translation += (ball.velocity * time.delta_secs()).extend(0.0);
        }
    }


    pub fn fixedupdate_bounce_system(query: Query<(Entity, &mut Ball)>, mut event_reader: EventReader<AABBCollisionEvent>) {

        if event_reader.is_empty() {
            return;
        }

        for (entity, mut ball) in query {
            for event in event_reader.read() {
                if let Some(other_entity) = event.try_other_entity(entity) {
                    // let prev_velo = ball.velocity;
                    let normal = event.normal_of(other_entity);
                    ball.velocity = ball.velocity.reflect(normal);
                    // println!("Reflecting \"{}\" from velo {} to {}", entity, prev_velo, ball.velocity);
                }
            }
        }
    }
}
