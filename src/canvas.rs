use image::RgbImage;
use bresenham::Bresenham;
use imageproc::drawing::draw_filled_rect;
use imageproc::rect::Rect;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub scale: f64,

    pub buffer: RgbImage,
}

impl Canvas {

    // Generates an image buffer with the desired background pixel
    /*
    pub fn gen_buffer(&mut self, pixel: image::Rgb<u8>) {
        self.buffer = RgbImage::from_pixel(self.width * self.scale, self.height * self.scale, pixel);
    }
    */

    // Saves the image buffer to the given file path
    pub fn save(&self, path: &str) {
        let _ = self.buffer.save(path);
    }
    
    // Draws a rasterized line between two points, using Bresenham's line algorithm.
    // TODO: Switch to custom implementation which does not use isize
    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, pixel: image::Rgb<u8>) {
        /*
        self.buffer.put_pixel(u32::min(209 * self.scale as u32, (self.scale * x2) as u32), u32::min(296 * self.scale as u32, (self.scale * y2) as u32), pixel);
        self.buffer.put_pixel(u32::min(209 * self.scale as u32, (self.scale * x1) as u32), u32::min(296 * self.scale as u32, (self.scale * y1) as u32), pixel);
        */
        
        for (x, y) in Bresenham::new(((x1 * self.scale) as isize, (y1 * self.scale) as isize), ((x2 * self.scale) as isize, (y2 * self.scale) as isize)) {
            self.buffer.put_pixel(u32::min(209 * self.scale as u32, x as u32), u32::min(296 * self.scale as u32, y as u32), pixel);
            // println!("Drawing pixel {} {}", u32::min(209 * self.scale as u32, x as u32), u32::min(296 * self.scale as u32, y as u32));
        }
    }

    pub fn point(&mut self, x: u32, y: u32, pixel: image::Rgb<u8>) {
        self.buffer = draw_filled_rect(&mut self.buffer, Rect::at((x * self.scale as u32) as i32, (y * self.scale as u32) as i32).of_size(4, 4), pixel);
    }

}
