#![windows_subsystem = "windows"]

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[tokio::main]
async fn main() -> wry::Result<()> {
    println!("Launching Vortex");

    let event_loop = EventLoop::new();

    // these are what gets injected during runtime
    let inject = include_str!("javascript/inject.js");
    let search = include_str!("javascript/search.js");
    let shader = include_str!("javascript/shader.js");
    let css = include_str!("style.css");

    let script = format!(r#"(() => {{{} {} {} run(`{}`)}})();"#, inject, search, shader, css); 

    let window = WindowBuilder::new()
        .with_title("Vortex Plus")
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new()
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