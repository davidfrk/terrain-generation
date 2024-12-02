
mod terrain_filter;


use noise::utils::{NoiseMapBuilder, PlaneMapBuilder, ColorGradient, ImageRenderer};
use std::path::Path;
use terrain_filter::*;

#[allow(unused_variables)]
fn main() {
    //Generation properties
    let seed:u32 = 3;
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

    

    let image = ImageRenderer::new()
        .set_gradient(ColorGradient::new().build_terrain_gradient())
        .render(&noise_map);
    

    // Save the noise map to a file
    image.write_to_file(Path::new("terrain.png"));
}
