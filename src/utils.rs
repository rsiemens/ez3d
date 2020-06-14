use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

use crate::geometry::Vec3;

#[derive(Debug)]
pub struct PolygonCollection {
    verts: Vec<Vec<f64>>,
    indicies: Vec<usize>,
    cursor: usize,
}

impl PolygonCollection {
    pub fn from_obj(fpath: &str) -> Self {
        let f = File::open(fpath).expect(&format!("Failed to open {}", fpath));
        let reader = BufReader::new(f);
        let mut polygons = Self {
            verts: Vec::new(),
            indicies: Vec::new(),
            cursor: 0,
        };

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(' ').collect();

            if (parts.len() == 4) && (parts[0] == "v") {
                let vecs = parts[1..]
                    .iter()
                    .map(|v| v.parse::<f64>().unwrap())
                    .collect();
                polygons.verts.push(vecs);
            } else if (parts.len() == 4) && (parts[0] == "f") {
                for t in parts[1..].iter() {
                    let indx: Vec<&str> = t.split("/").collect();
                    polygons
                        .indicies
                        .push(indx[0].parse::<usize>().unwrap() - 1);
                }
            }
        }

        polygons
    }

    pub fn len(&self) -> usize {
        self.indicies.len() / 3
    }
}

impl Iterator for PolygonCollection {
    type Item = Vec<Vec3>;

    fn next(&mut self) -> Option<Self::Item> {
        let verts = &self.verts;
        let indicies = &self.indicies;

        if self.cursor < self.indicies.len() {
            let triangle = vec![
                Vec3 {
                    x: verts[indicies[self.cursor]][0],
                    y: verts[indicies[self.cursor]][1],
                    z: verts[indicies[self.cursor]][2],
                },
                Vec3 {
                    x: verts[indicies[self.cursor + 1]][0],
                    y: verts[indicies[self.cursor + 1]][1],
                    z: verts[indicies[self.cursor + 1]][2],
                },
                Vec3 {
                    x: verts[indicies[self.cursor + 2]][0],
                    y: verts[indicies[self.cursor + 2]][1],
                    z: verts[indicies[self.cursor + 2]][2],
                },
            ];
            self.cursor += 3;
            Some(triangle)
        } else {
            None
        }
    }
}

pub fn min<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b {
        a
    } else {
        b
    }
}

pub fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a > b {
        a
    } else {
        b
    }
}
