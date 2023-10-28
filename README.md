<p align="center">
  <img src="https://github.com/cristicretu/http-server/assets/45521157/a8d228c7-45ed-4f0a-878a-9011805bbf26" alt="smolserver logo">
</p>

<p align="center">
    <b>smolserver - a small http-server written in Rust</b> <br /><br />
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" ></a>
</p>
<hr />

## ✨ Features

- **File Handling**: Supports both reading and creating files. Perfect for file manipulation operations through HTTP requests.

- **User Agent Routing**: Special handling for requests with the `User-Agent` header.

- **Echo Route**: You can echo back whatever is appended to the `/echo/` endpoint, making it useful for quick tests.

- **Educational**: Designed primarily as a learning exercise to understand the workings of a basic HTTP server in Rust.

- **Extensible**: Despite its simplicity, the server can be further developed and integrated into larger projects.

## 🛠 How It Works

- **`GET /`**: Returns HTTP 200 for index routes.

- **`GET /user-agent`**: Returns HTTP 200 and echoes back the `User-Agent` if present in the request headers.

- **`GET /echo/{message}`**: Returns HTTP 200 and echoes back the `{message}`. For example, a GET request to `/echo/hello` would return "hello".

- **`GET /files/{name_of_file}`**: If the file exists in the directory (provided as a command line argument), it returns the file with HTTP 200. Otherwise, returns HTTP 404.

- **`POST /files/{name_of_file}`**: Creates a new file in the directory (provided as a command line argument) with the content from the request body. Returns HTTP 201 with a success message.

## 💡 What to Learn

This is an excellent project to understand:

- Basic HTTP protocol operations
- Socket programming in Rust
- Multi-threading
- File I/O
- Simple routing logic


## 📜 License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information. Free forever.

Happy coding! 🦀
