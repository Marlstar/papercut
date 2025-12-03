#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Backend {
    Hyprpaper,
    None,
}

mod stringified;
stringified::backends![
    Hyprpaper "hyprpaper",
    None "none",
];
