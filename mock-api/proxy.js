const express = require('express');
const path = require('path');
const port = 8080;
const app = express();

// serve static assets normally
app.use(express.static(__dirname + '/../wasm-spa/app/static'));

app.get('*', function (request, response) {
  response.sendFile(path.resolve(__dirname + '/../wasm-spa/app/static/index.html'));
});

app.listen(port);