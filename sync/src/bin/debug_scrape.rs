#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    let name = "Black Lotus";
    let encoded = name.replace(' ', "+");

    // eBay
    let ebay_url = format!(
        "https://www.ebay.com/sch/i.html?_nkw={}&LH_Sold=1&LH_Complete=1",
        encoded
    );
    println!("=== EBAY ===");
    match client.get(&ebay_url).send().await {
        Ok(resp) => {
            let html = resp.text().await.unwrap_or_default();
            if let Some(pos) = html.to_lowercase().find("s-item__price") {
                println!("{}", &html[pos.saturating_sub(100)..pos + 300]);
            } else {
                println!("s-item__price not found in response");
                println!("First 2000 chars:\n{}", &html[..html.len().min(2000)]);
            }
        }
        Err(e) => println!("eBay error: {}", e),
    }

    // TCGPlayer
    let tcg_url = format!(
        "https://www.tcgplayer.com/search/all/product?q={}",
        encoded
    );
    println!("\n=== TCGPLAYER ===");
    match client.get(&tcg_url).send().await {
        Ok(resp) => {
            let html = resp.text().await.unwrap_or_default();
            if let Some(pos) = html.to_lowercase().find("price") {
                println!("{}", &html[pos.saturating_sub(100)..pos + 300]);
            } else {
                println!("'price' not found in response");
                println!("First 2000 chars:\n{}", &html[..html.len().min(2000)]);
            }
        }
        Err(e) => println!("TCGPlayer error: {}", e),
    }
}