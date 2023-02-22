export http_proxy=
export https_proxy=
export ROOT=/app/

cd $ROOT/simple-lstm-server && $PYTHON server.py &
sudo mongod --bind_ip_all &
cd $ROOT/dipiper-server && $NODE run.js &
cd $ROOT && $ROOT/server
