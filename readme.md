# Overview
Jackrabbit is a rust clone of [bunny1](https://github.com/ccheever/bunny1), a
tool that maps keywords in your search queries to urls. For example, you could
search the c++ standard library for the vector class with `std vector`, stackoverflow
with `so <query>`, view your active PRs with `pr` and so on. Jackrabbit operates
similar to browser keyword searches, but allows string templating bookmarks with
any url, instead of only on supported search sites.

## Usage
Jackrabbit provides an [OpenSearch](https://developer.mozilla.org/en-US/docs/Web/OpenSearch)
compatible interface that can be installed by compatible browsers. To use
Jackrabbit start the server and visit the root. You can then install the
the page as an additional search engine and set it to the default.

## Configuration
Jackrabbit uses a single toml file for configuring bookmarks. The config file
is passed to Jackrabbit at start time with the `-c` flag.
```toml
interface = "127.0.0.1:8080"
default = "https://duckduckgo.com/?q={}"

[bookmarks]
g = "https://duckduckgo.com/?q={}"
gh = "https://github.com/"
go = "https://pkg.go.dev/search?q={}"
rust = "https://docs.rs/releases/search?query={}"
std = "https://www.cplusplus.com/search.do?q={}"
```
