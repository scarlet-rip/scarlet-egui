use crate::egui::WidgetState;
use egui::{
    pos2, vec2, Color32, ColorImage, Context, Rect, TextureFilter, TextureHandle, TextureOptions,
    Ui, Vec2,
};
use image::GenericImageView;

// Standard for 9slice
// texture_size should be divided by this to find the border size
pub(crate) const SLICE_9_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR: f32 = 4.0;

#[derive(Clone)]
pub(crate) struct Slice9State {
    pub texture: TextureHandle,
    pub uvs: Vec<Rect>,
    pub destinations: Vec<Rect>,

    pub is_dirty: bool,
}

impl WidgetState for Slice9State {}

impl Slice9State {
    pub fn from_texture(ui: &Ui, texture_file_path: &str, target_area: Rect) -> Slice9State {
        let texture = load_9slice_texture(ui.ctx(), texture_file_path);

        let texture_size = texture.size_vec2();
        let border_size = texture_size / SLICE_9_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR;

        let uvs = calculate_9slice_uvs(&texture_size, &border_size);
        let destinations = calculate_9slice_destinations(&target_area, &border_size);

        Slice9State {
            texture,
            uvs,
            destinations,
            is_dirty: true,
        }
    }
}

pub(crate) fn draw_9slice(
    ui: &Ui,
    texture: &TextureHandle,
    uvs: &[Rect],
    destinations: &Vec<Rect>,
    tint: Option<Color32>,
) {
    for (uv, destination) in uvs.iter().zip(destinations) {
        ui.painter().image(
            texture.id(),
            *destination,
            *uv,
            tint.unwrap_or(Color32::WHITE),
        );
    }
}

pub(crate) fn load_9slice_texture(ctx: &Context, file_path: &str) -> TextureHandle {
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

pub(crate) fn calculate_9slice_uvs(texture_size: &Vec2, border_size: &Vec2) -> Vec<Rect> {
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

pub(crate) fn calculate_9slice_destinations(target_area: &Rect, border_size: &Vec2) -> Vec<Rect> {
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
