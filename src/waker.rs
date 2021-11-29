use surf::http::Body;
use surf::http::convert::{Serialize, Deserialize};
//use async_native_tls::Protocol;
//use async_native_tls::TlsConnector;
//use std::sync::Arc;
//use std::time::Duration;

// const CONTENT_TYPE:&str = "application/x-www-form-urlencoded";

fn generate_login_authorization(user_name: &str, password: &str) -> String {
	let s = format!("{}:{}", user_name, password);
	let auth = base64::encode(s);
	auth
}

#[derive(Debug, Serialize, Deserialize)]
struct Param<'a> {
	login_authorization: Option<&'a str>,
	group_id: Option<&'a str>,  //可选项
	modified: Option<&'a str>,  //可选项
	action_mode: Option<&'a str>, //必选项
	action_script: Option<&'a str>,  //可选项
	action_wait: Option<&'a str>,  //可选项
	current_page: Option<&'a str>, //必选项
	next_page: Option<&'a str>, //必选项
	firmver: Option<&'a str>,  //可选项
	first_time: Option<&'a str>,  //可选项
	preferred_lang: Option<&'a str>,  //可选项
	destIP:Option<&'a str>,  //可选项
	SystemCmd: Option<&'a str>, //必选项
	wollist_macAddr: Option<&'a str>,  //可选项
}

pub struct Waker {
	user_name: String,
	user_password: String,
	url: String,
	cookie: String,
	client: surf::Client,
}

impl Waker {
	pub fn new(url: String, user_name: String, user_password: String) -> Waker {
		/*
				let tls = TlsConnector::new()
					.min_protocol_version(Some(Protocol::Tlsv10))
					.max_protocol_version(Some(Protocol::Tlsv10))
					.danger_accept_invalid_certs(true)
					.danger_accept_invalid_hostnames(true)
					.use_sni(false);

				let cfg = surf::Config::new()
					.set_timeout(Some(Duration::from_secs(5)))
					.set_tls_config(Some(Arc::new(tls)));
		*/
		let client = surf::Client::new();

		Waker {
			user_name,
			user_password,
			url: url,
			cookie: String::from(""),
			client,
		}
	}

	#[allow(dead_code)]
	async fn get(&self, page: &str) -> Result<String,String> {
		let base_url = self.url.clone();
		let current_page = "Main_Login.asp";

		let url = format!("{}/{}", base_url, page);
		let referer = format!("{}/{}", base_url, current_page);

		let res = self.client
			.get(url)
			.header(surf::http::headers::REFERER, referer)
			.header(surf::http::headers::COOKIE, self.cookie.clone()) //必选项
			// .header(surf::http::headers::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")  //可选项
			// .header(surf::http::headers::ACCEPT_ENCODING, "gzip, deflate")  //可选项
			// .header(surf::http::headers::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2")  //可选项
			// .header(surf::http::headers::CONNECTION, "keep-alive")  //可选项
			// .header(surf::http::headers::HOST ,"192.168.5.1")  //可选项
			// .header(surf::http::headers::ORIGIN, base_url)  //可选项
			// .header("Upgrade-Insecure-Requests" ,"1")  //可选项
			.await;

		match res {
			Ok(mut r) => match r.body_string().await {
				Ok(body) => Ok(body),
				Err(e) => Err(e.to_string()),
			},
			Err(e) => Err(e.to_string())
		}
	}

