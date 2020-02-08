# Template for a local GUI with [seed](https://seed-rs.org) and [gotham](https://gotham.rs)
This is a template repository for creating locally running client-side/wasm in browser GUIs with 
* [seed](https://seed-rs.org) 
    > A Rust framework for creating web apps 
* [gotham](https://gotham.rs)
    > A flexible web framework that does not sacrifice safety, security or speed.

It displays a simple counter and two buttons to increment/decrement the counter.
Communication works via a REST api.

This approach effectively allows you to create in browser GUIs 
which can operate on the file system in the same way as non-browser GUIs because 
the usual sandboxing of code inside the browser doesn't affect the server.

## Index
* [Required dependencies](#required-dependencies)
* [Quickstart](#quickstart)
* [Details](#details)
    * [Workspace organisation](#workspace-organisation)
    * [Building and serving](#building-and-serving)
* [Helpful resources](#helpful-resources)
* [Contribution](#contribution)

## Required dependencies 
All required libraries are declared in the respective `Cargo.toml` file.
Nevertheless, some tools need to be installed manually:
* This is not strictly necessary, but good practice before starting a new project: 
    ```
    rustup update
    ```
* Add the wasm target to your toolchain:
    ```
    rustup target add wasm32-unknown-unknown
    ```
* Install `cargo make`, which automates building/serving the app:
    ```
    cargo install cargo-make 
    ```
  
## Quickstart
Build the app with `cargo make build` (`cargo make build_release`) 
or start it with `cargo make start` (`cargo make start_release`). 
Access the app on `localhost:8080` in your browser.

## Details

The detailed structure of the app template.

### Workspace organisation
The workspace is split in three crates:

* __api:__ The implementation of the rest api with _gotham_, allowing the client to access local file system resources.
Serves the `gui` as a wasm binary wrapped in a simple html site.
Could be named `server` too.
* __gui:__ The implementation of the gui with _seed_. 
Accesses the data via the rest api implementation of the `gui`. 
Could be named `client` too.
* __shared:__ The data model of the app, e.g. code that is needed both on gui (client) and api (server) side.

### Building and serving
The workspace root contains a `Makefile.toml` which adds the `build` and `start` tasks to the
`cargo make`  command. Both have a `_release` sibling that enables the release optimizations.
The `start` tasks builds everything and then starts the `api` binary that contains the server with the 
rest api implementation and the statically served gui. 

### CSS styling

The minified version of [Wing](https://kbrsh.github.io/wing) is loaded in the `index.html` and 
included in the repository.

## Helpful resources
The code (especially the `gui` implementation) is to a large extent just adopted from the examples in 
the libraries repositories.
Additionally, the documentation for both libraries is really helpful, so go take a look at it too. 
(And a big thank you to both libraries contributors for both the libraries itself and the amazing 
documentation, that isn't always in such a good condition.)

## Contribution
Some choices in this template are (very) opinionated, like for example the frameworks in use.
Although I want this to be a template for the rust community as a whole, 
it's sometimes difficult to please everyones  expectations and choices in software('s) design. 
Therefore I happily invite everyone to participate in the further development of this 
template repository, but also remind everyone that sometimes ideas just don't fit together
and we should respect that. 

Both the issue tracker and the merge requests are a great way to improve the repository.
For the reason mentioned above I'd prefer to talk about new ideas in issues first
and start merge requests only after discussion took place in an issue.
(Mainly to prevent somebody putting work into something they like only for it to
be discarded because there's disagreement if it really fits into this template.) 

## License
MIT License

Copyright (c) 2020 Florian Warzeche <liketechnik@disroot.org>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
