use codex_protocol::models::ResponseItem;
use serde_json::Value;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Compression {
    #[default]
    None,
    Zstd,
}

const COMPLETED_STATUS: &str = "completed";
const OUTPUT_TEXT_TYPE: &str = "output_text";

pub(crate) fn attach_item_ids(payload_json: &mut Value, original_items: &[ResponseItem]) {
    let Some(input_value) = payload_json.get_mut("input") else {
        return;
    };
    let Value::Array(items) = input_value else {
        return;
    };

    for (value, item) in items.iter_mut().zip(original_items.iter()) {
        let Some(obj) = value.as_object_mut() else {
            continue;
        };

        match item {
            ResponseItem::Reasoning { id, .. }
            | ResponseItem::WebSearchCall { id: Some(id), .. }
            | ResponseItem::FunctionCall { id: Some(id), .. }
            | ResponseItem::ToolSearchCall { id: Some(id), .. }
            | ResponseItem::LocalShellCall { id: Some(id), .. }
            | ResponseItem::CustomToolCall { id: Some(id), .. } => {
                if !id.is_empty() {
                    obj.insert("id".to_string(), Value::String(id.clone()));
                }
            }

            ResponseItem::Message {
                id: Some(id), role, ..
            } => {
                if id.is_empty() {
                    continue;
                }

                obj.insert("id".to_string(), Value::String(id.clone()));

                if role == "assistant" {
                    obj.insert(
                        "status".to_string(),
                        Value::String(COMPLETED_STATUS.to_string()),
                    );

                    attach_output_text_annotations(obj);
                }
            }

            _ => {}
        }
    }
}

fn attach_output_text_annotations(message_obj: &mut serde_json::Map<String, Value>) {
    let Some(Value::Array(content_items)) = message_obj.get_mut("content") else {
        return;
    };

    for content_item in content_items {
        let Some(content_obj) = content_item.as_object_mut() else {
            continue;
        };

        let is_output_text = content_obj
            .get("type")
            .and_then(Value::as_str)
            .is_some_and(|item_type| item_type == OUTPUT_TEXT_TYPE);

        if is_output_text && !content_obj.contains_key("annotations") {
            content_obj.insert("annotations".to_string(), Value::Array(Vec::new()));
        }
    }
}
