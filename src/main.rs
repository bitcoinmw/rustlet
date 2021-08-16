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

use log::*;
use nioruntime_http::{HttpConfig, HttpServer};
use nioruntime_util::Error;

debug!();

fn main() {
	match real_main() {
		Ok(_) => {}
		Err(e) => {
			error!("real_main returned error: {}", e.to_string());
		}
	}
}

fn real_main() -> Result<(), Error> {
	let config = HttpConfig {
		debug: true,
		..Default::default()
	};
	let mut http_server: HttpServer = HttpServer::new(config);

	http_server.start()?;
	http_server.add_mapping("/rustlet".to_string())?;
	std::thread::park();

	Ok(())
}
