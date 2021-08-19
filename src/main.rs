// Copyright 2021 The BMW Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use librustlet::*;
use nioruntime_util::{Error, ErrorKind};
use std::sync::{Arc, Mutex};

log::debug!();

#[allow(unreachable_code)]
fn fun() -> Result<(), Error> {
	rustlet!("error", {
		response!("<html><body>hi");
		return Err(ErrorKind::InternalError("test error".to_string()).into());
	});
	Ok(())
}

fn fun2() -> Result<(), Error> {
	rustlet!("panic", {
		response!("hi");
		let x: Option<bool> = None;
		x.unwrap();
	});
	Ok(())
}

fn main() {
	rustlet_init!(RustletConfig {
		http_config: HttpConfig {
			debug: true,
			..Default::default()
		},
	});

	let x = Arc::new(Mutex::new(0));
	let x_clone = x.clone();

	rustlet!("redir", {
		set_redirect!("http://www.disney.com");
	});

	rustlet!("myrustlet", {
		let name = request!("query", "name");
		let mut x = x.lock().unwrap();
		*x += 1;
		add_header!("my_header", "ok");
		set_content_type!("text/plain");
		response!("name: {}, x={}", name, x);
	});

	rustlet!("myrustlet2", {
		let query = request!("query");
		let ua = request!("header", "User-Agent");
		let mut x = x_clone.lock().unwrap();
		*x += 1;
		response!("ok\n");
		response!("q2: {} x={}, ua={}", query, x, ua);
	});

	rustlet!("printheaders", {
		let header_count: usize = request!("header_len").parse().unwrap_or(0);
		for i in 0..header_count {
			let header_name = request!("header_i_name", &format!("{}", i));
			let header_value = request!("header_i_value", &format!("{}", i));
			response!("header[{}] [{}] -> [{}]\n", i, header_name, header_value);
		}
		let method = request!("method");
		response!("method='{}'\n", method);
		let version = request!("version");
		response!("http version='{}'\n", version);
		let uri = request!("uri");
		response!("uri='{}'\n", uri);
		let unknown = request!("blah");
		response!("blah (should be empty)='{}'\n", unknown);
		let query = request!("query");
		response!("query='{}'\n", query);
		let content = request_content!();
		response!("content={:?}\n", content);
	});

	let _ = fun();
	let _ = fun2();

	rustlet_mapping!("/myrustlet", "myrustlet");
	rustlet_mapping!("/myrustlet2", "myrustlet2");
	rustlet_mapping!("/printheaders", "printheaders");
	rustlet_mapping!("/redir", "redir");
	rustlet_mapping!("/error", "error");
	rustlet_mapping!("/panic", "panic");

	std::thread::park();
}
