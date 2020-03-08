  
use yew::prelude::*;
// use yew::html::Properties;

pub struct Button {
    text: i32,
}

#[derive(Clone, Properties, Default)]
pub struct Props {
    pub text: i32,
    pub age: i32,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
       Button {
           text:props.text
       }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
   
        html! {
            <div>
                <article>{self.text}</article>
            </div>
        }
    }
}