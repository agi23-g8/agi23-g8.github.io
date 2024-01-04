use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]

pub enum Route {
    #[at("/")]
    Home,
    #[at("/peak-panic-game")]
    PeakPanicGame,
    #[at("/cubes-traveller-game")]
    CubesTravellerGame,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
        <div class="flex flex-col">
            <Switch<Route> render={switch} />
        </div>
        </HashRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{"Home"}</h1> },
        Route::PeakPanicGame => html! { <h1>{"Peak Panic Game"}</h1> },
        Route::CubesTravellerGame => html! { <h1>{"Cubes Traveller Game"}</h1> },
    }
}
