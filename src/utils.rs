pub fn check_hex(s: &str) -> usize {
    match s.len() {
        7 => {
            if s.starts_with('#')
                && s.chars()
                    .all(|f| "#0123456789abcdef".contains(f.to_ascii_lowercase()))
            {
                7
            } else {
                0
            }
        }
        9 => {
            if s.starts_with('#')
                && s.chars()
                    .all(|f| "#0123456789abcdef".contains(f.to_ascii_lowercase()))
            {
                9
            } else {
                0
            }
        }
        2 => {
            if s.chars()
                .all(|f| "0123456789abcdef".contains(f.to_ascii_lowercase()))
            {
                2
            } else {
                0
            }
        }
        4 => {
            if s.starts_with('#')
                && s.chars()
                    .all(|f| "#0123456789abcdef".contains(f.to_ascii_lowercase()))
            {
                4
            } else {
                0
            }
        }
        5 => {
            if s.starts_with('#')
                && s.chars()
                    .all(|f| "#0123456789abcdef".contains(f.to_ascii_lowercase()))
            {
                5
            } else {
                0
            }
        }
        _ => 0,
    }
}

pub fn expand_color(s: &str) -> String {
    let s_len = check_hex(s);
    assert!(!s_len.eq(&2usize), "Hex: {} cannot be expanded", s);
    match s_len {
        4 => {
            assert!(s.starts_with('#'), "Invalid hex: {}", s);
            let mut s = s.strip_prefix('#').unwrap().chars();
            format!(
                "#{red}{red}{green}{green}{blue}{blue}",
                red = s.next().unwrap(),
                green = s.next().unwrap(),
                blue = s.next().unwrap(),
            )
        }
        5 => {
            assert!(s.starts_with('#'), "Invalid hex: {}", s);
            let mut s = s.strip_prefix('#').unwrap().chars();
            format!(
                "#{red}{red}{green}{green}{blue}{blue}{alpha}{alpha}",
                red = s.next().unwrap(),
                green = s.next().unwrap(),
                blue = s.next().unwrap(),
                alpha = s.next().unwrap(),
            )
        }
        _ => s.to_string(),
    }
}

pub fn check_xrgba(s: &str) -> bool {
    if !s.matches('/').count().eq(&3usize) {
        return false;
    };
    let mut new_s = s.split('/');
    if !&new_s.clone().count().eq(&4usize) {
        return false;
    };
    new_s.all(|f: &str| {
        f.len().eq(&2usize)
            && f.chars()
                .all(|x| "0123456789abcdef".contains(x.to_ascii_lowercase()))
    })
}

/// up to three digit accuracy
pub fn u8_to_percentage(num: u8) -> f32 {
    let mut percentage = (num as f32 * 100f32) / 255.0f32;
    percentage = (percentage * 1000f32).round() / 1000.0f32;
    percentage.clamp(0.0f32, 100.0f32)
}

/// up to three digit accuracy
pub fn u8_to_f32_clamped(num: u8) -> f32 {
    let mut number = num as f32 / 255.0f32;
    number = (number * 1000f32).round() / 1000.0f32;
    number.clamp(0.0f32, 1.0f32)
}

pub fn hex_to_tuple(hex: &str) -> (u8, u8, u8) {
    let hex_len = check_hex(hex);
    assert!(matches!(hex_len, 7 | 9 | 4 | 5), "Invalid Hex: {}", hex);
    let hex = expand_color(hex);
    let red = u8::from_str_radix(&hex[1..=2], 16).unwrap();
    let green = u8::from_str_radix(&hex[3..=4], 16).unwrap();
    let blue = u8::from_str_radix(&hex[5..=6], 16).unwrap();
    (red, green, blue)
}

pub fn hex_to_tuple_alpha(hex: &str) -> (u8, u8, u8, u8) {
    let hex_len = check_hex(hex);
    assert!(matches!(hex_len, 7 | 9 | 4 | 5), "Invalid Hex: {}", hex);
    let hex = expand_color(hex);
    let red = u8::from_str_radix(&hex[1..=2], 16).unwrap();
    let green = u8::from_str_radix(&hex[3..=4], 16).unwrap();
    let blue = u8::from_str_radix(&hex[5..=6], 16).unwrap();
    let alpha = match hex_len {
        7 => 255,
        9 => u8::from_str_radix(&hex[7..=8], 16).unwrap(),
        _ => unreachable!(),
    };
    (red, green, blue, alpha)
}

pub fn xrgba_to_tuple(xrgba: &str) -> (u8, u8, u8) {
    assert!(check_xrgba(xrgba), "Invalid xrgba string: {}", xrgba);
    let mut chunks = xrgba.split('/');
    let red = u8::from_str_radix(chunks.next().unwrap(), 16).unwrap();
    let green = u8::from_str_radix(chunks.next().unwrap(), 16).unwrap();
    let blue = u8::from_str_radix(chunks.next().unwrap(), 16).unwrap();
    (red, green, blue)
}

