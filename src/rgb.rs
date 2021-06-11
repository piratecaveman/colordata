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
    pub fn red(&self) -> u8 {
        self.0
    }
    pub fn green(&self) -> u8 {
        self.1
    }
    pub fn blue(&self) -> u8 {
        self.2
    }
    pub fn set_red(&mut self, red: u8) {
        self.0 = red;
    }
    pub fn set_green(&mut self, green: u8) {
        self.1 = green;
    }
    pub fn set_blue(&mut self, blue: u8) {
        self.2 = blue;
    }
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red(), self.green(), self.blue())
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
        format!("{},{},{}", color.red(), color.green(), color.blue())
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
        assert_eq!(red.red(), 255);
        assert_eq!(red.green(), 0);
        assert_eq!(red.blue(), 0);
    }
    #[test]
    fn to_string() {
        let red = super::Rgb(255, 0, 0);
        assert!(red.to_string().eq("255,0,0"))
    }
}
