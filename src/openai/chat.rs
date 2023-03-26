use crate::config::Chat;
use ::openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use serde_json::json;

pub(crate) async fn chat(chat_config: &Chat, prompt: &str) -> serde_json::Value {
    set_key(chat_config.token.clone());

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: "You are a large language model built into a command line interface for PKU software course homework. Please speak in English only".to_string(),
        name: None,
    }];

    let prompt = prompt.to_string();

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: prompt,
        name: None,
    });

    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
        .create()
        .await;

    let chat_completion = match chat_completion {
        Ok(chat_completion) => chat_completion,
        Err(err) => {
            error!("Failed to get chat completion: {}", err);
            return serde_json::json!({
                "status": "failed",
                "error": "Network error",
            });
        }
    };

    let chat_completion = match chat_completion {
        Ok(chat_completion) => chat_completion,
        Err(err) => {
            error!("OpenAI error: {}", err);
            return serde_json::json!({
                "status": "failed",
                "error": "OpenAI error",
            });
        }
    };

    let returned_message = chat_completion.choices.first();
    let returned_message = match returned_message {
        Some(returned_message) => returned_message,
        None => {
            error!("No returned message");
            return serde_json::json!({
                "status": "failed",
                "error": "No returned message",
            });
        }
    };
    let returned_message = returned_message.message.clone();

    json!({
        "status": "ok",
        "text": returned_message,
    })
}
