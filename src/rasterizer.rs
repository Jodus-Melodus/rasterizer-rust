use std::fs::File;
use std::io::Cursor;
use std::io::Result;
use std::io::Write;
use std::vec;

use vector_2d_3d::Vector2D;

#[derive(PartialEq)]
pub struct Image {
    pixels: Vec<Vec<(u8, u8, u8)>>,
    offset: usize,
}

impl Image {
    pub const RED: (u8, u8, u8) = (0xFF, 0x00, 0x00);
    pub const GREEN: (u8, u8, u8) = (0x00, 0xFF, 0x00);
    pub const BLUE: (u8, u8, u8) = (0x00, 0x00, 0xFF);
    pub const WHITE: (u8, u8, u8) = (0xFF, 0xFF, 0xFF);
    pub const BLACK: (u8, u8, u8) = (0x00, 0x00, 0x00);

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.pixels
            .iter()
            .rev()
            .flat_map(|row| row.iter())
            .map(|&(r, g, b)| ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
            .collect()
    }

    pub fn new(width: usize, height: usize) -> Self {
        Image {
            pixels: vec![vec![(0, 0, 0); width]; height],
            offset: width / 2,
        }
    }

    pub fn from_array(pixels: Vec<Vec<(u8, u8, u8)>>) -> Self {
        let offset = pixels.len() / 2;
        Image { pixels, offset }
    }

    fn draw_point(&mut self, point: Vector2D, color: (u8, u8, u8)) {
        self.pixels[(point.y as isize + self.offset as isize) as usize]
            [(point.x as isize + self.offset as isize) as usize] = color;
    }

    fn get_max_min_coords(a: Vector2D, b: Vector2D, c: Vector2D) -> (isize, isize, isize, isize) {
        (
            a.x.min(b.x).min(c.x) as isize,
            a.x.max(b.x).max(c.x) as isize,
            a.y.min(b.y).min(c.y) as isize,
            a.y.max(b.y).max(c.y) as isize,
        )
    }

    pub fn save_bmp(&self, path: &str) -> Result<()> {
        println!("Saving...");
        let height = self.pixels.len();
        let width = if height > 0 { self.pixels[0].len() } else { 0 };

        // Each row in BMP must be padded to a multiple of 4 bytes
        let row_padding = (4 - (width * 3) % 4) % 4;
        let pixel_data_size = (width * 3 + row_padding) * height;
        let file_size = 14 + 40 + pixel_data_size;

        let mut buffer = Vec::with_capacity(file_size);
        let mut cursor = Cursor::new(&mut buffer);

        // === BITMAP FILE HEADER (14 bytes) ===
        cursor.write_all(b"BM")?; // Signature
        cursor.write_all(&(file_size as u32).to_le_bytes())?; // File size
        cursor.write_all(&[0u8; 4])?; // Reserved
        cursor.write_all(&(54u32).to_le_bytes())?; // Pixel data offset (14 + 40)

        // === DIB HEADER (BITMAPINFOHEADER, 40 bytes) ===
        cursor.write_all(&(40u32).to_le_bytes())?; // Header size
        cursor.write_all(&(width as i32).to_le_bytes())?; // Width
        cursor.write_all(&(height as i32).to_le_bytes())?; // Height
        cursor.write_all(&(1u16).to_le_bytes())?; // Color planes
        cursor.write_all(&(24u16).to_le_bytes())?; // Bits per pixel
        cursor.write_all(&[0u8; 4])?; // Compression (0 = BI_RGB)
        cursor.write_all(&(pixel_data_size as u32).to_le_bytes())?; // Image size
        cursor.write_all(&(2835u32).to_le_bytes())?; // X pixels per meter
        cursor.write_all(&(2835u32).to_le_bytes())?; // Y pixels per meter
        cursor.write_all(&[0u8; 4])?; // Colors in color table
        cursor.write_all(&[0u8; 4])?; // Important colors

        // === PIXEL DATA ===
        for row in self.pixels.iter() {
            for (r, g, b) in row {
                // BMP uses BGR order
                cursor.write_all(&[*b, *g, *r])?;
            }
            // Row padding
            for _ in 0..row_padding {
                cursor.write_all(&[0u8])?;
            }
        }

        let mut file = File::create(path)?;
        file.write_all(&buffer)?;
        println!("Save Successful");
        Ok(())
    }

