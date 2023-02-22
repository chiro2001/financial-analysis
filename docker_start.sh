export http_proxy=
export https_proxy=
export ROOT=/app/

cd $ROOT/simple-lstm-server && $PYTHON server.py &
cd $ROOT && $ROOT/server
