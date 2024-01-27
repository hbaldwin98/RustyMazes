use std::fs;

use crate::prelude::*;

pub struct Mask {
    pub mask: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

pub trait Maskable {
    fn mask(&mut self, mask: &Mask);
    fn from_mask(mask: &Mask) -> Self;
}

impl Mask {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            mask: vec![true; width * height],
            width,
            height,
        }
    }

    pub fn set(&mut self, point: Point, value: bool) {
        self.mask[point.x as usize + point.y as usize * self.width] = value;
    }

    pub fn from_txt(file_path: &str) -> Result<Mask, std::io::Error> {
        let data = fs::read_to_string(file_path)?;
        let mut lines = data.lines();

        let mut coords = lines.next().unwrap().split_whitespace();
        let width = coords.next().unwrap().parse::<usize>().unwrap();
        let height = coords.next().unwrap().parse::<usize>().unwrap();

        let mut mask = Mask::new(width, height);

        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => mask.set(Point::new(x as i32, y as i32), true),
                    'x' => mask.set(Point::new(x as i32, y as i32), false),
                    _ => panic!("Invalid character in mask file"),
                }
            }
        }

        return Ok(mask);
    }

    pub fn from_png(file_path: &str) -> Result<Mask, ImageError> {
        let img = open(file_path)?;
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();

        let mut mask = Mask::new(width as usize, height as usize);

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
                mask.set(Point::new(x as i32, y as i32), false);
            } else {
                mask.set(Point::new(x as i32, y as i32), true);
            }
        }

        return Ok(mask);
    }
}

