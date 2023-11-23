use std::{str::FromStr, collections::HashMap};

use anyhow::{Result, anyhow};
use colored::*;
use mime::Mime;
use reqwest::{Url, Client, Response, header};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "httpie")]
#[command(author = "Fatmouse <burningkernel@icloud.com>")]
#[command(about = "Rust httpie")]
#[command(version, long_about = "Rust httpie from python")]
#[command(next_line_help = true)]
struct Cli {
    /// 请使用-h 获取帮助信息
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// GET 请求
    Get(Get),
    /// POST 请求
    Post(Post),
}

#[derive(Args, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[arg(value_parser = parse_url)]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

/// 命令行中的key=value 可以通过parse_kv_pair 解析成KvPair结构
#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}

/// 实现自己的FromStr trait
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行split 这样会得到一个迭代器
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {s}"));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为KvPair实现了FromStr，这里可以直接s.parse()得到KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}


#[derive(Args, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[arg(value_parser = parse_url)]
    url: String,
    /// HTTP 请求的 body
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[tokio::main]
async fn main() -> Result<()>{
    let cli = Cli::parse();
    let client = Client::new();
    match cli.command {
        Commands::Get(ref args) => {
            get(client, args).await?
        },
        Commands::Post(ref args) => {
            post(client, args).await?
        }
    }
    Ok(())
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    print_resp(resp).await
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    print_resp(resp).await
}

/// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{status}\n");
}

/// 打印服务器HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!();
}

/// 打印服务器返回的body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        },
        _ => println!("{body}"),
    }
}

/// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

/// 将服务器返回的content-type类型解析成Mime类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}
