use crate::{
    egui::WidgetState,
    slice9::{draw_9slice, Slice9State, SLICE_9_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR},
};
use egui::{Color32, Frame, Response, Stroke, Ui};

pub struct Group<'a> {
    id_salt: &'a str,
    texture_file_path: &'a str,
}

impl<'a> Group<'a> {
    pub fn new(id_salt: &'a str, texture_file_path: &'a str) -> Self {
        Group {
            id_salt,
            texture_file_path,
        }
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) -> Response {
        let mut slice9_cache = Slice9State::load_or_new(ui, self.id_salt, || {
            Slice9State::from_texture(ui, self.texture_file_path, ui.min_rect())
        });

        draw_9slice(
            ui,
            &slice9_cache.texture,
            &slice9_cache.uvs,
            &slice9_cache.destinations,
            Some(Color32::DARK_RED),
        );

        let border_size = slice9_cache.texture.size_vec2()
            / SLICE_9_BORDER_SIZE_FROM_TEXTURE_SIZE_CONVERSION_FACTOR;

        // I'm sure there's a better way, but I don't wanna bother with all that right now.
        let frame_response = Frame::group(ui.style())
            .fill(Color32::TRANSPARENT)
            .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
            .inner_margin(egui::Margin::symmetric(border_size.x, border_size.y))
            .show(ui, |ui| {
                content(ui);
            })
            .response;

        if slice9_cache.is_dirty {
            slice9_cache =
                Slice9State::from_texture(ui, self.texture_file_path, frame_response.rect);
        }

        slice9_cache.save_state(ui, self.id_salt);

        frame_response
    }
}
