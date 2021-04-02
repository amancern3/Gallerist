extern crate reqwest; 
extern crate scraper;
extern crate tokio;
extern crate image;

use std::io::copy;
use std::fs::File;
use tempfile::Builder;

use scraper::{Html, Selector};

pub async fn object_of_the_day(url: &str) {
    
    let req = reqwest::get(url).await.unwrap();
    assert!(req.status().is_success());

    let body = Html::parse_document(&req.text().await.unwrap());


    // Now we have the entire html body, now to parse for the header tags 
    // impo ref: https://docs.rs/scraper/0.12.0/scraper/

    // Object of the day = oftd

    // tutorial to download images:
    // get img tag by parsing and then the img src by parsing
    // use the collected h2 tag as filename
    // write the link to the file ? 
    // ref: https://www.youtube.com/watch?v=m_agcM_ds1c

    // alt use crate image to write image buff 
    let oftd = Selector::parse("h2").unwrap();
    println!("made it !");

   
    let fig_selector = Selector::parse("figure").unwrap();
    let src_selector = Selector::parse("img").unwrap();
    /*for oftd in body.select(&oftd){
        let oftds = oftd.text().collect::<Vec<_>>();
        println!("{}", oftds[0]);
    }*/
    let fig = body.select(&fig_selector).next().unwrap();

    for element in fig.select(&src_selector){
            let value = element.value().name();
            //let srcs = element.text().collect::<Vec<_>>();
            //println!("{}", srcs[0]);
            println!("{:?}", element.value().attr("data-src").unwrap()); 
            let imgurl = element.value().attr("data-src").unwrap(); 
            
    }

}