use std::collections::VecDeque;
use std::f64;
use image::{DynamicImage, Pixel, Rgb, RgbaImage};

struct Point {
    x: usize,
    y: usize
}

struct Path {
    total_energy: f64,
    path: Vec<Point>
}

fn main() {
    process_image("image.png", "image_out.png", 10, 10);
    process_image("sky.png", "sky_out.png", 10, 10);
    process_image("surfer.jpg", "surfer_out.jpg", 10, 10);
    process_image("ocean.png", "ocean_out.png", 10, 10);
    process_image("night_city.jpg", "night_city_out.jpg", 10, 10);
}

fn process_image(in_file: &str, out_file: &str, vertical_count: i32, horizontal_count: i32) {
    let image = image::open(in_file).expect("File not found");

    let mut image = image.into_rgba8();
    for _ in 0..vertical_count {
        let energy_map = map_energy(&image);
        let row = first_row(&energy_map[0]);
        let paths = find_seam(row, 1, &energy_map);
        let shortest_path = lowest_energy_seam(paths);
        image = remove_seam(shortest_path,&image);
    }
    let rotated_image = DynamicImage::ImageRgba8(image.clone()).rotate90();
    image = rotated_image.into_rgba8();

    for _ in 0..horizontal_count {
        let energy_map = map_energy(&image);
        let row = first_row(&energy_map[0]);
        let paths = find_seam(row, 1, &energy_map);
        let shortest_path = lowest_energy_seam(paths);
        image = remove_seam(shortest_path,&image);
    }

    DynamicImage::ImageRgba8(image).rotate270().save(out_file).expect("Image could not save");
}

fn find_seam(mut paths: Vec<Path>, row_index: usize, energy_map: &Vec<Vec<f64>>) -> Vec<Path> {
    let next_row = &energy_map[row_index];
    for path in paths.iter_mut() {
        let mut new_energy = f64::MAX;
        let mut new_point = Point { x: 0, y: row_index };
        let column_index = path.path.last().expect("Path list empty").x;
        let mut positions: [Option<&f64>;3] = [None;3];
        let begin: usize = if column_index == 0  {
            1
        } else {
            0
        };
        for i in begin..positions.len() {
            positions[i] = next_row.get(i + column_index - 1);
        }
        for (i , &position) in positions.iter().enumerate() {
            if let Some(energy) = position {
                if path.total_energy + energy < new_energy {
                    new_energy = path.total_energy + energy;
                    new_point.x = i + column_index - 1
                }
            }
        }
        path.total_energy = new_energy;
        path.path.push(new_point);
    }

    if row_index + 1 >= energy_map.len() {
        return paths
    }
    find_seam(paths, row_index +1, energy_map)
}

fn lowest_energy_seam(paths: Vec<Path>) -> Vec<Point> {
    let mut shortest_path = Path {total_energy: f64::MAX, path: vec![]};
    for path in paths {
        if path.total_energy < shortest_path.total_energy {
            shortest_path = path;
        }
    }
    shortest_path.path
}

fn remove_seam(path: Vec<Point>, image: &RgbaImage) -> RgbaImage {
    let mut path = VecDeque::from(path);
    let (width, height) = image.dimensions();
    let mut reduced_image = RgbaImage::new(width - 1, height - 1);
    let mut image_pixels = image.pixels();
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            if x == path[0].x as u32 && y == path[0].y as u32 {
                image_pixels.next();
                path.pop_front();
            }
            reduced_image.put_pixel(x, y, image_pixels.next().unwrap().clone());
        }
    }
    reduced_image
}

fn first_row(first_row: &Vec<f64>) -> Vec<Path> {
    let mut row = vec![];
    for i in 0..first_row.len() {
        row.push(Path {total_energy: first_row[i], path: vec![Point{x: i, y: 0}]})
    }
    row
}

fn map_energy(image: &RgbaImage) -> Vec<Vec<f64>> {
    let (width, height) = image.dimensions();
    let mut map = vec![vec![0.0; width as usize]; height as usize];
    for (x, y, _) in image.enumerate_pixels() {
        let energy = calculate_energy(x, y, &image);
        map[y as usize][x as usize] = energy;
    }
    map
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

fn calculate_delta(pt1: Rgb<u8>, pt2: Rgb<u8>) -> f64 {
    let pt_itr_1 = pt1.channels().iter();
    let pt_itr_2 = pt2.channels().iter();
    let mut compare: Vec<f64> = vec![];
    for (&i, &j) in pt_itr_1.zip(pt_itr_2) {
        compare.push(((i as f64) - (j as f64)).powf(2.0))
    }
    compare.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
}