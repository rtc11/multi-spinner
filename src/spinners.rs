#![allow(dead_code)]

#[derive(Clone)]
pub enum Animation {
    Dots2(usize),
    Dots3(usize),
    Bars10(usize),
}

impl Animation {
    pub fn next_frame(&mut self) -> &str {
        match self {
            Self::Dots2(idx) => *idx = (*idx + 1) % DOTS_2.len(),
            Self::Dots3(idx) => *idx = (*idx + 1) % DOTS_3.len(),
            Self::Bars10(idx) => *idx = (*idx + 1) % BARS_10.len(),
        }
        match self {
            Self::Dots2(idx) => DOTS_2[*idx],
            Self::Dots3(idx) => DOTS_3[*idx],
            Self::Bars10(idx) => BARS_10[*idx],
        } 
    } 
}

static DOTS_2: &[&str] = &[
    "⠈", "⠉", "⠋", 
    "⠓", "⠒", "⠐", 
    "⠐", "⠒", "⠖", 
    "⠦", "⠤", "⠠",
    "⠠", "⠤", "⠦", 
    "⠖", "⠒", "⠐",
    "⠐", "⠒", "⠓", 
    "⠋", "⠉", "⠈"
];

static DOTS_3: &[&str] = &[
    " ⠈", " ⠉", "⠈⠉", "⠋ ", 
    "⠓ ", "⠐⠒", " ⠒", " ⠐", 
    " ⠐", " ⠒", "⠐⠒", "⠖ ", 
    "⠦ ", "⠠⠤", " ⠤", " ⠠",
    " ⠠", " ⠤", "⠠⠤", "⠦ ", 
    "⠖ ", "⠐⠒", " ⠒", " ⠐",
    " ⠐", " ⠒", "⠐⠒", "⠓ ", 
    "⠋ ", "⠈⠉", " ⠉", " ⠈"
];

static BARS_10: &[&str] = &[
    "▰▱▱▱▱▱▱▱▱▱",
    "▰▰▱▱▱▱▱▱▱▱",
    "▰▰▰▱▱▱▱▱▱▱",
    "▰▰▰▰▱▱▱▱▱▱",
    "▰▰▰▰▰▱▱▱▱▱",
    "▰▰▰▰▰▰▱▱▱▱",
    "▰▰▰▰▰▰▰▱▱▱",
    "▰▰▰▰▰▰▰▰▱▱",
    "▰▰▰▰▰▰▰▰▰▱",
    "▰▰▰▰▰▰▰▰▰▰",
];

