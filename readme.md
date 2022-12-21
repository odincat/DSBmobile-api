# DSBmobile & Untis "scraper"
This project is a Rust implementation of a scraper for the DSBmobile (staticly served Untis plans, to be specific), which is an app used by some schools in Germany to distribute their substitution plans.

## Features
- configuration based, supporting multiple schools / plans
- Scrapes substitution plan data from the DSBmobile
- Parses the data into a structured format for easy consumption
- Exposes the parsed data through a simple API
- (optional) protobuf support

## Setup / Running
Make sure you have [Rust](https://www.rust-lang.org/) installed. You should also have some credentials laying around, the public demo access won't work because it's not serving Untis plans.

1. Copy the example config and rename it to `config.toml`. (`cp ./example.config.toml ./config.toml`).
2. Fill in the config, be aware that some options simply won't do anything (e.g. logging, keys), because they haven't been implemented yet.
3. Run `cargo r --bin main`. 

If you see something along the lines of `Launching server on ...` and `available titles for XY: ["x"], congrats you are up and running and can continue to the next section.

BTW: If you want to use protobuf, you probably have to change the schema and the mapping in `handlers.rs` order to make it work for you.
No need to do that if you use JSON, tho.

## Endpoints
I have done something like this before (but in Typescript and full of antipatterns, incompatibility with multiple pages etc.) and made the mistake of splitting all the different pieces of information into different endpoints.
This was of course not convenient and if you wanted all the data you had to make multiple requests.

Now it's much simpler, all put into one URI:
```
GET /obtain/<school>
(school is the url_identifier specified in the config)

Optional parameters:
    - select: Give a comma seperated list with all the classes that you want to include (e.g. ?select=7b,7C)
    - remove: Also takes a comma seperated list, does the exact opposite (e.g. ?remove=13,5a)
    - proto: Specify true in order to recieve a binary response. (e.g. ?proto=true)

json response:
{
    "plan_url": string,
    "last_updated": string,
    "current": {
        "date": string,
        "weekday": string,
        "week_type": string,
        "news": string array,
        "content": object array,
        "affected_classes": string array
    },
    "upcoming": same as current
}

example requests:
- /obtain/schoolxy?select=8&remove=8d
- /obtain/schoolz?remove=13&proto=true
```

## TODO
- [ ] more unit / integration tests to ensure a stable experience
- [ ] generate json from protobuf structs -> no more mapping needed
- [ ] url parameter for requesting only a specific version of a plan (reduce payload)

## Credits / Mentions
- [TheNoim - DSBAPI](https://github.com/TheNoim/DSBAPI): original inspiration & API reference
- [fynngodau](https://notabug.org/fynngodau): documenting the [new API](https://notabug.org/fynngodau/DSBDirect/wiki/mobileapi.dsbcontrol.de)
