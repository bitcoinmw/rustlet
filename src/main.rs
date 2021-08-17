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
use log::*;
use std::sync::{Arc, Mutex};

debug!();

fn main() {
	rustlet_init!(RustletConfig {
		http_config: HttpConfig {
			debug: true,
			..Default::default()
		},
	});

	let x = Arc::new(Mutex::new(0));
	let x_clone = x.clone();

	rustlet!("myrustlet", {
		let query = request!("query");
		let mut x = x.lock().unwrap();
		*x += 1;
		response!("q: {}, x={}", query, x);
	});

	rustlet!("myrustlet2", {
		let query = request!("query");
		let mut x = x_clone.lock().unwrap();
		*x += 1;
		response!("ok\n");
		response!("q2: {} x={}", query, x);
	});

	rustlet_mapping!("/myrustlet", "myrustlet");
	rustlet_mapping!("/myrustlet2", "myrustlet2");

	std::thread::park();
}
