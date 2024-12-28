pub mod components;
pub mod constants;
pub mod entity;
pub mod features;
pub mod systems;

fn main() {
    features::app::create_app();
}
