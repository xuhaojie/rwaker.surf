mod config;
mod waker;
use waker::Waker;
extern crate getopts;
use getopts::Options;
use std::env;
use std::path::PathBuf;

pub async fn wake(waker: &mut Waker, target: &str) -> Result<(),String> {
	waker.login().await?;

	let cmd = format!("ether-wake -i br0 -b {}", target);
	let ret = waker.execute_command(cmd.as_str()).await;

	waker.logout().await?;
	ret
}

#[async_std::main]
async fn main() -> Result<(), String> {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optopt("m", "mac", "target mac", "MAC");
	opts.optopt("n", "name", "target name", "NAME");
	opts.optflag("g", "generate", "generate default config");
	opts.optflag("l", "list", "List targets");

	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m },
		Err(_) => { return Ok(()) }
	};

	let mut config_file : PathBuf;
	match dirs::config_dir(){
		Some(path) => {
			config_file = path;
		},
		None => {
			println!("failed to get config path");
			return Ok(())
		}
	}

	config_file.push("rwaker.cfg");
	if matches.opt_present("g") {
		match config::default().save(&config_file) {
			Ok(()) => println!("sample config {} generated.", config_file.display()),
			Err(e) => println!("generate sample config failed. {}", e),
		}
		return Ok(())
	}

	let cfg = config::load(&config_file)?;

	if matches.opt_present("l") {
		for target in cfg.targets {
			println!("{name:>width$} : {mac}",name=target.name,width=10, mac=target.mac);
		}
		return Ok(())
	}

	let mut target = String::from("");
	let mut  mac = String::from("");

	if matches.opt_present("m"){
		if let Some(m) = matches.opt_str("m") {
			mac = m.to_string();
			if mac.len() != 17 {
				return Err(format!("invalid mac address {}", mac))
			}
		}
	} else if matches.opt_present("n") {
		if let Some(n) = matches.opt_str("n") {
			target = n.to_string();
			mac =  cfg.find(&n)?;
		}
	} else {
		let brief = format!("Usage: {} [options]", program);
		print!("{}", opts.usage(&brief));
		return Ok(())
	}

	if target.is_empty() {
		println!("Wake {} ...", mac);
	} else {
		println!("Wake {} with mac {} ...", &target, mac);
	}

	let mut waker = Waker::new(cfg.url,cfg.user,cfg.password);

	wake(&mut waker,&mac).await?;
	println!("done.");
	Ok(())
}