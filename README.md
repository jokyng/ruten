# ruten

Fast, lightweight programming language with subprocess, multiprocessing, AI and other modules support.

![Version](https://img.shields.io/badge/1.1.2-stable-5d5d5d?style=flat-square&logo=rust) ![Rust](https://img.shields.io/badge/rust-1.70+-5d5d5d?style=flat-square&logo=rust) [![Give a star](https://img.shields.io/badge/Give%20a%20⭐-%20-5d5d5d?style=flat-square&logo=github)](https://github.com/ogcae/ruten/stargazers)

![Build](https://img.shields.io/github/actions/workflow/status/ogcae/ruten/ci.yml?branch=main&style=flat-square&label=build)


## Install & Test
```bash
$ git clone https://github.com/jokyng/ruten
$ cd ruten
$ cargo build --release
$ cargo test
$ ruten examples/blockchain.rt
```

## Quick Start
```bash
$ cargo install --path .   # ~ install
$ ruten                    # ~ repl
$ ruten script.rt          # ~ run file
```

## Modules
| module | description | example |
|--------|-------------|---------|
| `ai` | openai, claude, ollama integration | `ai.complete(prompt)` |
| `vision` | image processing, face detection | `vision.detect_faces(img)` |
| `nlp` | sentiment analysis, tokenization | `nlp.sentiment(text)` |
| `torrent` | multi-threaded downloads | `torrent.download(url)` |
| `subproc` | shell commands, process execution | `subproc.run(cmd)` |
| `multiproc` | parallel operations, cpu info | `multiproc.parallel(fn, data)` |
| `downloads` | file downloads with progress | `downloads.save(url, path)` |
| `apps` | cli utilities, progress bars | `apps.progress(total)` |
| `format` | code formatting and cleanup | `format.indent(code)` |
| `database` | key-value storage | `database.set(key, value)` |
| `email` | send emails | `email.send(to, subject, body)` |
| `logger` | advanced logging | `logger.info("message")` |
| `testing` | assertions and tests | `testing.assert_equal(a, b)` |
| `stats` | statistical operations | `stats.mean(numbers)` |
| `utils` | common utilities | `utils.uuid()` |
| `image` | image processing | `image.resize(path, w, h)` |
| `weather` | real-time weather data | `weather.current("london")` |
| `github` | github api access | `github.user("torvalds")` |
| `news` | latest news headlines | `news.headlines("us")` |
| `quotes` | inspirational quotes | `quotes.random()` |
| `jokes` | random jokes and humor | `jokes.programming()` |
| `facts` | interesting facts | `facts.random()` |
| `geo` | geolocation and ip lookup | `geo.myip()` |
| `blockchain` | crypto prices | `blockchain.btc_price()` |
| `crypto_prices` | cryptocurrency data | `crypto_prices.bitcoin()` |
| `http` | async http client with headers support | `http.get(url, headers)` |
| `webhook` | lightweight webhook server | `webhook.listen(port, handler)` |
| `crypto` | hashing, encryption, secure random | `crypto.sha256(data)` |
| `json` | fast json parsing and serialization | `json.parse(text)` |
| `math` | advanced math operations | `math.fibonacci(n)` |
| `strings` | regex, parsing, manipulation | `strings.match(pattern, text)` |

---

## Contributors

<a href="https://github.com/ogcae/ruten/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=ogcae/ruten" />
</a>

made with <a href="https://github.com/jokyng/ruten"><code>jokyng</code></a> by <a href="https://github.com/ogcae"><code>ogcae</code>
</a>



<a href="./LICENSE">
<code>LICENSE</code></a> · 
<a href="./CONTRIBUTING.md">
<code>CONTRIBUTE</code></a> ·
<a href="./CHANGELOG.md">
<code>CHANGELOG</code></a>