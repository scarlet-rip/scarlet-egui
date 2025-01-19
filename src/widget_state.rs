use egui::{Id, Ui};
use std::marker::{Send, Sync};

pub enum WidgetStateType {
    Runtime,
    Persistent,
}

pub trait WidgetState: Clone + Sync + Send + 'static {
    fn try_load(ui: &Ui, id_salt: Option<&str>, state_type: WidgetStateType) -> Option<Self> {
        let id = generate_id(ui, id_salt, &state_type);

        match state_type {
            WidgetStateType::Runtime => ui.ctx().memory(|memory| memory.data.get_temp::<Self>(id)),
            WidgetStateType::Persistent => ui
                .ctx()
                .memory_mut(|memory| memory.data.get_persisted::<Self>(id)),
        }
    }

    fn load_or_default(ui: &Ui, id_salt: Option<&str>, state_type: WidgetStateType) -> Self
    where
        Self: Default,
    {
        Self::try_load(ui, id_salt, state_type).unwrap_or_default()
    }

    fn load_or_new<C>(ui: &Ui, id_salt: Option<&str>, state_type: WidgetStateType, state: C) -> Self
    where
        C: FnOnce() -> Self,
    {
        Self::try_load(ui, id_salt, state_type).unwrap_or_else(state)
    }

    fn save_state(self, ui: &Ui, id_salt: Option<&str>, state_type: WidgetStateType) {
        let id = generate_id(ui, id_salt, &state_type);

        match state_type {
            WidgetStateType::Runtime => ui
                .ctx()
                .memory_mut(|memory| memory.data.insert_temp::<Self>(id, self)),
            WidgetStateType::Persistent => ui
                .ctx()
                .memory_mut(|memory| memory.data.insert_persisted::<Self>(id, self)),
        }
    }
}

fn generate_id(ui: &Ui, id_salt: Option<&str>, state_type: &WidgetStateType) -> Id {
    match state_type {
        WidgetStateType::Runtime => ui.id().with(id_salt),
        WidgetStateType::Persistent => ui.make_persistent_id(id_salt),
    }
}
