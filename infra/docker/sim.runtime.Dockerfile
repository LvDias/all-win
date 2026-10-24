# adapter-sim image from a host-compiled binary (infra/scripts/build-core-image.sh with ALLWIN_BUILD_PACKAGE=adapter-sim). Dev and E2E only.
FROM ubuntu:24.04
RUN useradd --system --uid 10002 allwinsim
COPY adapter-sim /usr/local/bin/adapter-sim
USER allwinsim
ENTRYPOINT ["/usr/local/bin/adapter-sim"]
