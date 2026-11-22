use crate::theme::ThemeColors;
use gpui_kit::base::{Disableable, StyledExt};
use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonVariants};
use gpui_kit::gpui::*;
use gpui_kit::prelude::FluentBuilder;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const MAX_ARTIFACT_BYTES: usize = 256 * 1024;
const MAX_UI_TEXT_BYTES: usize = 16 * 1024;
const MAX_UI_COLLECTION_ITEMS: usize = 256;
const MAX_UI_NODE_DEPTH: usize = 32;
const MAX_UI_NODE_COUNT: usize = 512;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UiNode {
    Card {
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        subtitle: Option<String>,
        #[serde(default)]
        badge: Option<BadgeNode>,
        child: Box<UiNode>,
    },
    Container {
        #[serde(default)]
        direction: LayoutDirection,
        #[serde(default)]
        gap: Option<f32>,
        #[serde(default)]
        align: Option<LayoutAlign>,
        children: Vec<UiNode>,
    },
    Text {
        content: String,
        #[serde(default)]
        variant: TextVariant,
    },
    Badge(BadgeNode),
    Button {
        id: String,
        label: String,
        #[serde(default)]
        variant: ButtonVariant,
        #[serde(default)]
        disabled: bool,
        #[serde(default)]
        action: Option<UiAction>,
    },
    ChoiceGroup {
        id: String,
        #[serde(default)]
        label: Option<String>,
        options: Vec<ChoiceOption>,
    },
    Select {
        id: String,
        #[serde(default)]
        label: Option<String>,
        options: Vec<String>,
        #[serde(default)]
        default_value: Option<String>,
        #[serde(default)]
        button_label: Option<String>,
    },
    InputField {
        id: String,
        #[serde(default)]
        label: Option<String>,
        #[serde(default)]
        placeholder: Option<String>,
        #[serde(default)]
        default_value: Option<String>,
        #[serde(default)]
        button_label: Option<String>,
        #[serde(default)]
        action_prompt: Option<String>,
    },
    Toggle {
        id: String,
        label: String,
        #[serde(default)]
        description: Option<String>,
        #[serde(default)]
        checked: bool,
        #[serde(default)]
        action: Option<UiAction>,
    },
    CheckboxGroup {
        id: String,
        #[serde(default)]
        label: Option<String>,
        items: Vec<CheckboxItem>,
    },
    KeyValueList {
        items: Vec<KeyValueItem>,
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Progress {
        value: f32,
        #[serde(default = "default_max")]
        max: f32,
        #[serde(default)]
        label: Option<String>,
        #[serde(default)]
        tone: BadgeTone,
    },
    Metric {
        label: String,
        value: String,
        #[serde(default)]
        change: Option<String>,
        #[serde(default)]
        change_tone: Option<BadgeTone>,
        #[serde(default)]
        subtext: Option<String>,
    },
    BarChart {
        #[serde(default)]
        title: Option<String>,
        items: Vec<BarItem>,
        #[serde(default)]
        direction: LayoutDirection,
        #[serde(default)]
        max_value: Option<f32>,
    },
    LineChart {
        #[serde(default)]
        title: Option<String>,
        points: Vec<f32>,
        #[serde(default)]
        tone: Option<BadgeTone>,
        #[serde(default)]
        summary: Option<String>,
    },
    Sparkline {
        #[serde(default)]
        title: Option<String>,
        points: Vec<f32>,
        #[serde(default)]
        tone: Option<BadgeTone>,
        #[serde(default)]
        summary: Option<String>,
    },
    Callout {
        #[serde(default)]
        tone: BadgeTone,
        #[serde(default)]
        title: Option<String>,
        message: String,
    },
    Accordion {
        title: String,
        #[serde(default)]
        subtitle: Option<String>,
        child: Box<UiNode>,
    },
    Artifact {
        id: String,
        title: String,
        #[serde(default = "default_artifact_kind")]
        kind: String,
        content: String,
    },
    PieChart {
        #[serde(default)]
        title: Option<String>,
        slices: Vec<PieSlice>,
        #[serde(default)]
        donut: bool,
        #[serde(default)]
        summary: Option<String>,
    },
    Form {
        id: String,
        #[serde(default)]
        title: Option<String>,
        #[serde(default)]
        description: Option<String>,
        #[serde(default)]
        submit_label: Option<String>,
        fields: Vec<FormField>,
    },
    Rating {
        id: String,
        #[serde(default)]
        label: Option<String>,
        #[serde(default = "default_max_rating")]
        max: u32,
        #[serde(default)]
        value: u32,
    },
    Divider,
}

fn default_max() -> f32 {
    100.0
}

fn default_max_rating() -> u32 {
    5
}

fn default_artifact_kind() -> String {
    "code".to_string()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PieSlice {
    pub label: String,
    pub value: f32,
    #[serde(default)]
    pub tone: Option<BadgeTone>,
    #[serde(default)]
    pub display_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormField {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub field_type: FormFieldType,
    #[serde(default)]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FormFieldType {
    #[default]
    Text,
    Select,
    Switch,
    Textarea,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LayoutDirection {
    #[default]
    Column,
    Row,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LayoutAlign {
    #[default]
    Start,
    Center,
    End,
    Between,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextVariant {
    #[default]
    Body,
    Title,
    Heading,
    Subtle,
    Code,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BadgeNode {
    pub label: String,
    #[serde(default)]
    pub tone: BadgeTone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BadgeTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValueItem {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub badge: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub tone: Option<BadgeTone>,
    #[serde(default)]
    pub prompt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckboxItem {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub checked: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BarItem {
    pub label: String,
    pub value: f32,
    #[serde(default)]
    pub display_value: Option<String>,
    #[serde(default)]
    pub tone: Option<BadgeTone>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiAction {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub prompt: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub copy_text: Option<String>,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContentSegment {
    Markdown(String),
    GenerativeUi(Box<UiNode>),
}

/// Parses message text containing optional ```json-ui, ```ui, ```generative-ui,
/// ```adaptive-card, ```html:artifact, or <agent-embed> blocks into a sequence of Markdown text and native UI nodes.
#[cfg(test)]
pub fn parse_message_segments(text: &str) -> Vec<ContentSegment> {
    parse_message_segments_internal(text, None)
}

pub fn parse_message_segments_with_workspace(
    text: &str,
    workspace_dir: &Path,
) -> Vec<ContentSegment> {
    parse_message_segments_internal(text, Some(workspace_dir))
}

fn parse_message_segments_internal(
    text: &str,
    workspace_dir: Option<&Path>,
) -> Vec<ContentSegment> {
    let mut segments = Vec::new();
    let mut markdown_start = 0;
    let mut search_offset = 0;

    while let Some((relative_start, marker)) = next_message_marker(&text[search_offset..]) {
        let block_start = search_offset + relative_start;
        let Some(block) = parse_message_block(&text[block_start..], marker, workspace_dir) else {
            search_offset = block_start + marker.text.len();
            continue;
        };
        search_offset = block_start + block.length;
        if let Some(mut node) = block.node.filter(is_safe_ui_node) {
            if block_start > markdown_start {
                segments.push(ContentSegment::Markdown(
                    text[markdown_start..block_start].to_string(),
                ));
            }
            if !matches!(marker.kind, MessageBlockKind::Json) {
                if let UiNode::Artifact { id, .. } = &mut node {
                    let prefix = if matches!(marker.kind, MessageBlockKind::Embed) {
                        "embed"
                    } else {
                        "artifact"
                    };
                    *id = format!("{prefix}_{block_start}");
                }
            }
            segments.push(ContentSegment::GenerativeUi(Box::new(node)));
            markdown_start = search_offset;
        }
    }
    if markdown_start < text.len() {
        segments.push(ContentSegment::Markdown(text[markdown_start..].to_string()));
    }
    segments
}

#[derive(Clone, Copy)]
enum MessageBlockKind {
    Json,
    Html,
    Embed,
}

#[derive(Clone, Copy)]
struct MessageMarker {
    text: &'static str,
    kind: MessageBlockKind,
}

struct ParsedMessageBlock {
    length: usize,
    node: Option<UiNode>,
}

fn next_message_marker(text: &str) -> Option<(usize, MessageMarker)> {
    use MessageBlockKind::{Embed, Html, Json};
    [
        ("```json-ui", Json),
        ("```ui", Json),
        ("```generative-ui", Json),
        ("```adaptive-card", Json),
        ("```html:artifact", Html),
        ("```artifact:html", Html),
        ("```html:embed", Html),
        ("<agent-embed", Embed),
    ]
    .into_iter()
    .filter_map(|(token, kind)| {
        find_ascii_case_insensitive(text, token)
            .map(|start| (start, MessageMarker { text: token, kind }))
    })
    .min_by_key(|(start, _)| *start)
}

fn parse_message_block(
    block: &str,
    marker: MessageMarker,
    workspace_dir: Option<&Path>,
) -> Option<ParsedMessageBlock> {
    match marker.kind {
        MessageBlockKind::Json => parse_json_block(block, marker.text.len()),
        MessageBlockKind::Html => parse_html_block(block, marker.text.len()),
        MessageBlockKind::Embed => parse_embed_block(block, workspace_dir),
    }
}

fn parse_json_block(block: &str, marker_length: usize) -> Option<ParsedMessageBlock> {
    let content_end = marker_length + block[marker_length..].find("```")?;
    let candidate = block[marker_length..content_end].trim();
    let node = if candidate.len() <= MAX_ARTIFACT_BYTES {
        serde_json::from_str(candidate).ok()
    } else {
        None
    };
    Some(ParsedMessageBlock {
        length: content_end + 3,
        node,
    })
}

fn parse_html_block(block: &str, marker_length: usize) -> Option<ParsedMessageBlock> {
    let line_end = marker_length + block[marker_length..].find('\n')?;
    let content_start = line_end + 1;
    let content_end = content_start + block[content_start..].find("```")?;
    let content = block[content_start..content_end].trim();
    let node = (content.len() <= MAX_ARTIFACT_BYTES).then(|| UiNode::Artifact {
        id: String::new(),
        title: attribute_value(&block[marker_length..line_end], "title")
            .unwrap_or_else(|| "HTML 交互产物 (Interactive Artifact)".to_string()),
        kind: "html".to_string(),
        content: content.to_string(),
    });
    Some(ParsedMessageBlock {
        length: content_end + 3,
        node,
    })
}

fn parse_embed_block(block: &str, workspace_dir: Option<&Path>) -> Option<ParsedMessageBlock> {
    let after_marker = &block["<agent-embed".len()..];
    if !after_marker.starts_with(|ch: char| ch.is_ascii_whitespace() || ch == '/' || ch == '>') {
        return None;
    }
    let header_end = tag_header_end(block)?;
    let header = &block["<agent-embed".len()..header_end];
    let content_start = header_end + 1;
    let (length, inline) = if header.trim_end().ends_with('/') {
        (content_start, "")
    } else {
        let close = find_ascii_case_insensitive(&block[content_start..], "</agent-embed>")?;
        let content_end = content_start + close;
        (
            content_end + "</agent-embed>".len(),
            &block[content_start..content_end],
        )
    };
    let node = (inline.trim().len() <= MAX_ARTIFACT_BYTES).then(|| UiNode::Artifact {
        id: String::new(),
        title: attribute_value(header, "title")
            .unwrap_or_else(|| "交互组件 (Interactive Widget)".to_string()),
        kind: "html".to_string(),
        content: embedded_content(inline, header, workspace_dir),
    });
    Some(ParsedMessageBlock { length, node })
}

fn tag_header_end(block: &str) -> Option<usize> {
    let mut quote = None;
    for (index, ch) in block.char_indices() {
        match (quote, ch) {
            (Some(open), close) if open == close => quote = None,
            (None, '\'' | '"') => quote = Some(ch),
            (None, '>') => return Some(index),
            _ => {}
        }
    }
    None
}

fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    let needle_bytes = needle.as_bytes();
    haystack.char_indices().find_map(|(index, _)| {
        haystack
            .as_bytes()
            .get(index..index + needle_bytes.len())
            .filter(|candidate| candidate.eq_ignore_ascii_case(needle_bytes))
            .map(|_| index)
    })
}

fn attribute_value(header: &str, name: &str) -> Option<String> {
    let mut remaining = header.trim();
    while !remaining.is_empty() {
        let name_end = remaining
            .find(|ch: char| ch.is_ascii_whitespace() || ch == '=' || ch == '/')
            .unwrap_or(remaining.len());
        let attribute_name = &remaining[..name_end];
        let after_name = remaining[name_end..].trim_start();
        let Some(after_equals) = after_name.strip_prefix('=') else {
            if name_end == 0 {
                return None;
            }
            remaining = after_name;
            continue;
        };
        let (attribute, rest) = attribute_contents(after_equals.trim_start())?;
        if attribute_name.eq_ignore_ascii_case(name) {
            return Some(attribute.to_string());
        }
        remaining = rest.trim_start();
    }
    None
}

fn attribute_contents(text: &str) -> Option<(&str, &str)> {
    let first = text.chars().next()?;
    if matches!(first, '\'' | '"') {
        let end = 1 + text[1..].find(first)?;
        Some((&text[1..end], &text[end + 1..]))
    } else {
        let end = text
            .find(|ch: char| ch.is_ascii_whitespace() || ch == '>')
            .unwrap_or(text.len());
        Some((&text[..end], &text[end..]))
    }
}

fn embedded_content(
    inline_content: &str,
    tag_header: &str,
    workspace_dir: Option<&Path>,
) -> String {
    let inline_content = inline_content.trim();
    let Some(source) = attribute_value(tag_header, "src") else {
        return inline_content.to_string();
    };

    let Some(workspace_dir) = workspace_dir else {
        return if inline_content.is_empty() {
            "<p>Embedded file sources require a workspace context.</p>".to_string()
        } else {
            inline_content.to_string()
        };
    };

    match read_workspace_artifact(workspace_dir, &source) {
        Ok(content) => content,
        Err(_) if !inline_content.is_empty() => inline_content.to_string(),
        Err(_) => "<p>Embedded file is unavailable inside the workspace.</p>".to_string(),
    }
}

fn read_workspace_artifact(workspace_dir: &Path, source: &str) -> std::io::Result<String> {
    let source = source.strip_prefix("file://").unwrap_or(source);
    let root = std::fs::canonicalize(workspace_dir)?;
    let candidate = if Path::new(source).is_absolute() {
        Path::new(source).to_path_buf()
    } else {
        root.join(source)
    };
    let path = std::fs::canonicalize(candidate)?;
    if !path.starts_with(&root) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "embedded file escapes workspace",
        ));
    }
    let metadata = std::fs::metadata(&path)?;
    if !metadata.is_file() || metadata.len() > MAX_ARTIFACT_BYTES as u64 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "embedded file is not a supported artifact",
        ));
    }
    read_bounded_artifact(std::fs::File::open(path)?)
}

fn read_bounded_artifact(reader: impl std::io::Read) -> std::io::Result<String> {
    use std::io::Read;
    let mut content = String::new();
    reader
        .take(MAX_ARTIFACT_BYTES as u64 + 1)
        .read_to_string(&mut content)?;
    if content.len() > MAX_ARTIFACT_BYTES {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "embedded file exceeds artifact size limit",
        ));
    }
    Ok(content)
}

fn bounded_text(value: &str) -> bool {
    value.len() <= MAX_UI_TEXT_BYTES
}

fn bounded_optional_text(value: Option<&String>) -> bool {
    value.is_none_or(|text| bounded_text(text))
}

fn bounded_action(action: Option<&UiAction>) -> bool {
    let Some(action) = action else {
        return true;
    };
    bounded_text(&action.id)
        && bounded_text(&action.label)
        && bounded_optional_text(action.prompt.as_ref())
        && bounded_optional_text(action.url.as_ref())
        && bounded_optional_text(action.copy_text.as_ref())
        && action.payload.as_ref().is_none_or(|payload| {
            serde_json::to_string(payload).is_ok_and(|json| json.len() <= MAX_UI_TEXT_BYTES)
        })
}

fn bounded_finite(value: f32) -> bool {
    value.is_finite()
}

fn is_safe_ui_node(node: &UiNode) -> bool {
    let mut count = 0;
    validate_ui_node(node, 0, &mut count)
}

fn validate_ui_node(node: &UiNode, depth: usize, count: &mut usize) -> bool {
    if depth > MAX_UI_NODE_DEPTH || *count >= MAX_UI_NODE_COUNT {
        return false;
    }
    *count += 1;
    match node {
        UiNode::Card {
            title,
            subtitle,
            badge,
            child,
        } => {
            bounded_optional_text(title.as_ref())
                && bounded_optional_text(subtitle.as_ref())
                && badge
                    .as_ref()
                    .is_none_or(|badge| bounded_text(&badge.label))
                && validate_ui_node(child, depth + 1, count)
        }
        UiNode::Container { gap, children, .. } => {
            gap.is_none_or(|value| bounded_finite(value) && (0.0..=128.0).contains(&value))
                && children.len() <= MAX_UI_COLLECTION_ITEMS
                && children
                    .iter()
                    .all(|child| validate_ui_node(child, depth + 1, count))
        }
        UiNode::Text { content, .. } => bounded_text(content),
        UiNode::Badge(badge) => bounded_text(&badge.label),
        UiNode::Button {
            id, label, action, ..
        } => bounded_text(id) && bounded_text(label) && bounded_action(action.as_ref()),
        UiNode::ChoiceGroup { id, label, options } => {
            bounded_text(id)
                && bounded_optional_text(label.as_ref())
                && options.len() <= MAX_UI_COLLECTION_ITEMS
                && options.iter().all(|option| {
                    bounded_text(&option.label)
                        && bounded_text(&option.value)
                        && bounded_optional_text(option.prompt.as_ref())
                })
        }
        UiNode::Select {
            id,
            label,
            options,
            default_value,
            button_label,
        } => {
            bounded_text(id)
                && bounded_optional_text(label.as_ref())
                && options.len() <= MAX_UI_COLLECTION_ITEMS
                && options.iter().all(|option| bounded_text(option))
                && bounded_optional_text(default_value.as_ref())
                && bounded_optional_text(button_label.as_ref())
        }
        UiNode::InputField {
            id,
            label,
            placeholder,
            default_value,
            button_label,
            action_prompt,
        } => {
            bounded_text(id)
                && bounded_optional_text(label.as_ref())
                && bounded_optional_text(placeholder.as_ref())
                && bounded_optional_text(default_value.as_ref())
                && bounded_optional_text(button_label.as_ref())
                && bounded_optional_text(action_prompt.as_ref())
        }
        UiNode::Toggle {
            id,
            label,
            description,
            action,
            ..
        } => {
            bounded_text(id)
                && bounded_text(label)
                && bounded_optional_text(description.as_ref())
                && bounded_action(action.as_ref())
        }
        UiNode::CheckboxGroup { id, label, items } => {
            bounded_text(id)
                && bounded_optional_text(label.as_ref())
                && items.len() <= MAX_UI_COLLECTION_ITEMS
                && items
                    .iter()
                    .all(|item| bounded_text(&item.label) && bounded_text(&item.value))
        }
        UiNode::KeyValueList { items } => {
            items.len() <= MAX_UI_COLLECTION_ITEMS
                && items.iter().all(|item| {
                    bounded_text(&item.key)
                        && bounded_text(&item.value)
                        && bounded_optional_text(item.badge.as_ref())
                })
        }
        UiNode::Table { headers, rows } => {
            headers.len() <= MAX_UI_COLLECTION_ITEMS
                && rows.len() <= MAX_UI_COLLECTION_ITEMS
                && headers.iter().all(|header| bounded_text(header))
                && rows.iter().all(|row| {
                    row.len() <= MAX_UI_COLLECTION_ITEMS
                        && row.iter().all(|cell| bounded_text(cell))
                })
        }
        UiNode::Progress {
            value, max, label, ..
        } => {
            bounded_finite(*value)
                && bounded_finite(*max)
                && *max > 0.0
                && bounded_optional_text(label.as_ref())
        }
        UiNode::Metric {
            label,
            value,
            change,
            subtext,
            ..
        } => {
            bounded_text(label)
                && bounded_text(value)
                && bounded_optional_text(change.as_ref())
                && bounded_optional_text(subtext.as_ref())
        }
        UiNode::BarChart {
            title,
            items,
            max_value,
            ..
        } => {
            bounded_optional_text(title.as_ref())
                && items.len() <= MAX_UI_COLLECTION_ITEMS
                && items.iter().all(|item| {
                    bounded_text(&item.label)
                        && bounded_finite(item.value)
                        && bounded_optional_text(item.display_value.as_ref())
                })
                && max_value.is_none_or(|value| bounded_finite(value) && value > 0.0)
        }
        UiNode::LineChart {
            title,
            points,
            summary,
            ..
        }
        | UiNode::Sparkline {
            title,
            points,
            summary,
            ..
        } => {
            bounded_optional_text(title.as_ref())
                && bounded_optional_text(summary.as_ref())
                && points.len() <= MAX_UI_COLLECTION_ITEMS
                && points.iter().all(|point| bounded_finite(*point))
        }
        UiNode::Callout { title, message, .. } => {
            bounded_optional_text(title.as_ref()) && bounded_text(message)
        }
        UiNode::Accordion {
            title,
            subtitle,
            child,
        } => {
            bounded_text(title)
                && bounded_optional_text(subtitle.as_ref())
                && validate_ui_node(child, depth + 1, count)
        }
        UiNode::Artifact {
            id,
            title,
            kind,
            content,
        } => {
            bounded_text(id)
                && bounded_text(title)
                && bounded_text(kind)
                && content.len() <= MAX_ARTIFACT_BYTES
        }
        UiNode::PieChart {
            title,
            slices,
            summary,
            ..
        } => {
            bounded_optional_text(title.as_ref())
                && bounded_optional_text(summary.as_ref())
                && slices.len() <= MAX_UI_COLLECTION_ITEMS
                && slices.iter().all(|slice| {
                    bounded_text(&slice.label)
                        && bounded_finite(slice.value)
                        && slice.value >= 0.0
                        && bounded_optional_text(slice.display_value.as_ref())
                })
        }
        UiNode::Form {
            id,
            title,
            description,
            submit_label,
            fields,
        } => {
            bounded_text(id)
                && bounded_optional_text(title.as_ref())
                && bounded_optional_text(description.as_ref())
                && bounded_optional_text(submit_label.as_ref())
                && fields.len() <= MAX_UI_COLLECTION_ITEMS
                && fields.iter().all(|field| {
                    bounded_text(&field.id)
                        && bounded_text(&field.label)
                        && bounded_optional_text(field.placeholder.as_ref())
                        && bounded_optional_text(field.default_value.as_ref())
                        && field.options.len() <= MAX_UI_COLLECTION_ITEMS
                        && field.options.iter().all(|option| bounded_text(option))
                })
        }
        UiNode::Rating {
            id,
            label,
            max,
            value,
            ..
        } => {
            bounded_text(id)
                && bounded_optional_text(label.as_ref())
                && (1..=10).contains(max)
                && value <= max
        }
        UiNode::Divider => true,
    }
}

/// Injects host semantic design-system tokens and allowed Tailwind CDN into an HTML artifact.
pub fn inject_theme_variables_to_html(content: &str, is_dark: bool, c: &ThemeColors) -> String {
    if content.contains("--card:") || content.contains("--background:") {
        return content.to_string();
    }

    let mode_class = if is_dark { "dark" } else { "light" };
    let theme_style = format!(
        "<style id=\"app-theme-tokens\">\n\
        :root {{\n\
            --background: #{:06x};\n\
            --content: #{:06x};\n\
            --card: #{:06x};\n\
            --sidebar: #{:06x};\n\
            --border: rgba({}, {}, {}, 0.12);\n\
            --foreground: #{:06x};\n\
            --muted-foreground: #{:06x};\n\
            --placeholder: rgba({}, {}, {}, 0.45);\n\
            --primary: #{:06x};\n\
            --primary-foreground: #{:06x};\n\
            --secondary: #{:06x};\n\
            --secondary-foreground: #{:06x};\n\
            --accent: #{:06x};\n\
        }}\n\
        body {{ background-color: var(--background); color: var(--foreground); font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }}\n\
        </style>",
        c.root_bg,
        c.panel_bg,
        c.card_bg,
        c.sidebar_bg,
        (c.text_primary >> 16) & 0xff,
        (c.text_primary >> 8) & 0xff,
        c.text_primary & 0xff,
        c.text_primary,
        c.foreground_subtle & 0x00ffffff,
        (c.text_primary >> 16) & 0xff,
        (c.text_primary >> 8) & 0xff,
        c.text_primary & 0xff,
        if is_dark { 0xffffff } else { 0x000000 },
        if is_dark { 0x000000 } else { 0xffffff },
        c.card_user_bg,
        c.text_primary,
        c.link,
    );

    if let Some(pos) = find_ascii_case_insensitive(content, "<head>") {
        let insert_idx = pos + 6;
        format!(
            "{}\n<script src=\"https://www.gstatic.com/antigravity/web/dev/tailwindcss.min.js\"></script>\n{}{}",
            &content[..insert_idx],
            theme_style,
            &content[insert_idx..]
        )
    } else if let Some(pos) = find_ascii_case_insensitive(content, "<html>") {
        let insert_idx = pos + 6;
        format!(
            "{}<head>\n<script src=\"https://www.gstatic.com/antigravity/web/dev/tailwindcss.min.js\"></script>\n{}</head>{}",
            &content[..insert_idx],
            theme_style,
            &content[insert_idx..]
        )
    } else {
        format!(
            "<!DOCTYPE html>\n<html class=\"{}\">\n<head>\n<meta charset=\"utf-8\">\n<script src=\"https://www.gstatic.com/antigravity/web/dev/tailwindcss.min.js\"></script>\n{}\n</head>\n<body class=\"p-4\">\n{}\n</body>\n</html>",
            mode_class,
            theme_style,
            content
        )
    }
}

/// Renders a `UiNode` into native GPUI elements using semantic theme colors and typography.
pub fn render_ui_node<T: 'static>(
    node: &UiNode,
    c: &ThemeColors,
    font_size: f32,
    is_dark: bool,
    on_action: impl Fn(&UiAction, &mut T, &mut Window, &mut Context<T>) + 'static + Clone,
    cx: &mut Context<T>,
) -> AnyElement {
    match node {
        UiNode::Card {
            title,
            subtitle,
            badge,
            child,
        } => {
            let mut header = div().w_full().h_flex().items_center().justify_between();
            let has_header = title.is_some() || subtitle.is_some() || badge.is_some();

            if title.is_some() || subtitle.is_some() {
                let mut title_col = div().v_flex().gap(px(2.));
                if let Some(t) = title {
                    title_col = title_col.child(
                        div()
                            .text_size(px(font_size + 1.))
                            .font_semibold()
                            .text_color(rgb(c.text_primary))
                            .child(t.clone()),
                    );
                }
                if let Some(s) = subtitle {
                    title_col = title_col.child(
                        div()
                            .text_size(px(font_size - 1.))
                            .text_color(rgba(c.foreground_subtle))
                            .child(s.clone()),
                    );
                }
                header = header.child(title_col);
            }

            if let Some(b) = badge {
                header = header.child(render_badge(b, c));
            }

            div()
                .w_full()
                .p_4()
                .rounded(px(12.))
                .bg(rgb(c.card_bg))
                .border_1()
                .border_color(rgba(c.card_border))
                .shadow_xs()
                .v_flex()
                .gap(px(12.))
                .when(has_header, |card| card.child(header))
                .child(render_ui_node(child, c, font_size, is_dark, on_action, cx))
                .into_any_element()
        }
        UiNode::Container {
            direction,
            gap,
            align,
            children,
        } => {
            let mut container = div().w_full();
            match direction {
                LayoutDirection::Column => {
                    container = container.v_flex();
                }
                LayoutDirection::Row => {
                    container = container.h_flex();
                }
            }

            let g = gap.unwrap_or(8.);
            container = container.gap(px(g));

            match align.unwrap_or_default() {
                LayoutAlign::Start => {
                    container = container.items_start();
                }
                LayoutAlign::Center => {
                    container = container.items_center();
                }
                LayoutAlign::End => {
                    container = container.items_end();
                }
                LayoutAlign::Between => {
                    container = container.justify_between();
                }
            }

            let on_action_clone = on_action;
            container
                .children(children.iter().map(|child| {
                    render_ui_node(child, c, font_size, is_dark, on_action_clone.clone(), cx)
                }))
                .into_any_element()
        }
        UiNode::Text { content, variant } => {
            let mut text_el = div().child(content.clone());
            match variant {
                TextVariant::Title => {
                    text_el = text_el
                        .text_size(px(font_size + 4.))
                        .font_semibold()
                        .text_color(rgb(c.text_primary));
                }
                TextVariant::Heading => {
                    text_el = text_el
                        .text_size(px(font_size + 2.))
                        .font_medium()
                        .text_color(rgb(c.text_primary));
                }
                TextVariant::Body => {
                    text_el = text_el
                        .text_size(px(font_size))
                        .text_color(rgb(c.text_primary));
                }
                TextVariant::Subtle => {
                    text_el = text_el
                        .text_size(px(font_size - 1.))
                        .text_color(rgba(c.foreground_subtle));
                }
                TextVariant::Code => {
                    text_el = text_el
                        .text_size(px(font_size - 1.))
                        .font_family(".AppleSystemUIFontMonospaced")
                        .px_1()
                        .rounded(px(4.))
                        .bg(rgba(c.inline_code_bg))
                        .text_color(rgb(c.text_primary));
                }
            }
            text_el.into_any_element()
        }
        UiNode::Badge(badge) => render_badge(badge, c).into_any_element(),
        UiNode::Button {
            id,
            label,
            variant,
            disabled,
            action,
        } => {
            let act = action.clone().unwrap_or_else(|| UiAction {
                id: id.clone(),
                label: label.clone(),
                prompt: None,
                url: None,
                copy_text: None,
                payload: None,
            });

            let mut btn = Button::new(format!("gen-ui-btn-{}", id))
                .h(px(30.))
                .rounded(px(8.))
                .px_3()
                .disabled(*disabled)
                .child(
                    div()
                        .text_size(px(font_size - 1.))
                        .font_medium()
                        .child(label.clone()),
                );

            match variant {
                ButtonVariant::Primary => {
                    let brand_btn = if is_dark {
                        ButtonCustomVariant::new(cx)
                            .color(rgb(0xffffff).into())
                            .foreground(rgb(0x000000).into())
                            .hover(rgba(0xffffffcc).into())
                            .active(rgba(0xffffffe6).into())
                    } else {
                        ButtonCustomVariant::new(cx)
                            .color(rgb(0x000000).into())
                            .foreground(rgb(0xffffff).into())
                            .hover(rgba(0x000000cc).into())
                            .active(rgba(0x000000e6).into())
                    };
                    btn = btn.custom(brand_btn);
                }
                ButtonVariant::Secondary => {
                    btn = btn.secondary();
                }
                ButtonVariant::Outline => {
                    btn = btn.outline();
                }
                ButtonVariant::Ghost => {
                    btn = btn.ghost();
                }
                ButtonVariant::Danger => {
                    btn = btn.danger();
                }
            }

            let on_action_cb = on_action;
            btn.on_click(cx.listener(move |this, _, window, cx| {
                on_action_cb(&act, this, window, cx);
            }))
            .into_any_element()
        }
        UiNode::ChoiceGroup { id, label, options } => {
            let mut col = div().w_full().v_flex().gap(px(6.));
            if let Some(lbl) = label {
                col = col.child(
                    div()
                        .text_size(px(font_size - 2.))
                        .font_medium()
                        .text_color(rgba(c.foreground_subtle))
                        .child(lbl.clone()),
                );
            }
            let mut row = div().w_full().h_flex().flex_wrap().gap(px(6.));
            for opt in options {
                let opt_val = opt.value.clone();
                let opt_lbl = opt.label.clone();
                let act = UiAction {
                    id: format!("{}:{}", id, opt_val),
                    label: opt_lbl.clone(),
                    prompt: opt
                        .prompt
                        .clone()
                        .or_else(|| Some(format!("选择【{}】", opt_lbl))),
                    url: None,
                    copy_text: None,
                    payload: Some(serde_json::json!({ "id": id, "value": opt_val })),
                };
                let on_action_cb = on_action.clone();
                let opt_btn = Button::new(format!("choice-{}-{}", id, opt_val))
                    .h(px(28.))
                    .rounded(px(6.))
                    .px_3()
                    .secondary()
                    .child(
                        div()
                            .text_size(px(font_size - 1.))
                            .font_medium()
                            .child(opt_lbl),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_action_cb(&act, this, window, cx);
                    }));
                row = row.child(opt_btn);
            }
            col.child(row).into_any_element()
        }
        UiNode::Select {
            id,
            label,
            options,
            default_value,
            button_label,
        } => {
            let mut col = div().w_full().v_flex().gap(px(6.));
            if let Some(lbl) = label {
                col = col.child(
                    div()
                        .text_size(px(font_size - 2.))
                        .font_medium()
                        .text_color(rgba(c.foreground_subtle))
                        .child(lbl.clone()),
                );
            }
            let selected_val = default_value
                .as_deref()
                .or_else(|| options.first().map(|s| s.as_str()))
                .unwrap_or("")
                .to_string();
            let btn_txt = button_label.as_deref().unwrap_or("确认");
            let act = UiAction {
                id: id.clone(),
                label: format!("选择: {}", selected_val),
                prompt: Some(format!("已选择【{}】", selected_val)),
                url: None,
                copy_text: None,
                payload: Some(serde_json::json!({ "id": id, "selected": selected_val })),
            };
            let on_action_cb = on_action;

            let select_box = div()
                .w_full()
                .h_flex()
                .items_center()
                .justify_between()
                .gap(px(8.))
                .p_2()
                .rounded(px(8.))
                .bg(rgba(c.card_user_bg))
                .border_1()
                .border_color(rgba(c.card_border))
                .child(
                    div()
                        .h_flex()
                        .items_center()
                        .gap(px(6.))
                        .child(
                            div()
                                .text_size(px(font_size - 1.))
                                .font_medium()
                                .text_color(rgb(c.text_primary))
                                .child(selected_val),
                        )
                        .child(
                            div()
                                .text_size(px(font_size - 2.))
                                .text_color(rgba(c.foreground_subtle))
                                .child("▾"),
                        ),
                )
                .child(
                    Button::new(format!("gen-ui-sel-btn-{}", id))
                        .h(px(26.))
                        .rounded(px(6.))
                        .px_2()
                        .secondary()
                        .child(
                            div()
                                .text_size(px(font_size - 2.))
                                .font_medium()
                                .child(btn_txt.to_string()),
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            on_action_cb(&act, this, window, cx);
                        })),
                );
            col.child(select_box).into_any_element()
        }
        UiNode::InputField {
            id,
            label,
            placeholder,
            default_value,
            button_label,
            action_prompt,
        } => {
            let mut col = div().w_full().v_flex().gap(px(6.));
            if let Some(lbl) = label {
                col = col.child(
                    div()
                        .text_size(px(font_size - 2.))
                        .font_medium()
                        .text_color(rgba(c.foreground_subtle))
                        .child(lbl.clone()),
                );
            }
            let btn_txt = button_label.as_deref().unwrap_or("提交");
            let prompt_val = action_prompt
                .clone()
                .unwrap_or_else(|| default_value.clone().unwrap_or_default());
            let act = UiAction {
                id: id.clone(),
                label: btn_txt.to_string(),
                prompt: Some(prompt_val),
                url: None,
                copy_text: None,
                payload: Some(serde_json::json!({ "id": id, "value": default_value })),
            };
            let display_val = default_value
                .as_deref()
                .or(placeholder.as_deref())
                .unwrap_or("");
            let on_action_cb = on_action;

            let input_box = div()
                .w_full()
                .h_flex()
                .items_center()
                .justify_between()
                .gap(px(8.))
                .p_2()
                .rounded(px(8.))
                .bg(rgba(c.card_user_bg))
                .border_1()
                .border_color(rgba(c.card_border))
                .child(
                    div()
                        .flex_1()
                        .text_size(px(font_size - 1.))
                        .text_color(if default_value.is_some() {
                            rgb(c.text_primary)
                        } else {
                            rgba(c.foreground_subtlest)
                        })
                        .child(display_val.to_string()),
                )
                .child(
                    Button::new(format!("gen-ui-input-btn-{}", id))
                        .h(px(26.))
                        .rounded(px(6.))
                        .px_2()
                        .secondary()
                        .child(
                            div()
                                .text_size(px(font_size - 2.))
                                .font_medium()
                                .child(btn_txt.to_string()),
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            on_action_cb(&act, this, window, cx);
                        })),
                );
            col.child(input_box).into_any_element()
        }
        UiNode::Toggle {
            id,
            label,
            description,
            checked,
            action,
        } => {
            let act = action.clone().unwrap_or_else(|| UiAction {
                id: id.clone(),
                label: label.clone(),
                prompt: Some(format!(
                    "切换【{}】状态为：{}",
                    label,
                    if !checked { "开启" } else { "关闭" }
                )),
                url: None,
                copy_text: None,
                payload: Some(serde_json::json!({ "id": id, "checked": !checked })),
            });
            let on_action_cb = on_action;
            let knob = div().w(px(10.)).h(px(10.)).rounded_full().bg(if *checked {
                rgb(c.accent_green)
            } else {
                rgba(c.foreground_subtle)
            });

            let toggle_btn = Button::new(format!("toggle-{}", id))
                .h(px(26.))
                .rounded(px(6.))
                .px_2()
                .ghost()
                .child(div().h_flex().items_center().gap(px(6.)).child(knob).child(
                    div().text_size(px(11.)).font_medium().child(if *checked {
                        "已开启"
                    } else {
                        "已关闭"
                    }),
                ))
                .on_click(cx.listener(move |this, _, window, cx| {
                    on_action_cb(&act, this, window, cx);
                }));

            div()
                .w_full()
                .h_flex()
                .items_center()
                .justify_between()
                .py_1()
                .child(
                    div()
                        .v_flex()
                        .gap(px(2.))
                        .child(
                            div()
                                .text_size(px(font_size - 1.))
                                .font_medium()
                                .text_color(rgb(c.text_primary))
                                .child(label.clone()),
                        )
                        .when_some(description.as_ref(), |this, desc| {
                            this.child(
                                div()
                                    .text_size(px(font_size - 2.))
                                    .text_color(rgba(c.foreground_subtle))
                                    .child(desc.clone()),
                            )
                        }),
                )
                .child(toggle_btn)
                .into_any_element()
        }
        UiNode::CheckboxGroup { id, label, items } => {
            let mut col = div().w_full().v_flex().gap(px(6.));
            if let Some(lbl) = label {
                col = col.child(
                    div()
                        .text_size(px(font_size - 2.))
                        .font_medium()
                        .text_color(rgba(c.foreground_subtle))
                        .child(lbl.clone()),
                );
            }
            let mut row = div().w_full().h_flex().flex_wrap().gap(px(6.));
            for item in items {
                let itm_val = item.value.clone();
                let itm_lbl = item.label.clone();
                let itm_checked = item.checked;
                let act = UiAction {
                    id: format!("{}:{}", id, itm_val),
                    label: itm_lbl.clone(),
                    prompt: Some(format!(
                        "选择复选【{}】（{}）",
                        itm_lbl,
                        if !itm_checked {
                            "勾选"
                        } else {
                            "取消勾选"
                        }
                    )),
                    url: None,
                    copy_text: None,
                    payload: Some(
                        serde_json::json!({ "id": id, "value": itm_val, "checked": !itm_checked }),
                    ),
                };
                let on_action_cb = on_action.clone();
                let check_icon = if itm_checked { "☑ " } else { "☐ " };
                let chk_btn = Button::new(format!("chk-{}-{}", id, itm_val))
                    .h(px(26.))
                    .rounded(px(6.))
                    .px_2()
                    .when(itm_checked, |b| b.secondary())
                    .when(!itm_checked, |b| b.ghost())
                    .child(
                        div()
                            .text_size(px(font_size - 1.))
                            .child(format!("{}{}", check_icon, itm_lbl)),
                    )
                    .on_click(cx.listener(move |this, _, window, cx| {
                        on_action_cb(&act, this, window, cx);
                    }));
                row = row.child(chk_btn);
            }
            col.child(row).into_any_element()
        }
        UiNode::KeyValueList { items } => div()
            .w_full()
            .v_flex()
            .gap(px(6.))
            .children(items.iter().map(|item| {
                div()
                    .w_full()
                    .h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(font_size - 1.))
                            .text_color(rgba(c.foreground_subtle))
                            .child(item.key.clone()),
                    )
                    .child(
                        div()
                            .h_flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_size(px(font_size - 1.))
                                    .font_medium()
                                    .text_color(rgb(c.text_primary))
                                    .child(item.value.clone()),
                            )
                            .when_some(item.badge.as_ref(), |row, b| {
                                row.child(
                                    div()
                                        .px(px(6.))
                                        .py(px(1.))
                                        .rounded(px(4.))
                                        .bg(rgba(c.card_user_bg))
                                        .text_size(px(font_size - 2.))
                                        .text_color(rgba(c.foreground_subtle))
                                        .child(b.clone()),
                                )
                            }),
                    )
            }))
            .into_any_element(),
        UiNode::Table { headers, rows } => {
            let col_count = headers.len().max(1);
            let mut table_col = div()
                .w_full()
                .rounded(px(8.))
                .border_1()
                .border_color(rgba(c.card_border))
                .overflow_hidden()
                .v_flex();

            let header_row = div()
                .w_full()
                .h_flex()
                .bg(rgba(c.card_user_bg))
                .py_2()
                .px_3()
                .border_b_1()
                .border_color(rgba(c.card_border))
                .children(headers.iter().map(|h| {
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_size(px(font_size - 2.))
                        .font_medium()
                        .text_color(rgba(c.foreground_subtlest))
                        .child(h.clone())
                }));
            table_col = table_col.child(header_row);

            for (idx, row) in rows.iter().enumerate() {
                let is_last = idx == rows.len().saturating_sub(1);
                let row_el = div()
                    .w_full()
                    .h_flex()
                    .py_2()
                    .px_3()
                    .when(!is_last, |r| {
                        r.border_b_1().border_color(rgba(c.card_border))
                    })
                    .children((0..col_count).map(|c_idx| {
                        let text = row.get(c_idx).cloned().unwrap_or_default();
                        div()
                            .flex_1()
                            .min_w_0()
                            .text_size(px(font_size - 1.))
                            .text_color(rgb(c.text_primary))
                            .child(text)
                    }));
                table_col = table_col.child(row_el);
            }

            table_col.into_any_element()
        }
        UiNode::Progress {
            value,
            max,
            label,
            tone,
        } => {
            let pct = if *max > 0.0 {
                (value / max).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let fill_color = match tone {
                BadgeTone::Success => rgb(c.accent_green),
                BadgeTone::Warning => rgb(c.workflow_active),
                BadgeTone::Danger => rgb(c.workflow_error),
                BadgeTone::Info => rgb(c.link),
                BadgeTone::Neutral => rgb(c.text_primary),
            };

            div()
                .w_full()
                .v_flex()
                .gap(px(4.))
                .when_some(label.as_ref(), |this, l| {
                    this.child(
                        div()
                            .h_flex()
                            .justify_between()
                            .text_size(px(font_size - 2.))
                            .text_color(rgba(c.foreground_subtle))
                            .child(l.clone())
                            .child(format!("{:.0}%", pct * 100.0)),
                    )
                })
                .child(
                    div()
                        .w_full()
                        .h(px(6.))
                        .rounded(px(3.))
                        .bg(rgba(c.card_user_bg))
                        .child(
                            div()
                                .w(relative(pct))
                                .h_full()
                                .rounded(px(3.))
                                .bg(fill_color),
                        ),
                )
                .into_any_element()
        }
        UiNode::Metric {
            label,
            value,
            change,
            change_tone,
            subtext,
        } => div()
            .w_full()
            .v_flex()
            .gap(px(4.))
            .child(
                div()
                    .w_full()
                    .h_flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(font_size - 2.))
                            .font_medium()
                            .text_color(rgba(c.foreground_subtle))
                            .child(label.clone()),
                    )
                    .when_some(change.as_ref(), |row, ch| {
                        let tone = change_tone.unwrap_or(BadgeTone::Neutral);
                        row.child(render_badge(
                            &BadgeNode {
                                label: ch.clone(),
                                tone,
                            },
                            c,
                        ))
                    }),
            )
            .child(
                div()
                    .text_size(px(font_size + 8.))
                    .font_bold()
                    .text_color(rgb(c.text_primary))
                    .child(value.clone()),
            )
            .when_some(subtext.as_ref(), |col, st| {
                col.child(
                    div()
                        .text_size(px(font_size - 2.))
                        .text_color(rgba(c.foreground_subtlest))
                        .child(st.clone()),
                )
            })
            .into_any_element(),
        UiNode::BarChart {
            title,
            items,
            direction,
            max_value,
        } => {
            let mut chart = div().w_full().v_flex().gap(px(8.));
            if let Some(t) = title {
                chart = chart.child(
                    div()
                        .text_size(px(font_size))
                        .font_semibold()
                        .text_color(rgb(c.text_primary))
                        .child(t.clone()),
                );
            }
            let max_val = max_value
                .unwrap_or_else(|| items.iter().map(|i| i.value).fold(0.0, f32::max))
                .max(0.001);

            match direction {
                LayoutDirection::Row => {
                    let bars = div()
                        .w_full()
                        .v_flex()
                        .gap(px(6.))
                        .children(items.iter().map(|item| {
                            let ratio = (item.value / max_val).clamp(0.0, 1.0);
                            let fill = match item.tone.unwrap_or(BadgeTone::Info) {
                                BadgeTone::Success => rgb(c.accent_green),
                                BadgeTone::Warning => rgb(c.workflow_active),
                                BadgeTone::Danger => rgb(c.workflow_error),
                                BadgeTone::Info => rgb(c.link),
                                BadgeTone::Neutral => rgb(c.text_primary),
                            };
                            let display = item
                                .display_value
                                .clone()
                                .unwrap_or_else(|| format!("{:.1}", item.value));

                            div()
                                .w_full()
                                .h_flex()
                                .items_center()
                                .gap(px(8.))
                                .child(
                                    div()
                                        .w(px(70.))
                                        .text_size(px(font_size - 2.))
                                        .text_color(rgba(c.foreground_subtle))
                                        .child(item.label.clone()),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .h(px(10.))
                                        .rounded(px(5.))
                                        .bg(rgba(c.card_user_bg))
                                        .child(
                                            div()
                                                .w(relative(ratio))
                                                .h_full()
                                                .rounded(px(5.))
                                                .bg(fill),
                                        ),
                                )
                                .child(
                                    div()
                                        .w(px(40.))
                                        .text_size(px(font_size - 2.))
                                        .font_medium()
                                        .text_color(rgb(c.text_primary))
                                        .child(display),
                                )
                        }));
                    chart = chart.child(bars);
                }
                LayoutDirection::Column => {
                    let bars = div()
                        .w_full()
                        .h(px(80.))
                        .h_flex()
                        .items_end()
                        .justify_around()
                        .gap(px(6.))
                        .children(items.iter().map(|item| {
                            let ratio = (item.value / max_val).clamp(0.0, 1.0);
                            let fill = match item.tone.unwrap_or(BadgeTone::Info) {
                                BadgeTone::Success => rgb(c.accent_green),
                                BadgeTone::Warning => rgb(c.workflow_active),
                                BadgeTone::Danger => rgb(c.workflow_error),
                                BadgeTone::Info => rgb(c.link),
                                BadgeTone::Neutral => rgb(c.text_primary),
                            };
                            div()
                                .flex_1()
                                .h_full()
                                .v_flex()
                                .items_center()
                                .justify_end()
                                .gap(px(4.))
                                .child(
                                    div()
                                        .w_full()
                                        .max_w(px(24.))
                                        .h(relative(ratio))
                                        .rounded(px(4.))
                                        .bg(fill),
                                )
                                .child(
                                    div()
                                        .text_size(px(font_size - 2.))
                                        .text_color(rgba(c.foreground_subtle))
                                        .child(item.label.clone()),
                                )
                        }));
                    chart = chart.child(bars);
                }
            }
            chart.into_any_element()
        }
        UiNode::LineChart {
            title,
            points,
            tone,
            summary,
        } => {
            let mut wrap = div().w_full().v_flex().gap(px(8.));
            if title.is_some() || summary.is_some() {
                let mut row = div().w_full().h_flex().items_center().justify_between();
                if let Some(t) = title {
                    row = row.child(
                        div()
                            .text_size(px(font_size))
                            .font_semibold()
                            .text_color(rgb(c.text_primary))
                            .child(t.clone()),
                    );
                }
                if let Some(s) = summary {
                    row = row.child(
                        div()
                            .text_size(px(font_size - 1.))
                            .font_medium()
                            .text_color(rgba(c.foreground_subtle))
                            .child(s.clone()),
                    );
                }
                wrap = wrap.child(row);
            }

            if points.is_empty() {
                return wrap.into_any_element();
            }

            let min_val = points.iter().copied().fold(f32::INFINITY, f32::min);
            let max_val = points.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            let val_range = (max_val - min_val).max(0.001);

            let tone_color = match tone.unwrap_or(BadgeTone::Info) {
                BadgeTone::Success => c.accent_green,
                BadgeTone::Warning => c.workflow_active,
                BadgeTone::Danger => c.workflow_error,
                BadgeTone::Info => c.link,
                BadgeTone::Neutral => c.text_primary,
            };
            let stroke_color = rgb(tone_color);
            let fill_color = rgba((tone_color & 0x00ffffff) | 0x26000000);

            let pts = points.clone();
            let chart_height = 110.0f32;

            let chart_canvas = gpui::canvas(
                |_, _, _| (),
                move |bounds, _, window, _| {
                    let n = pts.len();
                    if n < 2 {
                        return;
                    }
                    let w = f32::from(bounds.size.width);
                    let h = f32::from(bounds.size.height);
                    let padding_y = 12.0f32;
                    let eff_h = (h - padding_y * 2.0).max(10.0);

                    let coord = |i: usize| -> Point<Pixels> {
                        let x = (i as f32 / (n - 1) as f32) * w;
                        let norm = (pts[i] - min_val) / val_range;
                        let y = h - padding_y - norm * eff_h;
                        bounds.origin + point(px(x), px(y))
                    };

                    let mut area = gpui::PathBuilder::fill();
                    area.move_to(bounds.origin + point(px(0.), px(h)));
                    for i in 0..n {
                        area.line_to(coord(i));
                    }
                    area.line_to(bounds.origin + point(px(w), px(h)));
                    area.close();

                    if let Ok(path) = area.build() {
                        window.paint_path(path, fill_color);
                    }

                    for i in 0..n {
                        let pt = coord(i);
                        let dot_bounds = Bounds {
                            origin: pt - point(px(3.), px(3.)),
                            size: size(px(6.), px(6.)),
                        };
                        window.paint_quad(gpui::fill(dot_bounds, stroke_color));
                    }
                },
            )
            .w_full()
            .h(px(chart_height))
            .rounded(px(8.))
            .bg(rgba(c.card_user_bg))
            .border_1()
            .border_color(rgba(c.card_border));

            let footer = div()
                .w_full()
                .h_flex()
                .justify_between()
                .text_size(px(font_size - 2.))
                .text_color(rgba(c.foreground_subtlest))
                .child(format!("最小值: {:.1}", min_val))
                .child(format!(
                    "均值: {:.1}",
                    points.iter().sum::<f32>() / points.len() as f32
                ))
                .child(format!("峰值: {:.1}", max_val));

            wrap.child(chart_canvas).child(footer).into_any_element()
        }
        UiNode::Sparkline {
            title,
            points,
            tone,
            summary,
        } => {
            let mut wrap = div().w_full().v_flex().gap(px(4.));
            if let Some(t) = title {
                wrap = wrap.child(
                    div()
                        .w_full()
                        .h_flex()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(font_size - 2.))
                                .font_medium()
                                .text_color(rgba(c.foreground_subtle))
                                .child(t.clone()),
                        )
                        .when_some(summary.as_ref(), |row, s| {
                            row.child(
                                div()
                                    .text_size(px(font_size - 2.))
                                    .text_color(rgb(c.text_primary))
                                    .child(s.clone()),
                            )
                        }),
                );
            }
            let max_val = points.iter().copied().fold(0.0, f32::max).max(0.001);
            let fill = match tone.unwrap_or(BadgeTone::Info) {
                BadgeTone::Success => rgb(c.accent_green),
                BadgeTone::Warning => rgb(c.workflow_active),
                BadgeTone::Danger => rgb(c.workflow_error),
                BadgeTone::Info => rgb(c.link),
                BadgeTone::Neutral => rgb(c.text_primary),
            };

            let bars = div()
                .w_full()
                .h(px(28.))
                .h_flex()
                .items_end()
                .gap(px(2.))
                .children(points.iter().map(|&val| {
                    let ratio = (val / max_val).clamp(0.08, 1.0);
                    div().flex_1().h(relative(ratio)).rounded(px(2.)).bg(fill)
                }));
            wrap.child(bars).into_any_element()
        }
        UiNode::Callout {
            tone,
            title,
            message,
        } => {
            let (bg, border, text_color) = match tone {
                BadgeTone::Neutral => (
                    rgba(c.card_user_bg),
                    rgba(c.card_border),
                    rgb(c.text_primary),
                ),
                BadgeTone::Success => (
                    rgba(c.accent_green & 0x00ffffff | 0x18000000),
                    rgb(c.accent_green),
                    rgb(c.accent_green),
                ),
                BadgeTone::Warning => (
                    rgba(c.workflow_active & 0x00ffffff | 0x18000000),
                    rgb(c.workflow_active),
                    rgb(c.workflow_active),
                ),
                BadgeTone::Danger => (
                    rgba(c.destructive_surface),
                    rgb(c.workflow_error),
                    rgb(c.workflow_error),
                ),
                BadgeTone::Info => (
                    rgba(c.link & 0x00ffffff | 0x18000000),
                    rgb(c.link),
                    rgb(c.link),
                ),
            };

            div()
                .w_full()
                .h_flex()
                .rounded(px(8.))
                .bg(bg)
                .border_1()
                .border_color(border)
                .overflow_hidden()
                .child(div().w(px(4.)).h_full().bg(border))
                .child(
                    div()
                        .flex_1()
                        .p_3()
                        .v_flex()
                        .gap(px(4.))
                        .when_some(title.as_ref(), |col, t| {
                            col.child(
                                div()
                                    .text_size(px(font_size))
                                    .font_semibold()
                                    .text_color(text_color)
                                    .child(t.clone()),
                            )
                        })
                        .child(
                            div()
                                .text_size(px(font_size - 1.))
                                .text_color(rgb(c.text_primary))
                                .child(message.clone()),
                        ),
                )
                .into_any_element()
        }
        UiNode::Accordion {
            title,
            subtitle,
            child,
        } => div()
            .w_full()
            .rounded(px(8.))
            .border_1()
            .border_color(rgba(c.card_border))
            .overflow_hidden()
            .v_flex()
            .child(
                div()
                    .w_full()
                    .h_flex()
                    .items_center()
                    .justify_between()
                    .p_3()
                    .bg(rgba(c.card_user_bg))
                    .child(
                        div()
                            .h_flex()
                            .items_center()
                            .gap(px(6.))
                            .child(
                                div()
                                    .text_size(px(font_size - 2.))
                                    .text_color(rgba(c.foreground_subtle))
                                    .child("▼"),
                            )
                            .child(
                                div()
                                    .text_size(px(font_size - 1.))
                                    .font_medium()
                                    .text_color(rgb(c.text_primary))
                                    .child(title.clone()),
                            )
                            .when_some(subtitle.as_ref(), |row, sub| {
                                row.child(
                                    div()
                                        .text_size(px(font_size - 2.))
                                        .text_color(rgba(c.foreground_subtle))
                                        .child(format!("({})", sub)),
                                )
                            }),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .p_3()
                    .child(render_ui_node(child, c, font_size, is_dark, on_action, cx)),
            )
            .into_any_element(),
        UiNode::Artifact {
            id,
            title,
            kind,
            content,
        } => {
            let line_count = content.lines().count().max(1);
            let copy_act = UiAction {
                id: format!("copy-art-{}", id),
                label: "复制代码".to_string(),
                prompt: None,
                url: None,
                copy_text: Some(content.clone()),
                payload: None,
            };
            let sidebar_act = UiAction {
                id: format!("sidebar-art-{}", id),
                label: format!("侧栏预览: {}", title),
                prompt: None,
                url: None,
                copy_text: None,
                payload: Some(serde_json::json!({
                    "action": "open_sidebar",
                    "id": id,
                    "title": title,
                    "kind": kind,
                    "content": content
                })),
            };
            let is_web = matches!(
                kind.to_lowercase().as_str(),
                "html" | "html5" | "canvas" | "svg" | "web"
            );
            let web_act = if is_web {
                Some(UiAction {
                    id: format!("web-art-{}", id),
                    label: format!("浏览器预览: {}", title),
                    prompt: None,
                    url: None,
                    copy_text: None,
                    payload: Some(serde_json::json!({
                        "action": "open_web_window",
                        "id": id,
                        "title": title,
                        "kind": kind,
                        "content": content
                    })),
                })
            } else {
                None
            };

            let on_action_copy = on_action.clone();
            let on_action_sidebar = on_action.clone();
            let on_action_web = on_action;

            let header = div()
                .w_full()
                .h_flex()
                .items_center()
                .justify_between()
                .px_3()
                .py_2()
                .bg(rgba(c.card_user_bg))
                .border_b_1()
                .border_color(rgba(c.card_border))
                .child(
                    div()
                        .h_flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .px(px(6.))
                                .py(px(1.))
                                .rounded(px(4.))
                                .bg(rgba(c.hover))
                                .text_size(px(10.))
                                .font_semibold()
                                .text_color(rgb(c.link))
                                .child(kind.to_uppercase()),
                        )
                        .child(
                            div()
                                .text_size(px(font_size - 1.))
                                .font_medium()
                                .text_color(rgb(c.text_primary))
                                .child(title.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(font_size - 2.))
                                .text_color(rgba(c.foreground_subtle))
                                .child(format!("{} 行", line_count)),
                        ),
                )
                .child(
                    div()
                        .h_flex()
                        .items_center()
                        .gap_2()
                        .child(
                            Button::new(format!("btn-copy-art-{}", id))
                                .accessibility_label("复制代码")
                                .h(px(24.))
                                .rounded(px(6.))
                                .px_2()
                                .ghost()
                                .child(div().text_size(px(11.)).child("复制代码".to_string()))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    on_action_copy(&copy_act, this, window, cx);
                                })),
                        )
                        .child(
                            Button::new(format!("btn-sidebar-art-{}", id))
                                .accessibility_label("侧栏预览")
                                .h(px(24.))
                                .rounded(px(6.))
                                .px_2()
                                .secondary()
                                .child(div().text_size(px(11.)).child("侧栏预览".to_string()))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    on_action_sidebar(&sidebar_act, this, window, cx);
                                })),
                        )
                        .when_some(web_act, |actions, act| {
                            actions.child(
                                Button::new(format!("btn-web-art-{}", id))
                                    .accessibility_label("浏览器预览")
                                    .h(px(24.))
                                    .rounded(px(6.))
                                    .px_2()
                                    .outline()
                                    .child(div().text_size(px(11.)).child("浏览器预览".to_string()))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        on_action_web(&act, this, window, cx);
                                    })),
                            )
                        }),
                );

            let body = div()
                .w_full()
                .max_h(px(220.))
                .p_3()
                .bg(rgba(c.inline_code_bg))
                .overflow_hidden()
                .child(
                    div()
                        .text_size(px(font_size - 1.))
                        .font_family(".AppleSystemUIFontMonospaced")
                        .text_color(rgb(c.text_primary))
                        .child(content.clone()),
                );

            div()
                .w_full()
                .rounded(px(8.))
                .border_1()
                .border_color(rgba(c.card_border))
                .overflow_hidden()
                .v_flex()
                .child(header)
                .child(body)
                .into_any_element()
        }
        UiNode::PieChart {
            title,
            slices,
            donut,
            summary,
        } => {
            let mut wrap = div().w_full().v_flex().gap(px(8.));
            if title.is_some() || summary.is_some() {
                let mut row = div().w_full().h_flex().items_center().justify_between();
                if let Some(t) = title {
                    row = row.child(
                        div()
                            .text_size(px(font_size))
                            .font_semibold()
                            .text_color(rgb(c.text_primary))
                            .child(t.clone()),
                    );
                }
                if let Some(s) = summary {
                    row = row.child(
                        div()
                            .text_size(px(font_size - 1.))
                            .font_medium()
                            .text_color(rgba(c.foreground_subtle))
                            .child(s.clone()),
                    );
                }
                wrap = wrap.child(row);
            }

            let total: f32 = slices.iter().map(|s| s.value).sum();
            if total <= 0.0 || slices.is_empty() {
                return wrap.into_any_element();
            }

            let default_colors = [
                c.accent_green,
                c.link,
                c.workflow_active,
                c.workflow_error,
                c.destructive_border,
                c.text_primary,
            ];

            let slice_data: Vec<(f32, f32, u32)> = {
                let mut start_ang = -std::f32::consts::FRAC_PI_2;
                slices
                    .iter()
                    .enumerate()
                    .map(|(idx, s)| {
                        let sweep = (s.value / total) * std::f32::consts::TAU;
                        let end_ang = start_ang + sweep;
                        let color = match s.tone {
                            Some(BadgeTone::Success) => c.accent_green,
                            Some(BadgeTone::Warning) => c.workflow_active,
                            Some(BadgeTone::Danger) => c.workflow_error,
                            Some(BadgeTone::Info) => c.link,
                            Some(BadgeTone::Neutral) => c.text_primary,
                            None => default_colors[idx % default_colors.len()],
                        };
                        let ret = (start_ang, end_ang, color);
                        start_ang = end_ang;
                        ret
                    })
                    .collect()
            };

            let is_donut = *donut;
            let chart_canvas = gpui::canvas(
                |_, _, _| (),
                move |bounds, _, window, _| {
                    let w = f32::from(bounds.size.width);
                    let h = f32::from(bounds.size.height);
                    let center_x = w.min(130.0) / 2.0;
                    let center_y = h / 2.0;
                    let center = bounds.origin + point(px(center_x), px(center_y));
                    let r = (center_x.min(center_y) - 6.0).max(10.0);
                    let inner_r = if is_donut { r * 0.55 } else { 0.0 };

                    for &(start_ang, end_ang, color) in &slice_data {
                        if (end_ang - start_ang).abs() < 0.001 {
                            continue;
                        }
                        let mut path = gpui::PathBuilder::fill();
                        let steps = ((end_ang - start_ang).abs() / 0.08).ceil().max(2.0) as usize;

                        if is_donut {
                            let p0 =
                                center + point(px(r * start_ang.cos()), px(r * start_ang.sin()));
                            path.move_to(p0);
                            for step in 1..=steps {
                                let t = start_ang
                                    + (end_ang - start_ang) * (step as f32 / steps as f32);
                                path.line_to(center + point(px(r * t.cos()), px(r * t.sin())));
                            }
                            for step in (0..=steps).rev() {
                                let t = start_ang
                                    + (end_ang - start_ang) * (step as f32 / steps as f32);
                                path.line_to(
                                    center + point(px(inner_r * t.cos()), px(inner_r * t.sin())),
                                );
                            }
                            path.close();
                        } else {
                            path.move_to(center);
                            for step in 0..=steps {
                                let t = start_ang
                                    + (end_ang - start_ang) * (step as f32 / steps as f32);
                                path.line_to(center + point(px(r * t.cos()), px(r * t.sin())));
                            }
                            path.close();
                        }

                        if let Ok(built) = path.build() {
                            window.paint_path(built, rgb(color));
                        }
                    }
                },
            )
            .w(px(130.))
            .h(px(110.));

            let legend = div()
                .flex_1()
                .v_flex()
                .justify_center()
                .gap(px(4.))
                .children(slices.iter().enumerate().map(|(idx, s)| {
                    let pct = (s.value / total) * 100.0;
                    let color = match s.tone {
                        Some(BadgeTone::Success) => c.accent_green,
                        Some(BadgeTone::Warning) => c.workflow_active,
                        Some(BadgeTone::Danger) => c.workflow_error,
                        Some(BadgeTone::Info) => c.link,
                        Some(BadgeTone::Neutral) => c.text_primary,
                        None => default_colors[idx % default_colors.len()],
                    };
                    div()
                        .w_full()
                        .h_flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .h_flex()
                                .items_center()
                                .gap(px(6.))
                                .child(div().w(px(8.)).h(px(8.)).rounded_full().bg(rgb(color)))
                                .child(
                                    div()
                                        .text_size(px(font_size - 2.))
                                        .text_color(rgb(c.text_primary))
                                        .child(s.label.clone()),
                                ),
                        )
                        .child(
                            div()
                                .h_flex()
                                .items_center()
                                .gap(px(6.))
                                .when_some(s.display_value.as_ref(), |this, dv| {
                                    this.child(
                                        div()
                                            .text_size(px(font_size - 2.))
                                            .text_color(rgba(c.foreground_subtle))
                                            .child(dv.clone()),
                                    )
                                })
                                .child(
                                    div()
                                        .text_size(px(font_size - 2.))
                                        .font_medium()
                                        .text_color(rgb(c.text_primary))
                                        .child(format!("{:.1}%", pct)),
                                ),
                        )
                }));

            let body = div()
                .w_full()
                .p_3()
                .rounded(px(8.))
                .bg(rgba(c.card_user_bg))
                .border_1()
                .border_color(rgba(c.card_border))
                .h_flex()
                .items_center()
                .gap(px(16.))
                .child(chart_canvas)
                .child(legend);

            wrap.child(body).into_any_element()
        }
        UiNode::Form {
            id,
            title,
            description,
            submit_label,
            fields,
        } => {
            let mut col = div()
                .w_full()
                .p_4()
                .rounded(px(10.))
                .bg(rgb(c.card_bg))
                .border_1()
                .border_color(rgba(c.card_border))
                .v_flex()
                .gap(px(10.));

            if title.is_some() || description.is_some() {
                let mut header = div().v_flex().gap(px(2.));
                if let Some(t) = title {
                    header = header.child(
                        div()
                            .text_size(px(font_size + 1.))
                            .font_semibold()
                            .text_color(rgb(c.text_primary))
                            .child(t.clone()),
                    );
                }
                if let Some(d) = description {
                    header = header.child(
                        div()
                            .text_size(px(font_size - 1.))
                            .text_color(rgba(c.foreground_subtle))
                            .child(d.clone()),
                    );
                }
                col = col.child(header);
            }

            let mut summary_lines = Vec::new();
            let mut payload_map = serde_json::Map::new();

            for field in fields {
                let f_id = field.id.clone();
                let f_lbl = field.label.clone();
                let val_str = field.default_value.clone().unwrap_or_default();
                summary_lines.push(format!(
                    "{}: {}",
                    f_lbl,
                    if val_str.is_empty() {
                        "未填写"
                    } else {
                        &val_str
                    }
                ));
                payload_map.insert(f_id.clone(), serde_json::Value::String(val_str.clone()));

                let mut field_row = div().w_full().v_flex().gap(px(4.));
                let label_el = div()
                    .h_flex()
                    .items_center()
                    .gap(px(4.))
                    .child(
                        div()
                            .text_size(px(font_size - 2.))
                            .font_medium()
                            .text_color(rgba(c.foreground_subtle))
                            .child(f_lbl),
                    )
                    .when(field.required, |row| {
                        row.child(
                            div()
                                .text_size(px(font_size - 2.))
                                .text_color(rgb(c.workflow_error))
                                .child("*"),
                        )
                    });
                field_row = field_row.child(label_el);

                match field.field_type {
                    FormFieldType::Text | FormFieldType::Textarea => {
                        let disp = if !val_str.is_empty() {
                            val_str
                        } else {
                            field
                                .placeholder
                                .clone()
                                .unwrap_or_else(|| "请输入...".to_string())
                        };
                        field_row = field_row.child(
                            div()
                                .w_full()
                                .p_2()
                                .rounded(px(6.))
                                .bg(rgba(c.card_user_bg))
                                .border_1()
                                .border_color(rgba(c.card_border))
                                .text_size(px(font_size - 1.))
                                .text_color(if field.default_value.is_some() {
                                    rgb(c.text_primary)
                                } else {
                                    rgba(c.foreground_subtlest)
                                })
                                .child(disp),
                        );
                    }
                    FormFieldType::Select => {
                        let selected = field
                            .default_value
                            .clone()
                            .or_else(|| field.options.first().cloned())
                            .unwrap_or_default();
                        field_row = field_row.child(
                            div()
                                .w_full()
                                .h_flex()
                                .items_center()
                                .justify_between()
                                .p_2()
                                .rounded(px(6.))
                                .bg(rgba(c.card_user_bg))
                                .border_1()
                                .border_color(rgba(c.card_border))
                                .child(
                                    div()
                                        .text_size(px(font_size - 1.))
                                        .text_color(rgb(c.text_primary))
                                        .child(selected),
                                )
                                .child(
                                    div()
                                        .text_size(px(font_size - 2.))
                                        .text_color(rgba(c.foreground_subtle))
                                        .child("▾"),
                                ),
                        );
                    }
                    FormFieldType::Switch => {
                        let is_on = field.default_value.as_deref() == Some("true")
                            || field.default_value.as_deref() == Some("1");
                        field_row = field_row.child(
                            div()
                                .h_flex()
                                .items_center()
                                .gap(px(8.))
                                .child(
                                    div()
                                        .w(px(28.))
                                        .h(px(16.))
                                        .p(px(2.))
                                        .rounded(px(8.))
                                        .bg(if is_on {
                                            rgb(c.accent_green)
                                        } else {
                                            rgba(c.card_border)
                                        })
                                        .h_flex()
                                        .items_center()
                                        .when(is_on, |s| s.justify_end())
                                        .child(
                                            div()
                                                .w(px(12.))
                                                .h(px(12.))
                                                .rounded_full()
                                                .bg(rgb(0xffffff)),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_size(px(font_size - 2.))
                                        .text_color(rgba(c.foreground_subtle))
                                        .child(if is_on { "已开启" } else { "已关闭" }),
                                ),
                        );
                    }
                }
                col = col.child(field_row);
            }

            let btn_txt = submit_label
                .clone()
                .unwrap_or_else(|| "提交表单".to_string());
            let act = UiAction {
                id: format!("form-submit-{}", id),
                label: btn_txt.clone(),
                prompt: Some(format!(
                    "已提交表单【{}】:\n{}",
                    title.as_deref().unwrap_or("未命名表单"),
                    summary_lines.join("\n")
                )),
                url: None,
                copy_text: None,
                payload: Some(serde_json::Value::Object(payload_map)),
            };

            let on_action_cb = on_action;
            let brand_btn = if is_dark {
                ButtonCustomVariant::new(cx)
                    .color(rgb(0xffffff).into())
                    .foreground(rgb(0x000000).into())
                    .hover(rgba(0xffffffcc).into())
                    .active(rgba(0xffffffe6).into())
            } else {
                ButtonCustomVariant::new(cx)
                    .color(rgb(0x000000).into())
                    .foreground(rgb(0xffffff).into())
                    .hover(rgba(0x000000cc).into())
                    .active(rgba(0x000000e6).into())
            };

            let submit_btn = Button::new(format!("btn-form-submit-{}", id))
                .h(px(32.))
                .rounded(px(8.))
                .custom(brand_btn)
                .w_full()
                .child(
                    div()
                        .w_full()
                        .h_flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(font_size - 1.))
                        .font_medium()
                        .child(btn_txt),
                )
                .on_click(cx.listener(move |this, _, window, cx| {
                    on_action_cb(&act, this, window, cx);
                }));

            col.child(submit_btn).into_any_element()
        }
        UiNode::Rating {
            id,
            label,
            max,
            value,
        } => {
            let max_r = (*max).clamp(1, 10);
            let cur_r = (*value).clamp(0, max_r);
            let mut row = div().w_full().v_flex().gap(px(4.));
            if let Some(lbl) = label {
                row = row.child(
                    div()
                        .text_size(px(font_size - 2.))
                        .font_medium()
                        .text_color(rgba(c.foreground_subtle))
                        .child(lbl.clone()),
                );
            }
            let on_action_cb = on_action;
            let stars = div()
                .h_flex()
                .items_center()
                .gap(px(4.))
                .children((1..=max_r).map(|star_idx| {
                    let is_filled = star_idx <= cur_r;
                    let star_char = if is_filled { "★" } else { "☆" };
                    let act = UiAction {
                        id: format!("{}:{}", id, star_idx),
                        label: format!("评分 {} 星", star_idx),
                        prompt: Some(format!("已评分：{} 星（满分 {} 星）", star_idx, max_r)),
                        url: None,
                        copy_text: None,
                        payload: Some(serde_json::json!({ "id": id, "rating": star_idx })),
                    };
                    let on_act = on_action_cb.clone();
                    Button::new(format!("star-{}-{}", id, star_idx))
                        .h(px(26.))
                        .px_2()
                        .ghost()
                        .child(
                            div()
                                .text_size(px(font_size + 2.))
                                .text_color(if is_filled {
                                    rgb(c.workflow_active)
                                } else {
                                    rgba(c.foreground_subtlest)
                                })
                                .child(star_char),
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            on_act(&act, this, window, cx);
                        }))
                }));
            row.child(stars).into_any_element()
        }
        UiNode::Divider => div()
            .w_full()
            .h(px(1.))
            .bg(rgba(c.card_border))
            .into_any_element(),
    }
}

