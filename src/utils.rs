use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct PolygonCollection {
    pub verts: Vec<Vec<f64>>,
    pub indicies: Vec<usize>,
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

// Load the verts and faces of an .obj file
pub fn obj_loader(fpath: &str) -> PolygonCollection {
    let f = File::open(fpath).expect(&format!("Failed to open {}", fpath));
    let reader = BufReader::new(f);
    let mut polygons = PolygonCollection {
        verts: Vec::new(),
        indicies: Vec::new(),
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
                polygons.indicies.push(indx[0].parse::<usize>().unwrap() - 1);
            }
        }
    }

    polygons
}
