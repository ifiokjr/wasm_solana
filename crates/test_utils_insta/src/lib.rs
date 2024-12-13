use std::fmt::Display;

use assert2::check;
use insta::internals::Content;
use insta::internals::ContentPath;

#[allow(impl_trait_overcaptures)]
pub fn create_insta_redaction<T: Display>(
	value: T,
	replacement: &str,
) -> impl Fn(Content, ContentPath) -> String + Clone + 'static {
	let replacement = format!("[{replacement}]");
	let value = value.to_string();

	move |content: Content, _: ContentPath| {
		let content_value = content.as_str().unwrap();
		check!(
			content_value == &value,
			"redacated content value is not valid: {replacement}"
		);

		replacement.clone()
	}
}

#[allow(impl_trait_overcaptures)]
pub fn create_insta_redaction_u128<T: Display>(
	value: T,
	replacement: &str,
) -> impl Fn(Content, ContentPath) -> String + Clone + 'static {
	let replacement = format!("[{replacement}]");
	let value = value.to_string();

	move |content: Content, _: ContentPath| {
		let content_value = content.as_u128().unwrap().to_string();
		check!(
			&content_value == &value,
			"redacted int value is not valid: {replacement}"
		);

		replacement.clone()
	}
}
