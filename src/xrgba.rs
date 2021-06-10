#[derive(Debug, Clone)]
pub struct Xrgba(String);

impl Default for Xrgba {
    fn default() -> Xrgba {
        Xrgba::from_hex("#000000")
    }
}

impl std::fmt::Display for Xrgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Xrgba {
    pub fn from_hex<T: ToString>(color_hex: T) -> Xrgba {
        let color_hex = color_hex.to_string();
        assert!(color_hex.starts_with('#'), "Hex color starts with a #");
        assert!(color_hex.len().eq(&7usize), "Use form: #000000");
        assert!(
            color_hex
                .chars()
                .all(|f: char| { "0123456789abcdef#".contains(f.to_ascii_lowercase()) }),
            "Invalid hex"
        );
        let red = &color_hex[1..=2];
        let green = &color_hex[3..=4];
        let blue = &color_hex[5..=6];
        Xrgba(format!(
            "{}/{}/{}/ff",
            red.to_ascii_lowercase(),
            green.to_ascii_lowercase(),
            blue.to_ascii_lowercase(),
        ))
    }
    pub fn to_hex(&self) -> String {
        let mut splits = self.0.split('/');
        format!(
            "#{}{}{}",
            splits.next().unwrap(),
            splits.next().unwrap(),
            splits.next().unwrap()
        )
    }
}

impl From<&Xrgba> for String {
    fn from(color: &Xrgba) -> String {
        color.0.clone()
    }
}

mod xrgba_tests {
    #[test]
    fn from_hex() {
        let blue = super::Xrgba::from_hex("#0000ff");
        assert!(blue.0.eq("00/00/ff/ff"));
    }
    #[test]
    fn to_hex() {
        let blue = super::Xrgba(String::from("00/00/ff/ff"));
        assert!(blue.to_hex().eq("#0000ff"));
    }
    #[test]
    fn to_string() {
        let blue = super::Xrgba(String::from("00/00/ff/ff"));
        assert!(blue.to_string().eq("00/00/ff/ff"));
    }
}
