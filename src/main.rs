mod common;

use std::io;
use common::CommonContracts;
use genai::{
    chat::{ChatMessage, 
        ChatRequest
    }, 
    client::Client, 
    utils::print_chat_stream
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let mut chat_req = ChatRequest::default().with_system("Answer with one sentence.");

    println!("{}", CommonContracts::Sentence::QUESTION);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let questions = [
        // --
        input.trim(),
    ];

    let model = "llama3"; // LM Studio 다운로드 후, http://localhost:11434

    for question in questions {
        println!("\n-- Question:\n{question}");
        chat_req = chat_req.append_message(ChatMessage::user(format!("{} {}", CommonContracts::Sentence::FIX_KOREAN, question)));

        println!("\n-- Answer (from {model})");
        let chat_res = client.exec_chat_stream(model, chat_req.clone(), None).await?;
        let response = print_chat_stream(chat_res, None).await?;

        chat_req = chat_req.append_message(ChatMessage::assistant(response));
    }

    // Multi-AI
    // let client = Client::default();

    // let mut chat_req = ChatRequest::default().with_system("Answer with one sentence.");

    // let questions = [
    //     // --
    //     "rust lang으로 main.rs 에서 run 시, 입력값을 넣을 수 있게 예제코드 보여줘",
    // ];

    // let models = ["llama3", "gpt-3.5-turbo"];

    // for (question, model) in questions.into_iter().zip(models) {
    //     println!("\n-- Question:\n{question}");
    //     chat_req = chat_req.append_message(ChatMessage::user(question));

    //     println!("\n-- Answer (from {model})");
    //     let chat_res = client.exec_chat_stream(model, chat_req.clone(), None).await?;
    //     let response = print_chat_stream(chat_res, None).await?;

    //     chat_req = chat_req.append_message(ChatMessage::assistant(response));
    // }

    Ok(().into())
}
