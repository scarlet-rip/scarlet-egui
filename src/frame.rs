use crate::{
    nine_slice::{
        draw_nine_slice, NineSliceCache, NINE_SLICE_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR,
    },
    widget_state::{WidgetState, WidgetStateType},
};
use egui::{Color32, Frame as EguiFrame, Response, Stroke, Ui};

pub struct Frame<'a> {
    id_salt: Option<&'a str>,
    texture_file_path: &'a str,
    tint: Option<Color32>,
    inner_frame: EguiFrame,
    is_inner_frame_transparent: bool,
}

impl<'a> Frame<'a> {
    pub fn new(texture_file_path: &'a str) -> Self {
        Frame {
            id_salt: None,
            texture_file_path,
            tint: None,
            inner_frame: EguiFrame::none(),
            is_inner_frame_transparent: true,
        }
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        let mut nine_slice_cache =
            NineSliceCache::load_or_new(ui, self.id_salt, WidgetStateType::Runtime, || {
                NineSliceCache::from_texture(ui, self.texture_file_path, ui.min_rect())
            });

        draw_nine_slice(
            ui,
            &nine_slice_cache.texture,
            &nine_slice_cache.uvs,
            &nine_slice_cache.destinations,
            self.tint,
        );

        let border_size = nine_slice_cache.texture.size_vec2()
            / NINE_SLICE_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR;

        let mut frame = self.inner_frame.inner_margin(
            self.inner_frame.inner_margin + egui::Margin::symmetric(border_size.x, border_size.y),
        );

        if self.is_inner_frame_transparent {
            frame = frame
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
        }

        let frame_response = frame
            .show(ui, |ui| {
                content(ui);
            })
            .response;

        if nine_slice_cache.is_dirty {
            nine_slice_cache =
                NineSliceCache::from_texture(ui, self.texture_file_path, frame_response.rect);
        }

        nine_slice_cache.save_state(ui, self.id_salt, WidgetStateType::Runtime);

        frame_response
    }

    pub fn id_salt(mut self, id_salt: &'a str) -> Self {
        self.id_salt = Some(id_salt);

        self
    }

    pub fn tint(mut self, tint: Color32) -> Self {
        self.tint = Some(tint);

        self
    }

    pub fn transparent_inner_frame(mut self, is_transparent: bool) -> Self {
        self.is_inner_frame_transparent = is_transparent;

        self
    }

    pub fn edit_inner_frame(mut self, inner_frame: impl FnOnce(EguiFrame) -> EguiFrame) -> Self {
        self.inner_frame = inner_frame(self.inner_frame);

        self
    }
}
