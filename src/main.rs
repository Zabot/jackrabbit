use std::collections::HashMap;
use std::fs;

use askama::Template;
use tracing::info;
use tracing_subscriber;
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
    interface: String,
    default: String,
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

fn handle_search(bookmarks: &HashMap<String, String>, default: &str, request: &Request) -> Response {
    match request.get_param("q") {
        None => Response::text("Missing query").with_status_code(400),
        Some(query) => {
            for (key, target) in bookmarks {
                // If key is perfect equality, do a permanent redirect
                if key.eq(&query) {
                    info!(key = key.as_str(), target = target.as_str(), "permanent redirect");
                    return Response::redirect_301(target.clone());
                }

                // Append a space and check for prefix
                let prefix = format!("{} ", key);
                match query.strip_prefix(&prefix) {
                    Some(m) => {
                        let url = target.replace("{}", m.trim());
                        info!(key = key.as_str(), target = url.as_str(), "temporary redirect");
                        return Response::redirect_302(url);
                    }
                    None => {}
                }
            }

            // Fall back to default
            let url = default.replace("{}", query.trim());
            info!(target = url.as_str(), "default redirect");
            return Response::redirect_302(url);
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let config_path = "jackrabbit.toml";
    info!(config = config_path, "loading config");
    let config_contents = fs::read_to_string(config_path).expect("Failed to read config");
    let config: Config = toml::from_str(&config_contents).unwrap();

    for (key, value) in &config.bookmarks {
        info!(key = key.as_str(), target = value.as_str(), "loaded bookmark");
    }

    info!(interface = config.interface.as_str(), "jackrabbit running");
    rouille::start_server(config.interface, move |request| {
        rouille::router!(request,
            (GET) ["/"] => handle_index(&request),
            (GET) ["/opensearch.xml"] => handle_plugin(&request),
            (GET) ["/search"] => handle_search(&config.bookmarks, &config.default, &request),
            _ => Response::text("Not found").with_status_code(404)
        )
    });
}
