use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct GalleryProps {
    pub images: Vec<String>,
}

#[function_component(Gallery)]
pub fn gallery(props: &GalleryProps) -> Html {
    let images = &props.images;

    let images = images
        .iter()
        .enumerate()
        .map(|(i, image)| {
            html! {
                <div>
                    <img class="h-auto max-w-full rounded-lg hover:scale-150 transition delay-50 ease-in-out duration-150" src={image.clone()} alt={"image"} />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="grid lg:grid-cols-2 gap-4 m-10">
            {images}
        </div>
    }
}
