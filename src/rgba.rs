#[derive(Debug, Clone, Copy)]
pub struct Rgba(u8, u8, u8, f32);

impl Default for Rgba {
    fn default() -> Rgba {
        Rgba(0, 0, 0, 1.0)
    }
}

impl std::fmt::Display for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({},{},{},{:.2})", self.0, self.1, self.2, self.3)
    }
}

impl Rgba {
    pub fn new() -> Rgba {
        Rgba::default()
    }
    pub fn to_hex(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            self.0,
            self.1,
            self.2,
            (self.3 * 255.0) as u8
        )
    }
    pub fn from_hex<T: ToString>(color_hex: T) -> Rgba {
        let color_hex = color_hex.to_string();
        assert!(color_hex.starts_with('#'), "Hex color starts with a #");
        assert!(
            color_hex.len().eq(&9usize),
            "Use form: #000000ff with alpha value at the end"
        );
        assert!(
            color_hex
                .chars()
                .all(|f: char| { "0123456789abcdef#".contains(f.to_ascii_lowercase()) }),
            "Invalid hex"
        );
        let red = u8::from_str_radix(&color_hex[1..=2], 16).expect("Invalid hex");
        let green = u8::from_str_radix(&color_hex[3..=4], 16).expect("Invalid hex");
        let blue = u8::from_str_radix(&color_hex[5..=6], 16).expect("Invalid hex");
        let alpha = (u8::from_str_radix(&color_hex[7..=8], 16).expect("Invalid hex") as f32
            / 255.0)
            .clamp(0.0, 1.0);
        Rgba(red, green, blue, alpha)
    }
}

impl From<&Rgba> for String {
    fn from(color: &Rgba) -> String {
        format!("rgba({},{},{},{})", color.0, color.1, color.2, color.3)
    }
}

mod rgba_tests {
    #[test]
    fn to_hex() {
        let green = super::Rgba(0, 255, 0, 1f32);
        assert_eq!(green.to_hex().as_str(), "#00ff00ff");
    }
    #[test]
    fn from_hex() {
        let green = super::Rgba::from_hex("#00ff00ff");
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
        assert!((green.3 - 1f32).abs() < f32::EPSILON);
    }
    #[test]
    fn to_string() {
        let green = super::Rgba::from_hex("#00ff007f");
        dbg!(green.to_string());
        assert!(green.to_string().eq("rgba(0,255,0,0.50)"));
    }
}
