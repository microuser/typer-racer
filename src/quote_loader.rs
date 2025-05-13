use crate::MeditationQuote;

#[cfg(not(target_arch = "wasm32"))]
pub fn load_expanded_meditations() -> Vec<MeditationQuote> {
    let path = "level/expanded-meditations.json";
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return vec![],
    };
    let mut contents = String::new();
    use std::io::Read;
    if file.read_to_string(&mut contents).is_err() {
        return vec![];
    }
    let parsed: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(val) => val,
        Err(_) => return vec![],
    };
    let Some(array) = parsed.get("expanded_meditations").and_then(|v| v.as_array()) else {
        return vec![];
    };
    array.iter().filter_map(|item| {
        let meditation = item.get("expanded_meditation")?.as_str()?.to_string();
        Some(MeditationQuote {
            original_quotes: vec![meditation.clone()],
            expanded_meditation: meditation,
        })
    }).collect()
}

#[cfg(target_arch = "wasm32")]
pub fn load_expanded_meditations() -> Vec<MeditationQuote> {
    vec![MeditationQuote {
        original_quotes: vec!["Practice makes perfect.".to_string()],
        expanded_meditation: "Focus on steady improvement.".to_string(),
    }]
}
