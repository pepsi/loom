use std::rc::Rc;

use ybc::{Button, Input, InputType, Table};
use yew::services::{ConsoleService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
mod spool;
use spool::Spool;

use ybc::NavbarFixed::*;
use ybc::TileCtx::{Ancestor, Child, Parent};
use ybc::TileSize::*;
use yew::prelude::*;


pub enum Msg {
    Increment,
    Decrement,
    EmailUpdated(String),
    PasswordUpdated(String),

}
#[derive(Debug, PartialEq)]
pub enum Screen {
    LaunchPad, //Sign up/register
    ChoosingChat,
    Chatting,
}
pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
    screen: Screen,
    should_fade: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            screen: Screen::LaunchPad,
            should_fade: true
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one");
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one");
                true
            }
            Msg::EmailUpdated(s) => {
                println!("EmailUpdated! {}", s);
                false
            },
            Msg::PasswordUpdated(s) => {
                println!("PasswordUpdated! {}", s);
                false
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let brand = html! {<h class="is-size-2">{"Loom"}</h>};

        let current_screen = match self.screen {
            Screen::LaunchPad => self.launchpad(),
            Screen::ChoosingChat => self.choosing_chat(),
            Screen::Chatting => self.chatting(),
        };
        let nav = if self.screen != Screen::LaunchPad {
            html!(<ybc::Navbar classes={classes!("is-primary")} fixed=Top navbrand=brand spaced=true/>)} else {html!(<></>)};

        html! {
            <div class="fadeIn" id="jerome">
            <section>
            {nav}
                <ybc::Container fluid=true>
                    <ybc::Tile ctx=Ancestor>
                       {current_screen}
                    </ybc::Tile>
                </ybc::Container>
                </section>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
            let document = web_sys::window().unwrap().document().unwrap();
            let e = document.get_element_by_id("jerome").unwrap();
            // e.set_class_name("fadeIn");
            // let e = web_sys::get_element_by_id()

        

    }

    
}
impl Model {
    fn launchpad(&self) -> Html {
        let email_cb = self.link.callback(|text| {Msg::EmailUpdated(text)});
        let password_cb = self.link.callback(|text| {Msg::PasswordUpdated(text)});
        
        html!(<>
            // <Container>
                <ybc::Tile ctx=Parent vertical=true size=Twelve>
          
                <ybc::Tile ctx=Child   size=Two>

                    <h class="is-size-1 is-pimary">{"Sign in"}</h>
                    <h1>{"Please enter your email and password."}</h1>

                </ybc::Tile>
                <ybc::Tile ctx=Child size=Three>
                    <Input placeholder="email@example.com" rounded=true name="email address" r#type=InputType::Email value="" update={email_cb}/>
                </ybc::Tile>

                <ybc::Tile ctx=Child size=Three>
                    <Input placeholder="Password" rounded=true name="password" r#type=InputType::Password value="" update={password_cb}/>
                </ybc::Tile>

                <ybc::Tile ctx=Child size=Twelve>
                    <Button>{"Next"}</Button>
                </ybc::Tile>

                </ybc::Tile>
            // </Container>
        </>)
    }
    fn choosing_chat(&self) -> Html {
        let s = (0..1000).map(|_| generate_spool()).collect::<Vec<_>>();

        html! {
            <>
            <ybc::Tile ctx=Parent vertical=true size=Four>
            <Table>
                <thead>
                 <tr>
                    <th><p>{"pfp"}</p></th>
                 </tr>
                </thead>
                {s}
            </Table>
        </ybc::Tile>
        <ybc::Tile ctx=Parent vertical=true size=Four>
            <h>{"MESSAGES GO HERE"}</h>
        </ybc::Tile>
        </>
        }
    }
    fn chatting(&self) -> Html {
        html!(<p>{"todo chatting"}</p>)
    }

}
fn main() {
    yew::start_app::<Model>();
}

fn generate_spool() -> yew::Html {
    let name = lipsum::lipsum(3);
    let desc = lipsum::lipsum(8);
    let lmt: i64 = 1632065840;
    let o: bool = rand::random();
    html!(<Spool name=Rc::from(name) description=Rc::from(desc) avatar=Rc::from("") last_message_time={lmt} online=o />)
}
