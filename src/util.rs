use std::{f32::consts::FRAC_1_SQRT_2, result};

use bevy::prelude::*;


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
        const ALL = 0b0000_1111;
        const NONE = 0b0000_0000;

        const LEFT_YFLAGS = 0b0000_1110;
        const RIGHT_YFLAGS = 0b0000_1101;
        const UP_XFLAGS = 0b0000_0111;
        const DOWN_XFLAGS = 0b0000_1011;
    }
}

impl RectSegment {
    pub fn normal(&self) -> Vec2 {
        match *self {
            RectSegment::DOWN | RectSegment::DOWN_XFLAGS => Vec2::new(0.0, -1.0),
            RectSegment::UP | RectSegment::UP_XFLAGS => Vec2::new(0.0, 1.0),
            RectSegment::RIGHT | RectSegment::RIGHT_YFLAGS => Vec2::new(1.0, 0.0),
            RectSegment::LEFT | RectSegment::LEFT_YFLAGS => Vec2::new(-1.0, 0.0),
            RectSegment::UPPERRIGHT => Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            RectSegment::LOWERRIGHT => Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            RectSegment::UPPERLEFT => Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            RectSegment::LOWERLEFT => Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            RectSegment::MIDDLE | RectSegment::NONE | RectSegment::ALL | RectSegment::XFLAGS | RectSegment::YFLAGS => Vec2::ZERO,
            _ => {
                println!("ERROR: RectSegment {:#08b} undefined", self);
                Vec2::ZERO
            },
        }
    }
}


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


pub fn rect_expand(rect: Rect, expand: Vec2) -> Rect {
    Rect { min: rect.min - expand / 2.0, max: rect.max + expand / 2.0 }
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


pub fn rect_dist_outside(rect: Rect, from: Vec2) -> (RectSegment, f32) {
    let distances_outside = [
        (RectSegment::RIGHT, rect.max.x - from.x),
        (RectSegment::LEFT, from.x - rect.min.x),
        (RectSegment::UP, rect.max.y - from.y),
        (RectSegment::DOWN, from.y - rect.min.y),
    ];

    // We can just use x because in that case
    const NONE: (RectSegment, f32) = (RectSegment::NONE, f32::MAX);
    let mut result = NONE;

    for (seg, dist) in distances_outside {

        let is_outside = dist < 0.0;
        if is_outside {
            return NONE;
        }

        if dist <= result.1 {
            result.0 = seg;
            result.1 = dist;
        }
    }

    result
}


pub fn rect_to_outside(rect: Rect, from: Vec2) -> Vec2 {
    let (seg, dist) = rect_dist_outside(rect, from);
    seg.normal() * dist
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_to_outside_tests() {
        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(10.0)),
            Vec2::new(15.0, 10.0)
        ) == Vec2::ZERO);

        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(10.0)),
            Vec2::new(1.0, 10.0)
        ) == Vec2::ZERO);

        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(10.0)),
            Vec2::new(-25.0, 10.0)
        ) == Vec2::ZERO);

        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(10.0)),
            Vec2::new(100.0, -10.0)
        ) == Vec2::ZERO);

        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(10.0)),
            Vec2::new(10.0, -10.0)
        ) == Vec2::ZERO);

        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::new(0.0, 5.0), Vec2::splat(10.0)),
            Vec2::new(0.0, -6.0)
        ) == Vec2::ZERO);

        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(10.0)),
            Vec2::new(0.0, -6.0)
        ) == Vec2::new(0.0, -4.0));

        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(10.0)),
            Vec2::new(8.0, 9.0)
        ) == Vec2::new(0.0, 1.0));

        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(10.0)),
            Vec2::new(9.0, -3.0)
        ) == Vec2::new(1.0, 0.0));

        assert!(rect_to_outside(
            Rect::from_center_half_size(Vec2::new(0.0, -5.0), Vec2::splat(10.0)),
            Vec2::new(0.0, -3.0)
        ) == Vec2::new(0.0, 8.0));
    }
}
