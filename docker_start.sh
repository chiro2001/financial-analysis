export ROOT=/work/

cd $ROOT/simple-lstm-server && $PYTHON server.py &
/usr/bin/mongod --bind_ip_all &
cd $ROOT && $ROOT/server
