#[macro_use]
extern crate conrod_core;
extern crate conrod_glium;
extern crate glium;

use conrod_core::{ widget, color, Colorable, Borderable, Sizeable, Positionable, Labelable, Widget, UiBuilder };
use glium::{ Surface, Display };
use glium::glutin::{ EventsLoop, WindowBuilder, ContextBuilder, ControlFlow, Event };
use conrod_glium::Renderer;

widget_ids! (
    struct Ids {
        button,

    }    
);

const Title: &'static str = "Calculator App";
const WIDTH: u32 = 300;
const HEIGHT: u32 = 300;

fn main() {
    let mut event_loop = EventsLoop::new();
    let window = WindowBuilder::new()
    .with_title(Title)
    .with_dimensions((WIDTH, HEIGHT).into());

    let context = ContextBuilder::new()
    .with_vsync(true)
    .with_multisampling(4);
    let display = Display::new(window, context, &event_loop).unwrap();
    let mut ui = UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    let ids = &mut Ids::new(ui.widget_id_generator());
    let renderer = Renderer::new(&display).unwrap();

    let mut events: Vec<Event> = Vec::new();

    'render: loop {
        events.clear();

        event_loop.poll_events( |event| {
            events.push(event);
        })

        if events.is_empty() {
            event_loop.run_forever( |event| {
                events.push(event);
                ControlFlow::Break
            })
        }

        for event in events.drain(..) {
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::CloseRequested | 
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            }
                        } => break 'render,  
                        _ => (),                      
                    },
                    _ => (),
                },
                _ => (),
            }
        }
    }
}