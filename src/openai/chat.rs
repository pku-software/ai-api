use crate::config::Chat;
use ::openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use openai::completions::Completion;
use serde_json::json;

pub(crate) async fn chat(chat_config: &Chat, prompt: &str) -> serde_json::Value {
    set_key(chat_config.token.clone());

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: "You are a large language model built into a command line interface as the PKU software course homework robot.".to_string(),
        name: None,
    }];

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: prompt.to_string(),
        name: None,
    });

    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
        .create()
        .await
        .unwrap()
        .unwrap();

    let returned_message = chat_completion.choices.first().unwrap().message.clone();

    json!({
        "status": "ok",
        "text": returned_message.content.trim(),
    })
}
