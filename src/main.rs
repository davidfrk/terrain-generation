use noise::*;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder, ColorGradient, ImageRenderer};
use std::path::Path;

pub struct DomainWarp<WarpNoise, PrimaryNoise> {
    warp_strength: f64,
    warp_freq: f64,
    warp_noise: WarpNoise,
    primary_noise: PrimaryNoise,
}

impl<WarpNoise, PrimaryNoise> DomainWarp<WarpNoise, PrimaryNoise>
where
    WarpNoise: NoiseFn<f64, 2>,
    PrimaryNoise: NoiseFn<f64, 2>,
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

impl<WarpNoise, PrimaryNoise> NoiseFn<f64, 3> for DomainWarp<WarpNoise, PrimaryNoise>
where
    WarpNoise: NoiseFn<f64, 2>,
    PrimaryNoise: NoiseFn<f64, 2>,
{
    fn get(&self, point: [f64; 3]) -> f64 {
        let x = point[0];
        let y = point[1];

        let offset = 100.0;
        // Warp the coordinates using the warp noise function
        let warp_x = x + self.warp_noise.get([self.warp_freq * x, self.warp_freq * y]) * self.warp_strength;
        let warp_y = y + self.warp_noise.get([self.warp_freq * x + offset, self.warp_freq * y + offset]) * self.warp_strength;

        // Use the warped coordinates in the primary noise function
        self.primary_noise.get([warp_x, warp_y])
    }
}

pub struct Scalar{
    value: f64,
}

impl Scalar{
    pub fn new(value:f64) -> Self {
        Scalar {
            value,
        }
    }
}

impl NoiseFn<f64, 3> for Scalar {
    fn get(&self, _point: [f64; 3]) -> f64 {
        self.value
    }
}

pub struct ReduceTo2D<'a, Noise> {
    noise: &'a Noise,
    fixed_z: f64,
}

impl<'a, Noise> ReduceTo2D<'a, Noise> {
    pub fn new(noise: &'a Noise, fixed_z: f64) -> Self {
        Self { noise, fixed_z }
    }
}

impl<'a, Noise> NoiseFn<f64, 2> for ReduceTo2D<'a, Noise>
where
    Noise: NoiseFn<f64, 3>,
{
    fn get(&self, point: [f64; 2]) -> f64 {
        // Extend the 2D point to 3D by adding the fixed_z coordinate
        self.noise.get([point[0], point[1], self.fixed_z])
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

    let abs_perlin_noise: Abs<f64, &Fbm<Perlin>, 3> = Abs::new(&fbm_perlin_noise);
    let reversed_perlin_noise = Multiply::new(Add::new(&abs_perlin_noise, Scalar::new(-0.2)), Scalar::new(-2.0));

    // Reduce reversed_perlin_noise to 2D
    let reduced_perlin_noise = ReduceTo2D::new(&reversed_perlin_noise, 0.0);

    // Create the DomainWarp instance with generics
    let perlin_warp_noise = DomainWarp::new(&fbm_perlin_noise, &fbm_perlin_noise, warp_strength, warp_freq);
    let worley_warp_noise = DomainWarp::new(fbm_worley_noise, &fbm_perlin_noise, warp_strength, warp_freq);
    let billow_warp_noise = DomainWarp::new(billow_noise, &fbm_perlin_noise, warp_strength, warp_freq);
    let reversed_perlin_warp_noise = DomainWarp::new(reduced_perlin_noise, &fbm_perlin_noise, warp_strength, warp_freq);

    let terrain = Add::new(perlin_warp_noise, Scalar::new(0.2));

    // Build a noise map using PlaneMapBuilder with the wrapped noise function
    let noise_map = PlaneMapBuilder::new(terrain)  // Use wrapped noise function
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
