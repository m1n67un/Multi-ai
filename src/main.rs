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

    let questions = [
        // --
        "러스트 언어로 웹소켓 생성하는 예제코드 보여줘. 한글로 보여줘.",
    ];

    let model = "llama3"; // LM Studio 다운로드 후, http://localhost:11434

    for question in questions {
        println!("\n-- Question:\n{question}");
        chat_req = chat_req.append_message(ChatMessage::user(question));

        println!("\n-- Answer (from {model})");
        let chat_res = client.exec_chat_stream(model, chat_req.clone(), None).await?;
        let response = print_chat_stream(chat_res, None).await?;

        chat_req = chat_req.append_message(ChatMessage::assistant(response));
    }

    // Multi-AI
    // let models = ["llama3", "gpt-3.5-turbo"];

    // for (question, model) in questions.into_iter().zip(models) {
    //     println!("\n-- Question:\n{question}");
    //     chat_req = chat_req.append_message(ChatMessage::user(question));

    //     println!("\n-- Answer (from {model})");
    //     let chat_res = client.exec_chat_stream(model, chat_req.clone(), None).await?;
    //     let response = print_chat_stream(chat_res, None).await?;

    //     chat_req = chat_req.append_message(ChatMessage::assistant(response));
    // }

    Ok(())
}
