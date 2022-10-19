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

What would you like to scrape?
p
Start Time: 2022-10-19_11.33.35
2022-10-19_11.33.36
95% of newly issued coins go directly to users via the permissionless, free-market mining process, a process open to anyone in the world with a GPU and an internet connection.
No need for trips to the bank or ATM. Your Ycash is always at your fingertips, 24 hours a day, 365 days a year. Send Ycash to anyone on the planet within minutes.
Borderless, inflation-free cash is empowering and fun! You will love earning, saving, and spending your Ycash.
Completion Time: 2022-10-19_11.33.36
Time Elapsed: PT0.482481532Ster a domain name:

```

After running, a CSV file will be generated with the name of the website (example y.cash-p.csv)
