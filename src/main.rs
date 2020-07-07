use reqwest;
use scraper::{Selector, Html};

fn main() {
    println!("Fetching pitch page to get companies");
    let res = reqwest::blocking::get("http://pitchdeck.business/pitch_cards/mindfulness");
    match res {
        Ok(v) => get_companies(v),
        Err(e) => println!("error fetching pitch page: {:?}",e)
    }
}

fn get_companies(pitch_resp: reqwest::blocking::Response) {
    let html = pitch_resp.text().unwrap();
    let doc = Html::parse_document(&html);
    let selector = Selector::parse("td > a").unwrap();

    let company_names = doc.select(&selector).map(|element| {
        get_company_name(element.value().attr("href").unwrap().to_string())
    });

    for name in company_names {
        println!("{:?}", name);
    }
}

fn get_company_name(path: String) -> String {
    // Hi, Internet. If you're looking at this, know that in production code, I would
    // use a shared client instead of calling the get convenience method. I read the docs
    let res = reqwest::blocking::get(&["http://pitchdeck.business", &path].concat());
    let html = res.unwrap().text().unwrap();
    let doc = Html::parse_document(&html);
    let selector = Selector::parse(".company_name").unwrap();

    return doc.select(&selector).next().unwrap().text().collect::<String>().trim().to_string();
}
