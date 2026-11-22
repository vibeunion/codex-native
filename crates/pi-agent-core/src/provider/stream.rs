use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};

use crate::types::{AgentEvent, Message, ToolCall};

const MAX_EVENT_BYTES: usize = 4 * 1024 * 1024;
const MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;
const MAX_TOOL_CALLS: usize = 128;
const MAX_TOOL_IDENTITY_BYTES: usize = 1024;
const MAX_TOOL_ARGUMENT_BYTES: usize = 1024 * 1024;

#[derive(Default)]
pub(super) struct ResponseStream {
    line: Vec<u8>,
    data: String,
    skip_lf: bool,
    started: bool,
    received_bytes: usize,
    done: bool,
    received_choice: bool,
    terminal_reason: bool,
    text: String,
    reasoning: String,
    total_tokens: Option<u64>,
    tools: BTreeMap<usize, PendingToolCall>,
    events: Vec<AgentEvent>,
}

impl ResponseStream {
    pub(super) fn push(&mut self, bytes: &[u8]) -> Result<()> {
        self.received_bytes = self.received_bytes.saturating_add(bytes.len());
        if self.received_bytes > MAX_RESPONSE_BYTES {
            bail!("Provider response exceeded 16 MiB");
        }
        for &byte in bytes {
            if self.skip_lf {
                self.skip_lf = false;
                if byte == b'\n' {
                    continue;
                }
            }
            if matches!(byte, b'\r' | b'\n') {
                self.skip_lf = byte == b'\r';
                let line = std::mem::take(&mut self.line);
                if let Some(data) = self.accept_line(&line)? {
                    self.accept_event(&data)?;
                    if self.done {
                        break;
                    }
                }
            } else {
                if self.line.len() >= MAX_EVENT_BYTES {
                    bail!("Provider event exceeded 4 MiB");
                }
                self.line.push(byte);
            }
        }
        Ok(())
    }

    fn accept_line(&mut self, bytes: &[u8]) -> Result<Option<String>> {
        let line = std::str::from_utf8(bytes).context("Provider returned invalid UTF-8")?;
        let line = if std::mem::replace(&mut self.started, true) {
            line
        } else {
            line.strip_prefix('\u{feff}').unwrap_or(line)
        };
        if line.is_empty() && !self.data.is_empty() {
            self.data.pop();
            return Ok(Some(std::mem::take(&mut self.data)));
        }
        let (field, data) = line.split_once(':').unwrap_or((line, ""));
        if field == "data" {
            let data = data.strip_prefix(' ').unwrap_or(data);
            if self.data.len().saturating_add(data.len() + 1) > MAX_EVENT_BYTES {
                bail!("Provider event exceeded 4 MiB");
            }
            self.data.push_str(data);
            self.data.push('\n');
        }
        Ok(None)
    }

    fn accept_event(&mut self, data: &str) -> Result<()> {
        if data.trim() == "[DONE]" {
            self.done = true;
            return Ok(());
        }
        let event: ProviderEvent =
            serde_json::from_str(data).context("Malformed provider event")?;
        if event.error.is_some() {
            bail!("Provider reported a streaming error");
        }
        if event.choices.is_none() && event.usage.is_none() {
            bail!("Provider event is missing choices and usage");
        }
        if let Some(usage) = event.usage {
            self.total_tokens = usage.total_tokens.or(self.total_tokens);
        }
        let mut choices = event.choices.unwrap_or_default().into_iter();
        let Some(first) = choices.next() else {
            return Ok(());
        };
        self.received_choice = true;
        for choice in choices {
            choice.validate_shape()?;
            validate_finish_reason(choice.finish_reason.as_deref())?;
        }
        first.validate_shape()?;
        if self.terminal_reason && first.delta.as_ref().is_some_and(ProviderDelta::has_output) {
            bail!("Provider emitted output after its finish reason");
        }
        if let Some(delta) = first.delta {
            self.accept_delta(delta)?;
        }
        validate_finish_reason(first.finish_reason.as_deref())?;
        self.terminal_reason |= first.finish_reason.is_some();
        Ok(())
    }

    fn accept_delta(&mut self, delta: ProviderDelta) -> Result<()> {
        if let Some(thought) = delta.reasoning_content.or(delta.reasoning) {
            if !thought.is_empty() {
                self.reasoning.push_str(&thought);
                self.events.push(AgentEvent::Thought(thought));
            }
        }
        if let Some(text) = delta.content {
            if !text.is_empty() {
                self.text.push_str(&text);
                self.events.push(AgentEvent::TextDelta(text));
            }
        }
        for call in delta.tool_calls.unwrap_or_default() {
            if call.index >= MAX_TOOL_CALLS {
                bail!("Provider tool index exceeds the 128 call limit");
            }
            self.tools.entry(call.index).or_default().append(call)?;
        }
        Ok(())
    }

