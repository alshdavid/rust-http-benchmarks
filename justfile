PORT := env('PORT', '8080')

install:
  npm install

run package *ARGS:
  cd {{package}} && just run {{ARGS}}

watch package *ARGS:
  cargo watch --watch {{package}} -- bash -c "cd {{package}} && exec just run {{ARGS}}"

fmt:
  cargo +nightly fmt

benchmark:
  just benchmark_one go_std
  just benchmark_one nodejs_std
  just benchmark_one nodejs_uws
  just benchmark_one rust_actix
  just benchmark_one rust_hyper
  just benchmark_one rust_may_minihttp
  just benchmark_one rust_tiny_http
  just benchmark_one rust_tokio_raw
  
benchmark_one package *ARGS:
  echo "Running {{package}}"
  cd _runner && env PACKAGE={{package}} cargo run --release

vegeta package:
  rm -rf _reports/{{package}}
  mkdir -p _reports/{{package}}

  echo warmup
  echo "GET http://localhost:{{PORT}}/" | \
  vegeta attack \
    -duration=1s \
    -rate=100 \
    -keepalive=false \
    -connections=500 \
    -rate=100 &> /dev/null
  
  echo "GET http://localhost:{{PORT}}/" | \
  vegeta attack \
    -duration=4s \
    -rate=100 \
    -keepalive=false \
    -connections=50000 \
    -rate=1000 \
    -output=_reports/{{package}}/attack.bin

  vegeta plot -title={{package}} _reports/{{package}}/attack.bin > _reports/{{package}}/index.html
  @echo "<pre><code>" >> _reports/{{package}}/index.html
  cat _reports/{{package}}/attack.bin | vegeta report >> _reports/{{package}}/index.html
  @echo "</code></pre>" >> _reports/{{package}}/index.html

oha package:
  oha -n 50000 -c 5000 --latency-correction http://localhost:8080