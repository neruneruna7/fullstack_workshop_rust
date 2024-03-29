use std::fmt;

pub enum ButtonType {
    Primary,
    Secondary,
}

// ボタンタイプに基づいてtailwindのクラスを返す
impl fmt::Display for ButtonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ButtonType::Primary => write!(f, "bg-blue-700 hover:bg-blue-800 active:bg-blue-900"),
            ButtonType::Secondary => write!(f, "bg-rose-700 hover:bg-rose-800 active:bg-rose-900"),
        }
    }
}
