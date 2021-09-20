use std::rc::Rc;

use chrono::{TimeZone, Utc};
use ybc::{Image, Level, LevelItem, LevelLeft, LevelRight, Table};
use yew::{classes, html, Classes, Component, Properties};

pub struct Spool {
    props: SpoolProps,
}
pub enum Msg {}
#[derive(Debug, Clone, Properties)]
pub struct SpoolProps {
    pub name: Rc<str>,
    pub description: Rc<str>,
    pub avatar: Rc<str>,

    #[prop_or_default]
    pub last_message_time: i64,
    #[prop_or(true)]
    pub online: bool,
}

impl Component for Spool {
    type Message = Msg;

    type Properties = SpoolProps;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        false
    }

    // fn view(&self) -> yew::Html {
    //     // like telegram but maybe have an icon to the left of name
    //     //TODO: real avatar support with props
            
    //     html! {
    //         <div>
    //         <Level classes={classes!("is-mobile", "spool")}>
    //             <LevelLeft>

    //             <Image classes={classes!("is-64x64")}>
    //                 <img class={classes!("is-rounded")} src="https://picsum.photos/200"/>
    //             </Image>
    //                 // <p class={classes!("level-left")}>{"TestL"}</p>

               

    //             </LevelLeft>
    //             <LevelLeft>
    //                 <p class={classes!("")}>{"Test R"}</p>
    //                 <p class={classes!("")}>{"Test R"}</p>
    //             </LevelLeft>
    //             <LevelRight>
    //                 <p>{"9:28pm"}</p>
    //             </LevelRight>
    //         </Level>
    //         </div>
    //     }
    // }

    fn view(&self) -> yew::Html {
        let t = self.props.last_message_time;
        let dt = Utc.timestamp(t, 0);
        let message_time = dt.to_rfc2822();
        html!(
            <div class=classes!("ripple")>
               <tr>
               <td>
                    <Image classes={classes!("is-64x64")}>
                        <img class={classes!("is-rounded")} src="https://picsum.photos/200"/>
                    </Image>
                    </td>
                <td>
                    <p>{self.props.name.clone()}</p>
                </td>
                <td>
                    {message_time}
                </td>
               </tr>
            </div>
        )
    }
}
