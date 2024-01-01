use html2md;
use reqwest;
use reqwest::header;
use std::fs;

#[allow(dead_code)]
pub fn hhello() {
    println!("Hello");
}
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

// #[test]
fn task_fetch_question() {
    let day = "-1";
    let url = format!("https://console.shuttle.rs/cch/challenge/{}", day);
    let output = format!("assets/days/rust{}.md", day);
    let output1 = format!("assets/days/rust{}.html", day);
    assert_eq!(day, "-1");

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output1, body.as_bytes()).unwrap();

    fs::write(output, md.as_bytes()).unwrap();
    // println!("Converted markdown hash been saved in {}", output);
}

// /*
// curl 'https://console.shuttle.rs/api/cch/description/-1' \
//   -H 'authority: console.shuttle.rs' \
//   -H 'accept: */*' \
//   -H 'accept-language: zh-CN,zh;q=0.9,en;q=0.8' \
//   -H 'cache-control: no-cache' \
//   -H 'cookie: xxxxxx
//   -H 'dnt: 1' \
//   -H 'pragma: no-cache' \
//   -H 'referer: https://console.shuttle.rs/cch/challenge/-1' \
//   -H 'sec-ch-ua: "Google Chrome";v="119", "Chromium";v="119", "Not?A_Brand";v="24"' \
//   -H 'sec-ch-ua-mobile: ?0' \
//   -H 'sec-ch-ua-platform: "macOS"' \
//   -H 'sec-fetch-dest: empty' \
//   -H 'sec-fetch-mode: cors' \
//   -H 'sec-fetch-site: same-origin' \
//   -H 'sec-gpc: 1' \
//   -H 'user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36' \
//   --compressed
// */
#[test]
fn task_fetch_question_description() {
    let day = "5";
    let url = format!("https://console.shuttle.rs/api/cch/description/{}", day);
    let output = format!("assets/days/rust{}.md", day);
    assert_eq!("-1", "-1");
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", header::HeaderValue::from_static("console.shuttle.rs"));
    headers.insert("cookie", header::HeaderValue::from_static("")); 
   
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    println!("Fetching url: {}", url);

    let body = client.get(url).send().unwrap().text().unwrap();

    fs::write(output, body.as_bytes()).unwrap();
}

#[test]
fn split_test() {
    let key = "elf";
    let spls = "I have some elf ,and these elfs are very cute".split(key);
    let cnt = spls.count();
    assert_eq!(cnt, 3);
    // spls.iter();
}
