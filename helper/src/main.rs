#![windows_subsystem = "windows"]

use std::cmp::{max, min};
use std::fs::File;
use std::io::Read;
use std::io;
use std::ops::RangeInclusive;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (terrain, size, geo_transform) = load_geo()?;
    let points_regex = Regex::new(r"(?m)\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\), \(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)")?;
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)?;
        let Some((_full, [regexa_x, regexa_y, regexa_z, regexb_x, regexb_y, regexb_z])) =
            points_regex.captures(buffer.as_str()).map(|caps| caps.extract()) else { println!("false"); continue; };
        let point_pair = ((regexa_x.parse::<f64>()?, regexa_y.parse::<f64>()?, regexa_z.parse::<f64>()?), (regexb_x.parse::<f64>()?, regexb_y.parse::<f64>()?, regexb_z.parse::<f64>()?));
    
        let (a_x, a_y) = world_to_pixel(&geo_transform, point_pair.0.0, point_pair.0.1)?;
        let (b_x, b_y) = world_to_pixel(&geo_transform, point_pair.1.0, point_pair.1.1)?;
    
        let point_a: (i32, i32, i32) = (a_x as i32, a_y as i32, point_pair.0.2.round() as i32);
        let point_b: (i32, i32, i32) = (b_x as i32, b_y as i32, point_pair.1.2.round() as i32);
        let los_r = line_of_sight(point_a, point_b, &(terrain), size);
        if los_r.is_err() {println!("false"); continue;}
        println!("{}", los_r.unwrap());
    }
}

fn load_geo() -> Result<(Vec<f64>, (usize, usize), (f64, f64, f64, f64, f64, f64)), Box<dyn std::error::Error>> {
    let mut terr_file = File::open("geodata/geo.data")?;
    let mut meta_file = File::open("geodata/meta.json")?;
    let mut meta_json = String::new();
    meta_file.read_to_string(&mut meta_json).expect("Unable to read file");
    let meta: ((f64, f64, f64, f64, f64, f64), (usize, usize)) = serde_json::from_str(&meta_json)?;
    let geotransform = meta.0;
    let size= meta.1;
    let mut t_encoded = Vec::new();
    let _ = terr_file.read_to_end(&mut t_encoded);
    let terr: Vec<f64> = bincode::deserialize(&t_encoded)?;
    Ok((terr, size, geotransform))
}

fn world_to_pixel(geo_transform: &(f64, f64, f64, f64, f64, f64), latitude: f64, longitude: f64) -> Result<(isize, isize), Box<dyn std::error::Error>> {
    let pixel_x = (longitude - geo_transform.0) / geo_transform.1;
    let pixel_y = (latitude - geo_transform.3) / geo_transform.5;
    Ok((pixel_x.round() as isize, pixel_y.round() as isize))
}

fn line_of_sight(point_a: (i32, i32, i32), point_b: (i32, i32, i32), terrain: &Vec<f64>, size: (usize, usize)) -> Result<bool, i32> {
    let cpoint_a: (i32, i32, i32) = (point_a.0, point_a.1, point_a.2);
    let cpoint_b: (i32, i32, i32) = (point_b.0, point_b.1, point_b.2);
    let mut hit_total = false;
    let vector = (cpoint_b.0 - cpoint_a.0, cpoint_b.1 - cpoint_a.1, cpoint_b.2 - cpoint_a.2);
    let max_dir = max(max(vector.0, vector.1), vector.2);
    let max_range: RangeInclusive<i32>;
    let path_function: &dyn Fn(i32,&(i32, i32, i32),&(i32, i32, i32)) -> f32;
    if max_dir == vector.0 {path_function = &|a, vector, point| {((a-point.0) as f32)/(vector.0 as f32) as f32}; max_range=min(cpoint_a.0, cpoint_b.0)..=max(cpoint_a.0, cpoint_b.0);}
    else if max_dir == vector.1 {path_function = &|a, vector, point| {((a-point.1) as f32)/(vector.1 as f32) as f32}; max_range=min(cpoint_a.1, cpoint_b.1)..=max(cpoint_a.1, cpoint_b.1);}
    else if max_dir == vector.2 {path_function = &|a, vector, point| {((a-point.2) as f32)/(vector.2 as f32) as f32}; max_range=min(cpoint_a.2, cpoint_b.2)..=max(cpoint_a.2, cpoint_b.2);}
    else {return Ok(false)}
    for a in max_range {
        let k = path_function(a, &vector, &cpoint_a);
        let x = (cpoint_a.0 as f32 + k*(vector.0 as f32)).round() as i32;
        let y = (cpoint_a.1 as f32 + k*(vector.1 as f32)).round() as i32;
        let z = (cpoint_a.2 as f32 + k*(vector.2 as f32)).round() as i32;

        let i = (size.0 as i32)*y + x;
        if i as usize > terrain.len() {return Err(1);}
        let terrain_height = terrain[i as usize] as usize;
        let hit = z as usize <= terrain_height;
        if hit {hit_total = true;}
    }
    Ok(!hit_total)
}
