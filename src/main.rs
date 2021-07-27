 use std::process::exit;
 use std::env;

 /// Sends a message to and receives the message from a queue.
 #[tokio::main]
 async fn main() -> Result<(), sqs::Error> {
    tracing_subscriber::fmt::init();
    let client = sqs::Client::from_env();
    let myqueue = "test_queue";
    let account = match env::var("AWS_ACCOUNT"){
       Ok(account)  => account,
       Err(e) => {
           eprintln!("You did not set the AWS_ACCOUNT environment variable. Please do so and try again. {}", e);
            exit(1);
       }
    };
    let queue_url = "https://sqs.us-east-1.amazonaws.com/".to_owned()+&account+"/"+myqueue;
 
    println!(
        "Sending messages to SQS Queue: `{}` in account `{:#?}`",
        queue_url,
        account
    );
    let words = ["cat","dog","horse","pig", "Mercury","Gemini","Apollo","Skylab","Skylab B","ISS"];
    for word in words.iter() {
        let rsp = client
            .send_message()
            .queue_url(&queue_url)
            .message_body(word.to_string())
            .send()
            .await?;
        println!("Response from sending a message: {:#?}", rsp);
    }
    Ok(())
 }