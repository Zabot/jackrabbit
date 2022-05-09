use std::collections::HashMap;
use std::fs;

use askama::Template;
use rouille::Request;
use rouille::Response;
use serde_derive::Deserialize;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "opensearch.xml")]
struct PluginTemplate<'a> {
    host: &'a str,
}

#[derive(Deserialize)]
struct Config {
    bookmarks: HashMap<String, String>,
}

fn handle_index(_request: &Request) -> Response {
    let index = IndexTemplate {};
    Response::html(index.render().unwrap())
}

fn handle_plugin(request: &Request) -> Response {
    match request.header("Host") {
        None => Response::text("Missing Host").with_status_code(400),
        Some(host) => {
            let search = PluginTemplate { host: host };
            let body = search.render().unwrap().into_bytes();
            Response::from_data("text/xml", body)
        }
    }
}

fn handle_search(bookmarks: &HashMap<String, String>, request: &Request) -> Response {
    match request.get_param("q") {
        None => Response::text("Missing query").with_status_code(400),
        Some(query) => {
            for (key, value) in bookmarks {
                // If key is perfect equality, do a permanent redirect
                if key.eq(&query) {
                    return Response::redirect_301(value.clone());
                }

                // Append a space and check for prefix
                let prefix = format!("{} ", key);
                match query.strip_prefix(&prefix) {
                    Some(m) => {
                        let url = value.replace("{}", m.trim());
                        return Response::redirect_302(url);
                    }
                    None => {}
                }
            }
            Response::text(query).with_status_code(200)
        }
    }
}

fn main() {
    println!("Loading bookmarks");
    let config_contents = fs::read_to_string("jackrabbit.toml").expect("Failed to read config");
    let config: Config = toml::from_str(&config_contents).unwrap();

    rouille::start_server("0.0.0.0:8080", move |request| {
        rouille::router!(request,
            (GET) ["/"] => handle_index(&request),
            (GET) ["/opensearch.xml"] => handle_plugin(&request),
            (GET) ["/search"] => handle_search(&config.bookmarks, &request),
            _ => Response::text("Not found").with_status_code(404)
        )
    });
}
