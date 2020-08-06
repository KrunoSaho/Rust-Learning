// Add (crate-type = ["cdylib"]) to Cargo.toml, and under [lib]

use krogue_simulation;


#[macro_use]
extern crate gdnative;

// use gdnative::GlobalConstants;

#[derive(gdnative::NativeClass)]
#[inherit(gdnative::Node2D)]
struct Main {
    observer: krogue_simulation::Observer,
}

#[gdnative::methods]
impl Main {
    fn _init(_owner: gdnative::Node2D) -> Self {
        Main {
            observer: krogue_simulation::Observer {
                print: |x| {
                    godot_print!("{}", x);
                },
            },
        }
    }

    #[export]
    fn _ready(&mut self, _owner: gdnative::Node2D) {}

    #[export]
    fn _process(&mut self, _owner: gdnative::Node2D, dt: f32) {
        krogue_simulation::run(&self.observer, dt);
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<Main>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
