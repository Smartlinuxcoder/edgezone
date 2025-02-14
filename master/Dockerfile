FROM node:20-slim as builder


ENV DATABASE_URL=file:data/local.db

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

FROM node:20-slim

WORKDIR /app
COPY --from=builder /app/build ./build
COPY --from=builder /app/package*.json ./
COPY --from=builder /app/drizzle.config.* ./
COPY --from=builder /app/src/lib/server/db/schema.js ./src/lib/server/db/schema.js
RUN npm install --production && npm install drizzle-kit

EXPOSE 3000

RUN mkdir -p /app/data

ENV DATABASE_URL=file:data/local.db


RUN echo '#!/bin/sh\n\
npm run db:push -- --force\n\
exec node ./build\n\
' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

CMD ["/app/entrypoint.sh"]