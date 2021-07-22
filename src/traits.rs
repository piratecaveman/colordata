use crate::utils;

pub trait ToTuple {
    fn to_tuple(&self) -> (u8, u8, u8);
    fn to_tuple_alpha(&self) -> (u8, u8, u8, u8);
}

pub trait FromTuple {
    fn from_tuple(tuple: (u8, u8, u8)) -> Self;
    fn from_tuple_alpha(tuple: (u8, u8, u8, u8)) -> Self;
}

pub trait ComponentAsu8
where
    Self: ToTuple,
{
    fn red_u8(&self) -> u8 {
        self.to_tuple().0
    }
    fn green_u8(&self) -> u8 {
        self.to_tuple().1
    }
    fn blue_u8(&self) -> u8 {
        self.to_tuple().2
    }
    fn alpha_u8(&self) -> u8 {
        self.to_tuple_alpha().3
    }
}

pub trait ComponentAsHexString
where
    Self: ToTuple,
{
    fn red_hex(&self) -> String {
        format!("{:02x}", self.to_tuple().0)
    }
    fn green_hex(&self) -> String {
        format!("{:02x}", self.to_tuple().1)
    }
    fn blue_hex(&self) -> String {
        format!("{:02x}", self.to_tuple().2)
    }
    fn alpha_hex(&self) -> String {
        format!("{:02x}", self.to_tuple_alpha().3)
    }
}

pub trait ComponentAsPercentage
where
    Self: ToTuple,
{
    fn red_percentage(&self) -> f32 {
        utils::u8_to_percentage(self.to_tuple().0)
    }
    fn red_percentage_rounded(&self) -> u8 {
        utils::u8_to_percentage_rounded(self.to_tuple().0)
    }

    fn green_percentage(&self) -> f32 {
        utils::u8_to_percentage(self.to_tuple().1)
    }
    fn green_percentage_rounded(&self) -> u8 {
        utils::u8_to_percentage_rounded(self.to_tuple().1)
    }

    fn blue_percentage(&self) -> f32 {
        utils::u8_to_percentage(self.to_tuple().2)
    }
    fn blue_percentage_rounded(&self) -> u8 {
        utils::u8_to_percentage_rounded(self.to_tuple().2)
    }

    fn alpha_percentage(&self) -> f32 {
        utils::u8_to_percentage(self.to_tuple_alpha().3)
    }
    fn alpha_percentage_rounded(&self) -> u8 {
        utils::u8_to_percentage_rounded(self.to_tuple_alpha().3)
    }
}

pub trait ComponentAsf32
where
    Self: ToTuple,
{
    fn red_f32(&self) -> f32 {
        utils::u8_to_f32_clamped(self.to_tuple().0)
    }
    fn green_f32(&self) -> f32 {
        utils::u8_to_f32_clamped(self.to_tuple().1)
    }
    fn blue_f32(&self) -> f32 {
        utils::u8_to_f32_clamped(self.to_tuple().2)
    }
    fn alpha_f32(&self) -> f32 {
        utils::u8_to_f32_clamped(self.to_tuple_alpha().3)
    }
}

pub trait MakeString
where
    Self: ToTuple,
{
    fn hex(&self) -> String {
        let tuple = self.to_tuple();
        format!("#{:02x}{:02x}{:02x}", tuple.0, tuple.1, tuple.2)
    }
    fn hex_stripped(&self) -> String {
        let tuple = self.to_tuple();
        format!("{:02x}{:02x}{:02x}", tuple.0, tuple.1, tuple.2)
    }
    fn hex8(&self) -> String {
        let tuple = self.to_tuple_alpha();
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            tuple.0, tuple.1, tuple.2, tuple.3
        )
    }
    fn hex8_stripped(&self) -> String {
        let tuple = self.to_tuple_alpha();
        format!(
            "{:02x}{:02x}{:02x}{:02x}",
            tuple.0, tuple.1, tuple.2, tuple.3
        )
    }
    fn rgb(&self) -> String {
        let tuple = self.to_tuple();
        format!("rgb({},{},{})", tuple.0, tuple.1, tuple.2)
    }
    fn rgb_stripped(&self) -> String {
        let tuple = self.to_tuple();
        format!("{},{},{}", tuple.0, tuple.1, tuple.2)
    }
    fn rgb_percentage(&self) -> String {
        let tuple = self.to_tuple();
        format!(
            "rgb({}%,{}%,{}%)",
            utils::u8_to_percentage(tuple.0),
            utils::u8_to_percentage(tuple.1),
            utils::u8_to_percentage(tuple.2),
        )
    }
    fn rgb_percentage_rounded(&self) -> String {
        let tuple = self.to_tuple();
        format!(
            "rgb({}%,{}%,{}%)",
            utils::u8_to_percentage_rounded(tuple.0),
            utils::u8_to_percentage_rounded(tuple.1),
            utils::u8_to_percentage_rounded(tuple.2),
        )
    }
    fn rgba(&self) -> String {
        let tuple = self.to_tuple_alpha();
        format!(
            "rgba({},{},{},{})",
            tuple.0,
            tuple.1,
            tuple.2,
            utils::u8_to_f32_clamped(tuple.3)
        )
    }
    fn rgba_stripped(&self) -> String {
        let tuple = self.to_tuple_alpha();
        format!(
            "{},{},{},{}",
            tuple.0,
            tuple.1,
            tuple.2,
            utils::u8_to_f32_clamped(tuple.3)
        )
    }
    fn rgba_percentage(&self) -> String {
        let tuple = self.to_tuple_alpha();
        format!(
            "rgba({}%,{}%,{}%,{})",
            utils::u8_to_percentage(tuple.0),
            utils::u8_to_percentage(tuple.1),
            utils::u8_to_percentage(tuple.2),
            utils::u8_to_f32_clamped(tuple.3),
        )
    }
    fn rgba_percentage_rounded(&self) -> String {
        let tuple = self.to_tuple_alpha();
        format!(
            "rgba({}%,{}%,{}%,{})",
            utils::u8_to_percentage_rounded(tuple.0),
            utils::u8_to_percentage_rounded(tuple.1),
            utils::u8_to_percentage_rounded(tuple.2),
            utils::u8_to_f32_clamped(tuple.3),
        )
    }
    fn xrgba(&self) -> String {
        let tuple = self.to_tuple_alpha();
        format!(
            "{:02x}/{:02x}/{:02x}/{:02x}",
            tuple.0, tuple.1, tuple.2, tuple.3
        )
    }
}

pub trait FromString
where
    Self: FromTuple + Sized,
{
    fn from_hex(s: &str) -> Self {
        let tuple = utils::hex_to_tuple(s);
        Self::from_tuple(tuple)
    }
    fn from_hex8(s: &str) -> Self {
        let tuple = utils::hex_to_tuple_alpha(s);
        Self::from_tuple_alpha(tuple)
    }
    fn from_rgb(s: &str) -> Self {
        let tuple = utils::rgb_to_tuple(s);
        Self::from_tuple(tuple)
    }
    fn from_rgba(s: &str) -> Self {
        let tuple = utils::rgba_to_tuple_alpha(s);
        Self::from_tuple_alpha(tuple)
    }
    fn from_xrgba(s: &str) -> Self {
        let tuple = utils::xrgba_to_tuple_alpha(s);
        Self::from_tuple_alpha(tuple)
    }
}
