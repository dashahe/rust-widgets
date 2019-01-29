extern crate reqwest;

use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};
use std::collections::HashMap;
use std::fs;

// get all questions's name

// get question detail for each question

// parse it, and convert html to markdown

// save


fn get_questions() -> String {
    let QUESTION_LIST_URL = "https://leetcode-cn.com/api/problems/all/";

    // let client = reqwest::Client::new();
    
    // let resp = client.get(QUESTION_LIST_URL).send()?;
    // println!("{:?}", resp);
    "ok".to_string()
}

fn main() -> Result<(), Box<std::error::Error>> {
    // get_questions();

    let resp = reqwest::get("https://leetcode-cn.com/api/problems/all/")?.text()?;
    println!("{:#?}", resp);
    Ok(())

}