# Simple Rust KuantoKusta Web Scraper

The primary goals of this project are to test the **Rust** programming language and develop a basic web scraper. It goes through [KuantoKusta](https://www.kuantokusta.pt/) website and retrieves the cheapest product for the given search.

**This project is not longer being maintained**


## Requirements

- Rust programming language installed on your system
- Cargo package manager (usually installed with Rust)

## Usage

```
cargo run <search_terms>
```


Replace `<search_terms>` with the terms you want to search for on the website. For example:

## Notes

- Make sure to replace `"Your User Agent"` in the code with an appropriate User-Agent header string.
- This application currently scrapes URLs from the KuantoKusta website.
- Depending on the website's structure and restrictions, the scraping behavior may need to be adjusted.
