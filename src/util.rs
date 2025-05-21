use bevy::prelude::*;

pub fn rect_box_in_with_rects(rect: Rect, result_width: f32) -> [Rect; 4] {
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


pub fn rect_offset(rect: Rect, offset: Vec2) -> Rect {
    Rect { min: rect.min + offset, max: rect.max + offset }
}


/// Offsets the rect by translation, and resizes it by the scale from the center
/// 
/// All Operations use the xy() of Vec3s
pub fn rect_transform_no_rot(rect: Rect, trs: &Transform) -> Rect {
    Rect::from_center_size(rect.center() + trs.translation.xy(),
        rect.size() * trs.scale.xy())
}

/// Offsets the rect by translation, and resizes it by the scale from the center
/// 
/// All Operations use the xy() of Vec3s
pub fn rect_try_transform_no_rot(rect: Rect, trs: Option<&Transform>) -> Rect {
    if let Some(trs) = trs {
        return rect_transform_no_rot(rect, trs);
    }
    rect
}


pub fn rect_segment_of_point(rect: Rect, p: Vec2) -> RectSegment {
    let mut result = RectSegment::MIDDLE;
    if p.x < rect.min.x {
        result |= RectSegment::LEFT;
    }
    else if p.x > rect.max.x {
        result |= RectSegment::RIGHT;
    }
    if p.y < rect.min.y {
        result |= RectSegment::DOWN;
    }
    else if p.y > rect.max.y {
        result |= RectSegment::UP;
    }
    result
}


bitflags::bitflags! {
    #[derive(PartialEq)]
    pub struct RectSegment : u8 {
        const LEFT = 0b0000_0010;
        const RIGHT = 0b0000_0001;
        const UP = 0b0000_0100;
        const DOWN = 0b0000_1000;
        const MIDDLE = 0b0000_0000;
        const UPPERLEFT = 0b0000_0110;
        const UPPERRIGHT = 0b0000_0101;
        const LOWERLEFT = 0b0000_1010;
        const LOWERRIGHT = 0b0000_1001;
    }
}
