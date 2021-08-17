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
use rustlet::*;

debug!();

fn rustlet1(request: &mut RustletRequest, response: &mut RustletResponse) -> Result<(), Error> {
	let query = request.get_query()?;
	let query = format!("query={}", query);
	response.write(query.as_bytes())?;
	Ok(())
}

fn main() {
	match real_main() {
		Ok(_) => {}
		Err(e) => {
			error!("real_main returned error: {}", e.to_string());
		}
	}
}

fn real_main() -> Result<(), Error> {
	let config = RustletConfig {
		http_config: HttpConfig {
			debug: true,
			..Default::default()
		},
	};

	let mut rustlet_container = RustletContainer::new(config);
	rustlet_container.start()?;
	rustlet_container.add_rustlet("myrustlet", rustlet1)?;
	rustlet_container.add_rustlet_mapping("myrustlet", "/myrustlet")?;

	std::thread::park();

	Ok(())
}
