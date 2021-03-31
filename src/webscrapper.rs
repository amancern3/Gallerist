extern crate reqwest; 
extern crate scraper;
extern crate tokio;


use scraper::{Html, Selector};

pub async fn object_of_the_day(url: &str) {
    
    let req = reqwest::get(url).await.unwrap();
    assert!(req.status().is_success());

    let body = Html::parse_document(&req.text().await.unwrap());

    // Object of the day = OFTD
    let oftd = Selector::parse(".h2").unwrap();
    println!("made it !");
    for oftd in body.select(&oftd){
        let oftds = oftd.text().collect::<Vec<_>>();
        println!("{}", oftds[0]);
    }

}