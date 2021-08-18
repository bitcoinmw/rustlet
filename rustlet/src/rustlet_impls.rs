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

use lazy_static::lazy_static;
use log::*;
pub use nioruntime_http::{HttpConfig, HttpServer};
use nioruntime_http::{HttpMethod, HttpVersion, WriteHandle};
pub use nioruntime_util::{Error, ErrorKind};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, RwLock};

info!();
const MAIN_LOG: &str = "mainlog";

#[derive(Clone)]
pub struct RustletRequest {
	content: Vec<u8>,
	http_method: HttpMethod,
	http_version: HttpVersion,
	http_config: HttpConfig,
	uri: String,
	query: String,
	headers: Vec<(Vec<u8>, Vec<u8>)>,
	keep_alive: bool,
	query_map: Option<HashMap<String, String>>,
	header_map: Option<HashMap<String, String>>,
}

impl RustletRequest {
	pub fn new(
		uri: String,
		query: String,
		content: Vec<u8>,
		http_method: HttpMethod,
		http_version: HttpVersion,
		http_config: HttpConfig,
		headers: Vec<(Vec<u8>, Vec<u8>)>,
		keep_alive: bool,
	) -> Self {
		RustletRequest {
			uri,
			query,
			content,
			http_method,
			http_version,
			http_config,
			headers,
			keep_alive,
			query_map: None,
			header_map: None,
		}
	}

	pub fn get_header_len(&mut self) -> Result<usize, Error> {
		if self.header_map.is_none() {
			self.build_header_map()?;
		}

		Ok(match self.header_map.as_ref() {
			Some(map) => map.len(),
			None => 0,
		})
	}

	pub fn get_header_i_value(&self, i: usize) -> Result<String, Error> {
		let vec_len = self.headers.len();
		if i >= vec_len {
			Ok("".to_string())
		} else {
			Ok(std::str::from_utf8(&self.headers[i].1)
				.unwrap_or(&"".to_string())
				.to_string())
		}
	}

	pub fn get_header_i_name(&self, i: usize) -> Result<String, Error> {
		let vec_len = self.headers.len();
		if i >= vec_len {
			Ok("".to_string())
		} else {
			Ok(std::str::from_utf8(&self.headers[i].0)
				.unwrap_or(&"".to_string())
				.to_string())
		}
	}

	pub fn get_header(&mut self, name: &str) -> Result<Option<String>, Error> {
		let name = name.to_string();
		if self.header_map.is_none() {
			self.build_header_map()?;
		}

		match self.header_map.as_ref() {
			Some(map) => {
				let value = map.get(&name);
				match value {
					Some(value) => Ok(Some((*value).clone())),
					None => Ok(None),
				}
			}
			None => Ok(None),
		}
	}

	pub fn get_headers(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>, Error> {
		Ok(self.headers.clone())
	}

	pub fn get_http_method(&self) -> Result<HttpMethod, Error> {
		Ok(self.http_method.clone())
	}

	pub fn get_http_version(&self) -> Result<HttpVersion, Error> {
		Ok(self.http_version.clone())
	}

	pub fn get_content(&self) -> Result<Vec<u8>, Error> {
		Ok(self.content.clone())
	}

	pub fn get_uri(&self) -> Result<String, Error> {
		Ok(self.uri.clone())
	}

	pub fn get_query(&self) -> Result<String, Error> {
		Ok(self.query.clone())
	}

	pub fn get_query_parameter(&mut self, name: &str) -> Result<Option<String>, Error> {
		let name = name.to_string();
		if self.query_map.is_none() {
			self.build_query_map()?;
		}

		match self.query_map.as_ref() {
			Some(map) => {
				let value = map.get(&name);
				match value {
					Some(value) => Ok(Some((*value).clone())),
					None => Ok(None),
				}
			}
			None => Ok(None),
		}
	}

	fn build_header_map(&mut self) -> Result<(), Error> {
		let mut map = HashMap::new();
		let vec_len = self.headers.len();
		for i in 0..vec_len {
			let key = std::str::from_utf8(&self.headers[i].0);
			let value = std::str::from_utf8(&self.headers[i].1);

			// we don't accept non utf-8 headers
			if key.is_err() || value.is_err() {
				continue;
			}

			map.insert(key.unwrap().to_string(), value.unwrap().to_string());
		}
		self.header_map = Some(map);
		Ok(())
	}

	fn build_query_map(&mut self) -> Result<(), Error> {
		let vec = querystring::querify(&self.query);
		let vec_len = vec.len();
		let mut map = HashMap::new();
		for i in 0..vec_len {
			map.insert(vec[i].0.to_string(), vec[i].1.to_string());
		}
		self.query_map = Some(map);
		Ok(())
	}
}

#[derive(Clone)]
pub struct RustletResponse {
	wh: WriteHandle,
	config: HttpConfig,
	headers_written: bool,
	keep_alive: bool,
}

impl RustletResponse {
	pub fn new(wh: WriteHandle, config: HttpConfig, keep_alive: bool) -> Self {
		RustletResponse {
			wh,
			config,
			headers_written: false,
			keep_alive,
		}
	}

	pub fn write(&mut self, data: &[u8]) -> Result<(), Error> {
		if !self.headers_written {
			HttpServer::write_headers(&self.wh, &self.config, true, self.keep_alive)?;
			self.headers_written = true;
		}

		let amt = data.len();
		if self.keep_alive {
			let msg_len_bytes = format!("{:X}\r\n", amt);
			let msg_len_bytes = msg_len_bytes.as_bytes();
			self.wh
				.write(msg_len_bytes, 0, msg_len_bytes.len(), false)?;
			self.wh.write(&data, 0, amt, false)?;
			self.wh.write("\r\n".as_bytes(), 0, 2, false)?;
		} else {
			self.wh.write(&data, 0, amt, false)?;
		}

		Ok(())
	}

