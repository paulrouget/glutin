#[cfg(target_os = "android")]
#[macro_use]
extern crate android_glue;

extern crate glutin;

use std::io;

mod support;

#[cfg(target_os = "android")]
android_start!(main);

#[cfg(not(feature = "window"))]
fn main() { println!("This example requires glutin to be compiled with the `window` feature"); }

#[cfg(feature = "window")]
fn main() {
    // enumerating monitors
    let monitor = {
        for (num, monitor) in glutin::get_available_monitors().enumerate() {
            println!("Monitor #{}: {:?}", num, monitor.get_name());
        }

        print!("Please write the number of the monitor to use: ");

        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let num = num.trim().parse().ok().expect("Please enter a number");
        let monitor = glutin::get_available_monitors().nth(num).expect("Please enter a valid ID");

        println!("Using {:?}", monitor.get_name());

        monitor
    };

    let window = glutin::WindowBuilder::new()
        .with_gl_profile(glutin::GlProfile::Compatibility)
        .with_title("Hello world!".to_string())
        .with_fullscreen(monitor)
        .build()
        .unwrap();

    unsafe { window.make_current() };

    
    let context = support::load(&window);

    for event in window.wait_events() {
        context.draw_frame((0.0, 1.0, 0.0, 1.0));
        window.swap_buffers();

        println!("{:?}", event);

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
