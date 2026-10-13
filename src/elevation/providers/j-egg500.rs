use std::fs;

use crate::coordinate_system::geographic::LLBBox;
use crate::elevation::provider::RawElevationGrid;

struct JeggPoint {
    lat: f64,
    lon: f64,
    depth_m: f64,
}

fn load_jegg500(path: &str) -> Result<Vec<JeggPoint>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;

    let mut points = Vec::new();

    for line in text.lines() {
        let values: Vec<&str> = line.split_whitespace().collect();

        if values.len() != 4 {
            continue;
        }

        let lat: f64 = values[1].parse()?;
        let lon: f64 = values[2].parse()?;
        let depth: f64 = values[3].parse()?;

        points.push(JeggPoint {
            lat,
            lon,
            depth_m: -depth,
        });
    }

    Ok(points)
}

fn points_to_grid(
    points: &[JeggPoint],
    bbox: &LLBBox,
    grid_width: usize,
    grid_height: usize,
) -> RawElevationGrid {
    let min_lat = bbox.min().lat();
    let max_lat = bbox.max().lat();
    let min_lon = bbox.min().lng();
    let max_lon = bbox.max().lng();

    let mut heights = vec![vec![f64::NAN; grid_width]; grid_height];

    if points.is_empty() || grid_width == 0 || grid_height == 0 {
        return RawElevationGrid {
            heights_meters: heights,
        };
    }

    let lat_range = max_lat - min_lat;
    let lon_range = max_lon - min_lon;

    if lat_range <= 0.0 || lon_range <= 0.0 {
        return RawElevationGrid {
            heights_meters: heights,
        };
    }

    for point in points {
        if point.lat < min_lat
            || point.lat > max_lat
            || point.lon < min_lon
            || point.lon > max_lon
        {
            continue;
        }

        let x = ((point.lon - min_lon) / lon_range
            * (grid_width - 1) as f64)
            .round() as usize;

        let y = ((max_lat - point.lat) / lat_range
            * (grid_height - 1) as f64)
            .round() as usize;

        heights[y][x] = point.depth_m;
    }

    RawElevationGrid {
        heights_meters: heights,
    }
}

pub struct Jegg500;

impl crate::elevation::provider::ElevationProvider for Jegg500 {
    fn name(&self) -> &'static str {
        "jegg500"
    }

    fn coverage_bboxes(&self) -> Option<Vec<LLBBox>> {
        Some(vec![
            LLBBox::new(34.0, 137.0, 34.80875, 137.99997).unwrap(),
        ])
    }

    fn native_resolution_m(&self) -> f64 {
        500.0
    }

    fn fetch_raw(
        &self,
        bbox: &LLBBox,
        grid_width: usize,
        grid_height: usize,
    ) -> Result<RawElevationGrid, Box<dyn std::error::Error>> {
        let points = load_jegg500("mesh500_34_137(2).txt")?;

        Ok(points_to_grid(
            &points,
            bbox,
            grid_width,
            grid_height,
        ))
    }
}


