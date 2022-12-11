use std::f64;
use image::{DynamicImage, GenericImageView, Pixel, Rgb, Rgba, RgbaImage};
use std::thread;

struct Point {
    x: usize,
    y: usize
}

struct Path {
    total_energy: f64,
    path: Vec<Point>
}


fn main() {
    let mut pic_threads = vec![];

    pic_threads.push( thread::spawn(|| {
        let image = image::open("image.png").expect("File not found");
        let energy_map = map_energy(&image);
        let row = first_row(&energy_map[0]);
        let paths = find_seam(row,1, &energy_map);
        let shortest_path = lowest_energy_seam(paths);
        let output = show_seam(shortest_path, image);
        output.save("image_out.png").expect("Image could not save");
    }));


    pic_threads.push( thread::spawn(|| {
        let image = image::open("sky.png").expect("File not found");
        let energy_map = map_energy(&image);
        let row = first_row(&energy_map[0]);
        let paths = find_seam(row,1, &energy_map);
        let shortest_path = lowest_energy_seam(paths);
        let output = show_seam(shortest_path, image);
        output.save("sky_out.png").expect("Image could not save");
    }));


    pic_threads.push( thread::spawn(|| {
        let image = image::open("surfer.jpg").expect("File not found");
        let energy_map = map_energy(&image);
        let row = first_row(&energy_map[0]);
        let paths = find_seam(row,1, &energy_map);
        let shortest_path = lowest_energy_seam(paths);
        let output = show_seam(shortest_path, image);
        output.save("surfer_out.jpg").expect("Image could not save");
    }));


    pic_threads.push( thread::spawn(|| {
        let image = image::open("ocean.png").expect("File not found");
        let energy_map = map_energy(&image);
        let row = first_row(&energy_map[0]);
        let paths = find_seam(row,1, &energy_map);
        let shortest_path = lowest_energy_seam(paths);
        let output = show_seam(shortest_path, image);
        output.save("ocean_out.png").expect("Image could not save");
    }));


    pic_threads.push( thread::spawn(|| {
        let image = image::open("night_city.jpg").expect("File not found");
        let energy_map = map_energy(&image);
        let row = first_row(&energy_map[0]);
        let paths = find_seam(row,1, &energy_map);
        let shortest_path = lowest_energy_seam(paths);
        let output = show_seam(shortest_path, image);
        output.save("night_city_out.jpg").expect("Image could not save");
    }));

    for pic_thread in pic_threads {
        pic_thread.join().unwrap();
    }
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
    return find_seam(paths, row_index +1, energy_map)
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

fn show_seam(path: Vec<Point>, image: DynamicImage) -> RgbaImage {
    let mut output = image.into_rgba8();
    for pt in path {
        output.put_pixel(pt.x as u32, pt.y as u32, Rgba([255,0,0,255]))
    }
    output
}
fn first_row(first_row: &Vec<f64>) -> Vec<Path> {
    let mut row = vec![];
    for i in 0..first_row.len() {
        row.push(Path {total_energy: first_row[i], path: vec![Point{x: i, y: 0}]})
    }
    row
}

fn map_energy(image: &DynamicImage) -> Vec<Vec<f64>> { //TODO try to apply multi threading
    let mut max_energy = 0.0;
    let (width, height) = image.dimensions();
    let mut map = vec![vec![0.0; width as usize]; height as usize];
    let buffer = image.to_rgba8();
    for (x, y, _) in image.pixels() {
        let energy = calculate_energy(x, y, &buffer);
        if energy > max_energy {
            max_energy = energy;
        }
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

// fn _calculate_intensity(image: DynamicImage) -> RgbaImage { //TODO try to apply multi threading
//     let mut max_energy = 0.0;
//     let (width, height) = image.dimensions();
//     let mut map = vec![vec![0.0; width as usize]; height as usize];
//     let buffer = image.to_rgba8();
//     let mut output = RgbaImage::new(width, height);
//     for (x, y, _) in image.pixels() {
//         let energy = calculate_energy(x, y, &buffer);
//         if energy > max_energy {
//             max_energy = energy;
//         }
//         map[y as usize][x as usize] = energy;
//     }
//
//     for (x, y, _) in image.pixels() {
//         let energy = map[y as usize][x as usize];
//         let intensity = (255.0 * energy / max_energy) as u8;
//         output.put_pixel(x, y, Rgba([intensity,intensity,intensity, 255]));
//     }
//     output
// }