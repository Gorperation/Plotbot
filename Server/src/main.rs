use tide::{Request, Response, StatusCode};
// use tide::prelude::*;

mod slicer;

async fn index(_req: Request<()>) -> tide::Result {
    Ok("Plotbot API!".into())
}

pub async fn gcode(mut req: Request<()>) -> Result<Response, tide::Error> {
    println!("Processing gcode");
    let body_maybe: Result<slicer::SlicerOptions, tide::Error> = req.body_json().await;
    let body = match body_maybe {
        Ok(body) => body,
        Err(e) => {
            return Ok(Response::builder(StatusCode::UnprocessableEntity)
                .body(e.to_string())
                .build());
        },
    };
    
    let gcode_maybe = slicer::slice(body).await;
    let gcode = match gcode_maybe {
        Ok(gcode) => gcode,
        Err(e) => {
            return Ok(Response::builder(StatusCode::InternalServerError)
                .body(e.to_string())
                .build())
        },
    };

    Ok(gcode.into())
}

#[async_std::main]
async fn main() ->tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(index);
    app.at("/cam").post(gcode);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}