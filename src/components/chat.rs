use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    ToggleEmojiPanel,
    SelectEmoji(String),
    ChangeBackground(String),
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    emoji_panel_open: bool,
    current_background: String,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
            .is_ok()
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            emoji_panel_open: false,
            current_background: "bg-white".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://api.dicebear.com/9.x/big-smile/svg?seed={}",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
            Msg::ToggleEmojiPanel => {
                self.emoji_panel_open = !self.emoji_panel_open;
                true
            }
            Msg::SelectEmoji(emoji) => {
                if let Some(input) = self.chat_input.cast::<HtmlInputElement>() {
                    let current_value = input.value();
                    input.set_value(&format!("{} {}", current_value, emoji));
                }
                self.emoji_panel_open = false;
                true
            }
            Msg::ChangeBackground(bg_class) => {
                self.current_background = bg_class;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let toggle_emoji = ctx.link().callback(|_| Msg::ToggleEmojiPanel);
        
        let bg_white = ctx.link().callback(|_| Msg::ChangeBackground("bg-white".to_string()));
        let bg_blue = ctx.link().callback(|_| Msg::ChangeBackground("bg-blue-100".to_string()));
        let bg_green = ctx.link().callback(|_| Msg::ChangeBackground("bg-green-100".to_string()));
        let bg_purple = ctx.link().callback(|_| Msg::ChangeBackground("bg-purple-100".to_string()));

        html! {
            <div class={format!("flex w-screen {}", self.current_background)}>
                <div class="flex-none w-56 h-screen bg-gray-100">
                    <div class="text-xl p-3">{"Users"}</div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 bg-white rounded-lg p-2">
                                    <div>
                                        <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between">
                                            <div>{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs text-gray-400">
                                            {"Online"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="grow h-screen flex flex-col">
                    <div class="w-full h-14 border-b-2 border-gray-300 flex items-center justify-between px-4">
                        <div class="text-xl p-3">{"💬 Chat!"}</div>
                        <div class="flex space-x-2">
                            <button onclick={bg_white} class="w-6 h-6 bg-white border border-gray-300 rounded-full"></button>
                            <button onclick={bg_blue} class="w-6 h-6 bg-blue-100 border border-gray-300 rounded-full"></button>
                            <button onclick={bg_green} class="w-6 h-6 bg-green-100 border border-gray-300 rounded-full"></button>
                            <button onclick={bg_purple} class="w-6 h-6 bg-purple-100 border border-gray-300 rounded-full"></button>
                        </div>
                    </div>
                    <div class="w-full grow overflow-auto border-b-2 border-gray-300">
                        {
                            self.messages.iter().map(|m| {
                                let default_profile = UserProfile {
                                    name: m.from.clone(),
                                    avatar: format!("https://api.dicebear.com/9.x/big-smile/svg?seed={}", m.from),
                                };
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap_or(&default_profile);
                                
                                html!{
                                    <div class="flex items-end w-3/6 bg-gray-100 m-8 rounded-tl-lg rounded-tr-lg rounded-br-lg ">
                                        <img class="w-8 h-8 rounded-full m-3" src={user.avatar.clone()} alt="avatar"/>
                                        <div class="p-3">
                                            <div class="text-sm">
                                                {m.from.clone()}
                                            </div>
                                            <div class="text-xs text-gray-500">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-3" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    
                    <div class="relative">
                        {
                            if self.emoji_panel_open {
                                let emojis = vec!["😊", "😂", "❤️", "👍", "😍", "🔥", "👋", "🎉", "👏"];
                                html! {
                                    <div class="absolute bottom-16 left-4 bg-white shadow-lg rounded-lg p-2 z-10 grid grid-cols-3 gap-2">
                                        {
                                            emojis.iter().map(|emoji| {
                                                let emoji_str = emoji.to_string();
                                                let select_emoji = ctx.link().callback(move |_| Msg::SelectEmoji(emoji_str.clone()));
                                                html! {
                                                    <button onclick={select_emoji} class="text-2xl p-2 hover:bg-gray-100 rounded">
                                                        {emoji}
                                                    </button>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    
                    <div class="w-full h-14 flex px-3 items-center">
                        <button 
                            onclick={toggle_emoji}
                            class="p-2 text-xl hover:bg-gray-100 rounded"
                        >
                            {"😊"}
                        </button>
                        <input ref={self.chat_input.clone()} type="text" placeholder="Message" class="block w-full py-2 pl-4 mx-3 bg-gray-100 rounded-full outline-none focus:text-gray-700" name="message" required=true 
                            onkeypress={
                                let link = ctx.link().clone();
                                Callback::from(move |e: KeyboardEvent| {
                                    if e.key() == "Enter" {
                                        link.send_message(Msg::SubmitMessage);
                                    }
                                })
                            }
                        />
                        <button onclick={submit} class="p-3 shadow-sm bg-blue-600 w-10 h-10 rounded-full flex justify-center items-center color-white">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}