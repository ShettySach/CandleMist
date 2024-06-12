use crate::model::conversation::Conversation;
use leptos::{html::Div, *};

const USER_MESSAGE_CLASS: &str = "max-w-lg p-4 mb-5 self-end";
const USER_MESSAGE_DARK_MODE_COLORS: &str = "bg-gradient-to-r from-user_d to-user_l text-white";
const USER_MESSAGE_LIGHT_MODE_COLORS: &str = "bg-blue-700 text-white";

const MODEL_MESSAGE_CLASS: &str = "max-w-lg p-4 mb-5 self-start";
const MODEL_MESSAGE_DARK_MODE_COLORS: &str = "bg-gradient-to-r from-model_d to-model_l text-white";
const MODEL_MESSAGE_LIGHT_MODE_COLORS: &str = "bg-gray-200 text-black";

const CHAT_AREA_CLASS: &str = "h-screen pb-24 w-full flex flex-col overflow-y-auto p-5";
const CHAT_AREA_DARK_MODE_COLORS: &str = "bg-background";
const CHAT_AREA_LIGHT_MODE_COLORS: &str = "bg-gray-100";

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let dark_mode =
        use_context::<ReadSignal<bool>>().expect("Should be able to get dark mode state");

    let user_message_class = Signal::derive(move || {
        if dark_mode.get() {
            format!("{USER_MESSAGE_CLASS} {USER_MESSAGE_DARK_MODE_COLORS}")
        } else {
            format!("{USER_MESSAGE_CLASS} {USER_MESSAGE_LIGHT_MODE_COLORS}")
        }
    });

    let model_message_class = Signal::derive(move || {
        if dark_mode.get() {
            format!("{MODEL_MESSAGE_CLASS} {MODEL_MESSAGE_DARK_MODE_COLORS}")
        } else {
            format!("{MODEL_MESSAGE_CLASS} {MODEL_MESSAGE_LIGHT_MODE_COLORS}")
        }
    });

    let chat_div_ref = create_node_ref::<Div>();
    create_effect(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    let chat_area_class = Signal::derive(move || {
        if dark_mode.get() {
            format!("{CHAT_AREA_CLASS} {CHAT_AREA_DARK_MODE_COLORS}")
        } else {
            format!("{CHAT_AREA_CLASS} {CHAT_AREA_LIGHT_MODE_COLORS}")
        }
    });

    view! {
          <div class={chat_area_class.get()} node_ref=chat_div_ref>
          {move || conversation.get().messages.iter().map(move |message| {
              let class_str = if message.user { user_message_class.get() } else { model_message_class.get()};
              view! {
                <div class={class_str}>
                  {message.text.clone()}
                </div>
              }
            }).collect::<Vec<_>>()
          }
        </div>
    }
}
