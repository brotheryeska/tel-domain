use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::components::RouterAnchor;
// use yew::services::ConsoleService;
// use yewdux::prelude::WithDispatch;
// use yewdux::prelude::*;
// use yewtil::NeqAssign;

use crate::pages::{
    // reducer_global::ReducerGlobal,
    applications::register_page::register::RegisterPage,
    applications::login_page::home::LoginPage,
};


#[derive(Switch, Clone)]
enum Route {
    #[to = "/register"]
    RegisterPage,
}

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {

        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: Route| match switch {
            Route::RegisterPage => html! {<RegisterPage/>},
        });
        type Anchor = RouterAnchor<Route>;
        html! {
            <div>
                <Anchor route=Route::RegisterPage classes="item">        
                </Anchor>
                <main>
                    <Router<Route, ()> render=render/>
                </main>
                <LoginPage/>
            </div>
        }
    }
}
