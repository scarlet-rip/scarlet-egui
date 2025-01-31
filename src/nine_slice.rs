use crate::widget_state::WidgetState;
use egui::{load::SizedTexture, pos2, vec2, Rect, Shape, Vec2};

pub(crate) const NINE_SLICE_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR: f32 = 4.0;

#[derive(Clone)]
pub(crate) struct NineSliceCache {
    pub texture: SizedTexture,
    pub uvs: Vec<Rect>,
    pub destinations: Vec<Rect>,
    pub shape: Option<Shape>,

    pub last_available_ui_size: Vec2,
}

impl WidgetState for NineSliceCache {}

impl NineSliceCache {
    pub fn from_texture(
        texture: SizedTexture,
        target_area: Rect,
        available_ui_size: Vec2,
    ) -> NineSliceCache {
        let border_size = texture.size / NINE_SLICE_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR;
        let uvs = calculate_nine_slice_uvs(&texture.size, &border_size);
        let destinations = calculate_nine_slice_destinations(&target_area, &border_size);

        NineSliceCache {
            texture,
            uvs,
            shape: None,
            destinations,
            last_available_ui_size: available_ui_size,
        }
    }
}

pub(crate) fn calculate_nine_slice_uvs(texture_size: &Vec2, border_size: &Vec2) -> Vec<Rect> {
    vec![
        // Top-left
        Rect::from_min_max(
            pos2(0.0, 0.0),
            pos2(
                border_size.x / texture_size.x,
                border_size.y / texture_size.y,
            ),
        ),
        // Top
        Rect::from_min_max(
            pos2(border_size.x / texture_size.x, 0.0),
            pos2(
                (texture_size.x - border_size.x) / texture_size.x,
                border_size.y / texture_size.y,
            ),
        ),
        // Top-right
        Rect::from_min_max(
            pos2((texture_size.x - border_size.x) / texture_size.x, 0.0),
            pos2(1.0, border_size.y / texture_size.y),
        ),
        // Left
        Rect::from_min_max(
            pos2(0.0, border_size.y / texture_size.y),
            pos2(
                border_size.x / texture_size.x,
                (texture_size.y - border_size.y) / texture_size.y,
            ),
        ),
        // Center
        Rect::from_min_max(
            pos2(
                border_size.x / texture_size.x,
                border_size.y / texture_size.y,
            ),
            pos2(
                (texture_size.x - border_size.x) / texture_size.x,
                (texture_size.y - border_size.y) / texture_size.y,
            ),
        ),
        // Right
        Rect::from_min_max(
            pos2(
                (texture_size.x - border_size.x) / texture_size.x,
                border_size.y / texture_size.y,
            ),
            pos2(1.0, (texture_size.y - border_size.y) / texture_size.y),
        ),
        // Bottom-left
        Rect::from_min_max(
            pos2(0.0, (texture_size.y - border_size.y) / texture_size.y),
            pos2(border_size.x / texture_size.x, 1.0),
        ),
        // Bottom
        Rect::from_min_max(
            pos2(
                border_size.x / texture_size.x,
                (texture_size.y - border_size.y) / texture_size.y,
            ),
            pos2((texture_size.x - border_size.x) / texture_size.x, 1.0),
        ),
        // Bottom-right
        Rect::from_min_max(
            pos2(
                (texture_size.x - border_size.x) / texture_size.x,
                (texture_size.y - border_size.y) / texture_size.y,
            ),
            pos2(1.0, 1.0),
        ),
    ]
}

pub(crate) fn calculate_nine_slice_destinations(
    target_area: &Rect,
    border_size: &Vec2,
) -> Vec<Rect> {
    vec![
        Rect::from_min_max(
            target_area.min,
            target_area.min + vec2(border_size.x, border_size.y),
        ),
        Rect::from_min_max(
            target_area.min + vec2(border_size.x, 0.0),
            target_area.min + vec2(target_area.width() - border_size.x, border_size.y),
        ),
        Rect::from_min_max(
            target_area.min + vec2(target_area.width() - border_size.x, 0.0),
            target_area.min + vec2(target_area.width(), border_size.y),
        ),
        Rect::from_min_max(
            target_area.min + vec2(0.0, border_size.y),
            target_area.min + vec2(border_size.x, target_area.height() - border_size.y),
        ),
        Rect::from_min_max(
            target_area.min + vec2(border_size.x, border_size.y),
            target_area.min
                + vec2(
                    target_area.width() - border_size.x,
                    target_area.height() - border_size.y,
                ),
        ),
        Rect::from_min_max(
            target_area.min + vec2(target_area.width() - border_size.x, border_size.y),
            target_area.min + vec2(target_area.width(), target_area.height() - border_size.y),
        ),
        Rect::from_min_max(
            target_area.min + vec2(0.0, target_area.height() - border_size.y),
            target_area.min + vec2(border_size.x, target_area.height()),
        ),
        Rect::from_min_max(
            target_area.min + vec2(border_size.x, target_area.height() - border_size.y),
            target_area.min + vec2(target_area.width() - border_size.x, target_area.height()),
        ),
        Rect::from_min_max(
            target_area.min
                + vec2(
                    target_area.width() - border_size.x,
                    target_area.height() - border_size.y,
                ),
            target_area.max,
        ),
    ]
}
