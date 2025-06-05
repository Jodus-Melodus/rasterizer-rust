use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::io::Result;
use std::io::Write;
use std::time::Instant;
use std::vec;

use vector_2d_3d::Vector2D;
use vector_2d_3d::Vector3D;

fn make_font() -> HashMap<char, [u8; 15]> {
    let mut font = HashMap::new();

    font.insert('A', [0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1]);
    font.insert('B', [1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 0]);
    font.insert('C', [0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1]);
    font.insert('D', [1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0]);
    font.insert('E', [1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 1]);
    font.insert('F', [1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0]);
    font.insert('G', [0, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1]);
    font.insert('H', [1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1]);
    font.insert('I', [1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1]);
    font.insert('J', [0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0]);
    font.insert('K', [1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1]);
    font.insert('L', [1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1]);
    font.insert('M', [1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1]);
    font.insert('N', [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1]);
    font.insert('O', [0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0]);
    font.insert('P', [1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0]);
    font.insert('Q', [0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1]);
    font.insert('R', [1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1]);
    font.insert('S', [0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0]);
    font.insert('T', [1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0]);
    font.insert('U', [1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0]);
    font.insert('V', [1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0]);
    font.insert('W', [1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1]);
    font.insert('X', [1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 1]);
    font.insert('Y', [1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0]);
    font.insert('Z', [1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1]);
    font.insert(':', [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0]);
    font.insert('0', [0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0]);
    font.insert('1', [0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1]);
    font.insert('2', [1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1]);
    font.insert('3', [1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0]);
    font.insert('4', [1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1]);
    font.insert('5', [1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0]);
    font.insert('6', [0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0]);
    font.insert('7', [1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0]);
    font.insert('8', [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]);
    font.insert('9', [0, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0]);
    font.insert(' ', [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    font.insert('a', [0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1]);
    font.insert('b', [1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0]);
    font.insert('c', [0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1]);
    font.insert('d', [0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1]);
    font.insert('e', [0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1]);
    font.insert('f', [0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0]);
    font.insert('g', [0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0]);
    font.insert('h', [1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1]);
    font.insert('i', [0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1]);
    font.insert('j', [0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0]);
    font.insert('k', [1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1]);
    font.insert('l', [1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1]);
    font.insert('m', [0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1]);
    font.insert('n', [0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1]);
    font.insert('o', [0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0]);
    font.insert('p', [0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0]);
    font.insert('q', [0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1]);
    font.insert('r', [0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0]);
    font.insert('s', [0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0]);
    font.insert('t', [0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1]);
    font.insert('u', [0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1]);
    font.insert('v', [0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 0]);
    font.insert('w', [0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1]);
    font.insert('x', [0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1]);
    font.insert('y', [0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0]);
    font.insert('z', [0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1]);
    font.insert('.', [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]);
    font.insert('-', [0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0]);
    font.insert(';', [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0]);

    font
}

pub struct Image {
    pixels: Vec<u32>,
    width: usize,
    width_offset: usize,
    height: usize,
    height_offset: usize,
    font: HashMap<char, [u8; 15]>,
}

impl Image {
    pub fn get_pixels(&self) -> Vec<u32> {
        self.pixels.iter().rev().copied().collect()
    }

    pub fn rbg_to_u32(r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    pub fn u32_to_rgb(color: u32) -> (u8, u8, u8) {
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;
        (r, g, b)
    }

    pub fn new(width: usize, height: usize) -> Self {
        Image {
            pixels: vec![0; width * height],
            width,
            width_offset: width / 2,
            height,
            height_offset: height / 2,
            font: make_font(),
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.pixels.len() {
            self.pixels[i] = 0;
        }
    }

    pub fn draw_point(&mut self, point: Vector2D, color: u32) {
        let x = ((point.x as isize + self.width_offset as isize) as usize).clamp(0, self.width - 1);
        let y =
            ((point.y as isize + self.height_offset as isize) as usize).clamp(0, self.height - 1);
        let flipped_x = self.width - 1 - x;
        self.pixels[self.width * y + flipped_x] = color;
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
        let width = self.width;
        let height = self.height;

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
        // BMP stores rows bottom-to-top
        for y in (0..height).rev() {
            let row_start = y * width;
            let row_end = row_start + width;
            for &color in &self.pixels[row_start..row_end] {
                let (r, g, b) = Self::u32_to_rgb(color);
                cursor.write_all(&[b, g, r])?;
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

    pub fn draw_triangle1(&mut self, a: Vector2D, b: Vector2D, c: Vector2D, color: u32) {
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

    pub fn draw_triangle2(&mut self, a: Vector2D, b: Vector2D, c: Vector2D, color: u32) {
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

    pub fn draw_triangle3(&mut self, a: Vector2D, b: Vector2D, c: Vector2D, color: u32) {
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

    pub fn draw_triangle4(&mut self, a: Vector2D, b: Vector2D, c: Vector2D, color: u32) {
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

    pub fn project_3d_to_2d(
        &self,
        point: Vector3D,
        camera: Vector3D,
        focal_length: f32,
    ) -> Option<Vector2D> {
        let dz = point.z - camera.z;
        if dz >= 0.0 {
            return None;
        }
        let projected_x = (point.x - camera.x) * (focal_length / dz);
        let projected_y = (point.y - camera.y) * (focal_length / dz);
        if !projected_x.is_finite() || !projected_y.is_finite() {
            return None;
        }
        let screen_x = projected_x + self.width_offset as f32;
        let screen_y = projected_y + self.height_offset as f32;
        if screen_x < 0.0
            || screen_x >= self.width as f32
            || screen_y < 0.0
            || screen_y >= self.height as f32
        {
            return None;
        }
        Some(Vector2D::from_coord(-projected_x, -projected_y))
    }

    pub fn draw_text(&mut self, text: &str, coordinate: Vector2D, color: u32, scale: usize) {
        let mut offset = 0.0;
        for letter in text.chars() {
            let map = *self.font.get(&letter).unwrap_or(&[0; 15]);
            for r in 0..5 {
                for c in 0..3 {
                    if map[3 * r + c] == 1 {
                        // Draw a scale x scale block for each pixel
                        for dy in 0..scale {
                            for dx in 0..scale {
                                let point = Vector2D::from_coord(
                                    offset + (c as f32 * scale as f32) + dx as f32 + coordinate.x,
                                    coordinate.y - (r as f32 * scale as f32) - dy as f32,
                                );
                                self.draw_point(point, color);
                            }
                        }
                    }
                }
            }
            offset += 3.0 * scale as f32 + scale as f32;
        }
    }

    pub fn translate(point: Vector3D, translate_vector: Vector3D) -> Vector3D {
        Vector3D::from_coord(
            point.x + translate_vector.x,
            point.y + translate_vector.y,
            point.z + translate_vector.z,
        )
    }

    pub fn rotate_delta_time(
        point: Vector3D,
        delta_time: Instant,
        rotation: (bool, bool, bool),
    ) -> Vector3D {
        let angle = delta_time.elapsed().as_secs_f32() * 0.5;
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let mut result = point;

        if rotation.0 {
            // Rotation around X axis
            result = Vector3D::from_coord(
                result.x,
                result.y * cos_a - result.z * sin_a,
                result.y * sin_a + result.z * cos_a,
            );
        }

        if rotation.1 {
            // Rotation around Y axis
            result = Vector3D::from_coord(
                result.x * cos_a + result.z * sin_a,
                result.y,
                -result.x * sin_a + result.z * cos_a,
            );
        }
        if rotation.2 {
            // Rotation around Z axis
            result = Vector3D::from_coord(
                result.x * cos_a - result.y * sin_a,
                result.x * sin_a + result.y * cos_a,
                result.z,
            );
        }

        result
    }

    pub fn rotate_angle(point: Vector3D, angle: f32, rotation: (bool, bool, bool)) -> Vector3D {
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let mut result = point;

        if rotation.0 {
            // Rotation around X axis
            result = Vector3D::from_coord(
                result.x,
                result.y * cos_a - result.z * sin_a,
                result.y * sin_a + result.z * cos_a,
            );
        }

        if rotation.1 {
            // Rotation around Y axis
            result = Vector3D::from_coord(
                result.x * cos_a + result.z * sin_a,
                result.y,
                -result.x * sin_a + result.z * cos_a,
            );
        }
        if rotation.2 {
            // Rotation around Z axis
            result = Vector3D::from_coord(
                result.x * cos_a - result.y * sin_a,
                result.x * sin_a + result.y * cos_a,
                result.z,
            );
        }

        result
    }
}
