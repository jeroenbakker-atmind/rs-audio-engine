use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Component {
    pub exponent: i32,
    pub factor: f64,
}

impl Component {
    pub fn new(exponent: i32, factor: f64) -> Component {
        Component { exponent, factor }
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.factor == 0.0 {
            f.write_str("")
        } else if self.exponent == 0 {
            f.write_fmt(format_args!("{}", self.factor))
        } else if self.factor == 1.0 {
            if self.exponent == 1 {
                f.write_str("z")
            } else if self.exponent < 10 {
                f.write_fmt(format_args!("z^{}", self.exponent))
            } else {
                f.write_fmt(format_args!("z^{{{}}}", self.exponent))
            }
        } else {
            if self.exponent == 0 {
                f.write_fmt(format_args!("{}", self.factor))
            } else if self.exponent == 1 {
                f.write_fmt(format_args!("{}*z", self.factor))
            } else if self.exponent < 10 {
                f.write_fmt(format_args!("{}*z^{}", self.factor, self.exponent))
            } else {
                f.write_fmt(format_args!("{}*z^{{{}}}", self.factor, self.exponent))
            }
        }
    }
}
