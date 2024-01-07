//! Module for our Peak Panic game

use yew::prelude::*;
use yew_router::prelude::*;

/// This is the page for our Peak Panic game
/// Its a portolio page for the game
/// It will contain a description of the game
/// and images, videos
#[function_component(CubesTraveller)]
pub fn peak_panic() -> Html {
    let images = vec![
        "img/cubes_traveller/game/splash.png".to_string(),
        "img/cubes_traveller/game/cloud.jpg".to_string(),
        "img/cubes_traveller/game/dusk.jpg".to_string(),
        "img/cubes_traveller/game/night.jpg".to_string(),
        "img/cubes_traveller/game/pumpkin_night.png".to_string(),
        "img/cubes_traveller/game/leaning.jpg".to_string(),
    ];

    html! {
         <div class="container mx-auto flex flex-col my-10 px-4 sm:px-6 lg:px-8 items-center justify-items-center">
            <h1 class="text-4xl my-10 font-extrabold text-center tracking-tight sm:text-5xl lg:text-6xl">{"Cube's Traveller"}</h1>



            // <Carousel images={images} />
            <div class="carousel w-full">
                <div id="slide1" class="carousel-item relative w-full">
                    <img src="img/cubes_traveller/game/splash.png" class="w-full" />
                    <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                    <a href="#slide4" class="btn btn-circle">{"❮"}</a>
                    <a href="#slide2" class="btn btn-circle">{"❯"}</a>
                    </div>
                </div>
                <div id="slide2" class="carousel-item relative w-full">
                    <img src="img/cubes_traveller/game/cloud.jpg" class="w-full" />
                    <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                    <a href="#slide1" class="btn btn-circle">{"❮"}</a>
                    <a href="#slide3" class="btn btn-circle">{"❯"}</a>
                    </div>
                </div>
                <div id="slide3" class="carousel-item relative w-full">
                    <img src="img/cubes_traveller/game/dusk.jpg" class="w-full" />
                    <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                    <a href="#slide2" class="btn btn-circle">{"❮"}</a>
                    <a href="#slide4" class="btn btn-circle">{"❯"}</a>
                    </div>
                </div>
                <div id="slide4" class="carousel-item relative w-full">
                    <img src="img/cubes_traveller/game/night.jpg" class="w-full" />
                    <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                    <a href="#slide3" class="btn btn-circle">{"❮"}</a>
                    <a href="#slide5" class="btn btn-circle">{"❯"}</a>
                    </div>
                </div>
                <div id="slide5" class="carousel-item relative w-full">
                    <img src="img/cubes_traveller/game/pumpkin_night.png" class="w-full" />
                    <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                    <a href="#slide4" class="btn btn-circle">{"❮"}</a>
                    <a href="#slide6" class="btn btn-circle">{"❯"}</a>
                    </div>
                </div>
                <div id="slide6" class="carousel-item relative w-full">
                    <img src="img/cubes_traveller/game/leaning.jpg" class="w-full" />
                    <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                    <a href="#slide5" class="btn btn-circle">{"❮"}</a>
                    <a href="#slide1" class="btn btn-circle">{"❯"}</a>
                    </div>
                </div>
            </div>
        </div>
    }
}