	fn complete(&mut self) -> Result<(), Error> {
		let headers_written = crate::macros::LOCALRUSTLET.with(|f| match &(*f.borrow()) {
			Some((_request, response)) => response.headers_written,
			None => false,
		});
		if !headers_written {
			HttpServer::write_headers(&self.wh, &self.config, true, self.keep_alive)?;
			self.headers_written = true;
		}

		if self.keep_alive {
			self.wh.write("0\r\n\r\n".as_bytes(), 0, 5, false)?;
		} else {
			self.wh.close()?;
		}
		Ok(())
	}
}

pub type Rustlet =
	Pin<Box<dyn Fn(&mut RustletRequest, &mut RustletResponse) -> Result<(), Error> + Send + Sync>>;

pub(crate) struct RustletContainerHolder {
	rustlets: HashMap<String, Pin<Box<Rustlet>>>,
	mappings: HashMap<String, String>,
}

impl RustletContainerHolder {
	pub fn new() -> Self {
		RustletContainerHolder {
			rustlets: HashMap::new(),
			mappings: HashMap::new(),
		}
	}
}

lazy_static! {
	pub(crate) static ref RUSTLETS: Arc<RwLock<RustletContainerHolder>> =
		Arc::new(RwLock::new(RustletContainerHolder::new()));
}

pub struct RustletConfig {
	pub http_config: HttpConfig,
}

impl Default for RustletConfig {
	fn default() -> RustletConfig {
		RustletConfig {
			http_config: HttpConfig::default(),
		}
	}
}

fn api_callback(
	content: &[u8],                   // content of the request. len == 0 if none.
	method: HttpMethod,               // GET or POST
	config: HttpConfig,               // HttpServer's configuration
	wh: WriteHandle,                  // WriteHandle to write back data
	version: HttpVersion,             // HttpVersion
	uri: &str,                        // uri
	query: &str,                      // query
	headers: Vec<(Vec<u8>, Vec<u8>)>, // headers
	keep_alive: bool,                 // keep-alive
) -> Result<(), Error> {
	let rustlets = RUSTLETS.read().map_err(|e| {
		let error: Error = ErrorKind::InternalError(format!(
			"unexpected error: couldn't obtain RUSTLETS lock: {}",
			e.to_string(),
		))
		.into();
		error
	})?;

	let rustlet = rustlets.mappings.get(uri);
	match rustlet {
		Some(rustlet_name) => {
			let rustlet = rustlets.rustlets.get(rustlet_name);
			match rustlet {
				Some(rustlet) => {
					let mut request = RustletRequest::new(
						uri.to_string(),
						query.to_string(),
						content.to_vec(),
						method,
						version,
						config.clone(),
						headers,
						keep_alive,
					);
					let mut response = RustletResponse::new(wh, config, keep_alive);
					(rustlet)(&mut request, &mut response)?;
					response.complete()?;
				}
				None => {
					let mut response = RustletResponse::new(wh.clone(), config, keep_alive);
					response
						.write(format!("Rustlet '{}' does not exist.", rustlet_name).as_bytes())?;
					if keep_alive {
						wh.write("0\r\n\r\n".as_bytes(), 0, 5, false)?;
					} else {
						wh.close()?;
					}
				}
			}
		}
		None => {
			log_multi!(ERROR, MAIN_LOG, "error, no mapping for '{}'", uri);
		}
	}

	Ok(())
}

pub struct RustletContainer {
	config: Option<RustletConfig>,
	http: Option<HttpServer>,
}

impl RustletContainer {
	pub fn new() -> Self {
		RustletContainer {
			config: None,
			http: None,
		}
	}

	pub fn set_config(&mut self, config: RustletConfig) -> Result<(), Error> {
		let http = HttpServer::new(config.http_config.clone());
		self.config = Some(config);
		self.http = Some(http);
		Ok(())
	}

	pub fn start(&mut self) -> Result<(), Error> {
		let http = self.http.as_mut();
		match http {
			Some(mut http) => {
				http.config.callback = api_callback;
				http.start()?;
				http.add_api_extension("rsp".to_string())?;
			}
			None => {
				log_multi!(
					ERROR,
					MAIN_LOG,
					"Couldn't start rustlet: Configuration not found"
				);
			}
		}

		Ok(())
	}

	pub fn add_rustlet(&mut self, name: &str, rustlet: Rustlet) -> Result<(), Error> {
		let mut rustlets = RUSTLETS.write().map_err(|e| {
			let error: Error = ErrorKind::InternalError(format!(
				"unexpected error: couldn't obtain RUSTLETS lock: {}",
				e.to_string(),
			))
			.into();
			error
		})?;
		rustlets
			.rustlets
			.insert(name.to_string(), Box::pin(rustlet));

		Ok(())
	}

	pub fn add_rustlet_mapping(&mut self, path: &str, name: &str) -> Result<(), Error> {
		let mut rustlets = RUSTLETS.write().map_err(|e| {
			let error: Error = ErrorKind::InternalError(format!(
				"unexpected error: couldn't obtain RUSTLETS lock: {}",
				e.to_string(),
			))
			.into();
			error
		})?;

		match self.http.as_ref() {
			Some(http) => {
				http.add_api_mapping(path.to_string())?;
				rustlets.mappings.insert(path.to_string(), name.to_string());
			}
			None => {
				log_multi!(
					ERROR,
					MAIN_LOG,
					"Couldn't add rustlet mapping: Configuration not found"
				);
			}
		}

		Ok(())
	}
}
