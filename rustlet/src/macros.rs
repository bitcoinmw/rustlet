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

use crate::RustletContainer;
use crate::{RustletRequest, RustletResponse};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use std::thread_local;

thread_local!(
	pub static LOCALRUSTLET: RefCell<
		Option<
			(RustletRequest,RustletResponse)
		>
	> = RefCell::new(None)
);

lazy_static! {
	pub static ref RUSTLET_CONTAINER: Arc<RwLock<RustletContainer>> =
		Arc::new(RwLock::new(RustletContainer::new()));
}

#[macro_export]
macro_rules! rustlet {
	($a:expr,$b:expr) => {
		let mut container = librustlet::macros::RUSTLET_CONTAINER.write();
		match container {
			Ok(mut container) => {
				let res = container.add_rustlet(
					$a,
					Box::pin(
						move |request: &mut RustletRequest, response: &mut RustletResponse| {
							librustlet::macros::LOCALRUSTLET.with(|f| {
								*f.borrow_mut() = Some(((*request).clone(), (*response).clone()));
							});
							{
								$b
							}
							Ok(())
						},
					),
				);
				match res {
					Ok(_) => {}
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						log_multi!(
							ERROR,
							MAIN_LOG,
							"Error adding rustlet to container: {}",
							e.to_string()
						);
					}
				}
			}
			Err(e) => {
				const MAIN_LOG: &str = "mainlog";
				log_multi!(
					ERROR,
					MAIN_LOG,
					"Couldn't start rustlet: couldn't get lock: {}",
					e.to_string()
				);
			}
		}
	};
}

#[macro_export]
macro_rules! rustlet_init {
	($config:expr) => {{
		let mut container = librustlet::macros::RUSTLET_CONTAINER.write();

		match container {
			Ok(mut container) => {
				let res = container.set_config($config);
				match res {
					Ok(_) => {
						let res = container.start();
						match res {
							Ok(_) => {}
							Err(e) => {
								const MAIN_LOG: &str = "mainlog";
								log_multi!(
									ERROR,
									MAIN_LOG,
									"Couldn't start rustlet: start: {}",
									e.to_string()
								);
							}
						}
					}
					Err(e) => {
						const MAIN_LOG: &str = "mainlog";
						log_multi!(
							ERROR,
							MAIN_LOG,
							"Couldn't start rustlet: set_config: {}",
							e.to_string()
						);
					}
				}
			}
			Err(e) => {
				const MAIN_LOG: &str = "mainlog";
				log_multi!(
					ERROR,
					MAIN_LOG,
					"Couldn't start rustlet: couldn't get lock: {}",
					e.to_string()
				);
			}
		}
	}};
}

#[macro_export]
macro_rules! rustlet_mapping {
	($a:expr, $b:expr) => {{
		let mut container = librustlet::macros::RUSTLET_CONTAINER.write();

		match container {
			Ok(mut container) => match container.add_rustlet_mapping($a, $b) {
				Ok(_) => {}
				Err(e) => {
					const MAIN_LOG: &str = "mainlog";
					log_multi!(
						ERROR,
						MAIN_LOG,
						"Couldn't start rustlet: add_mapping: {}",
						e.to_string()
					);
				}
			},
			Err(e) => {
				const MAIN_LOG: &str = "mainlog";
				log_multi!(
					ERROR,
					MAIN_LOG,
					"Couldn't start rustlet: couldn't get lock: {}",
					e.to_string()
				);
			}
		}
	}};
}

#[macro_export]
macro_rules! response {
	($a:expr)=>{
                {
			librustlet::macros::LOCALRUSTLET.with(|f| {
				match &mut (*f.borrow_mut()) {
					Some((request,response)) => {
						let res = response.write(format!($a).as_bytes());
						match res {
							Ok(_) => {},
							Err(e) => {
								const MAIN_LOG: &str = "mainlog";
								log_multi!(
									ERROR,
									MAIN_LOG,
									"Couldn't call response.write: {}",
									e.to_string()
								);
							},
						}
					},
					None => {
						const MAIN_LOG: &str = "mainlog";
                                        	log_multi!(
                                                	ERROR,
                                                	MAIN_LOG,
                                                	"Couldn't find response struct",
                                        	);
					},
				}
			});
		}
	};
	($a:expr,$($b:tt)*)=>{
		{
                        librustlet::macros::LOCALRUSTLET.with(|f| {
                                match &mut (*f.borrow_mut()) {
                                        Some((request,response)) => {
                                                let res = response.write(format!($a, $($b)*).as_bytes());
                                                match res {
                                                        Ok(_) => {},
                                                        Err(e) => {
								mainlogerror!(
									"Couldn't call response.write: {}",
									e.to_string(),
								);
                                                        },
                                                }
                                        },
                                        None => {
						mainlogerror!("Couldn't find response struct");
                                        },
                                }
                        });
		}

	};
}

