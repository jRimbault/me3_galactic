```
galactic 0.7.0
Deploy missions and collect the rewards for galactic readiness in Mass Effect 3. You have to get the value of your
identifier cookie on the website, and it expires in a few hours. But running this program once or twice a day should be
enough

USAGE:
    galactic --cookie <cookie> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cookie <cookie>    identifier cookie for n7hq.masseffect.com [env: ME3N7HQSID]

SUBCOMMANDS:
    automatic    refresh every missions untils the cookie expires
    help         Prints this message or the help of the given subcommand(s)
    refresh      refresh every missions and quits, this is the default
```
