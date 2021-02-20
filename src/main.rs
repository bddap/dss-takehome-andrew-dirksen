mod api_types;
mod httpget;

fn main() {
    let _home = httpget::home();
    let _home = httpget::get_set("");
}
