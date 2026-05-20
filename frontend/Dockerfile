FROM node:24-alpine3.23 as builder

WORKDIR /app

COPY package*.json .

RUN npm install

COPY . .

CMD ["npm", "run", "build"]

FROM node:24-alpine3.23 as run

WORKDIR /app

COPY --from=builder /app/package*.json /app

RUN npm ci --omit dev

COPY --from=builder /app/build /app

EXPOSE 3000

CMD [ "node", "build" ]
