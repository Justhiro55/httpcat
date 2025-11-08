# httpcat

A CLI tool that sends HTTP requests and displays cat images in your terminal based on the response status code.

## Features

- Send HTTP requests and get responses
- Fetch and display cat images from https://http.cat based on status codes
- Display images in full color or ASCII art in the terminal
- Measure and display response time
- Optional display of response headers and body

## Installation

### Build from source

```bash
git clone https://github.com/yourusername/httpcat.git
cd httpcat
cargo build --release
```

The binary will be generated at `target/release/httpcat`.

## Usage

### Basic usage

```bash
httpcat https://example.com
```

### Options

```bash
httpcat [OPTIONS] <URL>
```

#### Arguments

- `<URL>` - Target URL to send the request to (required)

#### Options

- `-X, --method <METHOD>` - HTTP method to use (default: GET)
- `--size <N>` - Image width in terminal characters (default: 60)
- `--ascii` - Display image as ASCII art
- `--no-image` - Don't display cat image
- `--headers` - Show response headers
- `--body` - Show response body
- `-H, --header <HEADER>` - Add request header (can be used multiple times)
- `-h, --help` - Print help
- `-V, --version` - Print version

### Examples

#### GET request

```bash
httpcat https://example.com
```

#### POST request

```bash
httpcat -X POST https://api.example.com/data
```

#### Request with custom headers

```bash
httpcat -H "Authorization: Bearer token123" https://api.example.com
```

#### Show response headers and body

```bash
httpcat --headers --body https://example.com
```

#### Display as ASCII art

```bash
httpcat --ascii https://example.com
```

#### Check status only (no image)

```bash
httpcat --no-image https://example.com
```

#### Change image size

```bash
httpcat --size 80 https://example.com
```

## Output example

```
🌐 Requesting: https://example.com

⏳ Loading...

✅ 200 OK

🐱 Here's your cat:

[cat image]

⏱️  Response time: 234ms
```

## Dependencies

- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [image](https://github.com/image-rs/image) - Image processing
- [viuer](https://github.com/atanunq/viuer) - Terminal image display
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling
- [colored](https://github.com/colored-rs/colored) - Colored output

## License

See [LICENSE](LICENSE) file for details.

## Credits

Cat images are provided by [https://http.cat](https://http.cat).
