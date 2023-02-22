FROM mongo:latest

WORKDIR /work/

RUN apt update && apt install -y wget curl sudo

ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD true
ENV PUPPETEER_EXECUTABLE_PATH=/opt/google/chrome/chrome
RUN apt-get update && apt-get install curl gnupg -y \
  && curl --location --silent https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
  && sh -c 'echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list' \
  && apt-get update \
  && apt-get install google-chrome-stable -y --no-install-recommends \
  && rm -rf /var/lib/apt/lists/*

RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
RUN groupadd -r chiro && useradd -r -g chiro -G audio,video chiro && usermod -a -G sudo chiro
RUN chown -R chiro:chiro /work/
# RUN chmod -R 777 $HOME
USER chiro

RUN wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
RUN sudo sh Miniconda3-latest-Linux-x86_64.sh -b -p /work/conda/
RUN sudo /work/conda/bin/conda create -y -n lstm python=3.6
ENV PYTHON=/work/conda/envs/lstm/bin/python

COPY ./simple-lstm-server/requirements.txt /work/

COPY tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl /work/
RUN sudo $PYTHON -m pip install ./tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl

RUN sudo $PYTHON -m pip install -r requirements.txt

COPY financial-frontend/dist/ /work/dist/
COPY server/target/release/server /work/
COPY dipiper-server/dist/dipiper-server /work/
COPY simple-lstm-server/ /work/simple-lstm-server
COPY docker_start.sh /work/
EXPOSE 80 27017 51411 9090 8000

CMD [ "sh", "/work/docker_start.sh" ]