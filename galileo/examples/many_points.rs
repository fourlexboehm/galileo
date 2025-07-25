//! This examples shows the map performance when rendering millions of points over it.

use galileo::layer::feature_layer::symbol::Symbol;
use galileo::layer::feature_layer::{Feature, FeatureLayer};
use galileo::layer::raster_tile_layer::RasterTileLayerBuilder;
use galileo::render::point_paint::PointPaint;
use galileo::render::render_bundle::RenderBundle;
use galileo::{Color, Map, MapBuilder};
use galileo_types::cartesian::Point3;
use galileo_types::geo::Crs;
use galileo_types::geometry::Geom;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run()
}

pub(crate) fn run() {
    galileo_egui::InitBuilder::new(create_map())
        .init()
        .expect("failed to initialize");
}

struct ColoredPoint {
    point: Point3,
    color: Color,
}

impl Feature for ColoredPoint {
    type Geom = Point3;

    fn geometry(&self) -> &Self::Geom {
        &self.point
    }
}

struct ColoredPointSymbol {}
impl Symbol<ColoredPoint> for ColoredPointSymbol {
    fn render(
        &self,
        feature: &ColoredPoint,
        geometry: &Geom<Point3>,
        min_resolution: f64,
        bundle: &mut RenderBundle,
    ) {
        if let Geom::Point(point) = geometry {
            bundle.add_point(point, &PointPaint::dot(feature.color), min_resolution);
        }
    }
}

fn generate_points() -> Vec<ColoredPoint> {
    const LEVELS: u32 = 100;
    let phi = std::f64::consts::PI * (5f64.sqrt() - 1.0);
    let mut points = vec![];

    for level in 1..=LEVELS {
        let points_count = level * level * 10;
        let radius = 50_000.0 * level as f64;

        let color = ((level - 1) as f32 / (LEVELS - 1) as f32 * 150.0) as u8;
        let color = Color::rgba(255, color, 0, 150);

        for i in 0..points_count {
            let z = 1.0 - (i as f64 / (points_count - 1) as f64);
            let rel_radius = (1.0 - z * z).sqrt();
            let theta = phi * i as f64;
            let x = theta.cos() * rel_radius;
            let y = theta.sin() * rel_radius;

            let point = Point3::new(x * radius, y * radius, z * radius);
            points.push(ColoredPoint { point, color });
        }
    }

    println!("Generated {} points", points.len());

    points
}

fn create_map() -> Map {
    let raster_layer = RasterTileLayerBuilder::new_osm()
        .with_file_cache_checked(".tile_cache")
        .build()
        .expect("failed to create layer");

    let feature_layer = FeatureLayer::new(generate_points(), ColoredPointSymbol {}, Crs::EPSG3857);

    MapBuilder::default()
        .with_layer(raster_layer)
        .with_layer(feature_layer)
        .build()
}
