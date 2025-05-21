use bevy::prelude::*;


#[derive(Component, Default, Debug)]
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
