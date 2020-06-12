mod camera;
mod geometry;

use std::fs::File;
use std::io::Write;
use std::time::SystemTime;

use crate::camera::{CameraSettings, Canvas, ResolutionGate};
use crate::geometry::{edge, Matrix, Vec3};

fn convert_to_raster(
    v_world: &Vec3,
    world_to_camera: &Matrix,
    canvas: &Canvas,
    near: f64,
    image_w: usize,
    image_h: usize,
) -> Vec3 {
    let p_camera = world_to_camera.mul(v_world);
    let p_screen = Vec3 {
        x: p_camera.x / -p_camera.z * near,
        y: p_camera.y / -p_camera.z * near,
        z: 0.0,
    };
    let p_ndc = Vec3 {
        x: 2.0 * p_screen.x / (canvas.right - canvas.left)
            - (canvas.right + canvas.left) / (canvas.right - canvas.left),
        y: 2.0 * p_screen.y / (canvas.top - canvas.bottom)
            - (canvas.top + canvas.bottom) / (canvas.top - canvas.bottom),
        z: 0.0,
    };

    // to raster space
    Vec3 {
        x: (p_ndc.x + 1.0) / 2.0 * image_w as f64,
        // in raster space y is down so invert
        y: (1.0 - p_ndc.y) / 2.0 * image_h as f64,
        z: -p_camera.z,
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let camera = CameraSettings {
        resolution_gate: ResolutionGate::FILL,
        focal_length: 5,
        film_aperture_width: 0.980,
        film_aperture_height: 0.735,
        near_clipping_plane: 1.0,
        far_clipping_plan: 1000.0,
    };
    let canvas = camera.scale_canvas(WIDTH, HEIGHT);
    let mut frame_buffer = [(255, 255, 255); 640 * 480]; // TODO: fixme
    let mut depth_buffer = [camera.far_clipping_plan; 640 * 480]; // TODO: fixme
    let world_to_camera = Matrix {
        x: [1.0, 0.0, 0.0, 0.0],
        y: [0.0, 1.0, 0.0, 0.0],
        z: [0.0, 0.0, 1.0, -2.0],
        w: [0.0, 0.0, 0.0, 1.0],
    };

    let v0 = Vec3 {
        x: -48.0,
        y: -10.0,
        z: 82.0,
    };
    let v1 = Vec3 {
        x: 29.0,
        y: -15.0,
        z: 44.0,
    };
    let v2 = Vec3 {
        x: 13.0,
        y: 34.0,
        z: 114.0,
    };

    let start = SystemTime::now();

    let mut v0_raster = convert_to_raster(
        &v0,
        &world_to_camera,
        &canvas,
        camera.near_clipping_plane,
        WIDTH,
        HEIGHT,
    );
    let mut v1_raster = convert_to_raster(
        &v1,
        &world_to_camera,
        &canvas,
        camera.near_clipping_plane,
        WIDTH,
        HEIGHT,
    );
    let mut v2_raster = convert_to_raster(
        &v2,
        &world_to_camera,
        &canvas,
        camera.near_clipping_plane,
        WIDTH,
        HEIGHT,
    );

    // precompute reciprocal of vertex z coord
    v0_raster.z = 1.0 / v0_raster.z;
    v1_raster.z = 1.0 / v1_raster.z;
    v2_raster.z = 1.0 / v2_raster.z;

    let _xmin = min(v0_raster.x, min(v1_raster.x, v2_raster.x));
    let _ymin = min(v0_raster.y, min(v1_raster.y, v2_raster.y));
    let _xmax = max(v0_raster.x, max(v1_raster.x, v2_raster.x));
    let _ymax = max(v0_raster.y, max(v1_raster.y, v2_raster.y));

    let area = edge(&v0_raster, &v1_raster, &v2_raster);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel_sample = Vec3 {
                x: x as f64 + 0.5,
                y: y as f64 + 0.5,
                z: 0.0,
            };
            let mut w0 = edge(&v1_raster, &v2_raster, &pixel_sample);
            let mut w1 = edge(&v2_raster, &v0_raster, &pixel_sample);
            let mut w2 = edge(&v0_raster, &v1_raster, &pixel_sample);

            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                w0 = w0 / area;
                w1 = w1 / area;
                w2 = w2 / area;

                // interpolate z
                let z = 1.0 / (v0_raster.z * w0 + v1_raster.z * w1 + v2_raster.z * w2);

                let dx = y * WIDTH + x;
                if z < depth_buffer[dx] {
                    depth_buffer[dx] = z;

                    let v0_cam = world_to_camera.mul(&v0);
                    let v1_cam = world_to_camera.mul(&v1);
                    let v2_cam = world_to_camera.mul(&v2);

                    let px = (v0_cam.x / -v0_cam.z) * w0
                        + (v1_cam.x / -v1_cam.z) * w1
                        + (v2_cam.x / -v2_cam.z) * w2;
                    let py = (v0_cam.y / -v0_cam.z) * w0
                        + (v1_cam.y / -v1_cam.z) * w1
                        + (v2_cam.y / -v2_cam.z) * w2;

                    let pt = Vec3 {
                        x: px * z,
                        y: py * z,
                        z: -z,
                    };

                    let mut n = (v1_cam - v0_cam.clone()).cross(&(v2_cam - v0_cam));
                    let mut view_direction = Vec3 {
                        x: pt.x,
                        y: pt.y,
                        z: pt.z,
                    };

                    n.normalize();
                    view_direction.normalize();

                    let n_dot_view = max(0.0, n.dot(&view_direction));
                    let color = (n_dot_view * 255.0).floor() as u8;
                    frame_buffer[dx] = (color, color, color);
                }
            }
        }
    }

    let mut result_s = String::from("P3\n640 480\n255\n");

    for i in frame_buffer.iter() {
        result_s.push_str(&i.0.to_string());
        result_s.push_str(" ");
        result_s.push_str(&i.1.to_string());
        result_s.push_str(" ");
        result_s.push_str(&i.2.to_string());
        result_s.push_str("\n");
    }

    match File::create("img.ppm") {
        Ok(mut f) => f.write_all(&result_s.into_bytes()), // TODO: Fix me
        Err(e) => panic!("Problem opening file: {:?}", e),
    }.unwrap();
    println!("Done in {:?}", start.elapsed().unwrap());
}
