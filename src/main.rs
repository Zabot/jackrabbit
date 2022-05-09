use askama::Template;
use rouille::Request;
use rouille::Response;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate{}

#[derive(Template)]
#[template(path = "opensearch.xml")]
struct PluginTemplate<'a>{
    host: &'a str,
}

fn handle_index(request: &Request) -> Response {
    let index = IndexTemplate{};
    Response::html(index.render().unwrap())
}

fn handle_plugin(request: &Request) -> Response {
    match request.header("Host") {
        None => Response::text("Missing Host").with_status_code(400),
        Some(host) => {
            let search = PluginTemplate{
                host: host,
            };
            let body = search.render().unwrap().into_bytes();
            Response::from_data("text/xml", body)
        }
    }
}

fn main() {
    println!("Jackrabbit started");
    rouille::start_server("0.0.0.0:8080", move |request| {
		rouille::router!(request,
			(GET) ["/"] => handle_index(&request),
			(GET) ["/opensearch.xml"] => handle_plugin(&request),
            _ => Response::text("Not found").with_status_code(404)
		)
    });
}

