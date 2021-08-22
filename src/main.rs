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

use clap::load_yaml;
use clap::App;
use errno::errno;
use libc::close;
use librustlet::*;
use log::*;
use nioruntime_util::{Error, ErrorKind};
use std::convert::TryInto;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};

const MAX_BUF: usize = 100_000;

log::debug!();

fn fun() -> Result<(), Error> {
	rustlet!("error", {
		response!("<html><body>test of error");
		if true {
			return Err(ErrorKind::InternalError("test error".to_string()).into());
		}
	});
	Ok(())
}

fn fun2() -> Result<(), Error> {
	rustlet!("panic", {
		response!("<html><body>test of panic");
		flush!();
		let x: Option<bool> = None;
		x.unwrap();
	});
	Ok(())
}

// include build information
pub mod built_info {
	include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn client_thread(
	count: usize,
	id: usize,
	tlat_sum: Arc<Mutex<f64>>,
	tlat_max: Arc<Mutex<u128>>,
) -> Result<(), Error> {
	let mut lat_sum = 0.0;
	let mut lat_max = 0;
	let (mut stream, fd) = {
		let _lock = tlat_sum.lock();
		let stream = TcpStream::connect("127.0.0.1:8080")?;
		#[cfg(unix)]
		let fd = stream.as_raw_fd();
		#[cfg(target_os = "windows")]
		let fd = stream.as_raw_socket();
		(stream, fd)
	};
	let buf2 = &mut [0u8; MAX_BUF];
	let start_itt = std::time::SystemTime::now();
	let uri = "/empty";
	let request_string = format!(
		"GET {} HTTP/1.1\r\nHost: localhost:8080\r\nConnection: keep-alive\r\n\r\n",
		uri
	);
	let request_bytes = request_string.as_bytes();
	for i in 0..count {
		if i != 0 && i % 10000 == 0 {
			let elapsed = start_itt.elapsed().unwrap().as_millis();
			let qps = (i as f64 / elapsed as f64) * 1000 as f64;
			info!("Request {} on thread {}, qps={}", i, id, qps);
		}
		let start_query = std::time::SystemTime::now();
		let res = stream.write(&request_bytes);

		match res {
			Ok(_x) => {}
			Err(e) => {
				info!("Write Error: {}", e.to_string());
				std::thread::sleep(std::time::Duration::from_millis(1));
			}
		}

		let mut len_sum = 0;
		loop {
			let res = stream.read(&mut buf2[len_sum..]);
			match res {
				Ok(_) => {}
				Err(ref e) => {
					info!("Read Error: {}, fd = {}", e.to_string(), fd);
					assert!(false);
				}
			}
			let len = res.unwrap();
			len_sum += len;
			if len_sum >= 5
				&& buf2[len_sum - 1] == '\n' as u8
				&& buf2[len_sum - 2] == '\r' as u8
				&& buf2[len_sum - 3] == '\n' as u8
				&& buf2[len_sum - 4] == '\r' as u8
				&& buf2[len_sum - 5] == '0' as u8
			{
				break;
			}
		}

		let elapsed = start_query.elapsed().unwrap().as_nanos();
		lat_sum += elapsed as f64;
		if elapsed > lat_max {
			lat_max = elapsed;
		}

		// clear buf2
		for i in 0..len_sum {
			buf2[i] = 0;
		}
	}

	{
		let _lock = tlat_sum.lock();
		#[cfg(unix)]
		let close_res = unsafe { close(fd.try_into().unwrap_or(0)) };
		#[cfg(target_os = "windows")]
		let close_res = unsafe { ws2_32::closesocket(fd.try_into().unwrap_or(0)) };
		if close_res != 0 {
			let e = errno();
			info!("error close {} (fd={})", e.to_string(), fd);
		}
		drop(stream);
	}
	{
		let mut tlat_sum = tlat_sum.lock().unwrap();
		(*tlat_sum) += lat_sum;
	}
	{
		let mut tlat_max = tlat_max.lock().unwrap();
		if lat_max > *tlat_max {
			(*tlat_max) = lat_max;
		}
	}

	Ok(())
}

fn main() {
	let yml = load_yaml!("rustlet.yml");
	let args = App::from_yaml(yml)
		.version(built_info::PKG_VERSION)
		.get_matches();

	let client = args.is_present("client");

	if client {
		let threads = args.is_present("threads");
		let count = args.is_present("count");
		let itt = args.is_present("itt");

		let threads = match threads {
			true => args.value_of("threads").unwrap().parse().unwrap(),
			false => 1,
		};

		let count = match count {
			true => args.value_of("count").unwrap().parse().unwrap(),
			false => 1,
		};

		let itt = match itt {
			true => args.value_of("itt").unwrap().parse().unwrap(),
			false => 1,
		};

		info!("Running client");
		info!("Threads={}", threads);
		info!("Iterations={}", itt);
		info!("Requests per thread per iteration={}", count);
		info_no_ts!(
			"--------------------------------------------------------------------------------"
		);

		let time = std::time::SystemTime::now();
		let tlat_sum = Arc::new(Mutex::new(0.0));
		let tlat_max = Arc::new(Mutex::new(0));

		for x in 0..itt {
			let mut jhs = vec![];
			for i in 0..threads {
				let id = i.clone();
				let tlat_sum = tlat_sum.clone();
				let tlat_max = tlat_max.clone();
				jhs.push(std::thread::spawn(move || {
					let res = client_thread(count, id, tlat_sum.clone(), tlat_max.clone());
					match res {
						Ok(_) => {}
						Err(e) => {
							info!("Error in client thread: {}", e.to_string());
							assert!(false);
						}
					}
				}));
			}

			for jh in jhs {
				jh.join().expect("panic in thread");
			}
			info!("Iteration {} complete. ", x + 1);
		}

		let elapsed_millis = time.elapsed().unwrap().as_millis();
		let lat_max = tlat_max.lock().unwrap();
		info_no_ts!(
			"--------------------------------------------------------------------------------"
		);
		info!("Test complete in {} ms", elapsed_millis);
		let tlat = tlat_sum.lock().unwrap();
		let avg_lat = (*tlat) / (1_000_000 * count * threads * itt) as f64;
		//let qps_simple = (1000.0 / avg_lat) * threads as f64;
		let qps = (threads * count * itt * 1000) as f64 / elapsed_millis as f64;
		info!("QPS={}", qps);
		info!("Average latency={}ms", avg_lat,);
		info!("Max latency={}ms", (*lat_max) as f64 / (1_000_000 as f64));
	} else {
		rustlet_init!(RustletConfig {
			http_config: HttpConfig {
				debug: true,
				..Default::default()
			},
		});

		let x = Arc::new(Mutex::new(0));
		let x_clone = x.clone();

		rustlet!("empty", {});

		rustlet!("cookies", {
			let cookie = cookie!("abc");
			set_cookie!("abc", "def");
			response!("cookie={:?}\n", cookie);
		});

		rustlet!("async", {
			let ac = async_context!();
			response!("first message\n");

			std::thread::spawn(move || {
				async_context!(ac);
				// simulate long running task:
				std::thread::sleep(std::time::Duration::from_millis(1000));
				response!("second message\n");
				async_complete!();
			});
		});

		rustlet!("redir", {
			set_redirect!("http://www.disney.com");
		});

		rustlet!("myrustlet", {
			let name = query!("name");
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
			for i in 0..header_len!() {
				let header_name = header_name!(i);
				let header_value = header_value!(i);
				response!("header[{}] [{}] -> [{}]\n", i, header_name, header_value);
				flush!();
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
		rustlet_mapping!("/async", "async");
		rustlet_mapping!("/cookies", "cookies");
		rustlet_mapping!("/empty", "empty");

		std::thread::park();
	}
}
