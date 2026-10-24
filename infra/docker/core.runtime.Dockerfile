# Core image from a binary compiled on the HOST (infra/scripts/build-core-image.sh): the compile runs through allwin-heavy
# (semaphore, CPU quota, shared compile cache) instead of inside Docker. ubuntu:24.04 has the same glibc as the host.
# Build context: a directory that contains the `allwin-core` binary.
FROM ubuntu:24.04
RUN useradd --system --uid 10001 allwin
COPY allwin-core /usr/local/bin/allwin-core
USER allwin
ENTRYPOINT ["/usr/local/bin/allwin-core"]
