mod button;
use yew::prelude::*;
use button::Button;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    DoIt,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => self.value = self.value + 1
        }
        true
    }

    fn view(&self) -> Html {
       let props = <Button as Component>::Properties::default();
        html! {
            <div>
                

            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
