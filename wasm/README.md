# CoronaCard WebAssembly

[![License: MIT][l]](LICENSE.md)

Here, we bundle the [`coronacard`](..) into a website using webassmbly (`wasm-bindgen`).
To compile, type 
```bash
wasm-pack build --target=web --out-dir=www/pkg
```

Then start any webserver in the `www/` directory, e.g.
```bash
python -m http.server --directory=www
```

The website is also available at <https://eqt.github.io/coronacard/>.


[l]: https://img.shields.io/badge/license-MIT-brightgreen.svg
