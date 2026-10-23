# All Win Core (dev image). Build context: repository root.
# Package `app` builds the binary `allwin-core`.
FROM rust:1.99.1-slim-bookworm AS build
ARG APP_PACKAGE=app
ENV SQLX_OFFLINE=true
WORKDIR /src
COPY core/ core/
COPY contracts/ contracts/
WORKDIR /src/core
RUN cargo build --release --locked -p ${APP_PACKAGE}

FROM debian:bookworm-slim
RUN useradd --system --uid 10001 allwin
COPY --from=build /src/core/target/release/allwin-core /usr/local/bin/allwin-core
USER allwin
ENTRYPOINT ["/usr/local/bin/allwin-core"]
