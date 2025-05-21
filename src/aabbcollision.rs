use bevy::prelude::*;

use crate::util;


#[derive(Component, Debug, Default)]
pub struct AABBCollider {
    pub bounds: Rect,
}


#[derive(Event, Debug)]
pub struct AABBCollisionEvent {
    pub l_entity: Entity,
    pub l_bounds: Rect,
    pub r_entity: Entity,
    pub r_bounds: Rect,
}


impl AABBCollider {
    pub fn from_size(size: Vec2) -> Self {
        Self { bounds: Rect::from_center_size(Vec2::ZERO, size)}
    }


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

            let l_bounds = util::rect_try_transform_no_rot(l_col.bounds, l_trs);
            let r_bounds = util::rect_try_transform_no_rot(r_col.bounds, r_trs);
            let l_max = l_bounds.max;
            let l_min = l_bounds.min;
            let r_max = r_bounds.max;
            let r_min = r_bounds.min;


            if l_min.x < r_max.x && l_max.x > r_min.x && l_min.y < r_max.y && l_max.y > r_min.y {
                event_writer.write(AABBCollisionEvent {
                    l_entity,
                    l_bounds,
                    r_entity,
                    r_bounds,
                });
            }
        }
    }


    
}


impl AABBCollisionEvent {
    pub fn try_other_entity(&self, e: Entity) -> Option<Entity> {
        if e == self.l_entity {
            return Some(self.r_entity);
        }
        if e == self.r_entity {
            return Some(self.l_entity);
        }
        None
    }

    pub fn bounds_of(&self, e: Entity) -> Option<Rect> {
        if e == self.l_entity {
            return Some(self.l_bounds);
        }
        if e == self.r_entity {
            return Some(self.r_bounds);
        }
        None
    }


    pub fn debug_system(mut reader: EventReader<AABBCollisionEvent>) {
        for event in reader.read() {
            println!("{:?}", event);
        }
    }
}
