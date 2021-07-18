use std::error::Error;

use once_cell::sync::Lazy;
use regex::Regex;

pub static HEXCOLOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"#[a-fA-F0-9]{8}|#[a-fA-F0-9]{6}|#[a-fA-F0-9]{4}|#[a-fA-F0-9]{3}"#).unwrap()
});

pub static XRGBA_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"[a-fA-F0-9]{2}/[a-fA-F0-9]{2}/[a-fA-F0-9]{2}/[a-fA-F0-9]{2}"#).unwrap()
});

pub static RGB_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"[rR][gG][bB]\(((?:\d{1,3}\.?)?\d{1,3}%?),\s*?((?:\d{1,3}\.?)?\d{1,3}%?),\s*?((?:\d{1,3}\.?)?\d{1,3}%?)\)"#).unwrap()
});

pub static RGBA_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"[rR][gG][bB][aA]\(((?:\d{1,3}\.?)?\d{1,3}%?),\s*?((?:\d{1,3}\.?)?\d{1,3}%?),\s*?((?:\d{1,3}\.?)?\d{1,3}%?),\s*?(\d{1}\.\d{1,})\)"#,
    )
    .unwrap()
});

/// maps a number to the type of color
/// 0 - hex
/// 1 - rgb
/// 2 - rgba
/// 3 - xrgba
/// 9 - unsupported
pub fn classify(s: &str) -> u8 {
    if HEXCOLOR_REGEX.is_match(s) {
        0u8
    } else if RGB_REGEX.is_match(s) {
        1u8
    } else if RGBA_REGEX.is_match(s) {
        2u8
    } else if XRGBA_REGEX.is_match(s) {
        3u8
    } else {
        9u8
    }
}

pub fn check_hex(hex: &str) -> bool {
    HEXCOLOR_REGEX.is_match(hex)
}

pub fn expand_hex(hex: &str) -> Result<String, Box<dyn Error>> {
    match HEXCOLOR_REGEX.is_match(hex) {
        true => {
            let result = match hex.len() {
                7 | 9 => hex.to_string(),
                4 => {
                    format!(
                        "#{r}{r}{g}{g}{b}{b}",
                        r = &hex[1..2],
                        g = &hex[2..3],
                        b = &hex[3..],
                    )
                }
                5 => {
                    format!(
                        "#{r}{r}{g}{g}{b}{b}{a}{a}",
                        r = &hex[1..2],
                        g = &hex[2..3],
                        b = &hex[3..4],
                        a = &hex[4..],
                    )
                }
                _ => unreachable!(),
            };
            Ok(result)
        }
        false => Err("Invalid hex".into()),
    }
}

pub fn check_xrgba(xrgba: &str) -> bool {
    XRGBA_REGEX.is_match(xrgba)
}

pub fn check_rgb(rgb: &str) -> bool {
    match RGB_REGEX.is_match(rgb) {
        true => {
            let capture = RGB_REGEX.captures(rgb).unwrap();
            let red = &capture[1];
            let green = &capture[2];
            let blue = &capture[3];

            if [red, green, blue].iter().any(|f| f.ends_with('%')) {
                let consistent = [red, green, blue].iter().all(|f| f.ends_with('%'));
                if !consistent {
                    return false;
                };
            };

            macro_rules! check_this {
                ($color:ident) => {
                    match $color.ends_with('%') {
                        true => {
                            let value = $color.strip_suffix('%').unwrap();
                            match value.parse::<f32>() {
                                Ok(val) => (0.0f32..=100.0f32).contains(&val),
                                Err(_) => return false,
                            }
                        }
                        false => match $color.parse::<u8>() {
                            Ok(_) => true,
                            Err(_) => return false,
                        },
                    }
                };
            }
            let red_ok = check_this!(red);
            let green_ok = check_this!(green);
            let blue_ok = check_this!(blue);
            red_ok & green_ok & blue_ok
        }
        false => false,
    }
}

