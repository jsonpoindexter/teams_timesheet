# Teams Timesheet
Create list of filtered MS Teams messages

## Perequisites
[rust](https://www.rust-lang.org/)

## Bild/Run
```
git clone https://github.com/jsonpoindexter/teams_timesheet.git
cargo run --auth AUTH --name NAME
```

## Uage
```
List and organizes MS Teams messages

USAGE:
    teams_timesheet --auth <AUTH> --name <NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --auth <AUTH>    Teams authentication header value
    -n, --name <NAME>    Filter messages by display name
```
