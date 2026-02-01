# build environment
FROM oven/bun:1 AS builder
WORKDIR /usr/src/app
ENV PATH /usr/src/app/node_modules/.bin:$PATH
COPY package.json /usr/src/app/
RUN bun install
COPY . /usr/src/app
RUN bun run build
EXPOSE 4173
CMD ["bun", "run", "preview"]