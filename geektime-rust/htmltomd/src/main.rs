use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{ 
    let args:Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("You should specify a url and a file name");
        return Ok(());
    }
    // let url = "https://www.rust-lang.org";
    // let output = "rust-lang.md";
    let url = args[1].clone();
    let output = args[2].clone();
    println!("Fetching url: {url}");

    let body = reqwest::get(url).await?.text().await?;

    println!("Html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes())?;
    println!("Convert success!");
    Ok(())
}
