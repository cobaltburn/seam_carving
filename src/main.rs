use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, Rgba, RgbaImage};

fn main() {
    let image = image::open("image.png").expect("File not found");
    let output = calculate_intensity(image);
    output.save("image_out.png").expect("Image could not save");
    let image = image::open("sky.png").expect("File not found");
    let output = calculate_intensity(image);
    output.save("sky_out.png").expect("Image could not save");
}

fn calculate_intensity(image: DynamicImage) -> RgbaImage {
    let mut max_energy = 0.0;
    let (width, height) = image.dimensions();
    let mut map = vec![vec![0.0; width as usize]; height as usize];
    let buffer = image.to_rgba8();
    let mut output = RgbaImage::new(width, height);
    for (x, y, _) in image.pixels() {
        let energy = calculate_energy(x, y, &buffer);
        if energy > max_energy {
            max_energy = energy;
        }
        map[y as usize][x as usize] = energy;
    }

    for (x, y, _) in image.pixels() {
        let energy = map[y as usize][x as usize];
        let intensity = (255.0 * energy / max_energy) as u8;
        output.put_pixel(x, y, Rgba([intensity,intensity,intensity, 255]));
    }
    output
}

fn calculate_energy(x: u32, y: u32, image: &RgbaImage) -> f64 {
    let top = image.get_pixel_checked(x, y+1).unwrap_or(image.get_pixel(x, y)).to_rgb();
    let bot = match y {
        0 => image.get_pixel(x, y),
        _ => image.get_pixel(x,y-1)
    }.to_rgb();
    let right = image.get_pixel_checked(x+1, y).unwrap_or(image.get_pixel(x, y)).to_rgb();
    let left = match x {
        0 => image.get_pixel(x, y),
        _ => image.get_pixel(x-1, y)
    }.to_rgb();
    (calculate_delta(top, bot) + calculate_delta(right, left)).sqrt()
}

fn calculate_delta(a: Rgb<u8>, b: Rgb<u8>) -> f64 {
    let a = a.channels().iter();
    let b = b.channels().iter();
    let mut compare: Vec<f64> = vec![];
    for (&i, &j) in a.zip(b) {
        compare.push(((i as f64) - (j as f64)).powf(2.0))
    }
    compare.iter().sum()
}