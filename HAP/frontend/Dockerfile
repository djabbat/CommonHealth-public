FROM elixir:1.15-alpine AS build

RUN apk add --no-cache build-base npm git

WORKDIR /app

RUN mix local.hex --force && \
    mix local.rebar --force

ENV MIX_ENV=prod

COPY mix.exs mix.lock ./
RUN mix deps.get --only $MIX_ENV
RUN mkdir config
COPY config/config.exs config/${MIX_ENV}.exs config/

RUN mix deps.compile

COPY assets assets
RUN mix assets.setup
RUN mix assets.deploy

COPY lib lib
COPY priv priv

RUN mix compile
RUN mix release

FROM alpine:3.19

RUN apk add --no-cache openssl ncurses libstdc++

WORKDIR /app

RUN chown nobody:nobody /app

USER nobody:nobody

COPY --from=build --chown=nobody:nobody /app/_build/prod/rel/hap_frontend ./

ENV HOME=/app
ENV MIX_ENV=prod
ENV PORT=4010

EXPOSE 4010

CMD ["bin/hap_frontend", "start"]