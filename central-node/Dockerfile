FROM fedora:latest

RUN groupadd -r user && useradd --no-log-init -r -g user user

COPY target/release/central-node /central-node

USER user
EXPOSE 8080
CMD ["/central-node"]
