#![windows_subsystem = "windows"]

extern crate reqwest;

use std::borrow::Cow;
use std::path::Path;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebViewBuilder, WebViewBuilderExtWindows, http};

use std::{env, fs, io};

pub fn read_mods() -> io::Result<Vec<String>> {
    let exe = env::current_exe()?;
    let dir = exe.parent().unwrap();

    let mods = dir.join("mods");
    let mut files = Vec::new();

    for entry in fs::read_dir(mods)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let content = fs::read_to_string(path)?;
            files.push(content);
        }
    }
    Ok(files)
}

fn main() -> wry::Result<()> {
    println!("Launching Vortex");

    let event_loop = EventLoop::new();

    // these are what gets injected during runtime
    let overrider = include_str!("Vortex2+2/javascript/overrider.js");
    let search = include_str!("Vortex2+2/javascript/search.js");
    let maploader = include_str!("Vortex2+2/javascript/maploader.js");
    let inject = include_str!("Vortex2+2/javascript/inject.js");
    let css = include_str!("Vortex2+2/style.css");

    let mods = read_mods().unwrap_or_else(|_| std::process::exit(0));
    let mods_script = mods
        .into_iter()
        .map(|m| {
            format!(r#"try {{{}}} catch (error) {{console.warn(error);}}"#,m)}).collect::<String>();

    let script = format!(
        r#"(() => {{
        try {{{overrider}}} catch (error) {{ console.warn(error); }}
        try {{{search}}} catch (error) {{ console.warn(error); }}
        try {{{maploader}}} catch (error) {{ console.warn(error); }}
        try {{{inject}}} catch (error) {{ console.warn(error); }}

        {mods_script} /* from mods */

        run(`{css}`)
        }})();"#
    );

    let window = WindowBuilder::new()
        .with_title("Vortex AIS")
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new()
        .with_https_scheme(true)
        .with_custom_protocol("v22".into(), |_id, request| {
            let uri = request.uri().to_string();
            let file = uri.trim_start_matches("v22://").trim_end_matches('/');
            let body = match std::fs::read(format!("src/Vortex2+2/{}", file)) {
                Ok(v) => v,
                Err(_) => {
                    return http::Response::builder()
                        .status(404)
                        .body(Cow::Owned(b"not found".to_vec()))
                        .unwrap();
                }
            };
            let ctype = match Path::new(file).extension().and_then(|e| e.to_str()) {
                Some("js") => "application/javascript",
                Some("css") => "text/css",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                _ => "text/plain",
            };
            http::Response::builder()
                .status(200)
                .header("Content-Type", ctype)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "*")
                .header("Cache-Control", "no-cache")
                .body(Cow::Owned(body))
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
