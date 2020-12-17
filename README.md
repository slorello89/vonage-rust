# Vonage-Rust

Welcome to Steve's very clumsy first cut at rust-lang. Contained in here is the beginnings of a Rust library to interact with the [Vonage APIs](https://developer.nexmo.com).

## Configuration

Copy the `example.env` file to `.env` and make sure you fill in your `VONAGE_API_KEY`, `VONAGE_API_SECRET`, and your `FROM_NUMBER` from the values pertinent to your [Vonage API Account](https://dashboard.nexmo.com/)

## Test it out 

To test it out you can run the main file as a cli tool:

```sh
cargo run -- -s sendSms --to_number YOUR_NUMBER --text "Hello From Vonage"
```
