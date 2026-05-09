#![windows_subsystem = "windows"]

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebViewBuilder, http};
use std::borrow::Cow;

#[tokio::main]
async fn main() -> wry::Result<()> {
    println!("Launching Vortex");

    let event_loop = EventLoop::new();

    // these are what gets injected during runtime
    let inject = include_str!("javascript/inject.js");
    let search = include_str!("javascript/search.js");
    let shader = include_str!("javascript/shader.js");
    let maploader = include_str!("javascript/maploader.js");
    let css = include_str!("style.css");

    let script = format!(r#"(() => {{{inject} {search} {shader} {maploader} run(`{css}`)}})();"#);

    let window = WindowBuilder::new()
        .with_title("Vortex Plus")
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new()
        .with_custom_protocol("app".into(), |_webview_id, request| {
            let uri = request.uri().to_string();

            if uri.ends_with("js/engine.js") {
                let body: Cow<'static, [u8]> =
                    Cow::Borrowed(include_bytes!("javascript/newEngine.js"));

                return http::Response::builder()
                    .status(200)
                    .header("Content-Type", "application/javascript")
                    .body(body)
                    .unwrap();
            }

            http::Response::builder()
                .status(404)
                .header("Content-Type", "text/plain; charset=utf-8")
                .body(Cow::Owned(b"not found".to_vec()))
                .unwrap()
                
        })
        .with_url("https://vortex.towerstats.com/")
        .with_initialization_script(&script)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
