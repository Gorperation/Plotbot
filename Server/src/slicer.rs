use serde::Deserialize;
use std::{
    fs, io,
    io::Read,
    process::{Command, Output},
};

const GCODE_OUTPUT: &str = "superslicer/OpenSCAD Model.gcode";
const SCAD_OUTPUT: &str = "superslicer/model.3mf";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Fills {
    Rectilinear,
    Monotonic,
    Grid,
    Triangles,
    Stars,
    Line,
    Honeycomb,
    Hexagonal,
    Gyroid,
    Hilbertcurve,
    Archimedeanchords,
    Octagramspiral,
    Scatteredrectilinear,
}
impl Fills {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[derive(Debug, Deserialize)]
pub struct SlicerOptions {
    svg: String,
    fill_density: f32,
    fill_pattern: Fills,
    fill_connected: bool,
    fill_overlap: f32,
    fill_angle: i32,
    fill_speed: i32,
    perimeters: i32,
    perimeter_speed: i32, // perimeter_width: f32,
}
impl SlicerOptions {
    fn args(self) -> String {
        [
            format!("-g"),
            format!("--load slicer-config.ini"),
            format!("--fill-density {}%", self.fill_density * 100.0),
            format!("--fill-pattern {}", self.fill_pattern.to_string()),
            format!(
                "--infill-connection {}",
                if self.fill_connected {
                    "connected"
                } else {
                    "notconnected"
                }
            ),
            format!("--infill-overlap {}%", self.fill_overlap * 100.0),
            format!("--fill-angle {}", self.fill_angle),
            format!("--infill-speed {}", self.fill_speed),
            format!("--perimeters {}", self.perimeters),
            format!("--perimeter-speed {}", self.perimeter_speed),
            SCAD_OUTPUT.to_string(),
        ]
        .join(" ")
    }
}

fn openscad(svg: &str) -> Result<Output, io::Error> {
    println!("Processing OPENSCAD");
    fs::write("openscad/drawing.svg", svg)?;
    Command::new("openscad")
        .arg(format!("-o{}", SCAD_OUTPUT))
        .arg("openscad/convert.scad")
        .output()
}

fn superslice(options: SlicerOptions) -> std::io::Result<std::process::Output> {
    println!("Processing SUPERSLICER");
    Command::new("sh")
        .arg("superslicer/superslice.sh")
        .env("SARGS", options.args())
        .output()
}

async fn read_gcode() -> Result<String, io::Error> {
    println!("Reading gcode");
    let mut data = String::new();
    let mut f = fs::File::open(GCODE_OUTPUT)?;
    f.read_to_string(&mut data).expect("Unable to read string");
    Ok(data)
}

pub async fn slice(options: SlicerOptions) -> Result<String, io::Error> {
    let openscad_output = openscad(&options.svg)?;
    if openscad_output.stderr.len() > 0 {
        let err = format!("OpenSCAD Error {:?}", openscad_output.stderr);
        println!("{}", err);
        return Err(io::Error::new(io::ErrorKind::Other, err));
    }
    let superslicer_output = superslice(options)?;
    if superslicer_output.stderr.len() > 0 {
        let err = format!("SuperSlicer Error {:?}", superslicer_output.stderr);
        println!("{}", err);
        return Err(io::Error::new(io::ErrorKind::Other, err));
    }
    Ok(read_gcode().await?)
}
