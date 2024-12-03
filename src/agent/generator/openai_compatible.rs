use anyhow::Result;
use async_trait::async_trait;

use crate::agent::state::SharedState;

use super::{openai::OpenAIClient, ChatOptions, ChatResponse, Client};

pub struct OpenAiCompatibleClient {
    client: OpenAIClient,
}

#[async_trait]
impl Client for OpenAiCompatibleClient {
    fn new(_: &str, _: u16, model_name: &str, _: u32) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let client = OpenAIClient::custom_no_auth(
            "",
            &format!(
                "http://{}{}",
                model_name,
                match model_name.ends_with("/") {
                    true => "",
                    false => "/",
                },
            ),
        )?;

        Ok(Self { client })
    }

    async fn check_native_tools_support(&self) -> Result<bool> {
        self.client.check_native_tools_support().await
    }

    async fn chat(
        &self,
        state: SharedState,
        options: &ChatOptions,
    ) -> anyhow::Result<ChatResponse> {
        self.client.chat(state, options).await
    }
}

#[async_trait]
impl mini_rag::Embedder for OpenAiCompatibleClient {
    async fn embed(&self, text: &str) -> Result<mini_rag::Embeddings> {
        self.client.embed(text).await
    }
}