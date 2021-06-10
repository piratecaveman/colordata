use crate::rgb::Rgb;
use crate::rgba::Rgba;
use crate::xrgba::Xrgba;

#[derive(Debug, Clone, Default)]
pub struct Color {
    pub hex: String,
    pub rgb: Rgb,
    pub rgba: Rgba,
    pub xrgb: Xrgba,
}

impl Color {
    pub fn new<T: ToString>(hex: T) -> Color {
        let hex = hex.to_string();
        assert!(hex.starts_with('#'), "the hex color must start with #");
        assert!(
            hex.len().eq(&7usize) || hex.len().eq(&9usize),
            "Use form: #000000 or #000000ff"
        );
        assert!(
            hex.chars()
                .all(|f: char| { "0123456789abcdef#".contains(f.to_ascii_lowercase()) }),
            "Invalid hex"
        );
        let alpha = match hex.len().eq(&9usize) {
            true => &hex[7..=8],
            false => "ff",
        };
        let hex = hex[0..=6].to_string();
        let hex8 = format!("{}{}", &hex, &alpha);
        Color {
            hex: hex.clone(),
            rgb: Rgb::from_hex(&hex),
            rgba: Rgba::from_hex(&hex8),
            xrgb: Xrgba::from_hex(&hex),
        }
    }
}
