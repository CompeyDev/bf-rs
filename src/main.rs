mod interpreter;

use interpreter::{core, utils};

fn main() {
    let res = utils::strip_code(">++>+<-_REAL");

    core::BrainfuckInstance::new().load_string(res); // should be [ 0, 1, 1 ]
}
