# myscraper
A web scraper written in Rust. You must have Rust and Cargo installed to use it.
<h2>Setup</h2>

```
git clone https://github.com/nultinator/myscraper
cd myscraper
cargo build --release
cp target/release/nultiscraper nultiscraper
rm -rf target
```

<h2>Use</h2>

```
./nultiscraper
```

<h4>Output</h4>

```
Enter a domain name:
y.cash
You entered y.cash

Start Time: 2022-10-17_22.32.42
2022-10-17_22.32.43
Completion Time: 2022-10-17_22.32.43
Time Elapsed: PT0.276658686S
```

After running, a CSV file will be generated with the name of the website (example y.cash.csv)
