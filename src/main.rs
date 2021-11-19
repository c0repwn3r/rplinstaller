mod rpkg;

use std::collections::HashMap;

use online::sync::check;
use colored::Colorize;

use std::cmp::min;
use std::fs::File;
use std::io::Write;

use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("█  "));
    pb.set_message(&format!("Downloading {}", url));

    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    return Ok(());
}

fn main() {
  println!("[*] Welcome to the RPL++ installer!");
  println!("[*] Checking for internet connectivity");
  let mut is_online: bool = false;
  if !check(None).is_ok() {
    println!("{}: no internet, offline install", "warn".yellow().bold());
  } else {
    is_online = true;
  }
  if is_online {
    println!("{}: this installer build does not support online builds", "warn".yellow().bold());
  }

  let mut builds: HashMap<String, String> = HashMap::new();
  builds.insert("1.3.0".to_string(), "1.3.x/rpl-1.3.0.zip".to_string());
  builds.insert("1.4.0-rc1".to_string(), "1.4.x/rpl-1.4.0-rc1.zip".to_string());
  builds.insert("1.4.0-rc2".to_string(), "1.4.x/rpl-1.4.0-rc2.zip".to_string());
  builds.insert("1.4.0-rc3".to_string(), "1.4.x/rpl-1.4.0-rc3.zip".to_string());
  builds.insert("1.4.0-rc4".to_string(), "1.4.x/rpl-1.4.0-rc-final.zip".to_string());
  builds.insert("1.4.0".to_string(), "1.4.x/rpl-1.4.0.zip".to_string());
  builds.insert("1.4.0A".to_string(), "1.4.x/rpl-1.4.0A.zip".to_string());
  builds.insert("1.4.0B".to_string(), "1.4.x/rpl-1.4.0B.zip".to_string());
  let latest: String = "1.4.0B".to_string();
  builds.insert("latest".to_string(), latest.clone());

  // install steps
  /*
  welcome to rpl
  sourcing
  what version do you want to install?
  where do you want to install it?
  do you want to autoconfigure shell?
  summary
  go
  done
   */
  
  let mut input: String = "".to_string();
  
  print!("Avaliable versions (latest {}): ", builds["latest"]);
  for (key, _value) in &builds {
    if key == &"latest".to_string() {
      continue;
    }
    print!("{}, ", key.bold());
  }
  println!();
  println!("[*] Which version should be installed? [latest] ");
  std::io::stdin().read_line(&mut input).expect("Failed to get input!");
  input = input[0..input.len()-1].to_string();
  let build: String;
  let mut buildurl: String = "https://rpl-bash.tm85.repl.co/download/".to_string();
  if !builds.contains_key(&input) {
    if input != "" {
        println!("{} {} is not a valid version, defaulting to latest", "error:".red().bold(), input);
    }
    build = latest;
    buildurl += &builds[&build];
  } else {
    build = input;
    buildurl += &builds[&build];
  }
  println!("Configured to install version {} from {}", build.bold(), buildurl.bold());
  println!("[*] Where should it be installd? [~/.rpl/] ");
  input = "".to_string();
  std::io::stdin().read_line(&mut input).expect("Failed to get input!");
  input = input[0..input.len()-1].to_string();
  let iloc: String;
  if input == "".to_string() {
    iloc = "~/.rpl/".to_string();
  } else {
    iloc = input;
  }
  println!("Installing to {}", iloc.bold());
  println!("[*] Should rplinstaller autoconfigure your shell for RPL? [Y/n]");
  input = "".to_string();
  std::io::stdin().read_line(&mut input).expect("Failed to get input!");
  input = input[0..input.len()-1].to_string();
  let do_autoconf: bool;
  if input.to_lowercase() == "n".to_string() {
    do_autoconf = false;
  } else {
    do_autoconf = true;
  }
  println!("rplinstaller {} autoconfigure your shell for RPL.", if do_autoconf {"will".green().bold()} else {"will not".red().bold()});
  println!("rplinstaller is ready to install rpl.");
  println!("Summary of operations:");
  println!("Version {} will be installed to {} from mirror {}", build.bold(), iloc.bold(), buildurl.bold());
  println!("Press Enter to commence installation or Ctrl+C to cancel.");
  std::io::stdin().read_line(&mut input);
  // install phases:
  // 1: source build
  // 2: prepare sources directory
  // 3: download source
  // 4: unpack sources
  // 5: verify sources
  // 6: download initial rpkg repo
  // 7: source rpkg
  // 8: bootstrap rpkg
  // 9: rpkg --update-deps-initial --from-repo=https://rpl-bash.tm85.repl.co/download/rpkg/initial.repo
  // 10: rpkg --reinstall-deps --bootstrap
  // 11: rpkg check
  // 12: verify install
  // done
  println!("[0/12] install: preparing");
  // sources: in .A->buildid
  let pkgname: String = "rpl@".to_string() + &build;
  println!("[1/12] install: rpkg-bundled --source {}", pkgname);
  // source with bundled rpkg
  // basically does nothing as we already have the sources file above
}
