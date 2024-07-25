import { App } from "uWebSockets.js";

const PORT = process.env.PORT || 8080;

App()
  .get("/", (res, req) => {
    res.cork(() => {
      res.writeHeader("content-type", "text/html").end("hello world");
    });
  })
  .listen(port, (token) => {
    if (token) {
      console.log("Listening to port ", PORT);
    }
  });
