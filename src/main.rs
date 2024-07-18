use std::cmp::{max, min};
use std::fs::File;
use std::io::{Read, Write};
use std::ops::RangeInclusive;
// use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec;
extern crate nalgebra as na;
// use gdal::raster::{Buffer, RasterBand};
// use gdal::Dataset;
// use na::Vector3;

// use std::vec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("HEYYYYYYYYY");

    // let (a_x, a_y) = world_to_pixel(&geo_transform, 40.32195, -7.6134638889)?;
    // let (b_x, b_y) = world_to_pixel(&geo_transform, 40.2838055556, -7.5478777778)?;

    let (terrain, size, geo_transform) = load_geo()?;

    let point_pairs: Vec<((f64, f64, i32), (f64, f64, i32))> = vec![
            ((40.32195, -7.6134638889, 2050), (40.2750166667, -7.5665805556, 800)),
            ((40.32195, -7.6134638889, 3000), (38.6664972222, -8.2247388889, 10000)),
            // ((40.32195, -7.6134638889, 3000), (40.2838055556, -7.5478777778, 1000)),
            // ((40.32195, -7.6134638889, 0), (40.5404861111, -7.2499805556, 0)),
            // ((40.32195, -7.6134638889, 0), (40.2838055556, -7.5478777778, 0)),
            // ((40.32195, -7.6134638889, 0), (40.2838055556, -7.5478777778, 0))
        ];
    let mut terr: Vec<Vec<(i32, i32)>> = Vec::new();
    for point_pair in point_pairs {
        let (a_x, a_y) = world_to_pixel(&geo_transform, point_pair.0.0, point_pair.0.1)?;
        let (b_x, b_y) = world_to_pixel(&geo_transform, point_pair.1.0, point_pair.1.1)?;
        // let alt_a = get_altitude(a_x, a_y, &dataset)?;
        // let alt_b= get_altitude(b_x, b_y, &dataset)?;
        let point_a: (i32, i32, i32) = (a_x as i32, a_y as i32, point_pair.0.2 as i32);
        let point_b: (i32, i32, i32) = (b_x as i32, b_y as i32, point_pair.1.2 as i32);
        let (los, terr_) = line_of_sight(point_a, point_b, &(terrain), size)?;
        terr.push(terr_);
        println!("-------- {} --------", los);
    } 
    let json = serde_json::to_string(&terr)?;
    let mut graphfile = File::create("geodata/terr.json")?;
    let _ = graphfile.write(json.as_bytes());

    Ok(())
}

fn load_geo() -> Result<(Vec<f64>, (usize, usize), (f64, f64, f64, f64, f64, f64)), Box<dyn std::error::Error>> {
    let mut terr_file = File::open("geodata/geo.data")?;
    let mut meta_file = File::open("geodata/geometa.json")?;

    let mut meta_json = String::new();
    meta_file.read_to_string(&mut meta_json).expect("Unable to read file");
    let meta: ((f64, f64, f64, f64, f64, f64), (usize, usize)) = serde_json::from_str(&meta_json)?;
    let geotransform = meta.0;
    let size= meta.1;

    let mut t_encoded = Vec::new();
    let _ = terr_file.read_to_end(&mut t_encoded);
    let terr: Vec<f64> = bincode::deserialize(&t_encoded)?;

    // let dataset = Dataset::open(Path::new("geo.tif"))?;
    // let raster_band: RasterBand = dataset.rasterband(1)?;
    // let terrbuffer: Buffer<f64> = 
    //     raster_band.read_as::<f64>((0, 0), 
    //     (dataset.raster_size().0, dataset.raster_size().1), 
    //     (dataset.raster_size().0, dataset.raster_size().1), 
    //         None
    //     )?;

    Ok((terr, size, geotransform))
}

fn world_to_pixel(geo_transform: &(f64, f64, f64, f64, f64, f64), latitude: f64, longitude: f64) -> Result<(isize, isize), Box<dyn std::error::Error>> {
    let pixel_x = (longitude - geo_transform.0) / geo_transform.1;
    let pixel_y = (latitude - geo_transform.3) / geo_transform.5;
    Ok((pixel_x.round() as isize, pixel_y.round() as isize))
}

