export ROOT=/work/

cd $ROOT/dist && $PYTHON -m http.server -b 0.0.0.0 80 &
cd $ROOT/dipiper-server && $NODE run.js &
cd $ROOT/simple-lstm-server && $PYTHON server.py &
/usr/bin/mongod --bind_ip_all &
cd $ROOT && $ROOT/server
