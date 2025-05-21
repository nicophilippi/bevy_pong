use std::{f32::consts::FRAC_1_SQRT_2, result};

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


/// Middle is equivalent to from being inside rect
pub fn rect_outer_seg(rect: Rect, from: Vec2) -> RectSegment {
    let mut result = RectSegment::MIDDLE;
    if from.x < rect.min.x {
        result |= RectSegment::LEFT;
    }
    else if from.x > rect.max.x {
        result |= RectSegment::RIGHT;
    }
    if from.y < rect.min.y {
        result |= RectSegment::DOWN;
    }
    else if from.y > rect.max.y {
        result |= RectSegment::UP;
    }
    result
}

pub fn rect_seg_from_center(rect: Rect, from: Vec2) -> RectSegment {
    let mut result = RectSegment::MIDDLE;
    let center = rect.center();
    if from.x < center.x {
        result |= RectSegment::LEFT;
    }
    else if from.x > center.x {
        result |= RectSegment::RIGHT;
    }
    if from.y < center.y {
        result |= RectSegment::DOWN;
    }
    else if from.y > center.y {
        result |= RectSegment::UP;
    }
    result
}

pub fn rect_seg_normal(rect: Rect, from: Vec2) -> RectSegment {
    let mut result = rect_seg_from_center(rect, from);
    let center = rect.center();
    let size = rect.size();
    // Division by size is necessary. Without it, RIGHT would be
    // returned when passing a Rect with size (10,1) and (5,2)
    // as Vec2 despite UP clearly being expected
    let center_dist_x = (from.x - center.x).abs() / size.x;
    let center_dist_y = (from.y - center.y).abs() / size.y;
    if center_dist_x > center_dist_y {
        result &= RectSegment::XFLAGS;
    }
    else if center_dist_x < center_dist_y {
        result &= RectSegment::YFLAGS;
    }
    result
}


bitflags::bitflags! {
    #[derive(PartialEq, Debug)]
    pub struct RectSegment : u8 {
        const MIDDLE = 0b0000_0000;
        const LEFT = 0b0000_0010;
        const RIGHT = 0b0000_0001;
        const UP = 0b0000_0100;
        const DOWN = 0b0000_1000;
        const UPPERLEFT = 0b0000_0110;
        const UPPERRIGHT = 0b0000_0101;
        const LOWERLEFT = 0b0000_1010;
        const LOWERRIGHT = 0b0000_1001;

        const YFLAGS = 0b0000_1100;
        const XFLAGS = 0b0000_0011;
    }
}

impl RectSegment {
    pub fn normal(&self) -> Vec2 {
        match *self {
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
}
