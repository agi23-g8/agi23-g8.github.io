mod app;
mod carousel;
mod cubes_traveller;
mod gallery;
mod header;
mod peak_panic;
mod team;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
