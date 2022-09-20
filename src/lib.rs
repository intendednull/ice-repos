use yew_router::Routable;

pub mod components;
pub mod repository;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/ice-repos/review-and-submit")]
    ReviewAndSubmit,
    #[at("/ice-repos/about")]
    About,
    #[not_found]
    #[at("/ice-repos/404")]
    NotFound,
}
