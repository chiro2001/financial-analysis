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

# RUN mkdir /work/nodejs
# RUN wget https://nodejs.org/download/release/v16.15.1/node-v16.15.1-linux-x64.tar.gz
# RUN tar xzf node-v16.15.1-linux-x64.tar.gz -C /work/nodejs
# ENV NODE=/work/nodejs/node-v16.15.1-linux-x64/bin/node

COPY financial-frontend/dist/ /work/dist/
COPY server/target/release/server /work/
COPY dipiper-server/dist/dipiper-server /work/
COPY simple-lstm-server/ /work/simple-lstm-server
COPY docker_start.sh /work/
EXPOSE 80 27017 51411 9090 8000

CMD [ "sh", "/work/docker_start.sh" ]