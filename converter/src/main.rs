use bincode;
use gdal::Dataset;
use gdal::raster::{Buffer, RasterBand};
use gdal::GeoTransform;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dataset = Dataset::open(Path::new("geo.tif"))?;
    let raster_band: RasterBand = dataset.rasterband(1)?;
    let terrbuffer: Buffer<f64> = 
        raster_band.read_as::<f64>((0, 0), 
        (dataset.raster_size().0, dataset.raster_size().1), 
        (dataset.raster_size().0, dataset.raster_size().1), 
            None
        )?;
    let mut terr_file = File::create("geo.data")?;
    let _ = terr_file.write_all(&bincode::serialize(&terrbuffer.data)?);
    let geo_transform: GeoTransform = dataset.geo_transform()?;
    let mut geotransform_file = File::create("geometa.json")?;
    let json: String = serde_json::to_string(&((geo_transform[0], geo_transform[1], geo_transform[2], geo_transform[3], geo_transform[4], geo_transform[5]), terrbuffer.size))?;
    let _ = geotransform_file.write_all(json.as_bytes());
    Ok(())
}
