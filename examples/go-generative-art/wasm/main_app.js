workerObj.postMessage ({ foobar : 42 });

onmessage = function(msg) {
    console.log('Received message');
    var result = msg.data.foobar
    console.log('Posting message back');
    postMessage(result+42);

}

workerObj.onmessage = function(msg) {
  console.log(`Message received: ${msg.data}`);
}; 
