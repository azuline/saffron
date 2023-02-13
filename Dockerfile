FROM rust:1.49-buster as builder

WORKDIR /app

# Install deps.
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - \
    && apt-get install -y nodejs \
    && curl -f https://get.pnpm.io/v6.16.js | node - add --global pnpm

COPY . .

# Compile application.
RUN cd views/ && pnpm install && pnpm build
RUN cargo build --release

RUN mkdir /appdata
RUN echo 'DATABASE_URI=sqlite:///appdata/db.sqlite3\n\
UPLOAD_DIRECTORY=/appdata/files' > .env

ENTRYPOINT ["/app/target/release/saffron"]
