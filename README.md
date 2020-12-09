# saffron 🌷

A little private file hosting service.

- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)
- [License](#license)

## Installation

TODO. Probably publish on `crates.io`. And create a Dockerfile.

## Usage

- Run `saffron user create <username>` to add a user.
- Run `saffron start` to start the webserver.

To view the options in the CLI, use the `help` commands.

```
saffron
A little private file hosting service

USAGE:
    saffron <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    start    Start the webserver
    user     Users and token management
```

## Development

Database stuff requires `sqlx-cli`:

```sh
$ cargo install --version=0.2.0 sqlx-cli --no-default-features --features sqlite
```

CSS stuff requires `yarn` to build TailwindCSS. See `package.json` for the
developer commands.

#### TODO

- File upload route using Token header.
- Get error pages working (incl. on static routes...).
- Publish to crates.io.
- Consider streaming files instead of serving them statically.
- Gallery frontend view!
- Image thumbnailing for gallery.
- Create a Dockerfile.

wrt. crates.io, we'll need a way to include the built CSS, but I don't want to
check it into VCS because dev-version CSS would override it. Perhaps a CI to
ensure that CSS is prod css?

## License

```
saffron :: a little private file hosting service

Copyright (C) 2020 azuline

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU Affero General Public License for more
details.

You should have received a copy of the GNU Affero General Public License along
with this program.  If not, see <https://www.gnu.org/licenses/>.
```
