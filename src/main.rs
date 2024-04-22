use std::env;
use reqwest::Error;
use select::document::Document;
use select::predicate::{Name, Attr, Predicate};

#[tokio::main]
async fn main() -> Result<(), Error>{

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <search>", args[0]);
        return Ok(());
    }

    let mut client_search = String::new();
    for arg in args.iter().skip(1) {
        client_search.push_str(arg);
        client_search.push('+'); // Add space between arguments
    }

    let client = reqwest::Client::new();

    let res = client.get(&format!("https://www.kuantokusta.pt/search?q={}&sort=3", client_search))
    .header("User-Agent", "chrome")
    .header("Accept-Language", "en-US")
    // Add more headers if required
    .send()
    .await?;

    println!("Request status: {}", res.status());

    if res.status().is_success() {
        let body = res.text().await?;

        let document = Document::from(body.as_str());

        for node in document.find(Attr("data-test-id", "product-card").descendant(Name("a"))) {
            if let Some(href) = node.attr("href") {
                if href.contains(&client_search.replace("+", "").trim()) || href.contains(&client_search.replace("+", "-").trim()){
                    println!("https://www.kuantokusta.pt{}", href);
                    break;
                }
            }
        }
    } else {
        println!("Request failed with status: {}", res.status());
    }
    
    Ok(())
}