pub fn xrgba_to_tuple_alpha(xrgba: &str) -> (u8, u8, u8, u8) {
    assert!(check_xrgba(xrgba), "Invalid xrgba string: {}", xrgba);
    let mut chunks = xrgba.split('/');
    let red = u8::from_str_radix(chunks.next().unwrap(), 16).unwrap();
    let green = u8::from_str_radix(chunks.next().unwrap(), 16).unwrap();
    let blue = u8::from_str_radix(chunks.next().unwrap(), 16).unwrap();
    let alpha = u8::from_str_radix(chunks.next().unwrap(), 16).unwrap();
    (red, green, blue, alpha)
}

pub fn check_rgb(s: &str) -> bool {
    let s_lower = s.to_ascii_lowercase();
    let edges = s_lower.starts_with(r#"rgb("#) && s_lower.ends_with(')');
    if !edges {
        return edges;
    };
    let core = s_lower
        .strip_prefix(r#"rgb("#)
        .unwrap()
        .strip_suffix(')')
        .unwrap();
    let core = core.replace(" ", "");
    let mut core = core.split(',');
    if !core.clone().count().eq(&3usize) {
        return false;
    };
    core.all(|f| f.parse::<u8>().is_ok())
}

pub fn rgb_to_tuple(s: &str) -> (u8, u8, u8) {
    assert!(check_rgb(s), "Invalid rgb: {}", s);
    let s = s
        .to_ascii_lowercase()
        .strip_prefix(r#"rgb("#)
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .replace(" ", "");
    let mut s = s.split(',');
    let red = s.next().unwrap().parse::<u8>().unwrap();
    let green = s.next().unwrap().parse::<u8>().unwrap();
    let blue = s.next().unwrap().parse::<u8>().unwrap();
    (red, green, blue)
}

pub fn rgb_to_tuple_alpha(s: &str) -> (u8, u8, u8, u8) {
    assert!(check_rgb(s), "Invalid rgb: {}", s);
    let s = s
        .to_ascii_lowercase()
        .strip_prefix(r#"rgb("#)
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .replace(" ", "");
    let mut s = s.split(',');
    let red = s.next().unwrap().parse::<u8>().unwrap();
    let green = s.next().unwrap().parse::<u8>().unwrap();
    let blue = s.next().unwrap().parse::<u8>().unwrap();
    (red, green, blue, 255)
}

pub fn check_rgba(s: &str) -> bool {
    let s_lower = s.to_ascii_lowercase();
    let edges = s_lower.starts_with(r#"rgba("#) && s_lower.ends_with(')');
    if !edges {
        return false;
    };
    let core = s_lower
        .strip_prefix(r#"rgba("#)
        .unwrap()
        .strip_suffix(')')
        .unwrap();
    let core = core.replace(" ", "");
    let mut center = core.split(',').take(3);
    let center_is_ok = center.all(|f| f.parse::<u8>().is_ok());
    if !center_is_ok {
        return false;
    };
    let alpha = core.split(',').nth(3).unwrap().parse::<f32>();
    if alpha.is_ok() {
        let unw = match alpha {
            Ok(it) => it,
            _ => unreachable!(),
        };
        (0.0f32..=1.0f32).contains(&unw)
    } else {
        false
    }
}

pub fn rgba_to_tuple(s: &str) -> (u8, u8, u8) {
    assert!(check_rgba(s), "Invalid rgba: {}", s);
    let s = s
        .to_ascii_lowercase()
        .strip_prefix(r#"rgba("#)
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .replace(" ", "");
    let mut s = s.split(',');
    let red = s.next().unwrap().parse::<u8>().unwrap();
    let green = s.next().unwrap().parse::<u8>().unwrap();
    let blue = s.next().unwrap().parse::<u8>().unwrap();
    (red, green, blue)
}

pub fn rgba_to_tuple_alpha(s: &str) -> (u8, u8, u8, u8) {
    assert!(check_rgba(s), "Invalid rgba: {}", s);
    let s = s
        .to_ascii_lowercase()
        .strip_prefix(r#"rgba("#)
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .replace(" ", "");
    let mut s = s.split(',');
    let red = s.next().unwrap().parse::<u8>().unwrap();
    let green = s.next().unwrap().parse::<u8>().unwrap();
    let blue = s.next().unwrap().parse::<u8>().unwrap();
    let mut alpha = s.next().unwrap().parse::<f32>().unwrap();
    alpha *= 255.0f32;
    alpha = alpha.round();
    alpha = alpha.clamp(0.0f32, 255.0f32);
    (red, green, blue, alpha as u8)
}
