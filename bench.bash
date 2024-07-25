

# Warmup
oha -n 100 $URL

REQUESTS=50000
PORT=8080
CONCURRENCY=5000
URL="http://localhost:${PORT}"
reset
# oha -n $REQUESTS -c $CONCURRENCY --latency-correction --disable-keepalive $URL
oha -n $REQUESTS -c $CONCURRENCY --latency-correction $URL

  just run {{package}} {{ARGS}} & \
  just vegeta {{package}} && \
  kill $!