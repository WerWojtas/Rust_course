mod get_domain;
use std::collections::HashMap;

fn main(){
    let url = String::from("https://www.example.com");
    let mut url_cut = get_domain::check_url(&url);
    if url_cut == "false" {
        return;
    }
    match extract_from_url(url_cut, '/'){
        Some((domain, rest)) => {
            println!("Domain: {:?}", domain);
            url_cut = rest;
        }
        None => {
            println!("Domain not found");
            return;
        }
    }
    match extract_from_url(url_cut, '?'){
        Some((request_path, rest)) => {
            println!("Request path: {:?}", request_path);
            url_cut = rest;
        }
        None => {
            println!("Path not found");
            return;
        }
    }
    match extract_parameters(url_cut){
        Some(parameters) => {
            println!("Parameters: {:?}", parameters);
        }
        None => {
            println!("Parameters not found");
            return;
        }
    }
}

fn extract_from_url(url: &str, sign: char) -> Option<(&str,&str)> {
    if url.len() == 0 {
        return None;
    }
    let mut i = 0;
    loop {
        if i >= url.len() || url.chars().nth(i).unwrap() == sign {
            break;
        }
        i+=1;
    }
    Some((&url[..i], &url[i..]))
}

fn extract_parameters(url_cut: &str) -> Option<HashMap<String,String>> {
    if url_cut.len() == 0 {
        return None;
    }
    let url = &url_cut[1..];
    let mut i = 0;
    let mut parameters: HashMap<String, String> = HashMap::new();
    let mut key = String::new();
    let mut value = String::new();
    let mut reference = &mut key;
    while i < url.len() {
        if url.chars().nth(i).unwrap() == '&'{
            parameters.insert(key.clone(), value.clone());
            key.clear();
            value.clear();
            reference = &mut key;
        }
        else if url.chars().nth(i).unwrap() == '='{
            reference = &mut value;
        }
        else{
            reference.push(url.chars().nth(i).unwrap());
        }
        i+=1;
    }
    parameters.insert(key, value);
    Some(parameters)
}

