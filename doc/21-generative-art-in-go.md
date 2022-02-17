**WebAssembly**

[_I've been pondering where to take this ... and the Kindling question [how will tension be relieved at the end](https://wc3.akimbo.com/t/kindling-019-tension-relief-at-the-end/66332). My original plan was to teach enough Rust to be able to programme a smart contract. Currently Ethereum smart contracts are coded in Solidity; [in the future](https://www.coindesk.com/tech/2021/05/25/polkadots-gavin-wood-webassembly-is-the-future-of-smart-contracts-but-legacy-evm-is-right-now/) this will be opened up to other languages uses the WebAssembly runtime. The company behind Polkadot has created a [Domain-Specific Language](https://github.com/paritytech/ink) for smart contracts using Rust. So, my current plan is to create examples in WebAssembly, cover the language features involved, and give users the satisfaction of building projects on top of WebAssembly with Rust._]

Here goes.

This example is heavily excerpted from the book _Generative Art in Go_. Go is quite similar to Rust in some respects, particularly systems programming.

I don't plan to use this code in the eventual project, but want to get something working.

I already have Go [installed](https://go.dev/dl/) from some time ago.

Let's get something running ...

`$ mkdir generative-art`

`$ cd generative-art`

`$ mkdir wasm`

`$ touch wasm/main.go`

Add to wasm/main.go ...

```
package main

import "fmt"

func main() {
    fmt.Println("Hello, WASM")
}
```

_Note: we're still in the top-level generative-art folder._

We are going to create an HTML file `index.html` which loads a `wasm executor` which in turn loads the WASM binary.

To ensure that the WebAssembly code runs in parallel to the main web UI thread, we need to use a technology called `web workers`.

Web workers are unable to access the HTML page elements (known as the Domain Object Model), but can communicate with the JavaScript code that creates the web workers through messages.

Let's change our current directory to our `wasm` folder as we're going to add some more files.

`$ cd wasm`

`main_app.js`

```
workerObj.postMessage ({ foobar : 42 }); 
```

The worker can listen to messages from the main UI thread:

```
onmessage = function(msg) {
    console.log('Received message');
    var result = msg.data.foobar
    console.log('Posting message back');
    postMessage(result+42);

}
```

A web worker can also post back using the same API, and anyone who is listening on the other side will receive the message.

`main_app.js`

```
workerObj.onmessage = function(msg) {
  console.log(`Message received: ${msg.data}`);
}; 
```

Let's create a simple web worker, which provides a mechanism to pass messages to the main UI thread:

`go_worker.js`

```
console.log = (line) => {
  postMessage({log: line});
}

self.importScripts("wasm_exec.js");

const go = new self.Go();

let mod, inst;
let result;

WebAssembly.instantiateStreaming(fetch("main.wasm"), go.importObject)
  .then((result) => {
    mod => result.module;
    inst = result.instance;

    go.run(inst);
    console.log("WASM binary loaded");
})
.catch((err) => {
  console.error(err);
});
```

`importScripts` synchronously imports one or more scripts into the worker's scope. Note that it is the worker itselt that loads the `wasm_exec.js` bridge; and note the use of `self` which refers to the scope of the worker instance itself. The rest of the code will load and instantiate the WASM binary.

Now we create a minimal HTML and JavaScript setup to load our worker and see something printed back from our WASM-compiled Go app.

`index.html`

```
<html>
<head>
</head>
<body>
<script>
    function getWorker() {
      let worker = new window.Worker('go_worker.js');
      worker.addEventListener("message", async (event) => {
        if (event.data.log !== undefined) {
          console.log(event.data.log)
        }
      });
      return worker;
    }
    let worker = getWorker();
</script>
</body>
</html>
```

In this code we have set an event listener for messages coming from the worker, which is how we can receive information back from the worker.

Now let's create a simple server to serve our HTML page:

`$ cd ..` # Back to top-level 

`$ mkdir server`

`$ cd server`

`server.go`

```
package main

import (
    "fmt"
    "flag"
    "log"
    "net/http"
)

func main() {
    port := flag.String("p", "8080", "port to serve on")
    directory := flag.String("d", ".", "the directory where our assets are hosted")
    flag.Parse()

    http.Handle("/", http.FileServer(http.Dir(*directory)))

    log.Printf("Serving %s on HTTP port: %s\n", *directory, *port)
    log.Fatal(http.ListenAndServe(fmt.Sprintf(":%s", *port), nil))
}
```

`$ cd ..`

Let's build our `wasm` file which includes the logging:

`$ GOOS=js GOARCH=wasm go build -o wasm/main.wasm ./wasm`

This sets the environment variables `GOOS` and `GOARCH` and then calls `go build` to generate a file in WebAssembly format.

And now run the server:

`$ go run server/server.go -p 8080 -d wasm`

Open the address `http://localhost:8080/` in a browser and open the developer tools (`View > Developer > JavaScript Console` in Google Chrome) and you should see the messages sent from the worker logged in the console.

![Logged Messages](images/worker_logged_messages.png)

