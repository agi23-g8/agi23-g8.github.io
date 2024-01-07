use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CarouselProps {
    pub images: Vec<String>,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let images = &props.images;

    images.iter().enumerate().map(|(i, image)| {
        html! {
            <div id={format!("slide{}", i)} class="carousel-item relative w-full">
                <img src={image} class="w-full" />
                <div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
                <a href={format!("#slide{}", (i + images.len() - 1) % images.len())} class="btn btn-circle">{"❮"}</a>
                <a href={format!("#slide{}", (i + 1) % images.len())} class="btn btn-circle">{"❯"}</a>
                </div>
            </div>
        }
    }).collect::<Html>();

    html! {}
}
