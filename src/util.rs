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
