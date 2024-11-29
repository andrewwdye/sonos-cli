# sonos-cli

A command line application for [Sonos](https://www.sonos.com/en-us/home), written in Rust.

It uses a generated client from [sonos-api-docs](https://github.com/svrooij/sonos-api-docs) to access Sonos devices on the local network using UPnP. Unfortunately it seems that Sonos removed UPnP support during their S2 update. While discovery workers, all of the SOAP calls fail.

```bash
Usage: sonos-cli [OPTIONS] <COMMAND>

Commands:
  discover  Discover Sonos devices on the network
  speaker   Get the state of a Sonos speaker
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
  -V, --version     Print version
```
