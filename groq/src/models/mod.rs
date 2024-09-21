use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    /// **distil-whisper-large-v3-en** model from **HuggingFace**.
    #[serde(rename = "distil-whisper-large-v3-en")]
    DistilWhisper,

    /// **gemma2-9b-it** model from **Google**.
    #[serde(rename = "gemma2-9b-it")]
    Gemma29B,

    /// **gemma-7b-it** model from **Google**.
    #[serde(rename = "gemma-7b-it")]
    Gemma7B,

    /// **llama3-groq-70b-8192-tool-use-preview** model from **Groq**.
    #[serde(rename = "llama3-groq-70b-8192-tool-use-preview")]
    Llama3Groq70BToolUse,

    /// **llama3-groq-8b-8192-tool-use-preview** model from **Groq**.
    #[serde(rename = "llama3-groq-8b-8192-tool-use-preview")]
    Llama3Groq8BToolUse,

    /// **llama-3.1-70b-versatile** model from **Meta**.
    #[serde(rename = "llama-3.1-70b-versatile")]
    Llama3170B,

    /// **llama-3.1-70b-instant** model from **Meta**.
    #[serde(rename = "llama-3.1-8b-instant")]
    Llama318B,

    /// **llama-guard-3-8b** model from **Meta**.
    #[serde(rename = "llama-guard-3-8b")]
    LlamaGuard38B,

    /// **llava-v1.5-7b-4096-preview** model from **Haotian Liu**.
    #[serde(rename = "llava-v1.5-7b-4096-preview")]
    Llava157B,

    /// **llama3-70b-8192** model from **Meta**.
    #[serde(rename = "llama3-70b-8192")]
    Llama370B,

    /// **llama3-8b-8192** model from **Meta**.
    #[serde(rename = "llama3-8b-8192")]
    Llama38B,

    /// **mixtral-8x7b-32768** model from **Mistral**.
    #[serde(rename = "mixtral-8x7b-32768")]
    Mixtral87B,

    /// **whisper-large-v3** model from **OpenAI**.
    #[serde(rename = "whisper-large-v3")]
    Whisper,
}