pub fn check_rgba(rgba: &str) -> bool {
    match RGBA_REGEX.is_match(rgba) {
        true => {
            let capture = RGBA_REGEX.captures(rgba).unwrap();
            let red = &capture[1];
            let green = &capture[2];
            let blue = &capture[3];
            let alpha = &capture[4];

            if [red, green, blue].iter().any(|f| f.ends_with('%')) {
                let consistent = [red, green, blue].iter().all(|f| f.ends_with('%'));
                if !consistent {
                    return false;
                };
            };

            macro_rules! check_this {
                ($color:ident) => {
                    match $color.ends_with('%') {
                        true => {
                            let value = $color.strip_suffix('%').unwrap();
                            match value.parse::<f32>() {
                                Ok(val) => (0.0f32..=100.0f32).contains(&val),
                                Err(_) => return false,
                            }
                        }
                        false => match $color.parse::<u8>() {
                            Ok(_) => true,
                            Err(_) => return false,
                        },
                    }
                };
            }

            fn check_alpha(s: &str) -> bool {
                let value = s.parse::<f32>();
                match value {
                    Ok(val) => (0.0f32..=1.0f32).contains(&val),
                    Err(_) => false,
                }
            }

            let red_ok = check_this!(red);
            let green_ok = check_this!(green);
            let blue_ok = check_this!(blue);
            let alpha_ok = check_alpha(alpha);

            red_ok && blue_ok && green_ok && alpha_ok
        }
        false => false,
    }
}

/// up to three digit accuracy
pub fn u8_to_percentage(num: u8) -> f32 {
    let mut percentage = (num as f32 * 100f32) / 255.0f32;
    percentage = (percentage * 1000f32).round() / 1000.0f32;
    percentage.clamp(0.0f32, 100.0f32)
}

pub fn u8_to_percentage_rounded(num: u8) -> u8 {
    let mut percentage = (num as f32 * 100.0f32) / 255.0f32;
    percentage = percentage.round();
    let percentage = percentage as u8;
    percentage.clamp(0u8, 100u8)
}

/// up to three digit accuracy
pub fn u8_to_f32_clamped(num: u8) -> f32 {
    let mut number = num as f32 / 255.0f32;
    number = (number * 1000f32).round() / 1000.0f32;
    number.clamp(0.0f32, 1.0f32)
}

pub fn percentage_to_u8(percentage: f32) -> u8 {
    let percentage = percentage.clamp(0.0f32, 100.0f32);
    let mut num = (percentage * 255.0f32) / 100.0f32;
    num = (num * 1000.0f32).round() / 1000.0f32;
    num = num.clamp(0.0f32, 255.0f32);
    num as u8
}

pub fn clamped_f32_to_u8(num: f32) -> u8 {
    let mut num = num.clamp(0.0f32, 1.0f32);
    num *= 255.0f32;
    // num = (num * 1000.0f32).round() / 1000.0f32;
    num = num.clamp(0.0f32, 255.0f32);
    num as u8
}

pub fn hex_to_tuple(hex: &str) -> (u8, u8, u8) {
    assert!(check_hex(hex), "Invalid hex: {}", hex);
    let hex_new = expand_hex(hex).unwrap();
    let red = u8::from_str_radix(&hex_new[1..=2], 16).unwrap();
    let green = u8::from_str_radix(&hex_new[3..=4], 16).unwrap();
    let blue = u8::from_str_radix(&hex_new[5..=6], 16).unwrap();
    (red, green, blue)
}

pub fn hex_to_tuple_alpha(hex: &str) -> (u8, u8, u8, u8) {
    assert!(check_hex(hex), "Invalid hex: {}", hex);
    let hex_new = expand_hex(hex).unwrap();
    let red = u8::from_str_radix(&hex_new[1..=2], 16).unwrap();
    let green = u8::from_str_radix(&hex_new[3..=4], 16).unwrap();
    let blue = u8::from_str_radix(&hex_new[5..=6], 16).unwrap();
    let alpha = match hex_new.len() {
        7 => 255u8,
        9 => u8::from_str_radix(&hex_new[7..=8], 16).unwrap(),
        _ => unreachable!(),
    };
    (red, green, blue, alpha)
}

