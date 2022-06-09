use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use vlc::{self, EventType, Instance, Media, MediaPlayer, State};
use winit::{
    event::{ElementState, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};

#[derive(Debug)]
enum UserEvent {
    MediaStopped,
}

fn main() {
    // Create window
    let event_loop = EventLoop::<UserEvent>::with_user_event();

    let window = WindowBuilder::new()
        .with_title("vlc-rs sample")
        .with_inner_size(winit::dpi::PhysicalSize::new(800u16, 600u16))
        .build(&event_loop)
        .unwrap();

    // Instanciate libvlc
    let instance = Instance::new().unwrap();

    let args: Vec<String> = std::env::args().collect();
    let path = match args.get(1) {
        Some(s) => s,
        None => {
            println!("Usage: winit_player path_to_a_media_file");
            return;
        }
    };
    let md = Media::new_path(&instance, path).unwrap();

    let mdp = MediaPlayer::new(&instance).unwrap();

    // Request libvlc to draw in the window
    match window.raw_window_handle() {
        RawWindowHandle::AppKit(handle) => {
            mdp.set_nsobject(handle.ns_view);
        }
        RawWindowHandle::Xlib(handle) => {
            mdp.set_xwindow(handle.window as u32);
        }
        RawWindowHandle::Win32(handle) => {
            mdp.set_hwnd(handle.hwnd);
        }
        _ => {
            panic!("Unknown Window handle type")
        }
    }

    // Link EventLoop and libvlc
    let proxy = event_loop.create_proxy();

    let em = md.event_manager();
    let _ = em.attach(EventType::MediaStateChanged, move |e, _| match e {
        vlc::Event::MediaStateChanged(s) => {
            println!("State : {:?}", s);
            if s == State::Ended || s == State::Error || s == State::Stopped {
                proxy.send_event(UserEvent::MediaStopped).unwrap();
            }
        }
        _ => (),
    });

    // Start
    mdp.set_media(&md);
    mdp.play().unwrap();

    // Event loop
    let mut fullscreen = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: _,
            } => {
                mdp.stop();
            }
            winit::event::Event::UserEvent(UserEvent::MediaStopped) => {
                *control_flow = ControlFlow::Exit;
            }
            winit::event::Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if input.state != ElementState::Pressed {
                    return;
                }

                match input.virtual_keycode {
                    Some(VirtualKeyCode::F) => {
                        let next_fullscreen_mode = if fullscreen {
                            None
                        } else {
                            Some(Fullscreen::Borderless(None))
                        };

                        window.set_fullscreen(next_fullscreen_mode);

                        fullscreen = !fullscreen;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}