fn render_badge(badge: &BadgeNode, c: &crate::theme::ThemeColors) -> AnyElement {
    let (bg, border, text_color) = match badge.tone {
        BadgeTone::Neutral => (
            rgba(c.card_user_bg),
            rgba(c.card_border),
            rgba(c.foreground_subtle),
        ),
        BadgeTone::Success => (
            rgba(c.accent_green & 0x00ffffff | 0x20000000),
            rgba(c.accent_green & 0x00ffffff | 0x4d000000),
            rgb(c.accent_green),
        ),
        BadgeTone::Warning => (
            rgba(c.workflow_active & 0x00ffffff | 0x20000000),
            rgba(c.workflow_active & 0x00ffffff | 0x4d000000),
            rgb(c.workflow_active),
        ),
        BadgeTone::Danger => (
            rgba(c.destructive_surface),
            rgba(c.destructive_border),
            rgb(c.workflow_error),
        ),
        BadgeTone::Info => (
            rgba(c.link & 0x00ffffff | 0x20000000),
            rgba(c.link & 0x00ffffff | 0x4d000000),
            rgb(c.link),
        ),
    };

    div()
        .px(px(8.))
        .py(px(2.))
        .rounded(px(6.))
        .bg(bg)
        .border_1()
        .border_color(border)
        .child(
            div()
                .text_size(px(11.))
                .font_medium()
                .text_color(text_color)
                .child(badge.label.clone()),
        )
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::{
        parse_message_segments, parse_message_segments_with_workspace, BadgeTone, ContentSegment,
        LayoutDirection, TextVariant, UiNode,
    };

    #[test]
    fn parses_simple_card_json() {
        let json = r#"{
            "type": "card",
            "title": "任务审批",
            "subtitle": "需要人工复核",
            "badge": { "label": "待处理", "tone": "warning" },
            "child": {
                "type": "container",
                "direction": "column",
                "gap": 8.0,
                "children": [
                    { "type": "text", "content": "确认部署至预发环境？", "variant": "body" },
                    { "type": "button", "id": "btn_approve", "label": "批准部署", "variant": "primary" }
                ]
            }
        }"#;

        let node: Result<UiNode, _> = serde_json::from_str(json);
        assert!(node.is_ok());
        if let Ok(UiNode::Card {
            title,
            badge,
            child,
            ..
        }) = node
        {
            assert_eq!(title, Some("任务审批".to_string()));
            assert_eq!(badge.map(|b| b.tone), Some(BadgeTone::Warning));
            if let UiNode::Container { children, .. } = *child {
                assert_eq!(children.len(), 2);
            } else {
                panic!("Expected Container child");
            }
        } else {
            panic!("Expected Card");
        }
    }

    #[test]
    fn parses_message_segments_with_embedded_ui() {
        let msg = "这是前置文本：\n\n```json-ui\n{\n  \"type\": \"text\",\n  \"content\": \"卡片内容\"\n}\n```\n这是后续说明。";
        let segments = parse_message_segments(msg);
        assert_eq!(segments.len(), 3);
        assert_eq!(
            segments[0],
            ContentSegment::Markdown("这是前置文本：\n\n".to_string())
        );
        assert_eq!(
            segments[1],
            ContentSegment::GenerativeUi(Box::new(UiNode::Text {
                content: "卡片内容".to_string(),
                variant: TextVariant::Body,
            }))
        );
        assert_eq!(
            segments[2],
            ContentSegment::Markdown("\n这是后续说明。".to_string())
        );
    }

    #[test]
    fn ignores_invalid_json_blocks_gracefully() {
        let msg = "普通代码示例：\n```json-ui\n{ invalid json syntax }\n```";
        let segments = parse_message_segments(msg);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0], ContentSegment::Markdown(msg.to_string()));
    }

    #[test]
    fn parses_interactive_form_controls() {
        let json = r#"{
            "type": "choice_group",
            "id": "env_select",
            "label": "选择部署目标环境",
            "options": [
                { "label": "开发环境", "value": "dev" },
                { "label": "预发环境", "value": "staging" },
                { "label": "生产环境", "value": "prod", "tone": "danger" }
            ]
        }"#;
        let node: Result<UiNode, _> = serde_json::from_str(json);
        assert!(node.is_ok());
        if let Ok(UiNode::ChoiceGroup { options, .. }) = node {
            assert_eq!(options.len(), 3);
            assert_eq!(options[2].tone, Some(BadgeTone::Danger));
        } else {
            panic!("Expected ChoiceGroup");
        }

        let sel_json = r#"{
            "type": "select",
            "id": "db_replica",
            "label": "数据库副本数量",
            "options": ["1 副本", "3 副本 (推荐)", "5 副本"],
            "default_value": "3 副本 (推荐)"
        }"#;
        let sel_node: Result<UiNode, _> = serde_json::from_str(sel_json);
        assert!(sel_node.is_ok());

        let chk_json = r#"{
            "type": "checkbox_group",
            "id": "plugins",
            "label": "选择安装插件",
            "items": [
                { "label": "Redis 缓存", "value": "redis", "checked": true },
                { "label": "Prometheus 监控", "value": "prom", "checked": false }
            ]
        }"#;
        let chk_node: Result<UiNode, _> = serde_json::from_str(chk_json);
        assert!(chk_node.is_ok());
    }

    #[test]
    fn parses_data_visualization_and_metrics() {
        let metric_json = r#"{
            "type": "metric",
            "label": "当前 CPU 使用率",
            "value": "78.4%",
            "change": "+5.2%",
            "change_tone": "warning",
            "subtext": "最近 5 分钟均值"
        }"#;
        let metric: Result<UiNode, _> = serde_json::from_str(metric_json);
        assert!(metric.is_ok());

        let chart_json = r#"{
            "type": "bar_chart",
            "title": "服务响应时间",
            "direction": "row",
            "items": [
                { "label": "API 网关", "value": 24.5, "display_value": "24.5ms", "tone": "success" },
                { "label": "认证中心", "value": 112.0, "display_value": "112ms", "tone": "danger" }
            ]
        }"#;
        let chart: Result<UiNode, _> = serde_json::from_str(chart_json);
        assert!(chart.is_ok());
        if let Ok(UiNode::BarChart {
            direction, items, ..
        }) = chart
        {
            assert_eq!(direction, LayoutDirection::Row);
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected BarChart");
        }

        let line_json = r#"{
            "type": "line_chart",
            "title": "QPS 趋势图",
            "points": [120.0, 145.0, 190.0, 310.0, 280.0, 420.0],
            "tone": "info",
            "summary": "最高 420 QPS"
        }"#;
        let line_node: Result<UiNode, _> = serde_json::from_str(line_json);
        assert!(line_node.is_ok());
    }

    #[test]
    fn parses_callout_accordion_and_artifact() {
        let callout_json = r#"{
            "type": "callout",
            "tone": "success",
            "title": "部署成功",
            "message": "所有节点已通过就绪探针检查。"
        }"#;
        let callout: Result<UiNode, _> = serde_json::from_str(callout_json);
        assert!(callout.is_ok());

        let artifact_json = r#"{
            "type": "artifact",
            "id": "nginx_conf",
            "title": "反向代理配置",
            "kind": "nginx",
            "content": "server {\n  listen 80;\n  server_name app.local;\n}"
        }"#;
        let artifact: Result<UiNode, _> = serde_json::from_str(artifact_json);
        assert!(artifact.is_ok());
    }

    #[test]
    fn parses_pie_chart_and_donut_json() {
        let json = r#"{
            "type": "pie_chart",
            "title": "业务请求分布",
            "donut": true,
            "summary": "总调用量 100k",
            "slices": [
                { "label": "API", "value": 45.0, "tone": "success", "display_value": "45k" },
                { "label": "Web", "value": 35.0, "tone": "info", "display_value": "35k" },
                { "label": "Job", "value": 20.0, "tone": "warning", "display_value": "20k" }
            ]
        }"#;
        let node: Result<UiNode, _> = serde_json::from_str(json);
        assert!(node.is_ok());
        if let Ok(UiNode::PieChart {
            slices,
            donut,
            summary,
            ..
        }) = node
        {
            assert_eq!(slices.len(), 3);
            assert!(donut);
            assert_eq!(summary, Some("总调用量 100k".to_string()));
        } else {
            panic!("Expected PieChart");
        }
    }

    #[test]
    fn parses_form_and_rating_json() {
        let form_json = r#"{
            "type": "form",
            "id": "cluster_cfg",
            "title": "集群配置初始化",
            "submit_label": "保存配置",
            "fields": [
                { "id": "name", "label": "集群名称", "field_type": "text", "default_value": "prod-k8s", "required": true },
                { "id": "tier", "label": "规格等级", "field_type": "select", "options": ["标准型", "高并发型"], "default_value": "标准型" },
                { "id": "ha", "label": "高可用开关", "field_type": "switch", "default_value": "true" }
            ]
        }"#;
        let form_node: Result<UiNode, _> = serde_json::from_str(form_json);
        assert!(form_node.is_ok());
        if let Ok(UiNode::Form { fields, .. }) = form_node {
            assert_eq!(fields.len(), 3);
            assert!(fields[0].required);
        } else {
            panic!("Expected Form");
        }

        let rating_json = r#"{
            "type": "rating",
            "id": "agent_feedback",
            "label": "回复质量评分",
            "max": 5,
            "value": 4
        }"#;
        let rating_node: Result<UiNode, _> = serde_json::from_str(rating_json);
        assert!(rating_node.is_ok());
    }

    #[test]
    fn parses_agent_embed_tag_inline() {
        let msg = "如下是内嵌前端小组件：\n\n<agent-embed title=\"计数器小组件\">\n<button class=\"px-4 py-2 bg-blue-500\">点击递增</button>\n</agent-embed>\n请查收。";
        let segments = parse_message_segments(msg);
        assert_eq!(segments.len(), 3);
        assert_eq!(
            segments[0],
            ContentSegment::Markdown("如下是内嵌前端小组件：\n\n".to_string())
        );
        if let ContentSegment::GenerativeUi(node) = &segments[1] {
            let UiNode::Artifact {
                title,
                kind,
                content,
                ..
            } = node.as_ref()
            else {
                panic!("Expected Artifact from agent-embed");
            };
            assert_eq!(title, "计数器小组件");
            assert_eq!(kind, "html");
            assert!(content.contains("点击递增"));
        } else {
            panic!("Expected Artifact from agent-embed");
        }
        assert_eq!(
            segments[2],
            ContentSegment::Markdown("\n请查收。".to_string())
        );
    }

    #[test]
    fn parses_html_artifact_codeblock() {
        let msg = "已生成数据看板：\n\n```html:artifact title=\"业务大屏\"\n<div class=\"dashboard\">大屏内容</div>\n```\n可随时在侧栏或独立视窗预览。";
        let segments = parse_message_segments(msg);
        assert_eq!(segments.len(), 3);
        if let ContentSegment::GenerativeUi(node) = &segments[1] {
            let UiNode::Artifact {
                title,
                kind,
                content,
                ..
            } = node.as_ref()
            else {
                panic!("Expected Artifact from html:artifact code block");
            };
            assert_eq!(title, "业务大屏");
            assert_eq!(kind, "html");
            assert_eq!(content, "<div class=\"dashboard\">大屏内容</div>");
        } else {
            panic!("Expected Artifact from html:artifact code block");
        }
    }

    #[test]
    fn injects_theme_variables_to_html_properly() {
        use crate::theme::ThemeColors;
        let c = ThemeColors::dark();
        let raw_html = "<div>Hello World</div>";
        let injected = super::inject_theme_variables_to_html(raw_html, true, &c);
        assert!(injected.contains("--card:"));
        assert!(injected.contains("--background:"));
        assert!(injected.contains("tailwindcss.min.js"));
    }

    #[test]
    fn case_insensitive_markers_keep_unicode_offsets_safe() {
        let message = "前缀 İİ\n```JSON-UI\n{\"type\":\"text\",\"content\":\"安全\"}\n```";
        let segments = parse_message_segments(message);
        assert!(matches!(
            segments.get(1),
            Some(ContentSegment::GenerativeUi(node))
                if matches!(node.as_ref(), UiNode::Text { content, .. } if content == "安全")
        ));
    }

    #[test]
    fn oversized_ui_blocks_remain_markdown() {
        let content = "x".repeat(super::MAX_ARTIFACT_BYTES + 1);
        let message = format!("```json-ui\n{{\"type\":\"text\",\"content\":\"{content}\"}}\n```");
        let segments = parse_message_segments(&message);
        assert!(matches!(
            segments.as_slice(),
            [ContentSegment::Markdown(markdown)] if markdown == &message
        ));
    }

    #[test]
    fn preserves_invalid_blocks_before_valid_ui() {
        let invalid = "前缀\n```json-ui\n{invalid}\n```\n";
        let valid = "```ui\n{\"type\":\"text\",\"content\":\"valid\"}\n```";
        let segments = parse_message_segments(&format!("{invalid}{valid}尾部"));
        assert_eq!(segments[0], ContentSegment::Markdown(invalid.to_string()));
        assert!(matches!(segments[1], ContentSegment::GenerativeUi(_)));
        assert_eq!(segments[2], ContentSegment::Markdown("尾部".to_string()));
    }

    #[test]
    fn preserves_incomplete_embeds_and_plain_text() {
        for message in [
            "",
            "只有 Markdown",
            "前缀 <agent-embed title=\"未结束",
            "前缀 <agent-embed>没有关闭标签",
            "前缀 <agent-embedded>保留原文</agent-embed>",
        ] {
            let segments = parse_message_segments(message);
            let markdown: String = segments
                .iter()
                .map(|segment| match segment {
                    ContentSegment::Markdown(text) => text.as_str(),
                    _ => panic!("Malformed embed must remain markdown"),
                })
                .collect();
            assert_eq!(markdown, message);
        }
    }

    #[test]
    fn self_closing_embeds_do_not_consume_later_blocks() {
        let message = "<agent-embed title='first'/>between<agent-embed title='second'><p>second</p></agent-embed>after";
        let segments = parse_message_segments(message);
        assert_eq!(segments.len(), 4);
        assert!(matches!(
            &segments[0],
            ContentSegment::GenerativeUi(node)
                if matches!(node.as_ref(), UiNode::Artifact { title, content, .. }
                    if title == "first" && content.is_empty())
        ));
        assert_eq!(segments[1], ContentSegment::Markdown("between".to_string()));
        assert!(matches!(
            &segments[2],
            ContentSegment::GenerativeUi(node)
                if matches!(node.as_ref(), UiNode::Artifact { title, content, .. }
                    if title == "second" && content == "<p>second</p>")
        ));
        assert_eq!(segments[3], ContentSegment::Markdown("after".to_string()));
    }

    #[test]
    fn embed_attributes_respect_names_and_quotes() {
        let message = r#"<AGENT-EMBED data-src="missing.html" title='a > b; src="also-missing"'><p>inline</p></AGENT-EMBED>"#;
        let segments = parse_message_segments(message);
        assert!(matches!(
            segments.as_slice(),
            [ContentSegment::GenerativeUi(node)]
                if matches!(node.as_ref(), UiNode::Artifact { title, content, .. }
                    if title == "a > b; src=\"also-missing\"" && content == "<p>inline</p>")
        ));
        assert_eq!(super::attribute_value("data-src='a'", "src"), None);
        assert_eq!(super::attribute_value("title='src=\"a\"'", "src"), None);
        assert_eq!(
            super::attribute_value("disabled SRC = 'nested/a.html'", "src").as_deref(),
            Some("nested/a.html")
        );
    }

    #[test]
    fn json_artifacts_preserve_their_ids() {
        let message = r#"```ui
{"type":"artifact","id":"user_id","title":"sample","content":"sample"}
```"#;
        let segments = parse_message_segments(message);
        assert!(matches!(
            segments.as_slice(),
            [ContentSegment::GenerativeUi(node)]
                if matches!(node.as_ref(), UiNode::Artifact { id, .. } if id == "user_id")
        ));
    }

    #[test]
    fn oversized_html_and_embed_blocks_preserve_the_original_message() {
        let content = "x".repeat(super::MAX_ARTIFACT_BYTES + 1);
        for message in [
            format!("before```html:artifact\n{content}\n```after"),
            format!("before<agent-embed>{content}</agent-embed>after"),
        ] {
            assert_eq!(
                parse_message_segments(&message),
                vec![ContentSegment::Markdown(message)]
            );
        }
    }

    #[test]
    fn artifact_reads_enforce_the_limit_while_reading() {
        let allowed = "x".repeat(super::MAX_ARTIFACT_BYTES);
        assert_eq!(
            super::read_bounded_artifact(allowed.as_bytes()).unwrap(),
            allowed
        );
        let oversized = "x".repeat(super::MAX_ARTIFACT_BYTES + 1);
        assert!(super::read_bounded_artifact(oversized.as_bytes()).is_err());
    }

    #[test]
    fn embedded_sources_are_limited_to_the_workspace() {
        let workspace = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let allowed = workspace.path().join("widget.html");
        let blocked = outside.path().join("secret.html");
        std::fs::write(&allowed, "<p>allowed</p>").unwrap();
        std::fs::write(&blocked, "<p>secret</p>").unwrap();

        let allowed_message = format!(r#"<agent-embed src="{}"/>"#, allowed.display());
        let allowed_segments =
            parse_message_segments_with_workspace(&allowed_message, workspace.path());
        assert!(matches!(
            allowed_segments.as_slice(),
            [ContentSegment::GenerativeUi(node)]
                if matches!(node.as_ref(), UiNode::Artifact { content, .. } if content == "<p>allowed</p>")
        ));

        let blocked_message = format!(r#"<agent-embed src="{}"/>"#, blocked.display());
        let blocked_segments =
            parse_message_segments_with_workspace(&blocked_message, workspace.path());
        assert!(matches!(
            blocked_segments.as_slice(),
            [ContentSegment::GenerativeUi(node)]
                if matches!(node.as_ref(), UiNode::Artifact { content, .. } if !content.contains("secret"))
        ));
    }
}
