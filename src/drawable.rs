use crate::prelude::*;

pub trait Drawable {
    fn to_grid_image(&self, size: usize) -> ImageBuffer<image::Rgb<u8>, Vec<u8>>;

    fn background_color_for(&self, cell: &Cell, distances: &Distances) -> Rgb<u8> {
        let distance = distances.distance(cell.point);

        if distance.is_none() {
            return BLACK;
        }

        //let (max_distance, _) = distances.max(self);
        let max_distance = 0;
        if max_distance == 0 {
            return BLACK;
        }

        let intensity = (max_distance - distance.unwrap()) as f64 / max_distance as f64;
        let dark = (255.0 * intensity) as u8;
        let bright = 128 + (127.0 * intensity) as u8;
        let color = image::Rgb([dark, bright, dark]);

        return color;
    }

    fn draw_line(
        buff: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        mut x0: i32,
        mut y0: i32,
        x1: i32,
        y1: i32,
        color: Rgb<u8>,
    ) {
        let dx = i32::abs(x1 - x0);
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -i32::abs(y1 - y0);
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy; // error value e_xy

        loop {
            if x0 >= 0 && y0 >= 0 {
                let pixel = buff.get_pixel_mut(x0 as u32, y0 as u32);
                *pixel = color;
            }

            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn circle(
        imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        center_x: u32,
        center_y: u32,
        size: usize,
        color: image::Rgb<u8>,
    ) {
        let diameter = size;
        let mut x = diameter as i32;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            imgbuf.put_pixel(center_x + x as u32, center_y + y as u32, color);
            imgbuf.put_pixel(center_x + y as u32, center_y + x as u32, color);
            imgbuf.put_pixel(center_x - y as u32, center_y + x as u32, color);
            imgbuf.put_pixel(center_x - x as u32, center_y + y as u32, color);
            imgbuf.put_pixel(center_x - x as u32, center_y - y as u32, color);
            imgbuf.put_pixel(center_x - y as u32, center_y - x as u32, color);
            imgbuf.put_pixel(center_x + y as u32, center_y - x as u32, color);
            imgbuf.put_pixel(center_x + x as u32, center_y - y as u32, color);

            y += 1;
            err += 1 + 2 * y;
            if 2 * (err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2 * x;
            }
        }
    }
}
