FROM nginx:alpine

COPY ./webui/dist/ /usr/share/nginx/html/
