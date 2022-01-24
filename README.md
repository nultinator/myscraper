# myscraper
Web Scraper
A web scraper written in Rust. You must have Rust and Cargo installed to use it.
In line 17 of "main.rs" a vector named "domain_names" is declared. In order to change the sites you decide to scrape, change the vector to whichever sites you want. Remember to change the size of the vector as well. For example:

let domain_names = [String; 4]

If you wanted to process 3 sites instead, change this variable to:

let domain_names = [String; 3]

Afterward, simply replace the sites in the vector with those that you would like to process.

Once all of that is out of the way, to run once just cd into the folder and enter:

cargo run

If you want a standalone executable that can be reused (I don't recommend this until user input has been added to control the vector I discussed earlier), enter:

cargo build

or

cargo build --release (for an optimized build, as I said earlier though, I don't recommend that at this stage of the project).

Good luck, enjoy, and PRs welcome :)
