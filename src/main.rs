use rouille::Response;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate{}

#[derive(Template)]
#[template(path = "opensearch.xml")]
struct PluginTemplate<'a>{
    host: &'a str,
}

fn main() {
    println!("Jackrabbit started");
    rouille::start_server("0.0.0.0:8080", move |request| {
		rouille::router!(request,
			(GET) ["/"] => {
				let index = IndexTemplate{};
                Response::html(index.render().unwrap())
			},

			(GET) ["/opensearch.xml"] => {
                let maybe_host = request.header("Host");
                match maybe_host {
                    None => Response::text("Missing Host").with_status_code(400),
                    Some(host) => {
                        let search = PluginTemplate{
                            host: host,
                        };
                        let body = search.render().unwrap().into_bytes();
                        Response::from_data("text/xml", body)
                    }
                }
			},

            _ => Response::text("Not found").with_status_code(404)
		)
    });
}

