# All Win Core (dev image). Build context: repository root.
# NOT VERIFIED until `core/` exists (S1 core slice): APP_PACKAGE must name the binary crate wired in crates/app.
FROM rust:1.99.1-slim-bookworm AS build
ARG APP_PACKAGE=app
WORKDIR /src
COPY core/ core/
COPY contracts/ contracts/
WORKDIR /src/core
RUN cargo build --release --locked -p ${APP_PACKAGE}

FROM debian:bookworm-slim
ARG APP_PACKAGE=app
RUN useradd --system --uid 10001 allwin
COPY --from=build /src/core/target/release/${APP_PACKAGE} /usr/local/bin/allwin-core
USER allwin
ENTRYPOINT ["/usr/local/bin/allwin-core"]
