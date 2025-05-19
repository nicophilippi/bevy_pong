use bevy::prelude::*;


const PADDLE_SIZE: Vec2 = Vec2::new(30.0, 300.0);
const PADDLE_CENTER_OFFSET: f32 = 550.0;
const PADDLE_COLOR: Color = Color::WHITE;
const PADDLE_SPEED: f32 = 300.0;
const PADDLE_RIGHT_UP: KeyCode = KeyCode::ArrowUp;
const PADDLE_RIGHT_DOWN: KeyCode = KeyCode::ArrowDown;
const PADDLE_LEFT_UP: KeyCode = KeyCode::KeyW;
const PADDLE_LEFT_DOWN: KeyCode = KeyCode::KeyS;


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
            .add_systems(FixedUpdate, TransformInputMover::fixedupdate_move_system);
    }
}


fn start_up(mut commands: Commands) {

    commands.spawn(Camera2d);

    // Left
    spawn_paddle(&mut commands, PADDLE_LEFT_UP, PADDLE_LEFT_DOWN,
        Vec3::new(-PADDLE_CENTER_OFFSET, 0.0, 0.0));

    // Right
    spawn_paddle(&mut commands, PADDLE_RIGHT_UP, PADDLE_RIGHT_DOWN,
        Vec3::new(PADDLE_CENTER_OFFSET, 0.0, 0.0));
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

    commands.spawn((trs, sprite, mover));
}


#[derive(Component, Debug, Default)]
#[require(Transform)]
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


#[derive(Component, Default, Debug)]
struct AABBCollider {
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Event)]
struct AABBCollisionEvent(Entity, Entity);

impl AABBCollider {
    pub fn get_min(&self) -> Vec2 {
        self.offset - self.size / 2.0
    }

    pub fn get_min_offsetted(&self, trs: Option<&Transform>) -> Vec2 {
        if let Some(trs) = trs {
            return self.get_min() + trs.translation.xy();
        }
        self.get_min()
    }

    pub fn get_max(&self) -> Vec2 {
        self.offset + self.size / 2.0
    }

    pub fn get_max_offsetted(&self, trs: Option<&Transform>) -> Vec2 {
        if let Some(trs) = trs {
            return self.get_max() + trs.translation.xy();
        }
        self.get_max()
    }


    pub fn fixedupdate_collisiondetect_system(
        query: Query<(&AABBCollider, Option<&Transform>, Entity)>,
        event_writer: EventWriter<AABBCollisionEvent>,
    ) {
        for (l_col, l_trs, l_entity) in query {
            for (r_col, r_trs, r_entity) in query {

                // TODO: Prevent that a collision is detected 2 times in either order
                // iter_combinations_mut

                if std::ptr::eq(l_col,  r_col) {
                    continue;
                }

                let l_max = l_col.get_max_offsetted(l_trs);
                let l_min = l_col.get_min_offsetted(l_trs);
                let r_max = r_col.get_max_offsetted(r_trs);
                let r_min = r_col.get_min_offsetted(r_trs);

                if l_min.x <= r_max.x && l_max.x >= r_min.x && l_min.y <= r_max.y && l_max.y >= r_min.y {
                    event_writer
                }
            }
        }
    }
}
