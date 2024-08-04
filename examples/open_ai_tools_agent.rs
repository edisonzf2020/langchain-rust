use std::{error::Error, sync::Arc, env};

use async_trait::async_trait;
use langchain_rust::{
    agent::{AgentExecutor, OpenAiToolAgentBuilder}, chain::{options::ChainCallOptions, Chain}, language_models::llm, llm::openai::OpenAI, memory::SimpleMemory, prompt_args, tools::{CommandExecutor, DuckDuckGoSearchResults, SerpApi, Tool}
};
use langchain_rust::llm::OpenAIConfig;

use serde_json::Value;
struct Date {}

#[async_trait]
impl Tool for Date {
    fn name(&self) -> String {
        "Date".to_string()
    }
    fn description(&self) -> String {
        "Useful when you need to get the date,input is  a query".to_string()
    }
    async fn run(&self, _input: Value) -> Result<String, Box<dyn Error>> {
        Ok("25  of november of 2025".to_string())
    }
}

#[tokio::main]
async fn main() {
    // let llm = OpenAI::default();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let base_url = env::var("OPENAI_API_BASE").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    //or we can set config as
    let llm = OpenAI::default().with_config(
        OpenAIConfig::default()
            .with_api_base(base_url) //if you want to specify base url
            .with_api_key(api_key), //if you want to set you open ai key,
    )
    .with_model("deepseek-ai/DeepSeek-V2-Chat");
    let memory = SimpleMemory::new();
    let serpapi_tool = SerpApi::default();
    let duckduckgo_tool = DuckDuckGoSearchResults::default();
    let tool_calc = Date {};
    let command_executor = CommandExecutor::default();
    let agent = OpenAiToolAgentBuilder::new()
        .tools(&[
            Arc::new(serpapi_tool),
            Arc::new(tool_calc),
            Arc::new(command_executor),
            Arc::new(duckduckgo_tool),
        ])
        .options(ChainCallOptions::new().with_max_tokens(1000))
        .build(llm)
        .unwrap();

    let executor = AgentExecutor::from_agent(agent).with_memory(memory.into());

    let input_variables = prompt_args! {
        "input" => "当前文件目录的名称是什么，今天是几号",
    };

    match executor.invoke(input_variables).await {
        Ok(result) => {
            println!("Result: {:?}", result.replace("\n", " "));
        }
        Err(e) => panic!("Error invoking LLMChain: {:?}", e),
    }
}
