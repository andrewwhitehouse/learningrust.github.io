**Working with Smart Contracts**

[_This is not going to be the most easy-to-read post. I'm capturing the steps as I figure out what's possible, and whether I can do in Rust what's currently done with Solidity. This post involves me following the documentation to see if it makes sense, and mis-interpreting a couple of things. The test here is transferring a coin amount from one account to another on a local development blockchain._]

Where is this Rust coding leading?

I want to learn enough so that I can use it when Ethereum moves to version 2, and it supports _EWASM_, smart contracts compiled to run in WebAssembly.

Rust is one of the popular languages that can build applications with WebAssembly.

I _could_ write contracts in Solidity, the language currently supported by Ethereum. But I believe that's likely to go away, or smart contract developers will prefer to use other more general purpose languages.

There's _leading edge_ and then there's _bleeding edge_ (where the tools are all rather new, subject to change, don't always work as you might expect them to, and there's a smaller group of people who have tried it). 

I started off with [this post](https://medium.com/wasm/compile-and-deploy-an-erc20-contract-on-ewasm-d91d890665e6), written in 2019 which is a long time ago in blockchain evolution. There seems to be an error when it specifies the _ABI_ so I don't think I can rely on this.

The company SecondState has a Solidity-to-EWASM compiler called [SOLL](https://github.com/second-state/SOLL). I followed the steps and received an error about a [LLCM](dependency).

Right now I'm not interested in heroics; I want to get something working.

SecondState have other docs which I _hope_ are more up to date.

I need to [install the devchain](https://docs.secondstate.io/devchain/getting-started).

`docker pull secondstate/devchain:devchain`

(I have Docker already installed; it's a common tool for managing packaged applications.)

`docker run --rm -v $HOME/.devchain:/devchain secondstate/devchain:devchain node init --home /devchain`

```
Andrews-MacBook-Pro:soll andrewwhitehouse$ docker run --rm -v $HOME/.devchain:/devchain secondstate/devchain:devchain node init --home /devchain
INFO [02-08|18:59:07.783] Successfully init devchain database and create tables! 
INFO [02-08|18:59:07.785] Allocated cache and file handles         database=/devchain/vm/chaindata cache=16.00MiB handles=16
INFO [02-08|18:59:07.842] Writing custom genesis block 
INFO [02-08|18:59:07.844] Persisted trie from memory database      nodes=3 size=423.00B time=1.886ms gcnodes=0 gcsize=0.00B gctime=0s livenodes=1 livesize=0.00B
INFO [02-08|18:59:07.853] successfully wrote genesis block and/or chain rule set hash=131599…ff6994
Andrews-MacBook-Pro:soll andrewwhitehouse$
```

I have no idea what that did, but it seemed to be successful.

Start the Devchain node ...

`docker run --rm -v $HOME/.devchain:/devchain -p 26657:26657 -p 8545:8545 secondstate/devchain:devchain node start --home /devchain`

(Did something interesting ... :+1:)

Open another terminal window because the Devchain is still using the first.

`Andrews-MacBook-Pro:soll andrewwhitehouse$ docker container ls
CONTAINER ID   IMAGE                           COMMAND                  CREATED         STATUS         PORTS                                                         NAMES
3be637ba580d   secondstate/devchain:devchain   "./devchain node sta…"   2 minutes ago   Up 2 minutes   0.0.0.0:8545->8545/tcp, 0.0.0.0:26657->26657/tcp, 26656/tcp   stupefied_kowalevski
Andrews-MacBook-Pro:soll andrewwhitehouse$`

Connect to the running container with a command line prompt:

`docker exec -i -t 3be637ba580d bash`

Inside the Docker container ...

`./devchain attach http://localhost:8545`

```
Welcome to the Travis JavaScript console!

instance: vm/v1.9.2/linux-amd64/go1.10.3
coinbase: 0x7eff122b94897ea5b0e2a9abf47b86337fafebdc
at block: 264 (Tue, 08 Feb 2022 19:08:44 UTC)
 modules: cmt:1.0 eth:1.0 net:1.0 personal:1.0 rpc:1.0 web3:1.0

> cmt.syncing
{
  catching_up: false,
  latest_app_hash: "9C19894EC306262A3F7AD80ECE78066B405F00FD",
  latest_block_hash: "F507FA7A66361E4C160157B5354BDD5E206248B6",
  latest_block_height: 280,
  latest_block_time: "2022-02-08T19:09:00.7041041Z"
}
```

There are a couple of test accounts already set up with populated CMT balances (CMT is the currenty used in the SecondState Devchain):

```
0x77beb894fc9b0ed41231e51f128a347043960a9d is the coinbase account with 10,000,000,000,000,000 CMTs.
0x7eff122b94897ea5b0e2a9abf47b86337fafebdc is a validator account with 10,000,000,000,000,000 CMTs.
```

That long number starting with `0x77...` is the wallet id.

Now we add a function that allows us to check balances:

```
function checkAllBalances() {
  var totalBal = 0;
  for (var acctNum in cmt.accounts) {
      var acct = cmt.accounts[acctNum];
      var acctBal = web3.fromWei(cmt.getBalance(acct), "cmt");
      totalBal += parseFloat(acctBal);
      console.log("  cmt.accounts[" + acctNum + "]: \t" + acct + " \tbalance: " + acctBal + " CMT");
  }
  console.log("  Total balance: " + totalBal + "CMT");
};
```

I pasted it into the Devchain console.

```
> function checkAllBalances() {
...   var totalBal = 0;
...   for (var acctNum in cmt.accounts) {
......       var acct = cmt.accounts[acctNum];
......       var acctBal = web3.fromWei(cmt.getBalance(acct), "cmt");
......       totalBal += parseFloat(acctBal);
......       console.log("  cmt.accounts[" + acctNum + "]: \t" + acct + " \tbalance: " + acctBal + " CMT");
......   }
...   console.log("  Total balance: " + totalBal + "CMT");
... };
undefined
```

(no errors)

Let's call it ...

```
> checkAllBalances();
  cmt.accounts[0]: 	0x7eff122b94897ea5b0e2a9abf47b86337fafebdc 	balance: 10000000000000000 CMT
  cmt.accounts[1]: 	0x77beb894fc9b0ed41231e51f128a347043960a9d 	balance: 10000000000000000 CMT
  Total balance: 20000000000000000CMT
undefined
```

There's an object called `personal` that allows us to interact with the smart contract.

Create a new account so that we can transfer funds to it ...

```
> personal.newAccount("mypass")
"0xf315d15d4d69dcb0dd775dc4ad9374b3dfa1d9f3"
```

Let's see if we can generate a test transaction ...

`personal.unlockAccount("from_address")`

Ah, I need to put the wallet id with funds in there in there.

`personal.unlockAccount("0x77beb894fc9b0ed41231e51f128a347043960a9d")`

That didn't work either:

```
> personal.unlockAccount("0x77beb894fc9b0ed41231e51f128a347043960a9d")
Unlock account 0x77beb894fc9b0ed41231e51f128a347043960a9d
Passphrase: 
Error: could not decrypt key with given password
>

Try the other one ...

```
> personal.unlockAccount("0x7eff122b94897ea5b0e2a9abf47b86337fafebdc")




Unlock account 0x7eff122b94897ea5b0e2a9abf47b86337fafebdc
Passphrase: 
true
```

That worked. OK I see that I tried to use the Validator account but it looks like I should have used the other.

Let's try sending some funds to the newly created account.

```
> cmt.sendTransaction({"from": "0x7eff122b94897ea5b0e2a9abf47b86337fafebdc", "to": "0xf315d15d4d69dcb0dd775dc4ad9374b3dfa1d9f3", "value": web3.toWei(0.001, "cmt")})
"0xb871ea6e89363fde4049177123806fc113319e09ef8858ee8e3b060b703582e4"
>
```

```
> checkAllBalances();
  cmt.accounts[0]: 	0x7eff122b94897ea5b0e2a9abf47b86337fafebdc 	balance: 9999999999999999.999 CMT
  cmt.accounts[1]: 	0x77beb894fc9b0ed41231e51f128a347043960a9d 	balance: 10000000000000000 CMT
  cmt.accounts[2]: 	0xf315d15d4d69dcb0dd775dc4ad9374b3dfa1d9f3 	balance: 0.001 CMT
  Total balance: 20000000000000000CMT
undefined
>
```

I now have a slightly better mental model of how the pieces fit together to produce something that works, and I can "kick the tyres".

In the next post I'm going to see if I can access a smart contract running in a WebAssembly virtual machine, based on [these instructions](https://docs.secondstate.io/devchain/getting-started/run-an-ewasm-smart-contract).

@beaver @london3