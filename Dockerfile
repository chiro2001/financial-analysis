FROM python:3.6

WORKDIR /app/

ENV PYTHON=python3
ENV FRONTEND_STATIC_PATH="/app/dist/"
ENV MONGO_URI="mongodb://dipiper:1352040930@a.chiro.work"
ENV JRPC_HTTP_PREFIX="http://a.chiro.work:8000/api/v1"
EXPOSE 51411 9090

COPY ./simple-lstm-server/requirements.txt /app/
COPY tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl /app/
ENV http_proxy=http://r.chiro.work:14514
ENV https_proxy=http://r.chiro.work:14514
RUN $PYTHON -m pip install ./tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl
RUN $PYTHON -m pip install -r requirements.txt

COPY financial-frontend/dist/ /app/dist/
COPY server/target/x86_64-unknown-linux-musl/release/server /app/
COPY simple-lstm-server/ /app/simple-lstm-server
COPY docker_start.sh /app/

CMD [ "sh", "docker_start.sh" ]
