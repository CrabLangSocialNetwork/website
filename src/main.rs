use yew::prelude::*;
use yew_router::prelude::*;

struct Post {
    content: String
}

#[derive(Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home
}

#[function_component]
fn Home() -> Html {
    let posts = use_state(|| Vec::<Post>::new());

    html! {

    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<h1>{"Home"}</h1>}
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