    pub(super) fn is_done(&self) -> bool {
        self.done
    }

    pub(super) fn take_events(&mut self) -> Vec<AgentEvent> {
        std::mem::take(&mut self.events)
    }

    pub(super) fn finish(self) -> Result<(Message, Option<u64>)> {
        if !self.received_choice {
            bail!("Provider ended without an assistant choice");
        }
        if !self.done && (!self.terminal_reason || !self.line.is_empty() || !self.data.is_empty()) {
            bail!("Provider disconnected before completion; partial output retained");
        }
        let mut ids = HashSet::new();
        let mut calls = Vec::new();
        for pending in self.tools.into_values() {
            let call = pending.finish()?;
            if !ids.insert(call.id.clone()) {
                bail!("Provider returned duplicate tool call IDs");
            }
            calls.push(call);
        }
        let mut message = Message::assistant(self.text);
        message.reasoning = (!self.reasoning.is_empty()).then_some(self.reasoning);
        message.tool_calls = (!calls.is_empty()).then_some(calls);
        Ok((message, self.total_tokens))
    }
}

#[derive(Deserialize)]
struct ProviderEvent {
    choices: Option<Vec<ProviderChoice>>,
    usage: Option<ProviderUsage>,
    error: Option<Value>,
}

#[derive(Deserialize)]
struct ProviderUsage {
    total_tokens: Option<u64>,
}

#[derive(Deserialize)]
struct ProviderChoice {
    delta: Option<ProviderDelta>,
    finish_reason: Option<String>,
}

impl ProviderChoice {
    fn validate_shape(&self) -> Result<()> {
        if self.delta.is_none() && self.finish_reason.is_none() {
            bail!("Choice is missing delta and finish_reason");
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct ProviderDelta {
    content: Option<String>,
    reasoning_content: Option<String>,
    reasoning: Option<String>,
    tool_calls: Option<Vec<ProviderToolCall>>,
}

impl ProviderDelta {
    fn has_output(&self) -> bool {
        self.content.as_ref().is_some_and(|text| !text.is_empty())
            || self
                .reasoning_content
                .as_ref()
                .is_some_and(|text| !text.is_empty())
            || self.reasoning.as_ref().is_some_and(|text| !text.is_empty())
            || self
                .tool_calls
                .as_ref()
                .is_some_and(|calls| !calls.is_empty())
    }
}

fn validate_finish_reason(reason: Option<&str>) -> Result<()> {
    if let Some(reason) = reason {
        if !matches!(reason, "stop" | "tool_calls") {
            bail!("Provider did not complete the response: {reason}");
        }
    }
    Ok(())
}

#[derive(Deserialize)]
struct ProviderToolCall {
    index: usize,
    id: Option<String>,
    #[serde(rename = "type")]
    kind: Option<String>,
    function: Option<ProviderFunction>,
}

#[derive(Deserialize)]
struct ProviderFunction {
    name: Option<String>,
    arguments: Option<String>,
}

#[derive(Default)]
struct PendingToolCall {
    id: String,
    name: String,
    arguments: String,
}

impl PendingToolCall {
    fn append(&mut self, chunk: ProviderToolCall) -> Result<()> {
        if chunk.kind.is_some_and(|kind| kind != "function") {
            bail!("Provider returned an unsupported tool call type");
        }
        append_bounded(&mut self.id, chunk.id, MAX_TOOL_IDENTITY_BYTES)?;
        if let Some(function) = chunk.function {
            append_bounded(&mut self.name, function.name, MAX_TOOL_IDENTITY_BYTES)?;
            append_bounded(
                &mut self.arguments,
                function.arguments,
                MAX_TOOL_ARGUMENT_BYTES,
            )?;
        }
        Ok(())
    }

    fn finish(self) -> Result<ToolCall> {
        if self.id.trim().is_empty() || self.name.trim().is_empty() {
            bail!("Incomplete tool call identity");
        }
        let arguments: Value =
            serde_json::from_str(&self.arguments).context("Invalid tool arguments")?;
        if !arguments.is_object() {
            bail!("Tool arguments must be a JSON object");
        }
        Ok(ToolCall {
            id: self.id,
            name: self.name,
            arguments,
        })
    }
}

fn append_bounded(target: &mut String, fragment: Option<String>, limit: usize) -> Result<()> {
    if let Some(fragment) = fragment {
        if target.len().saturating_add(fragment.len()) > limit {
            bail!("Provider tool call field exceeded its size limit");
        }
        target.push_str(&fragment);
    }
    Ok(())
}
