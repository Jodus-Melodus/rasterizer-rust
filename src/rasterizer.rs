use std::fs::File;
use std::io::Cursor;
use std::io::Result;
use std::io::Write;

use vector_2d_3d::Vector2D;

#[derive(PartialEq)]
pub struct Image {
    pixels: Vec<Vec<(u8, u8, u8)>>,
}

impl Image {
    pub const RED: (u8, u8, u8) = (0xFF, 0x00, 0x00);
    pub const GREEN: (u8, u8, u8) = (0x00, 0xFF, 0x00);
    pub const BLUE: (u8, u8, u8) = (0x00, 0x00, 0xFF);
    pub const WHITE: (u8, u8, u8) = (0xFF, 0xFF, 0xFF);
    pub const BLACK: (u8, u8, u8) = (0x00, 0x00, 0x00);

    pub fn new(width: usize, height: usize) -> Self {
        Image {
            pixels: vec![vec![(0, 0, 0); width]; height],
        }
    }

    pub fn from_array(pixels: Vec<Vec<(u8, u8, u8)>>) -> Self {
        Image { pixels }
    }

    pub fn create_gradient(width: usize, height: usize) -> Self {
        let mut pixels = vec![vec![(0, 0, 0); width]; height];

        for x in 0..height {
            for y in 0..width {
                pixels[y][x] = (0, x as u8, y as u8);
            }
        }

        Image { pixels }
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

    pub fn draw_triangle1(
        &mut self,
        a: (u32, u32),
        b: (u32, u32),
        c: (u32, u32),
        color: (u8, u8, u8),
    ) {
        self.pixels[a.0 as usize][a.1 as usize] = color;
        self.pixels[b.0 as usize][b.1 as usize] = color;
        self.pixels[c.0 as usize][c.1 as usize] = color;

        let a = Vector2D::new(a.0 as f32, a.1 as f32);
        let b = Vector2D::new(b.0 as f32, b.1 as f32);
        let c = Vector2D::new(c.0 as f32, c.1 as f32);
        let ab = b - a;
        let bc = c - b;
        let ca = a - c;

        let min_x = a.x.min(b.x).min(c.x) as usize;
        let max_x = a.x.max(b.x).max(c.x) as usize;
        let min_y = a.y.min(b.y).min(c.y) as usize;
        let max_y = a.y.max(b.y).max(c.y) as usize;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Vector2D::new(x as f32, y as f32);

                let ap = p - a;
                let bp = p - b;
                let cp = p - c;

                let z1 = ab.cross_product(&ap);
                let z2 = bc.cross_product(&bp);
                let z3 = ca.cross_product(&cp);

                if (z1 >= 0.0 && z2 >= 0.0 && z3 >= 0.0) || (z1 <= 0.0 && z2 <= 0.0 && z3 <= 0.0) {
                    self.pixels[x][y] = color;
                }
            }
        }
    }

    pub fn draw_triangle2(
        &mut self,
        a: (u32, u32),
        b: (u32, u32),
        c: (u32, u32),
        color: (u8, u8, u8),
    ) {
        self.pixels[a.1 as usize][a.0 as usize] = color;
        self.pixels[b.1 as usize][b.0 as usize] = color;
        self.pixels[c.1 as usize][c.0 as usize] = color;

        let a = Vector2D::new(a.0 as f32, a.1 as f32);
        let b = Vector2D::new(b.0 as f32, b.1 as f32);
        let c = Vector2D::new(c.0 as f32, c.1 as f32);

        let min_x = a.x.min(b.x).min(c.x) as usize;
        let max_x = a.x.max(b.x).max(c.x) as usize;
        let min_y = a.y.min(b.y).min(c.y) as usize;
        let max_y = a.y.max(b.y).max(c.y) as usize;

        let (cond0, cond1, cond2) = Self::conditions(a, b, c);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if cond0(x as f32, y as f32)
                    && cond1(x as f32, y as f32)
                    && cond2(x as f32, y as f32)
                {
                    self.pixels[y][x] = color;
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
        let mab = (a.y - b.y) / (a.x - b.x); // prevent division by 0
        let mbc = (b.y - c.y) / (b.x - c.x); // prevent division by 0
        let mac = (a.y - c.y) / (a.x - c.x); // prevent division by 0

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
}
