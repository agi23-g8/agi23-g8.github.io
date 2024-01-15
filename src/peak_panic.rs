//! Module for our Peak Panic game

use yew::prelude::*;
use yew_router::prelude::*;

use crate::gallery::Gallery;

/// This is the page for our Peak Panic game
/// Its a portolio page for the game
/// It will contain a description of the game
/// and images, videos
#[function_component(PeakPanic)]
pub fn peak_panic() -> Html {
    let images = vec![
        "img/peak_panic/pp_environment_1.jpg",
        "img/peak_panic/pp_environment_2.jpg",
        "img/peak_panic/pp_environment_3.jpg",
        "img/peak_panic/pp_environment_4.jpg",
        "img/peak_panic/pp_environment_5.jpg",
        "img/peak_panic/pp_gameplay_1.png",
        "img/peak_panic/pp_gameplay_2.png",
        "img/peak_panic/splash.png",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    html! {
        <>
        <div class="container mx-auto flex flex-col my-6 px-4 sm:px-6 lg:px-8 items-center justify-items-center">
            <h1 class="text-4xl my-5 font-extrabold text-center tracking-tight sm:text-5xl lg:text-8xl">{"Peak Panic"}</h1>
            <p class="text-xl my-3 text-center tracking-tight">{"An easily accessible, fast-paced, and fun racing game where you compete against your friends"}</p>
            // Image of the game
            <figure class="flex flex-col items-center">
                <img class="rounded-lg shadow-lg" src="img/peak_panic/splash.png" alt="Cube's Traveller" />
            </figure>

            <h2 class="text-2xl my-5 font-extrabold text-center tracking-tight sm:text-3xl lg:text-5xl">{"A new way to play together"}</h2>

            <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                {"Peak Panic is a local multiplayer skiing game where you have to keep up the pace so you don't fall behind! Each player controls a character using their own phone by tilting it left and right."}
                <br/><br/>
                {"A player is eliminated when going out of view of the camera. The camera will follow the leading player with a slight delay for letting players behind catch up. The slope is filled with obstacles such as trees and rolling snowballs that you have to avoid. Additionally, there are speed boosts placed around the slope that a player can pick up in order to gain an advantage. Let the fastest skier win!"}
                <br/><br/>
                {"Our goal with the project was to create an accessible multiplayer experience where anyone with a phone could join and play. The motivation for this was we felt our previous project lacked the accessibility we wanted. It required a Xbox controller and a phone with a specific app we built. Instead, we wanted to make a game where everyone can play using the hardware they carry around in their pocket everyday."}
            </p>

            <Gallery images={images} />

            <div class="w-full my-10">
                <iframe class="w-full" height="720" src="https://www.youtube.com/embed/bw2AXm7WpNk?si=jdedPxbdXF9k8MEk" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
            </div>

            // Text about the game
            <div class="lg:columns-2 gap-10">
                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Interaction"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    {"The user input pipeline begins by hosting a WebGL Unity build on a github web page. The WebGL version simply enables the accelerometer and samples it. Additionally, it sets up a network connection for sending the sensor data through Unity Relay to the game server. The benefit of hosting the WebGL build on a public github web page is that it’s free, secure (HTTPS) and publicly available (anyone can connect with internet access). We had to use HTTPS for both accessing the accelerometer and sending the data. For example, an iPhone with the Safari web browser requires HTTPS for allowing a web page to access its sensor. "}
                    <br/><br/>
                    {"By sampling the accelerometer through the Unity Input System we can detect how much the user is tilting their phone. Figure 1 illustrates the movement. A roll movement along the Z axis, as the blue circular arrow indicates, will result in a positive or negative X value by the accelerometer. This enables us to detect when to make the player character carve left and right. This is the only input to the game."}
                    <br/><br/>
                    {"The justification for this approach was, to put simply: this is the only way we could get this to work in our limited time frame. And why was that? The key technology which enabled all of this was Unity Relay services. This was the only way we could get a network connecting from the phone’s web browser to the game. This is also why the web page hosts a Unity WebGL build. You could use plain JavaScript for sampling the accelerometer but that requires some other networking solution with secure WebSockets."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Graphics"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    <span class="font-bold">{"PBR Texturing and Noise-Based Snow Depth"}</span>
                    <br/>
                    {"The snow shader leverages Physically Based Rendering (PBR) texturing to achieve realistic lighting effects on the snow surface. Controlling the base depth of the snow cover is straightforward with a float parameter. To prevent a flat appearance, an additional noise texture is introduced, allowing for per-vertex variation in the snow depth."}
                    <br/><br/>

                    <span class="font-bold">{"World-Based UVs"}</span>
                    <br/>
                    {"The snowballs are created by combining a sphere mesh with a snowball texture. The snowball texture is a simple gradient from white to transparent, which is used to mask out the sphere mesh. This creates a snowball with a smooth transition from the center to the edges. The snowball is then rotated around its forward axis to simulate rolling."}
                    <br/><br/>

                    <span class="font-bold">{"Texture-Based Deformation"}</span>
                    <br/>
                    {"Dynamic objects marked as snow deformers are rendered from a specialized orthographic camera set up in a top-down perspective. This results in a \"raw\" deformation map capturing deformations occurring in the current frame. To maintain persistent snow deformation, the deformation map from the previous frame is blended with the \"raw\" current deformation in a fullscreen pass, producing the final deformation map."}
                    <br/><br/>

                    <span class="font-bold">{"Player-Centered Deformation"}</span>
                    <br/>
                    {"The orthographic snow camera is attached to the player, to capture snow deformation wherever it goes. During the compositing pass, adjustments are made to account for changes in origin, ensuring accurate accumulation of the previous deformation into the current deformation map. Future iterations aim to enhance the feature to support multiple players, ensuring smooth transitions when one player overtakes another."}
                    <br/><br/>

                    <span class="font-bold">{"Dynamic Terrain Tessellation"}</span>
                    <br/>
                    {"To achieve smooth vertex displacement while preserving good performances, selective mesh tessellation is employed, focusing on the neighboring area of the main camera. Both the level of tessellation and the maximum tessellation distance are configurable parameters within the material."}
                    <br/><br/>

                    <span class="font-bold">{"Normal Reconstruction"}</span>
                    <br/>
                    {"When displacing vertices, normal vectors are recomputed based on the deformation factor to maintain accurate lighting. The deformation map, acting as a reversed height map, allows sampling nearby pixels and applying the \"finite difference\" method to reconstruct vertex normals. Precision issues may arise, resulting in flickering normals, particularly when the tessellation level is not sufficiently high."}
                    <br/><br/>
                </p>
                </div>

                <div class="h-2"/> // Absolutely disgusting hack to make the columns work
                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Challenges and obstacles"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    <span class="font-bold">{"Technical challenges"}</span>
                    <br/>
                    {"To solve the accessibility problem, where anyone with a modern smartphone should be able to play, we needed to use its web browser for both accessing the sensor data and also sending it to the game server. We knew this would be the most challenging part of the project so we spent a lot of time on this. At first we explored the networking library Mirror (https://mirror-networking.com/) and tried setting up a local HTTPS for serving the WebGL client to the phone which also had a secure WebSocket connection to the game. It had to be secure for letting us access the accelerometer sensor data. This solution was not the way forward due to certificates and various obstacles. Instead, luckily for us, we found the Unity Relay services and integrated that into our WebGL client and game server."}
                    <br/><br/>
                    {"Another, very annoying obstacle, was that Unity WebGL simply does not support input fields. This meant we could not insert text into text fields, which is very important for connecting to the game. However, luckily for us, someone on the internet had written a custom library for solving this very specific issue. It works, but the user experience is not optimal. https://github.com/kou-yeung/WebGLInput"}
                    <br/><br/>
                    <span class="font-bold">{"Gameplay Challenges"}</span>
                    <br/>
                    {"For the game to be playable with more than one player we had to come up with a good camera solution and make each player easily visible."}
                    <br/><br/>
                    {"The camera solution landed in having a predefined camera path with points that specified both position and look rotation. We used the camera tool: Cinemachine (https://unity.com/unity/features/editor/art-and-design/cinemachine) for solving this. This made it easy to configure the camera and move it along the predefined path. We had to balance the distance between the camera and the leading player in order to make room for everyone else on the slope. Remember that a player gets eliminated when going out of view. With the Cinemachine tool we added a drag to delay the camera instead of instantly following the leading player. Additionally we added a depth of field effect which makes it harder for the leading player to see obstacles ahead - increasing the chance of a collision."}
                    <br/><br/>
                    {"For separating each player we let everyone select a personal name that we display about each character but also changing the player skin color to a selection of vibrant colors. This color is also shown on your phone while playing."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Related work"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    <br/>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                </p>
                </div>

                <div class="break-inside-avoid-column">
                <h2 class="text-2xl my-5 font-extrabold tracking-tight sm:text-3xl lg:text-5xl">{"Lessons Learned"}</h2>
                <p class="text-xl my-3 text-justify max-w-prose tracking-tight">
                    {"The main learning outcome was that trying to set up your own HTTPS web server with a secure WebSocket connection is much harder than expected. You need multiple certificates, both for the domain and for the web sockets. And these certificates should not be self signed because you want a smooth user experience - not a bunch of warnings stating this application is insecure!"}
                    <br/>
                    <br/>

                    {"Due to our idea being compatible with previous assets we had made we could very quickly get up to speed. Already within the first two weeks we had a game that looked like a skiing game and the interactions were … kind of working. This meant we could focus on really improving the interaction and adding other fun stuff to the game."}
                </p>
                </div>
            </div>
        </div>


        </>
    }
}
