#[allow(dead_code)]
pub enum ResolutionGate {
    FILL,
    OVERSCAN,
}

pub struct Canvas {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

pub struct CameraSettings {
    pub resolution_gate: ResolutionGate,
    pub focal_length: i32,
    pub film_aperture_width: f64,
    pub film_aperture_height: f64,
    pub near_clipping_plane: f64,
    pub far_clipping_plan: f64,
}

impl CameraSettings {
    fn canvas_coords(&self) -> (f64, f64) {
        let inch_to_mm = 25.4;
        let top = (self.film_aperture_height * inch_to_mm / 2.0) / self.focal_length as f64
            * self.near_clipping_plane;
        let right = (self.film_aperture_width * inch_to_mm / 2.0) / self.focal_length as f64
            * self.near_clipping_plane;

        (top, right)
    }

    pub fn scale_canvas(&self, device_width: usize, device_height: usize) -> Canvas {
        let film_aspect_ratio = self.film_aperture_width / self.film_aperture_height;
        let device_aspect_ratio = device_width as f64 / device_height as f64;
        let mut xscale = 1.0;
        let mut yscale = 1.0;

        match self.resolution_gate {
            ResolutionGate::FILL => {
                if film_aspect_ratio > device_aspect_ratio {
                    xscale = device_aspect_ratio / film_aspect_ratio;
                } else {
                    yscale = film_aspect_ratio / device_aspect_ratio;
                }
            }
            ResolutionGate::OVERSCAN => {
                if film_aspect_ratio > device_aspect_ratio {
                    yscale = film_aspect_ratio / device_aspect_ratio;
                } else {
                    xscale = device_aspect_ratio / film_aspect_ratio;
                }
            }
        }

        let (mut top, mut right) = self.canvas_coords();
        top = top * yscale;
        right = right * xscale;
        let bottom = -top;
        let left = -right;

        Canvas {
            top,
            bottom,
            left,
            right,
        }
    }
}
