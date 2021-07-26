use yew::prelude::*;

pub struct SingleSignOnPage {}

pub enum Msg {}

impl Component for SingleSignOnPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SingleSignOnPage {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"haha"}</h1> 
            </div> 
        }
    }
}
