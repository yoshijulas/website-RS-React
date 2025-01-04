FROM node:18 AS builder

WORKDIR /app

COPY ./.docker.env .env

COPY ./package*.json ./

RUN npm install



COPY . /app

RUN npm run build

FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 3000


COPY ./.docker.env .env

# Start nginx to serve the static files
CMD ["nginx", "-g", "daemon off;"]

