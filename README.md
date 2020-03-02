# Template for a local GUI with [seed](https://seed-rs.org) and [gotham](https://gotham.rs)
This is a template repository for creating locally running client-side/wasm in browser GUIs with 
* [seed](https://seed-rs.org) 
    > A Rust framework for creating web apps 
* [gotham](https://gotham.rs)
    > A flexible web framework that does not sacrifice safety, security or speed.
* [rust-embed](https://github.com/pyros2097/rust-embed)
    > Rust Custom Derive Macro which loads files into rust binary at compile time [...].
* [web-view](https://github.com/Boscop/web-view)
    > [...] provides a Rust binding to the original implementation of [webview](https://github.com/zserge/webview), a tiny cross-platform library to render web-based GUIs as desktop applications.
                                
in rust.

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
    * [CSS Styling](#css-styling)
* [Current State](#current-state)
    * [Pros](#pros)
    * [Cons / Problems](#cons--problems)
    * [Roadmap](#roadmap)
* [Helpful resources](#helpful-resources)
* [Contribution](#contribution)
* [License](#license)

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
* On linux, make sure ```Webkit2gtk``` is available (cf. web-view repo):
    I can't provide specific instructions here, but for compiling, you'll usually need some kind of ```-devel``` package additionally.
  
## Quickstart
Build the app with `cargo make build` (`cargo make build_release`) 
or start it with `cargo make start` (`cargo make start_release`). 
If you need access to the developer console you can open the gui on port 8080 in your browser.

## Details

The detailed structure of the app template.

### Workspace organisation
The workspace is split in four crates:

* __api:__ The implementation of the rest api with _gotham_, allowing the client to access local file system resources.
Serves the `gui` as a wasm binary wrapped in a simple html site.
For serving assets like the wasm binary or the html, ```rust-embed``` is used.
Could be named `server` too.
* __gui:__ The implementation of the gui with _seed_. 
Accesses the data via the rest api implementation of the `gui`. 
Could be named `client` too.
* __shared:__ The data model of the app, e.g. code that is needed both on gui (client) and api (server) side.
* __app:__ Bundles *api* and *gui* together as one binary. Starts the webview window and the api server.

### Building and serving
The workspace root contains a `Makefile.toml` which adds the `build` and `start` tasks to the
`cargo make`  command. Both have a `_release` sibling that enables the release optimizations.
The `start` tasks builds everything and then starts the `app` binary that contains the server with the 
rest api implementation and the statically served gui. 

### CSS styling

The minified version of [Wing](https://kbrsh.github.io/wing) is loaded in the `index.html` and 
included in the repository.

## Current state
Why you may want to use the approach of this template, why you may not want to (exactly) follow
the approach of this template. And of course what is planned for the future of the template.

### Pros
Why you may want to use this template. The expected use cases for this template 
(feel free to add more if you've found this useful in a different case too).

* in browser GUI with access to local file system: usually the sandbox of the browser 
prevents access to the local file system. The gui/api architecture bypasses this.
* rendering of markdown or math formula via rust or JavaScript 
* clear separation of gui and logic due to having two separate binaries -> 
* easier testing of the logic, because the api is fully testable without the gui crate
* Size: Although not a really good example, the full size of this app with bundled assets is around 7,5MB (the release, not the debug build). 

### Cons / Problems
Why you may not want to use this template. Use cases this was not designed for 
(may change in the future) and (current) limitations.  

* ~~no easy way of distributing binaries of the program: There are multiple binaries to distribute
and the `.html` and `.css` files for loading the gui need to be distributed. 
Currently you need to download the repository to start the program.~~ Solved with rust-embed.
* ~~The user has to manually open a browser window and point it to the server: 
This could be solved by just starting a browser and passing it the server url before starting
the server.~~ Solved with webview.
* No concurrency handling: If a user opens the gui in multiple browser tabs 
the gui state won't be synchronized, the server also doesn't expect this to happen. 
Because this isn't a concern for traditional GUIs too, this will always be the case.
But one could add a note in the gui.
* No authentication: The server is bound to a local port that is not reachable
from outside of the computer the program was started on. This is expected to always be the case.
Also not a concern for traditional GUIs, so out of scope here.

(`~`: part of the long therm roadmap; `!`: part of the short term roadmap)

### Roadmap
What is currently missing functionality to be showcased? What problems/limitations are solvable in
the (near) future?

Short term:
* [x] Move counter logic to server, integrate api.
* [x] Add shutdown logic that lets the server terminate itself 
when the user exits the gui in the browser (e. g. closes the tab).
* [x] ~~Open firefox (if available) and point it to the server's url on startup.~~ (Replaced by webkit functionality)
* [ ] Save and load the counter to show how to use files.
* [ ] Together with above: Add a navigation bar with different content, e. g. state handling.
(Similar to what maybe would be different windows in traditional GUIs)

Long term:
* [x] Create a third crate that uses webkit (or something with webkit like functionality) 
that starts the server and loads the webpage gui. (Would solve both current problems). Solved with web-view crate.
* [ ] Integrate a configuration framework to showcase/template configuration options.
* [x] Find a way for bundling/creating packages/releases. Handled with rust-embed.
* [ ] Try to integrate templating and/or internationalization into this.

## Helpful resources
The code (especially the `gui` implementation) is to a large extent just adopted from the examples in 
the libraries repositories.
Additionally, the documentation for both libraries is really helpful, so go take a look at it too. 
(And a big thank you to both libraries contributors for both the libraries itself and the amazing 
documentation, that isn't always in such a good condition.)

## Contribution
Some choices in this template are (very) opinionated, like for example the frameworks in use.
Although I want this to be a template for the rust community as a whole, 
it's sometimes difficult to please everyones expectations and choices in software('s) design. 
Therefore I happily invite everyone to participate in the further development of this 
template repository, but also remind everyone that sometimes ideas just don't fit together
and that we should respect that. 

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
