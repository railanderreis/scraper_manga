use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use reqwest::Url;
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {     
    
    for n in 0..2{
        let s = String::from(format!("https://mangaonline.biz/capitulo/solo-leveling-capitulo-{}", n));


        let base_url = Url::parse(&s)?;
    
        let body = get(base_url.as_str())?.text()?;
        let document = Html::parse_document(&body);
        let img_selector = Selector::parse("p > img").unwrap();
        let dir_name = String::from(format!("./images/cap_{}", n));
        create_dir_all(&dir_name)?;

        println!("Getting images...");

        for img in document.select(&img_selector) {
            if let Some(img_src) = img.value().attr("src") {
                let img_url = base_url.join(img_src)?;
                let mut img_response = get(img_url.as_str())?;
                let mut img_bytes = Vec::new();
                img_response.copy_to(&mut img_bytes)?;

                let img_name = img_src.split('/').last().unwrap_or("image.jpg");
                let img_path = format!("{}/{}", dir_name, img_name);
                let mut img_file = File::create(&img_path)?;
                img_file.write_all(&img_bytes)?;
                println!("Image saved: {}", img_path);
            }
        }
        println!("pause");
        thread::sleep(Duration::from_millis(10000));
    }
    println!("Done!");
    

    Ok(())
}