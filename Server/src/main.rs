use tide::{Request, Response, StatusCode};
// use sha2::Sha256;
// use hmac::{Hmac, Mac, NewMac};
// use tide::prelude::*;

mod slicer;

async fn index(_req: Request<()>) -> tide::Result {
    Ok("Plotbot API!".into())
}

async fn gcode(mut req: Request<()>) -> Result<Response, tide::Error> {
    println!("Slicing GCode");
    let body_maybe: Result<slicer::SlicerOptions, tide::Error> = req.body_json().await;
    let body = match body_maybe {
        Ok(body) => body,
        Err(e) => {
            return Ok(Response::builder(StatusCode::UnprocessableEntity)
                .body(e.to_string())
                .build());
        }
    };

    let gcode_maybe = slicer::slice(body).await;
    let gcode = match gcode_maybe {
        Ok(gcode) => gcode,
        Err(e) => {
            return Ok(Response::builder(StatusCode::InternalServerError)
                .body(e.to_string())
                .build())
        }
    };

    Ok(gcode.into())
}

async fn ghpost(mut req: Request<()>) -> tide::Result {
    let body = req.body_json().await?;
    println!("Pulling from Github, body: {:?}", body);
    let _token = std::env::var("GH_TOKEN")?;
    // let signature = format!("sha1={}", Sha256::digest(body.as_bytes()).to_string());
    // if body.
    println!("{:?}", body);

    Ok("Ok, pulling".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Starting...");
    let mut app = tide::new();
    app.at("/").get(index);
    println!("Listening on port 8080");
    app.at("/cam").post(gcode);
    app.at("/ghpush").post(ghpost);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
