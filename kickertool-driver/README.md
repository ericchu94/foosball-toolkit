# kickertool-driver

kickertool driver is a tool that extracts data from [kickertool](https://app.kickertool.de/) to allow usage elsewhere.

## building

Set up the rust with the nightly channel, and build via cargo.

```
cargo build
```

## usage

Close all existing Chrome windows.

Run via cargo.

```
cargo run
```

Chrome with remote debugging should run (with the default user data directory).
While the first tab is navigated to the listview of a tournament, the data will be fetched, and text files will be updated.

### OBS integration

#### text source

Add text sources that read from file.

## notes

### other websites

Integration with other websites are desired. Each website should be independent from another, relying on a common library.

### other sources

DOM structure may not be the best choice. Local storage and cookies may also provide value, or even be better structures.

Kickertool data is stored in localstorage, which would be more reliable than scraping HTML.

### other sinks

Perhaps we want to write the data to a database, post it to an web api, or any combination.

### other browsers

Firefox is nice.

### cross platform

Duh.

### common library

There are other usages for scraping SPAs outside of foosball, this library can be utilized.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
