extern crate alloc;

mod api_types;
mod httpget;
mod uistate;

fn main() {
    let _uistate = crate::uistate::UiState::from_interwebs().unwrap();
}