#[macro_export]
macro_rules! request_content {
	() => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &(*f.borrow()) {
			Some((request, response)) => match request.get_content() {
				Ok(content) => content,
				Err(e) => {
					mainlogerror!("unexpected error get_content generated: {}", e.to_string());
					vec![]
				}
			},
			None => {
				mainlogerror!("unexpected error no request/response found");
				vec![]
			}
		})
	}};
}

#[macro_export]
macro_rules! request {
	($a:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => {
				let value = $a.to_lowercase();
				if value == "query" {
					request.get_query().unwrap_or("".to_string())
				} else if value == "method" {
					match request
						.get_http_method()
						.unwrap_or(nioruntime_http::HttpMethod::Get)
					{
						nioruntime_http::HttpMethod::Get => "GET".to_string(),
						nioruntime_http::HttpMethod::Post => "POST".to_string(),
					}
				} else if value == "version" {
					match request
						.get_http_version()
						.unwrap_or(nioruntime_http::HttpVersion::V10)
					{
						nioruntime_http::HttpVersion::V09 => "V09".to_string(),
						nioruntime_http::HttpVersion::V10 => "V10".to_string(),
						nioruntime_http::HttpVersion::V11 => "V11".to_string(),
						nioruntime_http::HttpVersion::V20 => "V20".to_string(),
					}
				} else if value == "header_len" {
					format!("{}", request.get_header_len().unwrap_or(0))
				} else if value == "uri" {
					request.get_uri().unwrap_or("".to_string())
				} else {
					mainlogerror!("unknown parameter: '{}'", $a);
					"".to_string()
				}
			}
			None => {
				mainlogerror!("unexpected error no request/response found");
				"".to_string()
			}
		})
	}};
	($a:expr,$b:expr) => {{
		librustlet::macros::LOCALRUSTLET.with(|f| match &mut (*f.borrow_mut()) {
			Some((request, response)) => {
				let value = $a.to_lowercase();
				if value == "query" {
					let qp = request.get_query_parameter($b);
					match qp {
						Ok(qp) => match qp {
							Some(qp) => qp,
							None => "".to_string(),
						},
						Err(e) => {
							mainlogerror!("query error: {}", e);
							"".to_string()
						}
					}
				} else if value == "header_i_name" {
					let usize_value: usize = $b.parse().unwrap_or(usize::MAX);
					match request.get_header_i_name(usize_value) {
						Ok(name) => name,
						Err(e) => {
							mainlogerror!("header_i_name error: {}", e);
							"".to_string()
						}
					}
				} else if value == "header_i_value" {
					let usize_value: usize = $b.parse().unwrap_or(usize::MAX);
					match request.get_header_i_value(usize_value) {
						Ok(name) => name,
						Err(e) => {
							mainlogerror!("header_i_name error: {}", e);
							"".to_string()
						}
					}
				} else if value == "header" {
					let header = request.get_header($b);
					match header {
						Ok(header) => match header {
							Some(header) => header,
							None => "".to_string(),
						},
						Err(e) => {
							mainlogerror!("header error: {}", e);
							"".to_string()
						}
					}
				} else {
					"".to_string()
				}
			}
			None => {
				mainlogerror!("unexpected error no request/response found");
				"".to_string()
			}
		})
	}};
}

#[macro_export]
macro_rules! mainlogerror {
	($a:expr) => {{
		const MAIN_LOG: &str = "mainlog";
		log_multi!(
			ERROR,
			MAIN_LOG,
			$a,
		);
	}};
	($a:expr,$($b:tt)*)=>{{
                const MAIN_LOG: &str = "mainlog";
                log_multi!(
                        ERROR,
                        MAIN_LOG,
                        $a,
			$($b)*
                );
	}};

}
