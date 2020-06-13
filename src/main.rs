mod camera;
mod geometry;
mod utils;

use std::env;
use std::fs::File;
use std::io::Write;
use std::process;
use std::time::SystemTime;

use crate::camera::{CameraSettings, Canvas, ResolutionGate};
use crate::geometry::{edge, Matrix, Vec3};
use crate::utils::{max, min, PolygonCollection};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

// Get the bounding box coords around a triangle
fn bounding_box(tri: &Vec<Vec3>) -> (usize, usize, usize, usize) {
    let xmin = min(tri[0].x, min(tri[1].x, tri[2].x));
    let ymin = min(tri[0].y, min(tri[1].y, tri[2].y));
    let xmax = max(tri[0].x, max(tri[1].x, tri[2].x));
    let ymax = max(tri[0].y, max(tri[1].y, tri[2].y));

    let x0 = max(0, xmin.floor() as usize);
    let x1 = min(WIDTH - 1, xmax.floor() as usize) + 1;
    let y0 = max(0, ymin.floor() as usize);
    let y1 = min(HEIGHT - 1, ymax.floor() as usize) + 1;

    (x0, x1, y0, y1)
}

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

    Vec3 {
        x: (p_ndc.x + 1.0) / 2.0 * image_w as f64,
        y: (1.0 - p_ndc.y) / 2.0 * image_h as f64, // in raster space y is down so invert
        z: 1.0 / -p_camera.z,                      // precompute reciprocal
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ez3d <obj file>");
        process::exit(1);
    }

    let camera = CameraSettings {
        resolution_gate: ResolutionGate::FILL,
        focal_length: 22.0,
        film_aperture_width: 0.980,
        film_aperture_height: 0.735,
        near_clipping_plane: 1.0,
        far_clipping_plan: 1000.0,
    };
    let world_to_camera = Matrix {
        x: [1.0, 0.0, 0.0, 0.0],
        y: [0.0, 1.0, 0.0, 0.0],
        z: [0.0, 0.0, 1.0, -2.0],
        w: [0.0, 0.0, 0.0, 1.0],
    };
    let canvas = camera.scale_canvas(WIDTH, HEIGHT);
    let mut frame_buffer = [(255, 255, 255); WIDTH * HEIGHT];
    let mut depth_buffer = [camera.far_clipping_plan; WIDTH * HEIGHT];
    let polygons = PolygonCollection::from_obj(&args[1]);

    println!("Rendering {} triangles", polygons.len());
    let start = SystemTime::now();
    for triangle in polygons {
        let raster: Vec<Vec3> = triangle
            .iter()
            .map(|v| {
                convert_to_raster(
                    &v,
                    &world_to_camera,
                    &canvas,
                    camera.near_clipping_plane,
                    WIDTH,
                    HEIGHT,
                )
            })
            .collect();

        let area = edge(&raster[0], &raster[1], &raster[2]);
        let (xmin, xmax, ymin, ymax) = bounding_box(&raster);

        for y in ymin..ymax {
            for x in xmin..xmax {
                let pixel_sample = Vec3 {
                    x: x as f64 + 0.5,
                    y: y as f64 + 0.5,
                    z: 0.0,
                };
                let mut w0 = edge(&raster[1], &raster[2], &pixel_sample);
                let mut w1 = edge(&raster[2], &raster[0], &pixel_sample);
                let mut w2 = edge(&raster[0], &raster[1], &pixel_sample);

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    w0 = w0 / area;
                    w1 = w1 / area;
                    w2 = w2 / area;

                    // interpolate z depth in raster space
                    let z = 1.0 / (raster[0].z * w0 + raster[1].z * w1 + raster[2].z * w2);

                    let dx = y * WIDTH + x;
                    if z < depth_buffer[dx] {
                        depth_buffer[dx] = z;

                        let v0_cam = world_to_camera.mul(&triangle[0]);
                        let v1_cam = world_to_camera.mul(&triangle[1]);
                        let v2_cam = world_to_camera.mul(&triangle[2]);

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
                            x: -pt.x,
                            y: -pt.y,
                            z: -pt.z,
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
        Ok(mut f) => f.write_all(&result_s.into_bytes()),
        Err(e) => panic!("Problem opening file: {:?}", e),
    }
    .unwrap();
    println!("Done in {:?}", start.elapsed().unwrap());
}
