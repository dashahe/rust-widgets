extern crate reqwest;
extern crate serde_json;
extern crate serde;
extern crate html2md;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use std::io::Write;
use std::fs::File;
use html2md::parse_html;

#[derive(Serialize, Deserialize)]
struct ProblemStat {
    question_id: u32,
    question__article__live: Option<bool>,
    question__article__slug: Option<String>,
    question__title: Option<String>,
    question__title_slug: Option<String>,
    question__hide: Option<bool>,
    total_acs: u32,
    total_submitted: u32,
    frontend_question_id: u32,
    is_new_question: Option<bool>
}

#[derive(Serialize, Deserialize)]
struct Problem {
    stat: ProblemStat,
    status: Option<String>,
    difficulty: HashMap<String, i32>,
    paid_only: Option<bool>,
    is_favor: Option<bool>,
    frequency: u32,
    progress: u32
}

#[derive(Serialize, Deserialize)]
struct AllProblems {
    user_name: Option<String>,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_medium: u32,
    ac_hard: u32,
    stat_status_pairs: Vec<Problem>,
    frequency_high: u32,
    frequency_mid: u32,
    category_slug: Option<String>
}

#[derive(Serialize, Deserialize)]
struct Question {
    data: QuestionData
}

#[derive(Serialize, Deserialize)]
struct QuestionData {
    question: QuestionDetail
}

#[derive(Serialize, Deserialize)]
struct QuestionDetail {
    questionFrontendId: String,
    title: String,
    titleSlug: String,
    content: String,
    translatedTitle: String,
    translatedContent: String,
    difficulty: String,
    sampleTestCase: String,
    metaData: String
}

fn get_question_detail(slug: &String) -> Result<String, Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let mut header = HeaderMap::new();
    header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let query_str = r###"
    {
        "operationName": "questionData",
        "query": "query questionData($titleSlug: String!) {  question(titleSlug: $titleSlug) {      questionFrontendId   title    titleSlug    content    translatedTitle    translatedContent   difficulty sampleTestCase    metaData      }}",
        "variables": {
            "titleSlug": ""###.to_string() + slug + r###""
        }
    }
    "###;
    let resp = client.get("https://leetcode-cn.com/graphql")
        .headers(header)
        .body(query_str)
        .send()?
        .text()?;
    
    Ok(resp)
}

fn format_question(html: String) -> String {
    parse_html(&html)
}

fn write_question(detail: &String) -> Result<(), Box<std::error::Error>> {
    let parsed_question: Question = serde_json::from_str(&detail).unwrap();
    let question_detail = parsed_question.data.question;
    let output_name = format!("questions/{}_{}.md", 
        question_detail.questionFrontendId, 
        question_detail.translatedTitle);
    let mut output_file = File::create(output_name)?;
    let output_content = format_question(question_detail.translatedContent);

    output_file.write_all(format!("# {}.{}\n\n", question_detail.questionFrontendId, question_detail.translatedTitle).as_bytes())?;
    output_file.write_all(String::from("## 问题\n\n").as_bytes())?;
    output_file.write_all(output_content.as_bytes())?;
    output_file.write_all(String::from("\n\n## 解答\n\n").as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let resp = reqwest::get("https://leetcode-cn.com/api/problems/all/")?.text()?;
    let parsed: AllProblems = serde_json::from_str(&resp).unwrap();
   
    for x in parsed.stat_status_pairs {
        let slug = x.stat.question__title_slug.unwrap();
        let question_detail = get_question_detail(&slug).unwrap();
        println!("{} ok", slug);
        write_question(&question_detail)?;
    }  

    Ok(())
}