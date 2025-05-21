use bevy::prelude::*;

use crate::util;


#[derive(Component, Debug, Default)]
pub struct AABBCollider {
    pub bounds: Rect,
}


#[derive(Component)]
pub struct AABBCollisionAvoider;


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
    pub fn contains(&self, e: Entity) -> bool {
        e == self.l_entity || e == self.r_entity
    }


    pub fn try_other_entity(&self, this: Entity) -> Option<Entity> {
        if this == self.l_entity {
            return Some(self.r_entity);
        }
        if this == self.r_entity {
            return Some(self.l_entity);
        }
        None
    }


    pub fn other_bounds(&self, this: Entity) -> Rect {
        if this == self.l_entity {
            return self.r_bounds;
        }
        self.assert_entity_in_event(this);
        self.l_bounds
    }


    /// The normal of the contact point of the passed entity
    pub fn normal_of(&self, this: Entity) -> Vec2 {
        let (this_bounds, other_bounds) = self.this_other_bounds(this);
        util::rect_seg_normal(this_bounds, other_bounds.center()).normal()
    }


    /// What path this needs to take to get outside the Collision area of other
    pub fn to_avoid(&self, this: Entity) -> Vec2 {
        let (this_bounds, other_bounds) = self.this_other_bounds(this);
        let no_area = util::rect_expand(other_bounds, this_bounds.size());
        util::rect_to_outside(no_area, this_bounds.center())
    }


    pub fn debug_system(mut reader: EventReader<AABBCollisionEvent>) {
        for event in reader.read() {
            println!("{:?}", event);
        }
    }


    pub fn this_other_bounds(&self, this: Entity) -> (Rect, Rect) {
        self.assert_entity_in_event(this);
        if this == self.l_entity {
            return (self.l_bounds, self.r_bounds);
        }
        (self.r_bounds, self.l_bounds)
    }


    fn assert_entity_in_event(&self, e: Entity) {
        assert!(self.contains(e), "Entity not used in the Event. Use a try-method instead");
    }
}


impl AABBCollisionAvoider {
    pub fn fixedupdate_system(
        query: Query<(&mut Transform, Entity), With<AABBCollisionAvoider>>,
        mut reader: EventReader<AABBCollisionEvent>,
    ) {
        if reader.is_empty() {
            return;
        }

        for (mut trs, entity) in query {
            for event in reader.read() {
                if !event.contains(entity) {
                    continue;
                }

                // let prev_pos = trs.translation;
                trs.translation += event.to_avoid(entity).extend(0.0);
                // println!("Correcting \"{}\" from {} to {}", entity, prev_pos, trs.translation);
            }
        }
    }
}
