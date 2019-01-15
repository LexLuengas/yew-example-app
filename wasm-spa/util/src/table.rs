use yew::prelude::*;
use serde::Serialize;

pub trait TableData: 'static + Default + Clone + PartialEq + Serialize {
    fn view_data(&self) -> Html<Table<Self>>;
    fn get_field_as_html(&self, field_name: &str) -> &str;
}
// impl<T: 'static + Default + Clone + PartialEq + Serialize> TableData for T {}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Column {
    pub name: String,
    pub short_name: Option<String>,
    pub data_property: Option<String>,
}

#[derive(Clone, PartialEq, Default)]
pub struct Table<T> where T: TableData {
    columns: Vec<Column>,
    data: Vec<T>,
}

#[derive(SmartDefault, Serialize, Deserialize, Clone)]
pub enum Msg {
    #[default]
    NoOp,
}

#[derive(Clone, PartialEq, Default)]
pub struct TableProps<T> where T: TableData {
    pub columns: Vec<Column>,
    pub data: Vec<T>,
}

impl<T> Component for Table<T> where T: TableData {
    type Message = Msg;
    type Properties = TableProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Table {
            columns: props.columns,
            data: props.data,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        match msg {
            NoOp => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.columns = props.columns;
        self.data = props.data;
        true
    }
}

impl<T> Renderable<Table<T>> for Table<T> where T: TableData {
    fn view(&self) -> Html<Table<T>> {
        html! {
            <table class="table",>
                <thead>
                    { for self.columns.iter().map(Column::view) }
                </thead>
                <tbody>
                    { for self.data.iter().map(|d| self.view_row(d.clone())) }
                </tbody>
            </table>
        }
    }
}

impl<T> Renderable<Table<T>> for Column where T: TableData {
    fn view(&self) -> Html<Table<T>> {
        html! {
            <th><abbr title=self.name.clone(),>{ self.short_name.clone().unwrap_or(self.name.clone()) }</abbr></th>
        }
    }
}

impl<T> Table<T> where T: TableData {
    fn wrap_in_td(content: &str) -> Html<Table<T>> {
            html! {
                <td>{ content }</td>
            }
        }

    fn view_row(&self, datum: T) -> Html<Table<T>> {
        html! {
            <tr>
                { for self.columns.iter()
                    .map(|c| { c.data_property.clone().unwrap_or(c.name.clone()) })
                    .map(|ref c| { datum.get_field_as_html(c) })
                    .map(Self::wrap_in_td) }
            </tr>
        }
    }
}