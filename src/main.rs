use image::{RgbImage, Rgb};
use image::imageops::{flip_horizontal, flip_vertical};

mod canvas;
mod belts;
mod instructions;

fn main() {
    /*
    let mut blur_background = RgbImage::from_pixel(WIDTH, HEIGHT, Rgb([255, 255, 255]));
    for x in 45..=55 {
        for y in 25..=75 {
            blur_background.put_pixel(x, y, Rgb([0, 0, 0]));
        }
    }
    blur_background = gaussian_blur_f32(&blur_background, 2.5);
    blur_background.save("./img.png").expect("error saving image");
    */


    let mut canvas = canvas::Canvas {
        width: 210,
        height: 297,
        scale: 4.,
        buffer: RgbImage::from_pixel(210 * 4, 297 * 4, Rgb([255, 255, 255]))
    };


    let ins: Vec<(i16, i16)> = instructions::load_instructions("./ins.json");
    let mut belts = belts::Belts::new_by_cartesian((754. - 210.) / 1.98 + 20., 192. + 30., 754.);

    let mut last_xy = belts.get_cartesian();
    let initial_xy = last_xy.clone();

    for (ld, rd) in ins {
        belts.move_belts(ld, -rd);
        let (x, y) = belts.get_cartesian();
        // println!("lx:{} ly:{} x:{} y:{}", last_xy.0, last_xy.1, x, y);

        canvas.line(last_xy.0 - (754. - 210.) / 1.98, last_xy.1 - 192., x - ((754. - 210.) / 1.98), y - 192., Rgb([0, 0, 0]));
        last_xy = (x, y);
    }


    canvas.point((last_xy.0 - (754. - 210.) / 1.98) as u32, (last_xy.1 - 192.) as u32, Rgb([255, 0, 0]));
    canvas.point((initial_xy.0 - (754. - 210.) / 1.98) as u32, (initial_xy.1 - 192.) as u32, Rgb([0, 255, 0]));

    // technically needed to flip image which is what the drawer does
    // canvas.buffer = flip_horizontal(&canvas.buffer);
    // canvas.buffer = flip_vertical(&canvas.buffer);

    canvas.save("./img.png");
}
