
mod terrain_filter;

use noise::*;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder, ColorGradient, ImageRenderer};
use std::path::Path;
use terrain_filter::*;

#[allow(unused_variables)]
fn main() {
    //Generation properties
    let seed:u32 = 3;
    let length = 5.0;
    let resolution = 1000;

    // Domain warp properties
    let warp_strength: f64 = 1.5;
    let warp_freq: f64 = 1.0;

    // Available filters
    let perlin_noise = Perlin::new(seed);
    let fbm_perlin_noise = Fbm::<Perlin>::new(seed + 2);
    let worley_noise = Worley::new(seed + 13);
    let fbm_worley_noise = Fbm::<Worley>::new(seed + 129);
    let billow_noise = Billow::<Perlin>::new(seed + 864);

    // Take the absolute and re-scale to add back the negative part, then flip the height map
    let abs_perlin_noise = Abs::new(&fbm_perlin_noise);
    let reversed_perlin_noise = Multiply::new(Add::new(&abs_perlin_noise, Constant::new(-0.2)), Constant::new(-2.0));

    // Create the DomainWarp instance with generics
    let perlin_warp_noise = DomainWarp::<_, _, 2>::new(&fbm_perlin_noise, &fbm_perlin_noise, warp_strength, warp_freq);
    let worley_warp_noise = DomainWarp::<_, _, 2>::new(fbm_worley_noise, &fbm_perlin_noise, warp_strength, warp_freq);
    let billow_warp_noise = DomainWarp::<_, _, 2>::new(billow_noise, &fbm_perlin_noise, warp_strength, warp_freq);
    let reversed_perlin_warp_noise = DomainWarp::<_, _, 2>::new(&reversed_perlin_noise, &fbm_perlin_noise, warp_strength, warp_freq);

    let perlin_sum = Multiply::new(Add::new(&reversed_perlin_noise, &perlin_warp_noise), Constant::new(0.5));
    let worley_billow_sum = Multiply::new(Add::new(&worley_warp_noise, &billow_warp_noise), Constant::new(0.5));
    let terrain = Multiply::new(Add::new(&perlin_sum, &worley_billow_sum), Constant::new(0.5));
    //let terrain = Add::new(&terrain, Constant::new(0.2));
    let terrain_wrapper = TerrainWrapper::new(&billow_warp_noise);

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
