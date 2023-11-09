use core::fmt;
use std::{collections::HashMap, error::Error, fs, ops::Deref};

use itertools::Itertools;
use regex::Regex;

#[allow(dead_code)]
pub struct NamedColor {
    name: String,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug)]
pub struct ColorError {
    details: String,
}

impl ColorError {
    fn new(msg: &str) -> ColorError {
        ColorError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ColorError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(colors: (u8, u8, u8, u8)) -> Color {
        Color {
            r: colors.0,
            g: colors.1,
            b: colors.2,
            a: colors.3,
        }
    }

    pub fn from_hex(hex_code: &str) -> Result<Color, ColorError> {
        if hex_code.len() < 2 {
            return Err(ColorError::new("Too few chars"));
        }
        let mut offset: u8 = 1;
        if hex_code.len() == 7 {
            //#aabbcc format
            offset = 1;
        } else if hex_code.len() == 8 {
            //0xaabbcc format
            offset = 2;
        } else {
            return Err(ColorError::new("Incorrect hex format"));
        }
        let (r, g, b) = hex_code
            .chars()
            .skip(offset as usize)
            .collect_vec()
            .chunks(2)
            .map(|x| x.iter().collect::<String>())
            .map(|x| u8::from_str_radix(&x, 16).unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Color { r, g, b, a: 255 })
    }
}

#[derive(Default)]
pub struct ColorMap {
    colors: HashMap<String, Color>,
}

impl ColorMap {
    pub fn new() -> ColorMap {
        ColorMap {
            colors: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, c: Color) {
        self.colors.insert(name.to_owned(), c);
    }

    pub fn get(&self, name: &str) -> Option<Color> {
        let color = self.colors.get(name);
        color.copied()
    }

    pub fn from_conf(filename: &str) -> Result<ColorMap, ColorError> {
        let read_result = fs::read_to_string(filename);
        match read_result {
            Ok(read_result) => {
                let mut color_map = ColorMap::new();
                let separator = Regex::new(r"([\s\t]+)").expect("Invalid regex");
                for l in read_result.lines() {
                    let (name, number) = separator.split(l).collect_tuple().unwrap();
                    let hex = Color::from_hex(number);
                    match hex {
                        Ok(hex) => color_map.insert(name, hex),
                        Err(_) => continue,
                    }
                }
                Ok(color_map)
            }
            Err(_) => Err(ColorError::new("String not found")),
        }
    }
}

impl NamedColor {
    pub fn new(name: &str, colors: (u8, u8, u8, u8)) -> NamedColor {
        NamedColor {
            name: name.to_owned(),
            r: colors.0,
            g: colors.1,
            b: colors.2,
            a: colors.3,
        }
    }
}

impl From<NamedColor> for Color {
    fn from(value: NamedColor) -> Self {
        Color {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}
