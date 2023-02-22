export ROOT=/work/

export PATH=/work/chrome-linux/:$PATH

cd $ROOT/dist && $PYTHON -m http.server -b 0.0.0.0 8999 &
$ROOT/dipiper-server &
cd $ROOT/simple-lstm-server && $PYTHON server.py &
sudo /usr/bin/mongod --bind_ip_all &
cd $ROOT && $ROOT/server
