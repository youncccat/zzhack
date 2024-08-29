use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    #[prop_or(AttrValue::Static("none"))]
    pub color: AttrValue,
    pub height: String,
    pub width: String,
}

#[function_component]
pub fn GitHubIcon(props: &IconProps) -> Html {
    html! {
            <svg width={props.width.clone()} height={props.height.clone()} fill={props.color.clone()} xmlns="http://www.w3.org/2000/svg">
         <path fill-rule="evenodd" clip-rule="evenodd" d="M8.25683 0C3.69101 0 0 3.64386 0 8.15181C0 11.7553 2.36496 14.8055 5.64579 15.8851C6.05597 15.9662 6.20622 15.7097 6.20622 15.4939C6.20622 15.3049 6.1927 14.6571 6.1927 13.9822C3.89585 14.4681 3.41756 13.0104 3.41756 13.0104C3.04844 12.0657 2.50152 11.8228 2.50152 11.8228C1.74976 11.3235 2.55628 11.3235 2.55628 11.3235C3.39018 11.3775 3.82774 12.1602 3.82774 12.1602C4.56581 13.4018 5.75514 13.051 6.2336 12.835C6.30188 12.3086 6.52075 11.9442 6.75314 11.7418C4.92124 11.5529 2.99385 10.8511 2.99385 7.71985C2.99385 6.82909 3.32173 6.10032 3.84126 5.53353C3.75929 5.33113 3.47215 4.4942 3.9234 3.37405C3.9234 3.37405 4.62057 3.15807 6.19253 4.21081C6.86555 4.03237 7.55962 3.9416 8.25683 3.94084C8.95399 3.94084 9.66468 4.03541 10.3209 4.21081C11.8931 3.15807 12.5902 3.37405 12.5902 3.37405C13.0415 4.4942 12.7542 5.33113 12.6722 5.53353C13.2054 6.10032 13.5198 6.82909 13.5198 7.71985C13.5198 10.8511 11.5924 11.5393 9.74682 11.7418C10.0477 11.9982 10.3073 12.484 10.3073 13.2534C10.3073 14.3465 10.2937 15.2239 10.2937 15.4937C10.2937 15.7097 10.4442 15.9662 10.8542 15.8852C14.135 14.8053 16.5 11.7553 16.5 8.15181C16.5135 3.64386 12.8089 0 8.25683 0Z" fill="black"/>
        </svg>
    }
}
