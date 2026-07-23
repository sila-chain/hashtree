# ARG NODE_VERSION

FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64

COPY . /usr/src/sila-hashtree

WORKDIR /usr/src/sila-hashtree/hashtree/src

RUN make

WORKDIR /usr/src/sila-hashtree

RUN yarn config set supportedArchitectures.cpu "arm64" &&\
    yarn config set supportedArchitectures.libc "glibc" &&\
    yarn install

RUN yarn build --target aarch64-unknown-linux-gnu

RUN yarn test