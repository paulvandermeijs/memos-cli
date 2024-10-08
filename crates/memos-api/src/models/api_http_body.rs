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

use serde_with::serde_as;

/// ApiHttpBody : Message that represents an arbitrary HTTP body. It should only be used for payload formats that can't be represented as JSON, such as raw binary or an HTML page.   This message can be used both in streaming and non-streaming API methods in the request as well as the response.  It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body.  Example:      message GetResourceRequest {       // A unique request id.       string request_id = 1;        // The raw HTTP body is bound to this field.       google.api.HttpBody http_body = 2;      }      service ResourceService {       rpc GetResource(GetResourceRequest)         returns (google.api.HttpBody);       rpc UpdateResource(google.api.HttpBody)         returns (google.protobuf.Empty);      }  Example with streaming methods:      service CaldavService {       rpc GetCalendar(stream google.api.HttpBody)         returns (stream google.api.HttpBody);       rpc UpdateCalendar(stream google.api.HttpBody)         returns (stream google.api.HttpBody);      }  Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged.
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiHttpBody {
    /// The HTTP Content-Type header value specifying the content type of the body.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The HTTP request/response body as raw binary.
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    /// Application specific response metadata. Must be set in the first response for streaming APIs.
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<models::ProtobufAny>>,
}

impl ApiHttpBody {
    /// Message that represents an arbitrary HTTP body. It should only be used for payload formats that can't be represented as JSON, such as raw binary or an HTML page.   This message can be used both in streaming and non-streaming API methods in the request as well as the response.  It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body.  Example:      message GetResourceRequest {       // A unique request id.       string request_id = 1;        // The raw HTTP body is bound to this field.       google.api.HttpBody http_body = 2;      }      service ResourceService {       rpc GetResource(GetResourceRequest)         returns (google.api.HttpBody);       rpc UpdateResource(google.api.HttpBody)         returns (google.protobuf.Empty);      }  Example with streaming methods:      service CaldavService {       rpc GetCalendar(stream google.api.HttpBody)         returns (stream google.api.HttpBody);       rpc UpdateCalendar(stream google.api.HttpBody)         returns (stream google.api.HttpBody);      }  Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged.
    pub fn new() -> ApiHttpBody {
        ApiHttpBody {
            content_type: None,
            data: None,
            extensions: None,
        }
    }
}

