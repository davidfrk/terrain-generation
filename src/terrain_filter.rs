use noise::NoiseFn;

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

pub struct TerrainWrapper<'a> {
  terrain: &'a dyn NoiseFn<f64, 2>, // Use a reference to a trait object
}

impl<'a> TerrainWrapper<'a> {
  pub fn new(terrain: &'a dyn NoiseFn<f64, 2>) -> Self {
      TerrainWrapper { terrain }
  }
}

impl<'a> NoiseFn<f64, 3> for TerrainWrapper<'a> {
  fn get(&self, point: [f64; 3]) -> f64 {
      self.terrain.get([point[0], point[1]]) // Use only the first two coordinates
  }
}