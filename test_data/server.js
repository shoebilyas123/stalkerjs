const http = require("http");

const server = http.createServer((req, res) => {
  res.writeHead(200, { "Content-Type": "text/plain" });
  res.end("Hello world");
});

server.listen(4040, () => {
  const arg1 = process.argv[2];
  const arg2 = process.argv[3];

  console.log("Second argument:", arg2);

  console.log("Server running at http://localhost:4000/");
});