	pub async fn login(self: &mut Self) -> Result<(),String> {

		let base_url = self.url.clone();
		let current_page = "Main_Login.asp";
		let next_page = "index.asp";

		let auth = generate_login_authorization(&self.user_name, &self.user_password);
		let param = Param {
			login_authorization:Some(&auth),
			group_id: Some(""),
			action_script: Some(""),
			action_wait: Some("5"),
			action_mode: Some(""),
			current_page: Some(&current_page),
			next_page: Some(&next_page),
			modified: None,
			preferred_lang: None,
			destIP: None,
			firmver: None,
			first_time: None,
			SystemCmd: None,
			wollist_macAddr: None,
		};

		let url = format!("{}/{}", base_url, "login.cgi");
		let referer = format!("{}/{}", base_url, current_page);
		let form = match Body::from_form(&param){
			Ok(f) => f,
			Err(e) => return Err(e.to_string()),
		};

		let res = self.client
			.post(url)
			.header(surf::http::headers::REFERER, referer) // 必选项
			// .header(surf::http::headers::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")  //可选项
			// .header(surf::http::headers::ACCEPT_ENCODING, "gzip, deflate")  //可选项
			// .header(surf::http::headers::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2")  //可选项
			// .header(surf::http::headers::CONNECTION, "keep-alive")  //可选项
			// .header(surf::http::headers::HOST ,"192.168.5.1")  //可选项
			// .header(surf::http::headers::ORIGIN, base_url)  //可选项
			// .header("Upgrade-Insecure-Requests" ,"1")  //可选项
			//.body_string(params)
			.body(form) // 必选项
			.await;

		match res {
			Ok(r) => {
				let h = r.header("Set-Cookie");
				if let Some(v) = h {
					for i in v.iter() {
						let m = i.to_string().clone();
						let v: Vec<&str> = m.split(';').collect();
						let t: Vec<&str> = v[0].split('=').collect();
						let token = t[1].to_owned();
						self.cookie = format!("asus_token={}", token);
						break;
					}
				}
				if self.cookie.len() > 0 {
					Ok(())
				} else {
					Err("can't get token, login failed!".to_string())
				}
			},
			Err(e) => {
				Result::Err(e.to_string())
			}
		}
	}

	pub async fn execute_command(&self, cmd: &str) -> Result<(), String> {
		let current_page = "Main_WOL_Content.asp";
		let next_page = "Main_WOL_Content.asp";

		let param = Param{
			login_authorization: None,
			group_id: Some(""),
			action_script: Some(""),
			action_wait: Some("5"),
			action_mode: Some(" Refresh "),
			current_page: Some(&current_page),
			next_page: Some(&next_page),
			modified: Some(""),
			preferred_lang: None,
			destIP: Some(""),
			firmver: Some("3.0.0.4"),
			first_time: Some(""),
			SystemCmd: Some(cmd),
			wollist_macAddr: Some(""),
		};

		let form = match Body::from_form(&param){
			Ok(f) => f,
			Err(e) => return Err(e.to_string()),
		};

		let referer = format!("{}/{}", self.url, current_page);
		let url = format!("{}/{}", self.url, "apply.cgi");
		let res = self.client
			.post(url)
			.header(surf::http::headers::REFERER, referer)
			.header(surf::http::headers::COOKIE, self.cookie.clone()) //必选项
			// .header(surf::http::headers::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")  //可选项
			// .header(surf::http::headers::ACCEPT_ENCODING, "gzip, deflate")  //可选项
			// .header(surf::http::headers::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2")  //可选项
			// .header(surf::http::headers::CONNECTION, "keep-alive")  //可选项
			// .header(surf::http::headers::HOST ,"192.168.5.1")  //可选项
			// .header(surf::http::headers::ORIGIN, base_url)  //可选项
			// .header("Upgrade-Insecure-Requests" ,"1")  //可选项
			.body(form) // 必选项
			.await;

		match res {
			Ok(_) => Ok(()),
			Err(e) => Err(e.to_string())
		}
	}

	pub async fn logout(&self) -> Result<(),String> {
		let current_page = "Main_WOL_Content.asp";
//		let next_page = "Main_WOL_Content.asp";

		let referer = format!("{}/{}", self.url, current_page);
		let url = format!("{}/{}", self.url, "logout.asp");
		let res = self.client
			.get(url)
			.header(surf::http::headers::REFERER, referer)
			.header(surf::http::headers::COOKIE, self.cookie.clone()) //必选项
			// .header(surf::http::headers::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")  //可选项
			// .header(surf::http::headers::ACCEPT_ENCODING, "gzip, deflate")  //可选项
			// .header(surf::http::headers::ACCEPT_LANGUAGE, "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2")  //可选项
			// .header(surf::http::headers::CONNECTION, "keep-alive")  //可选项
			// .header(surf::http::headers::HOST ,"192.168.5.1")  //可选项
			// .header(surf::http::headers::ORIGIN, base_url)  //可选项
			// .header("Upgrade-Insecure-Requests" ,"1")  //可选项
			.await;

		match res {
			Ok(_) => Ok(()),
			Err(e) => Err(e.to_string()),
		}
	}
}
