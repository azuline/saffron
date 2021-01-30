# saffron 🌷

A little private file hosting service.

- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
  - [Uploading Files](#uploading-files)
- [Development](#development)
- [TODO](#todo)
- [License](#license)

## Installation

As of now, cloning the repository and running `cargo build` is the only method
of installation.

The frontend CSS must also be built, which requires `yarn`.

```sh
$ cd saffron
$ cd views
$ yarn install
$ yarn build
$ cd ..
$ cargo build --release
$ ./target/release/saffron user create <username>
$ ./target/release/saffron start
```

### Docker

Alternatively, one can use docker with the provided `Dockerfile`.

```sh
$ cd saffron
$ docker build -t saffron .
$ docker run -e HOST_URL="https://image.host/" saffron user create <username>
$ docker run -e HOST_URL="https://image.host/" -dp 8000:8000 saffron start -h 0.0.0.0
```

## Configuration

This application is configured via environment variables, documented below.

If desired, the environment variables can be set in an `.env` file. An example
`.env` file is available in the repository as `.env.sample`.

#### DATABASE_URI

The URI to the SQLite database. For a relative path, use
`sqlite://relative/path/to/db.sqlite`. For an absolute path, use
`sqlite:///var/lib/absolute/path/to/db.sqlite`.

#### UPLOAD_DIRECTORY

The directory to store uploaded files in.

#### HOST_URL

The URL of the server. Used to create the file URL in JSON responses to upload
requests.

### Uploading Files

At the moment, uploading from the website is not supported. For uploading, a
JSON API endpoint is available at `/upload`, accepting uploads of the
`multipart/form-data` content type.

Uploads must be authenticated with the `Authorization` header. The value must
be in the `Token <token>` format, where `<token>` is a user's hex-encoded
token.

An example cURL request:

```sh
$ curl -v -X POST -H "Content-Type:multipart/form-data" -H"Authorization:Token <token>" -F "upload=@<filepath>" https://image.host/upload
```

## Development

Developing database stuff requires `sqlx-cli`:

```sh
$ cargo install --version=0.2.0 sqlx-cli --no-default-features --features sqlite
```

Developing the views requires `yarn`, in order to build TailwindCSS. Running
`yarn install` inside `views/` will set up the environment. See
`views/package.json` for some helper commands.

## TODO

- File deletion.
  - CSRF protection.
- Clean up... this really is a terrible mess with Actix.

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