fn line_of_sight(point_a: (i32, i32, i32), point_b: (i32, i32, i32), terrain: &Vec<f64>, size: (usize, usize)) -> Result<(bool, Vec<(i32, i32)>), Box<dyn std::error::Error>> {

    let cpoint_a: (i32, i32, i32) = (point_a.0, point_a.1, point_a.2);
    let cpoint_b: (i32, i32, i32) = (point_b.0, point_b.1, point_b.2);
    
    let mut hit_total = false;
    let mut terr_: Vec<(i32, i32)> = vec![];
    println!("{}", (cpoint_a.0-cpoint_b.0).abs()*(cpoint_a.1-cpoint_b.1).abs()*(cpoint_a.2-cpoint_b.2).abs());
    
    let start =  SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    let vector = (cpoint_b.0 - cpoint_a.0, cpoint_b.1 - cpoint_a.1, cpoint_b.2 - cpoint_a.2);
    let max_dir = max(max(vector.0, vector.1), vector.2);
     
    let max_range: RangeInclusive<i32>;
    let path_function: &dyn Fn(i32,&(i32, i32, i32),&(i32, i32, i32)) -> f32;
    if max_dir == vector.0 {path_function = &|a, vector, point| {((a-point.0) as f32)/(vector.0 as f32) as f32}; max_range=min(cpoint_a.0, cpoint_b.0)..=max(cpoint_a.0, cpoint_b.0);}
    else if max_dir == vector.1 {path_function = &|a, vector, point| {((a-point.1) as f32)/(vector.1 as f32) as f32}; max_range=min(cpoint_a.1, cpoint_b.1)..=max(cpoint_a.1, cpoint_b.1);}
    else if max_dir == vector.2 {path_function = &|a, vector, point| {((a-point.2) as f32)/(vector.2 as f32) as f32}; max_range=min(cpoint_a.2, cpoint_b.2)..=max(cpoint_a.2, cpoint_b.2);}
    else {return Ok((false, Vec::new()))}

    for a in max_range {
        let k = path_function(a, &vector, &cpoint_a);
        let x = (cpoint_a.0 as f32 + k*(vector.0 as f32)).round() as i32;
        let y = (cpoint_a.1 as f32 + k*(vector.1 as f32)).round() as i32;
        let z = (cpoint_a.2 as f32 + k*(vector.2 as f32)).round() as i32;

        let i = (size.0 as i32)*y + x;
        // println!("x={} y={} z={} i={}", x, y, z, i);
        let terrain_height = terrain[i as usize] as usize;
        let hit = z as usize <= terrain_height;
        // print!("hit={} terr={} ", hit, terrain_height);
        // if hit {hit_total = true; println!("<<<<<<<<<<<<<<<<<<<<<<<<<<")}
        // else {println!("")}
        if hit {hit_total = true;}
        terr_.push((terrain_height as i32, z));
    }
    // 
    let end =  SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    println!("Took {}ms", end - start);

    Ok((!hit_total, terr_))
}


// fn calculate_path_x(x: i32, vector: (i32, i32, i32), start: (i32, i32, i32)) -> (i32, i32, i32) {
//     let k = (x-start.0) as f32/(vector.0 as f32) as f32;
//     let x = (start.0 as f32 + k*(vector.0 as f32)).round() as i32;
//     let y = (start.1 as f32 + k*(vector.1 as f32)).round() as i32;
//     let z = (start.2 as f32 + k*(vector.2 as f32)).round() as i32;
//     return (x, y, z)
// }

// fn calculate_path(point: Vector3<f64>, vector_dir: Vector3<f64>, vector_start: Vector3<f64>) -> bool {
//     let line_to_point = point - vector_start;
//     let cross_prod = line_to_point.cross(&vector_dir);
//     let distance = cross_prod.norm() / vector_dir.norm();
//     let margin = 4 as f64;
//     distance <= margin
// }

// fn calculate_path(x: i32, y: i32, z: i32, vector: (i32, i32, i32), start: (i32, i32, i32)) -> bool {
//     let mut ks: Vec<f32> = Vec::new();
//     if vector.0 != 0 {ks.push((((x-start.0) as f32)/(vector.0 as f32)) as f32)}
//     if vector.1 != 0 {ks.push((((y-start.1) as f32)/(vector.1 as f32)) as f32)}
//     if vector.2 != 0 {ks.push((((z-start.2) as f32)/(vector.2 as f32)) as f32)}
//     ks.iter().all(|&k| k == ks[0])
// }

//--------------------

// fn _get_altitude(pixel_x: isize, pixel_y: isize, dataset: &Dataset ) -> Result<i64, Box<dyn std::error::Error>> {
//     let raster_band: RasterBand = dataset.rasterband(1)?;
//     let buffer: Buffer<f64> = raster_band.read_as::<f64>((pixel_x, pixel_y), (1, 1), (1, 1), None)?;
//     let altitude = buffer.data[0];
//     return Ok(altitude.round() as i64);
// }

    // for x in min(cpoint_a.0, cpoint_b.0)..=max(cpoint_a.0, cpoint_b.0) {
    //     if x % resolution_divider != 0 {continue;}
    //     for y in min(cpoint_a.1, cpoint_b.1)..=max(cpoint_a.1, cpoint_b.1) {
    //         if y % resolution_divider != 0 {continue;}
    //         for z in min(cpoint_a.2, cpoint_b.2)..=max(cpoint_a.2, cpoint_b.2) {
    //             // let in_path = calculate_path(
    //             //     Vector3::new(x as f64, y as f64, z as f64), 
    //             //     Vector3::new((cpoint_b.0 - cpoint_a.0) as f64, (cpoint_b.1 - cpoint_a.1) as f64, (cpoint_b.2 - cpoint_a.2) as f64), 
    //             //     Vector3::new(cpoint_a.0 as f64, cpoint_a.1 as f64, cpoint_a.2 as f64));
    //             // if !in_path {continue;}

    //             let (x_, y_, z_) = calculate_path(x, y, z, vector, cpoint_a);
    //             let i = (size.0 as i32)*y_ + x_;
    //             // println!("x={} y={} z={} i={}", x, y, z, i);
    //             let terrain_height = terrain[i as usize] as usize;
    //             let hit = z_ as usize <= terrain_height;
    //             // print!("hit={} terr={} ", hit, terrain_height);
    //             // if hit {hit_total = true; println!("<<<<<<<<<<<<<<<<<<<<<<<<<<")}
    //             // else {println!("")}
    //             if hit {hit_total = true;}
    //             terr_.push((terrain_height as i32, z));
    //         }
    //     }

