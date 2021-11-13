use async_process::{Command, Output};
use async_std::task;
use rand::Rng;
use serde::Deserialize;
use std::time::Duration;
use std::{fs, io, io::Read};

const TEMP: &str = "/home/slicer/Plotbot/Server/temp";

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
    Adaptivecubic,
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
    fn args(self, run_id: &str) -> String {
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
            format!("--output {}/output-{}.gcode", TEMP, run_id),
            format!("{}/model-{}.3mf", TEMP, run_id),
        ]
        .join(" ")
    }
}

async fn openscad(svg: &str, run_id: &str) -> Result<Output, io::Error> {
    println!("Processing OPENSCAD ID:{}", run_id);
    fs::write(format!("temp/drawing-{}.svg", run_id), svg)?;
    let output = Command::new("openscad")
        .current_dir("temp")
        .arg(format!("-o model-{}.3mf", run_id))
        .arg(format!("-D id={}", run_id))
        .arg("../openscad/convert.scad")
        .output()
        .await;
    return output;
}

async fn superslice(options: SlicerOptions, run_id: &str) -> std::io::Result<std::process::Output> {
    println!("Processing SUPERSLICER ID:{}", run_id);
    Command::new("sh")
        .arg("superslicer/superslice.sh")
        .env("SARGS", options.args(run_id))
        .output()
        .await
}

async fn read_gcode(run_id: &str) -> Result<String, io::Error> {
    println!("Reading gcode");
    let mut data = String::new();
    let mut f = fs::File::open(format!("temp/output-{}.gcode", run_id))?;
    f.read_to_string(&mut data).expect("Unable to read GCode");
    Ok(data)
}

fn gen_id() -> String {
    let num = rand::thread_rng().gen_range(1..999);
    format!("{}", num)
}

// Remove all files in temp containg the run_id
fn purge_temp(run_id: &str) {
    let files = fs::read_dir(TEMP).expect("No temp dir!");
    for file in files {
        let file = file.unwrap();
        if file.file_name().to_str().unwrap().contains(run_id) {
            fs::remove_file(file.path()).unwrap();
        }
    }
}

async fn wait_for_model(run_id: &str) {
    fn check_for_model(run_id: &str) -> bool {
        let files = fs::read_dir(TEMP).expect("No temp dir!");
        for file in files {
            let file_name = file.unwrap().file_name();
            let file_name = file_name.to_str().unwrap();
            if file_name.contains(run_id) && file_name.contains("model") {
                return true;
            }
        }
        return false;
    }

    while !check_for_model(run_id) {
        task::sleep(Duration::from_millis(70)).await;
    }
    fs::rename(
        format!("temp/ model-{}.3mf", run_id),
        format!("temp/model-{}.3mf", run_id),
    )
    .expect("Rename failed!");
}

pub async fn slice(options: SlicerOptions) -> Result<String, io::Error> {
    let run_id = gen_id();
    // println!("{:?}", openscad(&options.svg, &run_id).await?);
    let openscad_res = openscad(&options.svg, &run_id).await?;
    if openscad_res.status.code() != Some(0) {
        let err = String::from_utf8(openscad_res.stderr)
            .unwrap()
            .replace("/home/slicer/Plotbot/Server", "");
        return Err(io::Error::new(io::ErrorKind::Other, err));
    }

    wait_for_model(&run_id).await;

    let superslicer_res = superslice(options, &run_id).await?;
    if superslicer_res.status.code() != Some(0) {
        let err = String::from_utf8(superslicer_res.stderr)
            .unwrap()
            .replace("/home/slicer/Plotbot/Server", "");
        return Err(io::Error::new(io::ErrorKind::Other, err));
    }

    let gcode = read_gcode(&run_id).await?;
    purge_temp(&run_id);
    Ok(gcode)
}
