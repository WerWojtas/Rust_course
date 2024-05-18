

fn main(){
    let url = String::from("https://www.google.com");
    let url_cut = check_url(&url);
    if url_cut == "false" {
        return;
    }
    if url_cut.len() == 0 {
        println!("Url only contains protocol, no domain provided");
        return;
    }
    let domain = get_domain(url_cut);
    println!("Domain: {:?}", domain);
}

pub fn check_url(url : &String) -> &str {
    let slice_https = &url[..8];
    let slice_http = &url[..7];
    if slice_https == "https://" {
        return &url[8..];
    }
    if slice_http == "http://" {
        return &url[7..];
    }
    println!("Wrong url format, please use https://...");
    return "false"; 
}

fn get_domain(url: &str) -> &str {
    let mut i = 0;
    loop {
        if i >= url.len() || url.chars().nth(i).unwrap() == '/' {
            break;
        }
        i+=1;
    }
    &url[..i]
}