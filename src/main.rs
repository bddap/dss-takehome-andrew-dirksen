extern crate alloc;

mod api_types;
mod httpget;
mod uistate;

fn main() {
    let _home = httpget::home();
    let _home = httpget::get_set("");
}
