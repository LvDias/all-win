# Web image from a Next.js standalone build made on the HOST (infra/scripts/build-web-image.sh).
# Build context: a directory with `standalone/` = the contents of .next/standalone, with .next/static and public/ already placed next to
# server.js by the script.
FROM node:24-alpine
ENV NODE_ENV=production HOSTNAME=0.0.0.0 PORT=3000
WORKDIR /app
COPY standalone/ ./
USER node
EXPOSE 3000
CMD ["node", "server.js"]
