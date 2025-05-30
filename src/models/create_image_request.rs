/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImageRequest {
    /// Allows to set transparency for the background of the generated image(s).  This parameter is only supported for `gpt-image-1`. Must be one of  `transparent`, `opaque` or `auto` (default value). When `auto` is used, the  model will automatically determine the best background for the image.  If `transparent`, the output format needs to support transparency, so it  should be set to either `png` (default value) or `webp`.
    #[serde(rename = "background")]
    pub background: Option<String>,
    /// The model to use for image generation. One of `dall-e-2`, `dall-e-3`, or `gpt-image-1`. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used.
    #[serde(rename = "model")]
    pub model: Option<Value>,
    /// Control the content-moderation level for images generated by `gpt-image-1`. Must be either `low` for less restrictive filtering or `auto` (default value).
    #[serde(rename = "moderation")]
    pub moderation: Option<String>,
    /// The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported.
    #[serde(rename = "n")]
    pub n: Option<i32>,
    /// The compression level (0-100%) for the generated images. This parameter is only supported for `gpt-image-1` with the `webp` or `jpeg` output formats, and defaults to 100.
    #[serde(rename = "output_compression")]
    pub output_compression: Option<i32>,
    /// The format in which the generated images are returned. This parameter is only supported for `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`.
    #[serde(rename = "output_format")]
    pub output_format: Option<String>,
    /// A text description of the desired image(s). The maximum length is 32000 characters for `gpt-image-1`, 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`.
    #[serde(rename = "prompt")]
    pub prompt: String,
    /// The quality of the image that will be generated.   - `auto` (default value) will automatically select the best quality for the given model. - `high`, `medium` and `low` are supported for `gpt-image-1`. - `hd` and `standard` are supported for `dall-e-3`. - `standard` is the only option for `dall-e-2`.
    #[serde(rename = "quality")]
    pub quality: Option<String>,
    /// The format in which generated images with `dall-e-2` and `dall-e-3` are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter isn't supported for `gpt-image-1` which will always return base64-encoded images.
    #[serde(rename = "response_format")]
    pub response_format: Option<String>,
    /// The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`, and one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3`.
    #[serde(rename = "size")]
    pub size: Option<String>,
    /// The style of the generated images. This parameter is only supported for `dall-e-3`. Must be one of `vivid` or `natural`. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images.
    #[serde(rename = "style")]
    pub style: Option<String>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).
    #[serde(rename = "user")]
    pub user: Option<String>,
}
