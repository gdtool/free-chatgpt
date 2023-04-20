use std::{str::FromStr, sync::Arc};




#[cfg(test)]
mod tests{
    use std::{str::FromStr, sync::Arc};
    use chatgpt::{Chatgpt, ChatgptParams};

    #[tokio::test]
    async fn test_ask() {
        let chatgpt_params = ChatgptParams{
            temperature: Some(0.5),
            max_tokens: Some(256),
            top_p: Some(1),
            frequency_penalty: Some(1),
            presence_penalty: Some(1),
            stop_sequences: Some(vec![]),
            prompt: String::from_str("你是一个程序员,我有问题需要问你。\n\n请问如何使用Python写一个五子棋。").unwrap(),
            model: Some(String::from_str("openai:gpt-3.5-turbo").unwrap()),
        };
        let chatgpt = Arc::new(Chatgpt::new().unwrap());
        let res = chatgpt.ask(chatgpt_params).await.unwrap();
        println!("{}", res);
    }
}