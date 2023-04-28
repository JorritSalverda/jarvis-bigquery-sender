FROM scratch AS runtime

USER 1000

COPY --from=rust:1.69 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY target/x86_64-unknown-linux-musl/release/jarvis-bigquery-sender .

ENTRYPOINT ["./jarvis-bigquery-sender"]