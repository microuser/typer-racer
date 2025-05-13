use crate::ReplayEvent;

#[cfg(target_arch = "wasm32")]
pub fn save_replay(replay: &[ReplayEvent]) {
    use web_sys::window;
    if let Ok(json) = serde_json::to_string(replay) {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("typer_racer_replay", &json);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_replay(replay: &[ReplayEvent]) {
    if let Ok(json) = serde_json::to_string(replay) {
        let _ = std::fs::write("replay.json", json);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load_replay() -> Vec<ReplayEvent> {
    use web_sys::window;
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(Some(json)) = storage.get_item("typer_racer_replay") {
            serde_json::from_str(&json).unwrap_or_default()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_replay() -> Vec<ReplayEvent> {
    if let Ok(json) = std::fs::read_to_string("replay.json") {
        serde_json::from_str(&json).unwrap_or_default()
    } else {
        Vec::new()
    }
}