    pub fn draw_triangle1(&mut self, a: Vector2D, b: Vector2D, c: Vector2D, color: (u8, u8, u8)) {
        let ab = b - a;
        let bc = c - b;
        let ca = a - c;
        let (min_x, max_x, min_y, max_y) = Self::get_max_min_coords(a, b, c);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Vector2D::from_coord(x as f32, y as f32);

                let ap = p - a;
                let bp = p - b;
                let cp = p - c;

                let z1 = ab.cross_product(&ap);
                let z2 = bc.cross_product(&bp);
                let z3 = ca.cross_product(&cp);

                if (z1 >= 0.0 && z2 >= 0.0 && z3 >= 0.0) || (z1 <= 0.0 && z2 <= 0.0 && z3 <= 0.0) {
                    self.draw_point(p, color);
                }
            }
        }
    }

    pub fn draw_triangle2(&mut self, a: Vector2D, b: Vector2D, c: Vector2D, color: (u8, u8, u8)) {
        let (min_x, max_x, min_y, max_y) = Self::get_max_min_coords(a, b, c);
        let (cond0, cond1, cond2) = Self::conditions(a, b, c);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Vector2D::from_coord(x as f32, y as f32);
                if cond0(p.x, p.y) && cond1(p.x, p.y) && cond2(p.x, p.y) {
                    self.draw_point(p, color);
                }
            }
        }
    }

    fn conditions(
        a: Vector2D,
        b: Vector2D,
        c: Vector2D,
    ) -> (
        Box<dyn Fn(f32, f32) -> bool>,
        Box<dyn Fn(f32, f32) -> bool>,
        Box<dyn Fn(f32, f32) -> bool>,
    ) {
        let mut mab = (a.y - b.y) / (a.x - b.x);
        let mut mbc = (b.y - c.y) / (b.x - c.x);
        let mut mac = (a.y - c.y) / (a.x - c.x);

        if mab == f32::NEG_INFINITY {
            mab = f32::MAX;
        }
        if mbc == f32::NEG_INFINITY {
            mbc = f32::MAX;
        }
        if mac == f32::NEG_INFINITY {
            mac = f32::MAX;
        }

        let ab = move |x: f32| mab * (x - a.x) + a.y;
        let bc = move |x: f32| mbc * (x - b.x) + b.y;
        let ac = move |x: f32| mac * (x - c.x) + c.y;

        let condition0: Box<dyn Fn(f32, f32) -> bool> = if c.y > ab(c.x) {
            Box::new(move |x, y| y >= ab(x))
        } else if c.y < ab(c.x) {
            Box::new(move |x, y| y <= ab(x))
        } else {
            Box::new(move |_, _| true)
        };

        let condition1: Box<dyn Fn(f32, f32) -> bool> = if a.y > bc(a.x) {
            Box::new(move |x, y| y >= bc(x))
        } else if a.y < bc(a.x) {
            Box::new(move |x, y| y <= bc(x))
        } else {
            Box::new(move |_, _| true)
        };

        let condition2: Box<dyn Fn(f32, f32) -> bool> = if b.y > ac(b.x) {
            Box::new(move |x, y| y >= ac(x))
        } else if b.y < ac(b.x) {
            Box::new(move |x, y| y <= ac(x))
        } else {
            Box::new(move |_, _| true)
        };

        (condition0, condition1, condition2)
    }

    pub fn draw_triangle3(&mut self, a: Vector2D, b: Vector2D, c: Vector2D, color: (u8, u8, u8)) {
        let (min_x, max_x, min_y, max_y) = Self::get_max_min_coords(a, b, c);
        let area = (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)).abs() / 2.0;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Vector2D::from_coord(x as f32, y as f32);

                let area_abp =
                    (a.x * (b.y - p.y) + b.x * (p.y - a.y) + p.x * (a.y - b.y)).abs() / 2.0;
                let area_bcp =
                    (b.x * (c.y - p.y) + c.x * (p.y - b.y) + p.x * (b.y - c.y)).abs() / 2.0;
                let area_acp =
                    (a.x * (c.y - p.y) + c.x * (p.y - a.y) + p.x * (a.y - c.y)).abs() / 2.0;

                if (area_abp + area_bcp + area_acp - area).abs() < 1e-6 {
                    self.draw_point(p, color);
                }
            }
        }
    }

    pub fn draw_triangle4(&mut self, a: Vector2D, b: Vector2D, c: Vector2D, color: (u8, u8, u8)) {
        let (min_x, max_x, min_y, max_y) = Self::get_max_min_coords(a, b, c);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Vector2D::from_coord(x as f32, y as f32);

                let w1 = (a.x * (c.y - a.y) + (p.y - a.y) * (c.x - a.x) - p.x * (c.y - a.y))
                    / ((b.y - a.y) * (c.x - a.x) - (b.x - a.x) * (c.y - a.y));
                let w2 = (p.y - a.y - w1 * (b.y - a.y)) / (c.y - a.y);

                if w1 >= 0.0 && w2 >= 0.0 && (w1 + w2 <= 1.0) {
                    self.draw_point(p, color);
                }
            }
        }
    }
}
