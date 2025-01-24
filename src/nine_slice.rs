use crate::widget_state::WidgetState;
use egui::{
    pos2, vec2, ColorImage, Context, Rect, Shape, TextureFilter, TextureHandle, TextureOptions, Ui,
    Vec2,
};
use image::GenericImageView;

pub(crate) const NINE_SLICE_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR: f32 = 4.0;

#[derive(Clone)]
pub(crate) struct NineSliceCache {
    pub texture: TextureHandle,
    pub uvs: Vec<Rect>,
    pub destinations: Vec<Rect>,
    pub shape: Option<Shape>,

    pub last_ui_size: Vec2,
}

impl WidgetState for NineSliceCache {}

impl NineSliceCache {
    pub fn from_texture(
        ui: &Ui,
        texture_file_path: &str,
        target_area: Rect,
        last_ui_size: Vec2,
    ) -> NineSliceCache {
        let texture = load_nine_slice_texture(ui.ctx(), texture_file_path);

        let texture_size = texture.size_vec2();
        let border_size = texture_size / NINE_SLICE_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR;

        let uvs = calculate_nine_slice_uvs(&texture_size, &border_size);
        let destinations = calculate_nine_slice_destinations(&target_area, &border_size);

        NineSliceCache {
            texture,
            uvs,
            shape: None,
            destinations,
            last_ui_size,
        }
    }
}

pub(crate) fn load_nine_slice_texture(ctx: &Context, file_path: &str) -> TextureHandle {
    let image = image::open(file_path).expect("Failed to open image");
    let (width, height) = image.dimensions();

    let image_rgba8 = image.to_rgba8();
    let image_data = image_rgba8.as_flat_samples();

    let color_image = ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        image_data.as_slice(),
    );

    let texture_options = TextureOptions {
        magnification: TextureFilter::Nearest,
        minification: TextureFilter::Nearest,

        ..Default::default()
    };

    ctx.load_texture(file_path, color_image, texture_options)
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
