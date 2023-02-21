FROM mongo:latest

WORKDIR /work/

RUN apt update && apt install -y nodejs python3
RUN apt install -y python3-pip

COPY ./simple-lstm-server/requirements.txt /work/

COPY tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl /work/
RUN python3 -m pip install ./tensorflow-2.4.0-cp36-cp36m-manylinux2010_x86_64.whl
RUN python3 -m pip install -r requirements.txt

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
# RUN sh rustup.sh -y

COPY . /work/
COPY server/target/release/server server /work/server/

# RUN cd dipiper-server && npm run dev
RUN cd dipiper-server && node run.js &
RUN cd financial-frontend/dist && python3 -m http.server 80 &
# RUN cd server && cargo build --release
RUN cd simple-lstm-server && python3 server.py &
# RUN cd server && cargo run --release &
RUN ./server/server &
