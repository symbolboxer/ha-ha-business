#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use scraper::{Html, Selector};
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize)]
struct CompanyData {
    name: String,
    description: String,
    logo_url: String,
}

#[derive(Debug, Serialize)]
struct PitchData {
    name: String,
    hashtags: Vec<String>,
}

fn main() {
    let command = std::env::args().nth(1).expect("no command given");

    if command == "companies" {
        get_companies();
    } else if command == "pitches" {
        get_pitches();
    }
}

fn get_companies() {
    let path = Path::new("companies.json");
    let display = path.display();
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    println!("Fetching pitch page to get companies");
    let pitch_res = reqwest::blocking::get("http://pitchdeck.business/pitch_cards/mindfulness");
    let mut companies: Option<Vec<CompanyData>> = None;
    match pitch_res {
        Ok(v) => companies = Some(companies_from_pitch_request(v)),
        Err(e) => println!("error fetching pitch page: {:?}", e),
    }

    if let Some(cs) = companies {
        match serde_json::to_writer(file, &cs) {
            Err(e) => panic!("Error serializing companies: {:?}", e),
            Ok(_) => println!("Successfully exported companies."),
        }
    }
}

fn companies_from_pitch_request(pitch_resp: reqwest::blocking::Response) -> Vec<CompanyData> {
    let html = pitch_resp.text().unwrap();
    let doc = Html::parse_document(&html);
    let selector = Selector::parse("td > a").unwrap();

    let companies: Vec<CompanyData> = doc
        .select(&selector)
        .map(|element| get_company_data(element.value().attr("href").unwrap().to_string()))
        .collect();

    println!("Retrieved {} companies", companies.len());
    companies
}

fn get_company_data(path: String) -> CompanyData {
    // Hi, Internet. If you're looking at this, know that in production code, I would
    // use a shared client instead of calling the get convenience method. I read the docs.
    let res = reqwest::blocking::get(&["http://pitchdeck.business", &path].concat());
    let html = res.unwrap().text().unwrap();
    let doc = Html::parse_document(&html);

    let name_selector = Selector::parse(".company_name").unwrap();
    let name = doc
        .select(&name_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let descr_selector = Selector::parse(".investor_notes").unwrap();
    let description = doc
        .select(&descr_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let logo_selector = Selector::parse(".company_logo > img").unwrap();
    let logo_url = doc
        .select(&logo_selector)
        .next()
        .unwrap()
        .value()
        .attr("src")
        .unwrap()
        .to_string();

    let cd = CompanyData {
        name,
        description,
        logo_url,
    };
    println!("{:?}", cd);
    cd
}

fn get_pitches() {
    let path = Path::new("pitches.json");
    let display = path.display();
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    println!("Fetching company page to get pitches");
    let pitch_res = reqwest::blocking::get("http://pitchdeck.business/company_cards/netflix");
    let mut pitches: Option<Vec<PitchData>> = None;
    match pitch_res {
        Ok(v) => pitches = Some(pitches_from_company_request(v)),
        Err(e) => println!("error fetching pitch page: {:?}", e),
    }

    if let Some(ps) = pitches {
        match serde_json::to_writer(file, &ps) {
            Err(e) => panic!("Error serializing pitches: {:?}", e),
            Ok(_) => println!("Successfully exported pitches."),
        }
    }
}

fn pitches_from_company_request(company_resp: reqwest::blocking::Response) -> Vec<PitchData> {
    let html = company_resp.text().unwrap();
    let doc = Html::parse_document(&html);
    let selector = Selector::parse("td > a").unwrap();

    let pitches: Vec<PitchData> = doc
        .select(&selector)
        .map(|element| get_pitch_data(element.value().attr("href").unwrap().to_string()))
        .collect();

    println!("Retrieved {} pitches", pitches.len());
    pitches
}

fn get_pitch_data(path: String) -> PitchData {
    // Hi, Internet. If you're looking at this, know that in production code, I would
    // use a shared client instead of calling the get convenience method. I read the docs.
    let res = reqwest::blocking::get(&["http://pitchdeck.business", &path].concat());
    let html = res.unwrap().text().unwrap();
    let doc = Html::parse_document(&html);

    let name_selector = Selector::parse(".pitch_name > a").unwrap();
    let name = doc
        .select(&name_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let hashtags_selector = Selector::parse("li > a").unwrap();
    let hashtags: Vec<String> = doc
        .select(&hashtags_selector)
        .map(|element| element.text().collect::<String>().trim().to_string())
        .collect();

    let cd = PitchData { name, hashtags };
    println!("{:?}", cd);
    cd
}
