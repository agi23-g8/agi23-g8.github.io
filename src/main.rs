mod app;
mod cubes_traveller;
mod header;
mod peak_panic;
mod team;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
