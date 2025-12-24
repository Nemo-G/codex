//! Standard type to use with the `--wire-api` CLI option.
//! Available when the `cli` feature is enabled for the crate.

use clap::ValueEnum;
use codex_core::WireApi;

#[derive(Clone, Copy, Debug, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum WireApiCliArg {
    /// Use the OpenAI Responses API (POST `/v1/responses`).
    Responses,
    /// Use the OpenAI Chat Completions API (POST `/v1/chat/completions`).
    Chat,
}

impl From<WireApiCliArg> for WireApi {
    fn from(value: WireApiCliArg) -> Self {
        match value {
            WireApiCliArg::Responses => WireApi::Responses,
            WireApiCliArg::Chat => WireApi::Chat,
        }
    }
}
