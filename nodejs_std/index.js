import * as http from "node:http";

const PORT = process.env.PORT || 8080;

let server = http.createServer((req, res) => {
  res.write("Hello World!");
  res.end();
});

console.log(`http:://localhost:${PORT}`);
server.listen(PORT);
