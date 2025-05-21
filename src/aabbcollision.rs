use std::f32::consts::FRAC_1_SQRT_2;

use bevy::prelude::*;

use crate::util::{self, RectSegment};


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
    pub fn entity_in_event(&self, e: Entity) -> bool {
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
        let this_bounds : Rect;
        let other_bounds : Rect;
        if this == self.l_entity {
            this_bounds = self.l_bounds;
            other_bounds = self.r_bounds;
        }
        else {
            self.assert_entity_in_event(this);
            this_bounds = self.r_bounds;
            other_bounds = self.l_bounds;
        }

        match util::rect_segment_of_point(this_bounds, other_bounds.center()) {
            RectSegment::DOWN => Vec2::new(0.0, -1.0),
            RectSegment::UP => Vec2::new(0.0, 1.0),
            RectSegment::RIGHT => Vec2::new(1.0, 0.0),
            RectSegment::LEFT => Vec2::new(-1.0, 0.0),
            RectSegment::UPPERRIGHT => Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            RectSegment::LOWERRIGHT => Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            RectSegment::UPPERLEFT => Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            RectSegment::LOWERLEFT => Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            RectSegment::MIDDLE => Vec2::ZERO,
            _ => panic!("RectSegment undefined"),
        }
    }


    pub fn debug_system(mut reader: EventReader<AABBCollisionEvent>) {
        for event in reader.read() {
            println!("{:?}", event);
        }
    }


    fn assert_entity_in_event(&self, e: Entity) {
        assert!(self.entity_in_event(e), "Entity not used in the Event. Use a try-method instead");
    }
}
