use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use serde::Serialize;
use tauri::{State, Window};

use crate::a2m8_config::A2;

// fn body_from_value<T: serde::Serialize>(value: T) -> Body {
//     let body = serde_json::to_string(&value).unwrap();
//     Body::from(body)
// }

#[derive(Serialize, Clone)]
pub struct CreateWithPromptPayload {
    pub name: String,
    pub content: String,
}

fn create_res(body: Body) -> Response<Body> {
    let mut res = Response::new(body);
    *res.status_mut() = hyper::StatusCode::OK;

    res.headers_mut().insert(
        "Access-Control-Allow-Origin",
        hyper::header::HeaderValue::from_static("*"),
    );
    res.headers_mut().insert(
        "Access-Control-Allow-Methods",
        hyper::header::HeaderValue::from_static("GET, POST, OPTIONS"),
    );

    res
}

async fn handle_req(req: Request<Body>, state: A2, window: Window) -> crate::Result<Response<Body>> {
    let uri = req.uri().clone();
    let res = match (uri.path(), req.method()) {
        ("/", &hyper::Method::GET) => {
            let state = state.lock().await;
            let body = serde_json::to_string(&state.scripts)?;
            let mut response = create_res(Body::from(body));
            *response.status_mut() = hyper::StatusCode::OK;
            return Ok(response);
        }
        ("/new", &hyper::Method::POST) => {
            let name = uri.query().unwrap_or("unknown.lua");

            let body = hyper::body::to_bytes(req.into_body()).await?;
            let body = String::from_utf8(body.to_vec())?;

            window.emit(
                "create_w_prompt",
                CreateWithPromptPayload {
                    name: name.to_string(),
                    content: body,
                },
            )?;

            let mut response = create_res(Body::empty());
            *response.status_mut() = hyper::StatusCode::CREATED;
            return Ok(response);
        }
        _ => Ok(create_res(Body::empty())),
    };
    res
}

pub async fn start_web(window: Window, state: A2) -> crate::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 5836));

    // And a MakeService to handle each connection...

    let make_service = make_service_fn(move |_client: &AddrStream| {
        // let ip = client.remote_addr();
        let state = state.clone();
        let window = window.clone();
        async move {
            // This is the request handler.
            Ok::<_, Infallible>(service_fn(move |req| {
                let state = state.clone();
                let window = window.clone();
                async {
                    let res = handle_req(req, state, window).await;
                    if res.is_err() {
                        let mut response = create_res(Body::empty());
                        *response.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
                        Ok(response)
                    } else {
                        res
                    }
                }
            }))
        }
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
