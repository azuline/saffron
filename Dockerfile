FROM rust:1.49-buster as builder

WORKDIR /app
COPY . .

# Install deps.
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update
RUN apt-get install -y yarn

# Compile application.
RUN cd views/ && yarn install && yarn build
RUN cargo build --release

RUN mkdir /appdata
RUN echo 'DATABASE_URI=sqlite:///appdata/db.sqlite3\nUPLOAD_DIRECTORY=/appdata/files' > .env

ENTRYPOINT ["/app/target/release/saffron"]
