use futures::StreamExt;
use std::env;
use langchain_rust::{
    chain::{Chain, LLMChainBuilder},
    fmt_message, fmt_template,
    llm::openai::OpenAI,
    llm::OpenAIConfig,
    message_formatter,
    prompt::HumanMessagePromptTemplate,
    prompt_args,
    schemas::messages::Message,
    template_fstring,
};
use async_openai::error::OpenAIError;
use langchain_rust::language_models::LLMError;

#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let base_url = env::var("OPENAI_API_BASE").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    let open_ai = OpenAI::default().with_config(
        OpenAIConfig::default()
            .with_api_base(base_url)
            .with_api_key(api_key),
    )
    .with_model("deepseek-ai/DeepSeek-V2-Chat");

    let prompt = message_formatter![
        fmt_message!(Message::new_system_message(
            "You are world class technical documentation writer."
        )),
        fmt_template!(HumanMessagePromptTemplate::new(template_fstring!(
            "{input}", "input"
        )))
    ];

    let chain = LLMChainBuilder::new()
        .prompt(prompt)
        .llm(open_ai.clone())
        .build()
        .unwrap();

    let mut stream = chain
        .stream(prompt_args! {
        "input" => "中国的四大名著是哪些？",
           })
        .await
        .unwrap();

    while let Some(result) = stream.next().await {
        match result {
            Ok(value) => value.to_stdout().unwrap(),
            Err(e) => {
                if let langchain_rust::chain::ChainError::LLMError(LLMError::OpenAIError(OpenAIError::StreamError(ref msg))) = e {
                    if msg == "Stream ended" {
                        break;
                    }
                }
                panic!("Error invoking LLMChain: {:?}", e);
            }
        }
    }
}
