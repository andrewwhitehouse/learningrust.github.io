**Smart Contracts in Rust**

_The way I think this book is heading is an introduction to writing blockchain smart contracts in Rust. There are a few different options, but one of the more established seems to be Solana. So I spent part of yesterday and this morning figuring out how to get that working._

The Solana [Quick Start](https://github.com/solana-labs/example-helloworld#quick-start) has a number of steps. I was thinking how to containerise these to provide a repeatable and relatively straightforward process, so I created a Dockerfileto do it. It took some trial and error, but it works.

Here's the end result:

```
$ docker exec -it 832 /bin/bash
root@8329f2e84b79:/home# cd example-helloworld/
root@8329f2e84b79:/home/example-helloworld# npm run start

> helloworld@0.0.1 start /home/example-helloworld
> ts-node src/client/main.ts

Let's say hello to a Solana account...
Connection to cluster established: http://localhost:8899 { 'feature-set': 1122441720, 'solana-core': '1.10.8' }
Using account H426Fn5qWwnuw5sGsZ2zApWke1VSi1TqLA9niwHf5kGu containing 499999999.3230719 SOL to pay for fees
Using program ADgw8SWqgzrw1vd5DcxAeiW5jMrwqiMHfwqLiXpEAHAm
Saying hello to Av6oppMcRJ6Hep3cXjppdLtQ52toGfzj8Uo6B5tA1G4i
Av6oppMcRJ6Hep3cXjppdLtQ52toGfzj8Uo6B5tA1G4i has been greeted 2 time(s)
Success
root@8329f2e84b79:/home/example-helloworld#
```

The next step is to figure out what the code is doing and then reverse engineer a tutorial.

@Beaver 
