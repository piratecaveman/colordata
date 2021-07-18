use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

use crate::traits::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl ToTuple for Color {
    fn to_tuple(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }

    fn to_tuple_alpha(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, self.alpha)
    }
}

impl FromTuple for Color {
    fn from_tuple(tuple: (u8, u8, u8)) -> Self {
        Color {
            red: tuple.0,
            green: tuple.1,
            blue: tuple.2,
            alpha: 255,
        }
    }

    fn from_tuple_alpha(tuple: (u8, u8, u8, u8)) -> Self {
        Color {
            red: tuple.0,
            green: tuple.1,
            blue: tuple.2,
            alpha: tuple.3,
        }
    }
}

impl ComponentAsu8 for Color {}
impl ComponentAsHexString for Color {}
impl ComponentAsPercentage for Color {}
impl ComponentAsf32 for Color {}
impl MakeString for Color {}
impl FromString for Color {}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hex())
    }
}

impl Color {
    pub fn new() -> Self {
        Self::default()
    }
}

mod tests {
    #[test]
    fn from_tests() {
        use super::*;
        let pink_hex = Color::from_hex("#ffbcca");
        let pink_rgb = Color::from_rgb("rgb(255, 188, 202)");
        let pink_rgba = Color::from_rgba("rgba(255, 188, 202, 1.0)");
        let pink_xrgba = Color::from_xrgba("ff/bc/ca/ff");
        assert_eq!(pink_hex, pink_rgb);
        assert_eq!(pink_hex, pink_rgba);
        assert_eq!(pink_hex, pink_xrgba);

        let aqua_hex = Color::from_hex8("#0fffff55");
        let aqua_rgb = Color::from_rgb("rgb(5.883%,100%,100%)");
        let aqua_rgba = Color::from_rgba("rgba(15,255,255,0.33334)");
        let aqua_xrgba = Color::from_xrgba("0f/ff/ff/55");
        assert_eq!(aqua_hex.red, aqua_rgb.red);
        assert_eq!(aqua_hex.green, aqua_rgb.green);
        assert_eq!(aqua_hex.blue, aqua_rgb.blue);
        assert_eq!(aqua_hex, aqua_rgba);
        assert_eq!(aqua_hex, aqua_xrgba);
    }
    #[test]
    fn to_tests() {
        use super::*;
        let green_blue = Color {
            red: 188,
            green: 255,
            blue: 245,
            alpha: 255,
        };
        assert_eq!(green_blue.hex(), "#bcfff5");
        assert_eq!(green_blue.hex8(), "#bcfff5ff");
        assert_eq!(green_blue.rgb(), "rgb(188,255,245)");
        assert_eq!(green_blue.rgb_percentage(), "rgb(73.725%,100%,96.078%)");
        assert_eq!(green_blue.rgb_percentage_rounded(), "rgb(74%,100%,96%)");
        assert_eq!(green_blue.rgba(), "rgba(188,255,245,1)");
        assert_eq!(green_blue.rgba_percentage(), "rgba(73.725%,100%,96.078%,1)");
        assert_eq!(green_blue.rgba_percentage_rounded(), "rgba(74%,100%,96%,1)");
        assert_eq!(green_blue.xrgba(), "bc/ff/f5/ff");

        let mint = Color {
            red: 171,
            green: 247,
            blue: 136,
            alpha: 128,
        };
        assert_eq!(mint.hex(), "#abf788");
        assert_eq!(mint.hex8(), "#abf78880");
        assert_eq!(mint.rgb(), "rgb(171,247,136)");
        assert_eq!(mint.rgb_percentage(), "rgb(67.059%,96.863%,53.333%)");
        assert_eq!(mint.rgb_percentage_rounded(), "rgb(67%,97%,53%)");
        assert_eq!(mint.rgb_stripped(), "171,247,136");
        assert_eq!(mint.rgba(), "rgba(171,247,136,0.502)");
        assert_eq!(mint.rgba_stripped(), "171,247,136,0.502");
        assert_eq!(
            mint.rgba_percentage(),
            "rgba(67.059%,96.863%,53.333%,0.502)"
        );
        assert_eq!(mint.rgba_percentage_rounded(), "rgba(67%,97%,53%,0.502)");
        assert_eq!(mint.xrgba(), "ab/f7/88/80");
    }
    #[test]
    fn component_tests() {
        use super::*;
        let orange = Color::from_hex8("#ff5500af");
        assert_eq!(orange.red_u8(), 255);
        assert!(orange.red_hex().eq("ff"));
        assert!((1.0 - orange.red_f32()).abs() < f32::EPSILON);
        assert!((0.686 - orange.alpha_f32()).abs() < f32::EPSILON);
    }
}
