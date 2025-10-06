use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageLayoutProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(PageLayout)]
pub fn page_layout(props: &PageLayoutProps) -> Html {
    let class_name = props.class.clone().unwrap_or_else(|| "page".to_string());

    html! {
        <div class={class_name}>
            <div class="container">
                { props.children.clone() }
            </div>
        </div>
    }
}