pub fn rgb_to_tuple(rgb: &str) -> (u8, u8, u8) {
    assert!(check_rgb(rgb), "Invalid rgb: {}", rgb);
    let capture = RGB_REGEX.captures(rgb).unwrap();
    let red = match &capture[1].ends_with('%') {
        true => {
            let percentage = (&capture)[1]
                .strip_suffix('%')
                .unwrap()
                .parse::<f32>()
                .unwrap();
            percentage_to_u8(percentage)
        }
        false => (&capture[1]).parse::<u8>().unwrap(),
    };
    let green = match &capture[2].ends_with('%') {
        true => {
            let percentage = (&capture)[2]
                .strip_suffix('%')
                .unwrap()
                .parse::<f32>()
                .unwrap();
            percentage_to_u8(percentage)
        }
        false => (&capture[2]).parse::<u8>().unwrap(),
    };
    let blue = match &capture[3].ends_with('%') {
        true => {
            let percentage = (&capture)[3]
                .strip_suffix('%')
                .unwrap()
                .parse::<f32>()
                .unwrap();
            percentage_to_u8(percentage)
        }
        false => (&capture[3]).parse::<u8>().unwrap(),
    };
    (red, green, blue)
}

pub fn rgba_to_tuple_alpha(rgba: &str) -> (u8, u8, u8, u8) {
    assert!(check_rgba(rgba), "Invalid rgba: {}", rgba);
    let capture = RGBA_REGEX.captures(rgba).unwrap();
    let red = match &capture[1].ends_with('%') {
        true => {
            let percentage = (&capture)[1]
                .strip_suffix('%')
                .unwrap()
                .parse::<f32>()
                .unwrap();
            percentage_to_u8(percentage)
        }
        false => (&capture[1]).parse::<u8>().unwrap(),
    };
    let green = match &capture[2].ends_with('%') {
        true => {
            let percentage = (&capture)[2]
                .strip_suffix('%')
                .unwrap()
                .parse::<f32>()
                .unwrap();
            percentage_to_u8(percentage)
        }
        false => (&capture[2]).parse::<u8>().unwrap(),
    };
    let blue = match &capture[3].ends_with('%') {
        true => {
            let percentage = (&capture)[3]
                .strip_suffix('%')
                .unwrap()
                .parse::<f32>()
                .unwrap();
            percentage_to_u8(percentage)
        }
        false => (&capture[3]).parse::<u8>().unwrap(),
    };
    let alpha = (&capture)[4].parse::<f32>().unwrap();
    let alpha = clamped_f32_to_u8(alpha);
    (red, green, blue, alpha)
}

pub fn xrgba_to_tuple(xrgba: &str) -> (u8, u8, u8) {
    assert!(check_xrgba(xrgba), "Invalid xrgba: {}", xrgba);
    let mut chunks = xrgba.split('/');
    let red = u8::from_str_radix(&chunks.next().unwrap(), 16).unwrap();
    let green = u8::from_str_radix(&chunks.next().unwrap(), 16).unwrap();
    let blue = u8::from_str_radix(&chunks.next().unwrap(), 16).unwrap();
    (red, green, blue)
}

pub fn xrgba_to_tuple_alpha(xrgba: &str) -> (u8, u8, u8, u8) {
    assert!(check_xrgba(xrgba), "Invalid xrgba: {}", xrgba);
    let mut chunks = xrgba.split('/');
    let red = u8::from_str_radix(&chunks.next().unwrap(), 16).unwrap();
    let green = u8::from_str_radix(&chunks.next().unwrap(), 16).unwrap();
    let blue = u8::from_str_radix(&chunks.next().unwrap(), 16).unwrap();
    let alpha = u8::from_str_radix(&chunks.next().unwrap(), 16).unwrap();
    (red, green, blue, alpha)
}
