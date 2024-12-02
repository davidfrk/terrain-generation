
mod terrain_filter;


use noise::utils::{NoiseMapBuilder, PlaneMapBuilder, ColorGradient, ImageRenderer};
use std::path::Path;
use terrain_filter::*;

#[allow(unused_variables)]
fn main() {
    //Generation properties
    let seed:u32 = 3978;
    let length = 5.0;
    let resolution = 1000;

    let terrain = generate_terrain(seed);
    let terrain_wrapper = TerrainWrapper::new(&terrain);

    // Build a noise map using PlaneMapBuilder with the wrapped noise function
    let noise_map = PlaneMapBuilder::new(terrain_wrapper)  // Use wrapped noise function
        .set_x_bounds(-length/2.0, length/2.0)  // Set the x-axis bounds
        .set_y_bounds(-length/2.0, length/2.0)  // Set the y-axis bounds
        .set_size(resolution, resolution)  // Set the resolution of the noise map
        .build();

    // Generate and save height map
    let height_map = ImageRenderer::new().render(&noise_map);
    height_map.write_to_file(Path::new("height_map.png"));

    // Generate and save colored map
    let image = ImageRenderer::new()
        .set_gradient(ColorGradient::new().build_terrain_gradient())
        .render(&noise_map);
    image.write_to_file(Path::new("terrain.png"));
}
