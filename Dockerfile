FROM node:18

WORKDIR /app/

RUN apt-get update \
    && apt-get install -y wget gnupg sudo vim nano

# RUN mkdir -p /app/nodejs
# RUN wget https://nodejs.org/download/release/v18.14.2/node-v18.14.2-linux-x64.tar.gz
# RUN tar xzf node-v18.14.2-linux-x64.tar.gz -C /app/nodejs
# ENV NODE=/app/nodejs/node-v18.14.2-linux-x64/bin/node
# ENV NPM=/app/nodejs/node-v18.14.2-linux-x64/bin/npm
# RUN ls -lahi /app/nodejs/node-v18.14.2-linux-x64/
# RUN $NODE --version

ENV NODE=node
ENV NPM=npm
RUN $NODE --version

# Install latest chrome dev package and fonts to support major charsets (Chinese, Japanese, Arabic, Hebrew, Thai and a few others)
# Note: this installs the necessary libs to make the bundled version of Chromium that Puppeteer
# installs, work.
RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
    && sh -c 'echo "deb [arch=amd64] http://dl.google.com/linux/chrome/deb/ stable main" >> /etc/apt/sources.list.d/google.list' \
    && apt-get update \
    && apt-get install -y google-chrome-stable fonts-ipafont-gothic fonts-wqy-zenhei fonts-thai-tlwg fonts-kacst fonts-freefont-ttf libxss1 \
      --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

# Uncomment to skip the chromium download when installing puppeteer. If you do,
# you'll need to launch puppeteer with:
#     browser.launch({executablePath: 'google-chrome-stable'})
ENV PUPPETEER_SKIP_CHROMIUM_DOWNLOAD true
# ENV PUPPETEER_EXECUTABLE_PATH=/opt/google/chrome/chrome
ENV PUPPETEER_EXECUTABLE_PATH=/usr/bin/google-chrome-stable

RUN wget https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh
RUN sh Miniconda3-latest-Linux-x86_64.sh -b -p /app/conda/
RUN /app/conda/bin/conda create -y -n lstm python=3.6
ENV PYTHON=/app/conda/envs/lstm/bin/python

COPY ./simple-lstm-server/requirements.txt /app/

COPY tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl /app/
RUN $PYTHON -m pip install ./tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl
RUN $PYTHON -m pip install -r requirements.txt

RUN groupadd -r pptruser && useradd -r -g pptruser -G audio,video pptruser \
    && mkdir -p /home/pptruser/Downloads \
    && chown -R pptruser:pptruser /home/pptruser
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
RUN usermod -a -G sudo pptruser

COPY --chown=pptruser:pptruser dipiper-server/dipiper /app/dipiper-server/dipiper/
COPY --chown=pptruser:pptruser dipiper-server/package.json /app/dipiper-server/
COPY --chown=pptruser:pptruser dipiper-server/*.js /app/dipiper-server/
RUN ls -lahi /app/

# ENV NODE_PATH=/app/nodejs/node-v18.14.2-linux-x64/bin
ENV http_proxy=http://r.chiro.work:14514
ENV https_proxy=http://r.chiro.work:14514
# Install puppeteer so it's available in the container.
# RUN PATH=$PATH:$NODE_PATH $NPM i -g yarn
# RUN cd dipiper-server && PATH=$PATH:$NODE_PATH yarn
RUN cd dipiper-server && yarn

ENV FRONTEND_STATIC_PATH=/app/dist/
ENV MONGO_URI="mongodb://dipiper:1352040930@a.chiro.work"

COPY --chown=pptruser:pptruser financial-frontend/dist/ /app/dist/
COPY --chown=pptruser:pptruser server/target/release/server /app/
COPY --chown=pptruser:pptruser simple-lstm-server/ /app/simple-lstm-server
COPY --chown=pptruser:pptruser docker_start.sh /app/
EXPOSE 27017 51411 9090 8000

# RUN chown -R pptruser:pptruser /app/

# Run everything after as non-privileged user.
USER pptruser

CMD [ "sh", "docker_start.sh" ]
