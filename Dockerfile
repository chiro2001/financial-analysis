FROM mongo:latest

WORKDIR /work/

RUN apt update && apt install -y wget curl

RUN wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
RUN sh Miniconda3-latest-Linux-x86_64.sh -b -p /work/conda/
RUN /work/conda/bin/conda create -n lstm python=3.6
ENV PYTHON=/work/conda/envs/lstm/bin/python

COPY ./simple-lstm-server/requirements.txt /work/

COPY tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl /work/
RUN $PYTHON -m pip install ./tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl

RUN $PYTHON -m pip install -r requirements.txt

COPY mongod.conf /work/
COPY docker_start.sh /work/
COPY financial-frontend/dist/ /work/dist/
COPY server/target/release/server /work/
COPY dipiper-server/ /work/dipiper-server/
COPY simple-lstm-server/ /work/simple-lstm-server
EXPOSE 80 27017 51411 9090 8000

RUN ls -lahi /work/
RUN mkdir /work/nodejs
RUN wget https://nodejs.org/download/release/v16.15.1/node-v16.15.1-linux-x64.tar.gz
RUN tar xzf node-v16.15.1-linux-x64.tar.gz -C /work/nodejs
ENV NODE=/work/nodejs/node-v16.15.1-linux-x64/bin/node
RUN ls -lahi /work/nodejs/node-v16.15.1-linux-x64/
RUN $NODE --version

CMD [ "sh", "/work/docker_start.sh" ]