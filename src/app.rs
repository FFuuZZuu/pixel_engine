use crate::ui::UI;
use log::*;
use pixels::{Pixels, SurfaceTexture};
use simple_logger::SimpleLogger;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

type UpdateFn = dyn Fn(&mut UI);

pub struct App {
    update: Option<Box<UpdateFn>>,
    width: usize,
    height: usize,
    scale: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            update: None,
            width: 256,
            height: 256,
            scale: 1,
        }
    }

    pub fn with_update(mut self, update: &'static UpdateFn) -> Self {
        self.update = Some(Box::new(update));
        self
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width as usize;
        self.height = height as usize;
        self
    }

    pub fn with_scale(mut self, scale: u32) -> Self {
        self.scale = scale as usize;
        self
    }

    pub fn run(self) -> Result<(), pixels::Error> {
        SimpleLogger::new().with_colors(true).init().unwrap();

        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("pixel_engine")
            .with_inner_size(PhysicalSize::new(
                (self.width * self.scale) as f64,
                (self.height * self.scale) as f64,
            ))
            .build(&event_loop)
            .unwrap();

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(self.width as u32, self.height as u32, surface_texture)?
        };

        let mut ui = UI::new(self.width as usize, self.height as usize);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            if let Event::RedrawRequested(_) = event {
                if let Some(x) = &self.update {
                    x(&mut ui);
                }
                ui.draw(pixels.get_frame());
                if pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
