use langchain_rust::llm::OpenAIConfig;

use langchain_rust::{language_models::llm::LLM, llm::openai::OpenAI};

use std::env;

#[tokio::main]
async fn main() {
    //OpenAI Example
    // let open_ai = OpenAI::default();
    // let response = open_ai.invoke("hola").await.unwrap();
    // println!("{}", response);

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let base_url = env::var("OPENAI_API_BASE").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    //or we can set config as
    let open_ai = OpenAI::default().with_config(
        OpenAIConfig::default()
            .with_api_base(base_url) //if you want to specify base url
            .with_api_key(api_key), //if you want to set you open ai key,
    )
    .with_model("gemma2-9b-it");

    let response = open_ai.invoke("说一个笑话").await.unwrap();
    println!("{}", response);
}
