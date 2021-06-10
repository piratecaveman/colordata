#[derive(Debug, Clone, Copy)]
pub struct Rgb(u8, u8, u8);

impl Default for Rgb {
    fn default() -> Rgb {
        Rgb(0, 0, 0)
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}

impl Rgb {
    pub fn new() -> Rgb {
        Rgb::default()
    }
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
    pub fn from_hex<T: ToString>(color_hex: T) -> Rgb {
        let color_hex = color_hex.to_string();
        assert!(color_hex.starts_with('#'), "Hex colour must start with #");
        assert!(color_hex.len().eq(&7usize), "Use form: #ffffff");
        assert!(
            color_hex
                .chars()
                .all(|f: char| { "0123456789abcdef#".contains(f.to_ascii_lowercase()) }),
            "Invalid hex"
        );
        let red = u8::from_str_radix(&color_hex[1..=2], 16).expect("Invalid hex");
        let green = u8::from_str_radix(&color_hex[3..=4], 16).expect("Invalid hex");
        let blue = u8::from_str_radix(&color_hex[5..=6], 16).expect("Invalid hex");
        Rgb(red, green, blue)
    }
}

impl From<&Rgb> for String {
    fn from(color: &Rgb) -> String {
        format!("{},{},{}", color.0, color.1, color.2)
    }
}

mod rgb_tests {
    #[test]
    fn to_hex() {
        let red = super::Rgb(255, 0, 0);
        assert_eq!(red.to_hex().as_str(), "#ff0000");
    }
    #[test]
    fn from_hex() {
        let red = super::Rgb::from_hex("#ff0000");
        assert_eq!(red.0, 255);
        assert_eq!(red.1, 0);
        assert_eq!(red.2, 0);
    }
    #[test]
    fn to_string() {
        let red = super::Rgb(255, 0, 0);
        assert!(red.to_string().eq("255,0,0"))
    }
}
