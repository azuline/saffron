# fragrance

A little private file hosting service.

Although functional and deployed, this project primarily exists as a playground
for Rust and a bit of web design. Maintenance and stability are not guaranteed.

## Installation

I'll see if I can publish this to `crates.io` later...

## Usage

- Run `fragrance user create <username>` to add a user.
- Run `fragrance start` to start the webserver.

To view the options in the CLI, use the `help` commands.

```
fragrance
A little private file hosting service

USAGE:
    fragrance <SUBCOMMAND>

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
- Consider streaming files instead of serving them statically.
- Gallery frontend view!
- Image thumbnailing for gallery.

## License

```
fragrance :: a little private file hosting service

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
