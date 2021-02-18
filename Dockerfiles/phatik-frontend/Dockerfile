FROM docker.io/library/node:13 AS build

ARG NPM_REGISTRY=https://registry.npmjs.org

WORKDIR /app
# Cache all dependencies as layer(s)
COPY phatik-frontend/package.json phatik-frontend/package-lock.json /app/
RUN npm install --registry ${NPM_REGISTRY}

# Do build
COPY phatik-frontend/*.js phatik-frontend/*.json phatik-frontend/index.html /app/
COPY phatik-frontend/src /app/src
COPY phatik-frontend/public /app/public
RUN npm run build

FROM docker.io/library/nginx
COPY Dockerfiles/phatik-frontend/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html
