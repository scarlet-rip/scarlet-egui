use crate::{
    nine_slice::{
        draw_nine_slice, NineSliceCache, NINE_SLICE_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR,
    },
    widget_state::{WidgetState, WidgetStateType},
};
use egui::{Color32, Frame as EguiFrame, Rect, Response, Stroke, Ui};

pub struct Frame<'a> {
    id_salt: Option<&'a str>,
    inner_frame: EguiFrame,
    is_inner_frame_transparent: bool,
    frame_border: FrameBorder<'a>,
}

pub enum FrameBorder<'a> {
    NineSlice(&'a str, Option<Color32>), // texture_file_path & tint
}

impl<'a> Frame<'a> {
    pub fn new(frame_border: FrameBorder<'a>) -> Self {
        Frame {
            id_salt: None,
            inner_frame: EguiFrame::none(),
            is_inner_frame_transparent: true,
            frame_border,
        }
    }

    pub fn show(&mut self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        let frame_response = self
            .inner_frame
            .show(ui, |ui| {
                content(ui);
            })
            .response;

        if self.is_inner_frame_transparent {
            self.inner_frame = self
                .inner_frame
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
        }

        match self.frame_border {
            FrameBorder::NineSlice(texture_file_path, tint) => {
                self.handle_nine_slice_frame_border(
                    ui,
                    texture_file_path,
                    tint,
                    frame_response.rect,
                );
            }
        }

        frame_response
    }

    fn handle_nine_slice_frame_border(
        &mut self,
        ui: &mut Ui,
        texture_file_path: &'a str,
        tint: Option<Color32>,
        target_area: Rect,
    ) {
        let mut nine_slice_cache =
            NineSliceCache::load_or_new(ui, self.id_salt, WidgetStateType::Runtime, || {
                NineSliceCache::from_texture(ui, texture_file_path, target_area)
            });

        draw_nine_slice(
            ui,
            &nine_slice_cache.texture,
            &nine_slice_cache.uvs,
            &nine_slice_cache.destinations,
            tint,
        );

        let border_size = nine_slice_cache.texture.size_vec2()
            / NINE_SLICE_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR;

        self.inner_frame = self.inner_frame.inner_margin(
            self.inner_frame.inner_margin + egui::Margin::symmetric(border_size.x, border_size.y),
        );

        if nine_slice_cache.is_dirty {
            nine_slice_cache = NineSliceCache::from_texture(ui, texture_file_path, target_area);
        }

        nine_slice_cache.save_state(ui, self.id_salt, WidgetStateType::Runtime);
    }

    pub fn id_salt(mut self, id_salt: &'a str) -> Self {
        self.id_salt = Some(id_salt);

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
