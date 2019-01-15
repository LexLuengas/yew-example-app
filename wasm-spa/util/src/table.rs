
#[derive(Clone, PartialEq, Default)]
pub struct Table {
    columns: Vec<String>,
}

#[derive(SmartDefault, Serialize, Deserialize, Clone)]
pub enum Msg {
    #[default]
    NoOp,
}

#[derive(Clone, PartialEq, Default)]
pub struct TableProps {
    pub columns: Vec<String>,
}

impl Component for Table {
    type Message = Msg;
    type Properties = TableProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Table {
            columns: props.columns,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        match msg {
            NoOp => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }
}