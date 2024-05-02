use clap::Parser;
use input::event::switch::SwitchState;
use input::event::SwitchEvent::Toggle;
use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;
use std::process::ExitCode;

struct Interface;

#[derive(Parser)]
struct Cli {
    /// The event pseudofile (e.g. /dev/input/event7)
    #[arg(short, long)]
    event_file: String,
}

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

fn main() -> ExitCode {
    let args = Cli::parse();
    let dev_file: &str = &args.event_file[..];

    let mut input = Libinput::new_from_path(Interface);
    input.path_add_device(dev_file);

    loop {
        input.dispatch().unwrap();

        for event in &mut input {
            if let input::event::Event::Switch(switch_event) = event {
                if let Toggle(toggle_event) = switch_event {
                    if toggle_event.switch_state() == SwitchState::On {
                        println!("Toggling to ON");
                    } else {
                        println!("Toggling to OFF");
                    }
                } else {
                    println!("Switch SubType is not supported");
                }
            } else {
                println!("Type is not supported");
            }
        }
    }
    return ExitCode::from(42);
}
