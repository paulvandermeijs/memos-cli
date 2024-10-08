/*
 * Memos API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1Node {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::V1NodeType>,
    #[serde(rename = "lineBreakNode", skip_serializing_if = "Option::is_none")]
    pub line_break_node: Option<serde_json::Value>,
    #[serde(rename = "paragraphNode", skip_serializing_if = "Option::is_none")]
    pub paragraph_node: Option<Box<models::V1ParagraphNode>>,
    #[serde(rename = "codeBlockNode", skip_serializing_if = "Option::is_none")]
    pub code_block_node: Option<Box<models::V1CodeBlockNode>>,
    #[serde(rename = "headingNode", skip_serializing_if = "Option::is_none")]
    pub heading_node: Option<Box<models::V1HeadingNode>>,
    #[serde(rename = "horizontalRuleNode", skip_serializing_if = "Option::is_none")]
    pub horizontal_rule_node: Option<Box<models::V1HorizontalRuleNode>>,
    #[serde(rename = "blockquoteNode", skip_serializing_if = "Option::is_none")]
    pub blockquote_node: Option<Box<models::V1BlockquoteNode>>,
    #[serde(rename = "listNode", skip_serializing_if = "Option::is_none")]
    pub list_node: Option<Box<models::V1ListNode>>,
    #[serde(rename = "orderedListItemNode", skip_serializing_if = "Option::is_none")]
    pub ordered_list_item_node: Option<Box<models::V1OrderedListItemNode>>,
    #[serde(rename = "unorderedListItemNode", skip_serializing_if = "Option::is_none")]
    pub unordered_list_item_node: Option<Box<models::V1UnorderedListItemNode>>,
    #[serde(rename = "taskListItemNode", skip_serializing_if = "Option::is_none")]
    pub task_list_item_node: Option<Box<models::V1TaskListItemNode>>,
    #[serde(rename = "mathBlockNode", skip_serializing_if = "Option::is_none")]
    pub math_block_node: Option<Box<models::V1MathBlockNode>>,
    #[serde(rename = "tableNode", skip_serializing_if = "Option::is_none")]
    pub table_node: Option<Box<models::V1TableNode>>,
    #[serde(rename = "embeddedContentNode", skip_serializing_if = "Option::is_none")]
    pub embedded_content_node: Option<Box<models::V1EmbeddedContentNode>>,
    #[serde(rename = "textNode", skip_serializing_if = "Option::is_none")]
    pub text_node: Option<Box<models::V1TextNode>>,
    #[serde(rename = "boldNode", skip_serializing_if = "Option::is_none")]
    pub bold_node: Option<Box<models::V1BoldNode>>,
    #[serde(rename = "italicNode", skip_serializing_if = "Option::is_none")]
    pub italic_node: Option<Box<models::V1ItalicNode>>,
    #[serde(rename = "boldItalicNode", skip_serializing_if = "Option::is_none")]
    pub bold_italic_node: Option<Box<models::V1BoldItalicNode>>,
    #[serde(rename = "codeNode", skip_serializing_if = "Option::is_none")]
    pub code_node: Option<Box<models::V1CodeNode>>,
    #[serde(rename = "imageNode", skip_serializing_if = "Option::is_none")]
    pub image_node: Option<Box<models::V1ImageNode>>,
    #[serde(rename = "linkNode", skip_serializing_if = "Option::is_none")]
    pub link_node: Option<Box<models::V1LinkNode>>,
    #[serde(rename = "autoLinkNode", skip_serializing_if = "Option::is_none")]
    pub auto_link_node: Option<Box<models::V1AutoLinkNode>>,
    #[serde(rename = "tagNode", skip_serializing_if = "Option::is_none")]
    pub tag_node: Option<Box<models::V1TagNode>>,
    #[serde(rename = "strikethroughNode", skip_serializing_if = "Option::is_none")]
    pub strikethrough_node: Option<Box<models::V1StrikethroughNode>>,
    #[serde(rename = "escapingCharacterNode", skip_serializing_if = "Option::is_none")]
    pub escaping_character_node: Option<Box<models::V1EscapingCharacterNode>>,
    #[serde(rename = "mathNode", skip_serializing_if = "Option::is_none")]
    pub math_node: Option<Box<models::V1MathNode>>,
    #[serde(rename = "highlightNode", skip_serializing_if = "Option::is_none")]
    pub highlight_node: Option<Box<models::V1HighlightNode>>,
    #[serde(rename = "subscriptNode", skip_serializing_if = "Option::is_none")]
    pub subscript_node: Option<Box<models::V1SubscriptNode>>,
    #[serde(rename = "superscriptNode", skip_serializing_if = "Option::is_none")]
    pub superscript_node: Option<Box<models::V1SuperscriptNode>>,
    #[serde(rename = "referencedContentNode", skip_serializing_if = "Option::is_none")]
    pub referenced_content_node: Option<Box<models::V1ReferencedContentNode>>,
    #[serde(rename = "spoilerNode", skip_serializing_if = "Option::is_none")]
    pub spoiler_node: Option<Box<models::V1SpoilerNode>>,
    #[serde(rename = "htmlElementNode", skip_serializing_if = "Option::is_none")]
    pub html_element_node: Option<Box<models::V1HtmlElementNode>>,
}

impl V1Node {
    pub fn new() -> V1Node {
        V1Node {
            r#type: None,
            line_break_node: None,
            paragraph_node: None,
            code_block_node: None,
            heading_node: None,
            horizontal_rule_node: None,
            blockquote_node: None,
            list_node: None,
            ordered_list_item_node: None,
            unordered_list_item_node: None,
            task_list_item_node: None,
            math_block_node: None,
            table_node: None,
            embedded_content_node: None,
            text_node: None,
            bold_node: None,
            italic_node: None,
            bold_italic_node: None,
            code_node: None,
            image_node: None,
            link_node: None,
            auto_link_node: None,
            tag_node: None,
            strikethrough_node: None,
            escaping_character_node: None,
            math_node: None,
            highlight_node: None,
            subscript_node: None,
            superscript_node: None,
            referenced_content_node: None,
            spoiler_node: None,
            html_element_node: None,
        }
    }
}

