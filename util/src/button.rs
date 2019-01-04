use yew::prelude::*;

pub struct Button {
    title: String,
    //color: Color,
    disabled: bool,
    onclick: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub disabled: bool,
    pub onclick: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "Button".into(),
            disabled: false,
            onclick: None,
        }
    }
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button {
            title: props.title,
            disabled: props.disabled,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref mut callback) = self.onclick {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.disabled = props.disabled;
        self.onclick = props.onclick;
        true
    }
}

impl Renderable<Button> for Button {
    fn view(&self) -> Html<Self> {
        html! {
            <button class=("btn", "green"), disabled=self.disabled, onclick=|_| Msg::Clicked,>{ &self.title }</button>
        }
    }
}
