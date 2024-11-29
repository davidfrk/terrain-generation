use noise::*;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder, ColorGradient, ImageRenderer};
use std::path::Path;

pub struct DomainWarp<WarpNoise, PrimaryNoise, const DIM: usize> {
    warp_strength: f64,
    warp_freq: f64,
    warp_noise: WarpNoise,
    primary_noise: PrimaryNoise,
}

impl<WarpNoise, PrimaryNoise, const DIM:usize> DomainWarp<WarpNoise, PrimaryNoise, DIM>
where
    WarpNoise: NoiseFn<f64, DIM>,
    PrimaryNoise: NoiseFn<f64, DIM>,
{
    // Initialize the struct with generics for the noise functions
    pub fn new(primary_noise: PrimaryNoise, warp_noise: WarpNoise, warp_strength: f64, warp_freq:f64) -> Self {
        DomainWarp {
            warp_strength,
            warp_freq,
            warp_noise,
            primary_noise,
        }
    }
}

impl<const DIM:usize, WarpNoise, PrimaryNoise> NoiseFn<f64, DIM> for DomainWarp<WarpNoise, PrimaryNoise, DIM>
where
    WarpNoise: NoiseFn<f64, DIM>,
    PrimaryNoise: NoiseFn<f64, DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        let mut warped_point = point;

        // Apply domain warping only to the first two dimensions if `DIM` is at least 2
        if DIM >= 2 {
            let offset = 100.0;

            // Scale and warp the first dimension
            warped_point[0] += self.warp_noise.get({
                let mut warp_input = point;
                warp_input.iter_mut().for_each(|v| *v *= self.warp_freq);
                warp_input
            }) * self.warp_strength;

            // Scale and warp the second dimension
            warped_point[1] += self.warp_noise.get({
                let mut warp_input = point;
                warp_input.iter_mut().for_each(|v| *v *= self.warp_freq);
                warp_input[0] += offset; // Add offset to the first dimension
                warp_input[1] += offset; // Add offset to the second dimension
                warp_input
            }) * self.warp_strength;
        }

        // Use the warped coordinates in the primary noise function
        self.primary_noise.get(warped_point)
    }
}

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
    let fbm_perlin_noise = Fbm::<Perlin>::new(seed);
    let worley_noise = Worley::new(seed);
    let fbm_worley_noise = Fbm::<Worley>::new(seed);
    let billow_noise = Billow::<Perlin>::new(seed);

    // Take the absolute and re-scale to add back the negative part, then flip the height map
    let abs_perlin_noise: Abs<f64, &Fbm<Perlin>, 3> = Abs::new(&fbm_perlin_noise);
    let reversed_perlin_noise = Multiply::new(Add::new(&abs_perlin_noise, Constant::new(-0.2)), Constant::new(-2.0));

    // Create the DomainWarp instance with generics
    let perlin_warp_noise = DomainWarp::<_, _, 2>::new(&fbm_perlin_noise, &fbm_perlin_noise, warp_strength, warp_freq);
    let worley_warp_noise = DomainWarp::<_, _, 2>::new(fbm_worley_noise, &fbm_perlin_noise, warp_strength, warp_freq);
    let billow_warp_noise = DomainWarp::<_, _, 2>::new(billow_noise, &fbm_perlin_noise, warp_strength, warp_freq);
    let reversed_perlin_warp_noise = DomainWarp::new(&reversed_perlin_noise, &fbm_perlin_noise, warp_strength, warp_freq);

    let terrain = Add::new(&perlin_warp_noise, Constant::new(0.2));

    // Build a noise map using PlaneMapBuilder with the wrapped noise function
    let noise_map = PlaneMapBuilder::new(reversed_perlin_warp_noise)  // Use wrapped noise function
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
