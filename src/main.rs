mod app;
mod cubes_traveller;
mod peak_panic;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
