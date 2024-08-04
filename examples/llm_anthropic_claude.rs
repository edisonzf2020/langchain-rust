use langchain_rust::{language_models::llm::LLM, llm::Claude};

#[tokio::main]
async fn main() {
    let claude = Claude::default()
        .with_model("claude-3-5-sonnet@20240620");
    let response = claude.invoke("hello").await.unwrap();
    println!("{}", response);
}
