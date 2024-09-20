use std::future::Future;

pub fn spawn_local<F>(fut: F)
where
	F: Future<Output = ()> + 'static,
{
	cfg_if::cfg_if! {
		if #[cfg(feature = "js")] {
			wasm_bindgen_futures::spawn_local(fut);
		} else if #[cfg(feature = "ssr")] {
			tokio::task::spawn_local(fut);
		}  else {
			futures::executor::block_on(fut);
		}
	}
}

pub(crate) fn get_ws_url(url: impl Into<String>) -> String {
	let url: String = url.into();

	if url.starts_with("http") {
		// Replace to wss
		let first_index = url.find(':').expect("Invalid URL");
		let mut url = url.to_string();
		url.replace_range(
			..first_index,
			if url.starts_with("https") {
				"wss"
			} else {
				"ws"
			},
		);

		// Increase the port number by 1 if the port is specified
		let last_index = url.rfind(':').unwrap();
		if last_index != first_index {
			if let Some(Ok(mut port)) = url.get(last_index + 1..).map(str::parse::<u16>) {
				port += 1;
				url.replace_range(last_index + 1.., &port.to_string());
			}
		}
	}

	url
}
