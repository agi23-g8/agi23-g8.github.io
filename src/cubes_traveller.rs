//! Module for our Peak Panic game

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{carousel::Carousel, gallery::Gallery};

/// This is the page for our Peak Panic game
/// Its a portolio page for the game
/// It will contain a description of the game
/// and images, videos
#[function_component(CubesTraveller)]
pub fn cubes_traveller() -> Html {
    let images = vec![
        "img/cubes_traveller/game/splash.png".to_string(),
        "img/cubes_traveller/game/cloud.jpg".to_string(),
        "img/cubes_traveller/game/dusk.jpg".to_string(),
        "img/cubes_traveller/game/night.jpg".to_string(),
        "img/cubes_traveller/game/pumpkin_night.png".to_string(),
        "img/cubes_traveller/game/leaning.jpg".to_string(),
    ];

    html! {
        <>
        <div class="container mx-auto flex flex-col my-10 px-4 sm:px-6 lg:px-8 items-center justify-items-center">
            <h1 class="text-4xl my-5 font-extrabold text-center tracking-tight sm:text-5xl lg:text-8xl">{"Cube's Traveller"}</h1>
            <p class="text-xl font-light my-3 text-center tracking-tight">{"A cozy, cooperative puzzle game for two players"}</p>
            // Image of the game
            <figure class="flex flex-col items-center">
                <img class="rounded-lg shadow-lg" src="img/cubes_traveller/game/splash.png" alt="Cube's Traveller" />
            </figure>

            <h2 class="text-2xl my-5 font-extrabold text-center  tracking-tight sm:text-3xl lg:text-5xl">{"A new way to play together"}</h2>

            <p class="text-xl font-light my-3 text-justify max-w-prose tracking-tight">{"Cube’s Traveller is a cooperative game where you are supposed to work together in an environment which is visually stunning, but challenges your world view. In this game, one player is controlling the world, which is a cube, and the other player controls a character on the cube. Our goal is to create a new interactive experience without the often disappointing experiences of VR/AR, and instead focus on different interaction technologies. Additionally, a major goal of our game is to make it visually beautiful to help players immerse themselves into the game world. We thought that previous projects have lacked the visual fidelity we want to achieve."}</p>

            <p class="text-xl font-light my-3 text-justify max-w-prose tracking-tight">{"The motivation for a cooperative game was simple, we think that playing games with a friend is really fun, therefore it was very obvious to select it as the main project focus. With the limited time and ambitious graphical goals, we limited ourselves to a simple, yet effective, art style where models and textures are basic but lighting is complex and beautiful. We believe that this helped us stand out from the rest and create a unique experience."}</p>

            // vertical space
            <div class="h-10"></div>

            <Gallery images={images} />
        </div>


        </>
    }
}
