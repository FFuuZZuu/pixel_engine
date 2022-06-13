use std::{fs::File, io::Read};

use fontdue::{
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle, VerticalAlign},
    Font, FontSettings, Metrics,
};
use log::*;

#[derive(Clone)]
pub struct FontOptions {
    size: f32,
    font_path: String,
}

#[derive(Clone)]
pub struct UI {
    framebuffer: Vec<u32>,
    width: usize,
    height: usize,
    font_options: FontOptions,
}

impl UI {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            framebuffer: {
                let mut tmp = Vec::new();
                tmp.resize(width * height, 0);
                tmp
            },
            width,
            height,
            // TODO: Fix terrible default font paths
            font_options: FontOptions::new("../../fonts/Roboto-Regular.ttf"),
        }
    }

    pub fn set_font(&mut self, font_options: &FontOptions) {
        self.font_options = font_options.clone();
    }

    pub fn place_pixel(&mut self, x: usize, y: usize, colour: u32) {
        let pos = (y * self.width) + x;
        self.framebuffer[pos as usize + 3] = colour;
    }

    pub fn write_glyph(&mut self, glyph: char, x: u32, y: u32) {
        let font_path = self.clone().font_options.font_path;
        let font = self.read_font(&font_path);
        let font = Font::from_bytes(font, FontSettings::default()).unwrap();
        let (metrics, bitmap) = font.rasterize(glyph, self.font_options.size);
        self.overlay_u8_bitmap_onto_framebuffer(bitmap, metrics, x, y);
    }

    pub fn write_string(&mut self, text: &str, x: u32, y: u32) {
        let font_path = self.clone().font_options.font_path;
        let font = self.read_font(&font_path);
        let font = Font::from_bytes(font, FontSettings::default()).unwrap();
        let fonts = &[font];
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x: x as f32,
            y: y as f32,
            max_width: Some(self.width as f32),
            max_height: Some(self.height as f32),
            vertical_align: VerticalAlign::Top,
            ..LayoutSettings::default()
        });
        layout.append(fonts, &TextStyle::new(text, self.font_options.size, 0));

        for glyph in layout.glyphs() {
            self.write_glyph(glyph.parent, glyph.x as u32, glyph.y as u32);
        }
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        //trace!("{}", self.framebuffer.len());
        let u8_arr = unsafe { self.framebuffer.align_to::<u8>().1 };
        //trace!("{}", u8_arr.len());
        (*frame).copy_from_slice(&u8_arr);
        self.reset();
    }

    fn read_font(&mut self, font_path: &str) -> Vec<u8> {
        let mut file = File::open(font_path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        info!("read font: {}", font_path);
        buf
    }

    fn reset(&mut self) {
        self.framebuffer = {
            let mut tmp = Vec::new();
            tmp.resize(self.width * self.height, 0);
            tmp
        };
        self.font_options = FontOptions::new("../../fonts/Roboto-Regular.ttf");
    }

    fn overlay_u8_bitmap_onto_framebuffer(
        &mut self,
        bitmap: Vec<u8>,
        metrics: Metrics,
        posx: u32,
        posy: u32,
    ) {
        let (mut x, mut y) = (posx as usize, posy as usize);
        for pixel in bitmap {
            if pixel > 0 {
                self.framebuffer[x + (y * self.width)] = 0xFFFFFFFF;
            }

            if x + 1 >= metrics.width + posx as usize {
                x = posx as usize;
                y += 1;
            } else {
                x += 1;
            }
        }
    }
}

impl FontOptions {
    pub fn new(font_path: &str) -> Self {
        Self {
            size: 16.0,
            font_path: font_path.to_string(),
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}
