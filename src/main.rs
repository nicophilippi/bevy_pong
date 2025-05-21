use bevy::prelude::*;


const PADDLE_SIZE: Vec2 = Vec2::new(30.0, 150.0);
const PADDLE_CENTER_OFFSET: f32 = (WORLD_SIZE.x - PADDLE_SIZE.x) / 2.0;
const PADDLE_COLOR: Color = Color::WHITE;
const PADDLE_SPEED: f32 = 300.0;
const PADDLE_RIGHT_UP: KeyCode = KeyCode::ArrowUp;
const PADDLE_RIGHT_DOWN: KeyCode = KeyCode::ArrowDown;
const PADDLE_LEFT_UP: KeyCode = KeyCode::KeyW;
const PADDLE_LEFT_DOWN: KeyCode = KeyCode::KeyS;

const WORLD_SIZE: Vec2 = Vec2::new(1100.0, 600.0);


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
            .add_systems(FixedUpdate, TransformInputMover::fixedupdate_move_system)
            .add_systems(FixedPostUpdate, (
                TransformBoundsBox::fixedpostupdate_system,
                AABBCollider::fixedupdate_collisiondetect_system,
                AABBCollisionEvent::debug_system,
            ))
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
}

fn spawn_aabb_col_box(commands: &mut Commands, in_rect: Rect) {
    const COL_WIDTH: f32 = 100.0;
    for r in rect_box_in_with_rects(in_rect, COL_WIDTH) {
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

    commands.spawn((trs, sprite, mover, bounds));
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
    pub fn fixedpostupdate_system(query: Query<(&TransformBoundsBox, &mut Transform)>) {
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



#[derive(Component, Default, Debug)]
struct AABBCollider {
    pub bounds: Rect,
}

#[derive(Event, Debug)]
struct AABBCollisionEvent {
    pub l_entity: Entity,
    pub l_bounds: Rect,
    pub r_entity: Entity,
    pub r_bounds: Rect,
}

impl AABBCollider {
    pub fn fixedupdate_collisiondetect_system(
        query: Query<(&AABBCollider, Option<&Transform>, Entity)>,
        mut event_writer: EventWriter<AABBCollisionEvent>,
    ) {
        for [l, r] in query.iter_combinations() {
            let (l_col, l_trs, l_entity) = l;
            let (r_col, r_trs, r_entity) = r;

            if std::ptr::eq(l_col,  r_col) {
                continue;
            }

            let l_offset = Self::offset_by_trs(l_trs);
            let r_offset = Self::offset_by_trs(r_trs);
            let l_max = l_col.bounds.max + l_offset;
            let l_min = l_col.bounds.min + l_offset;
            let r_max = r_col.bounds.max + r_offset;
            let r_min = r_col.bounds.min + r_offset;

            if l_min.x < r_max.x && l_max.x > r_min.x && l_min.y < r_max.y && l_max.y > r_min.y {
                event_writer.write(AABBCollisionEvent {
                    l_entity,
                    l_bounds: l_col.bounds,
                    r_entity,
                    r_bounds: r_col.bounds,
                });
            }
        }
    }


    fn offset_by_trs(trs: Option<&Transform>) -> Vec2 {
        if let Some(x) = trs {
            return x.translation.xy();
        } Vec2::ZERO
    }
}

impl AABBCollisionEvent {
    pub fn other_entity(&self, this: Entity) -> Entity {
        if this == self.l_entity {
            return self.r_entity;
        } self.l_entity
    }

    pub fn other_bounds(&self, this: Entity) -> Rect {
        if this == self.l_entity {
            return self.r_bounds;
        } self.l_bounds
    }


    pub fn debug_system(mut reader: EventReader<AABBCollisionEvent>) {
        for event in reader.read() {
            println!("{:?}", event);
        }
    }
}


fn rect_box_in_with_rects(rect: Rect, result_width: f32) -> [Rect; 4] {
    let center = rect.center();
    let half_result_width = result_width / 2.0;
    let rect_width = rect.width();
    let rect_height = rect.height();
    [
        // Upper
        Rect::from_center_size(Vec2::new(center.x, rect.max.y + half_result_width), Vec2::new(rect_width, result_width)),
        // Lower
        Rect::from_center_size(Vec2::new(center.x, rect.min.y - half_result_width), Vec2::new(rect_width, result_width)),
        // Right
        Rect::from_center_size(Vec2::new(rect.max.x + half_result_width, center.y), Vec2::new(result_width, rect_height)),
        // Left
        Rect::from_center_size(Vec2::new(rect.min.x - half_result_width, center.y), Vec2::new(result_width, rect_height)),
    ]
}
