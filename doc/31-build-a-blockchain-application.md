https://www.youtube.com/watch?v=coQ5dg8wM2o

awwapp.com

Compare a traditional application with a blockchain equivalent.

Client side application doesn't talk to web backend and database; it talks to the blockchain.

What is a blockchain
 - a separate peer-to-peer network of nodes that all talk to each other; it's distributed. All of the nodes in the network participate in running the netowrk and they all contain a copy of the code and the data on the blockchain. All of the data is contained in bundles of records called blocks which are chained together.

All of the code on the blockchain is contained in smart contracts. Smart contracts are written in solidity, and it's immutable. That means we can't change it "and that makes it secure". 

Build a client-side application in HTML, CSS and JavaScript.

Install Node.js; check with `node -v`.

Install [Ganache](https://trufflesuite.com/ganache/), which is a personal blockchain. Download the app, install and run it.

Ganache runs on your local machine and makes it easier to develop and test blockchain applications locally.

The Truffle framework is a suite of tools that allow us to develop smart contracts, write tests against the smart contracts, deploy smart contracts to the blockchain  ... it gives us a devleopment console and it also allows us to develop client-side applications inside our project.

`npm install -g truffle` 

(tutorial says use 5.0.2 - I'm going to ignore that for now; the latest is actually truffle@5.5.2)

The next dependency is the Metamask extension for Google Chrome; the Ethereum blockchain is a network and we need a special browser extension in order to connect to that network. And that's where Metamask comes into play. 

Metamask will allow us to connect to the blockchain with our personal account and interact with the smart contract that we'll develop in this tutorial.

Install Metamask by going to the Google Chrome web store and searching for Metamask, and clicking install. And once you've installed it, make sure you enable it inside of your Chrome extensions.

Now create the project.

`mkdir eth-todo-list`

`cd eth-todo-list`

`truffle version`

(apparently it needs to be version 5.0.2)

```
$ truffle version
Truffle v5.5.2 (core: 5.5.2)
Ganache v7.0.1
Solidity v0.5.16 (solc-js)
Node v12.16.1
Web3.js v1.5.3
$
```

```
$ truffle init

Starting init...
================

> Copying project files to /Users/andrewwhitehouse/code/eth-todo-list

Init successful, sweet!

Try our scaffold commands to get started:
  $ truffle create contract YourContractName # scaffold a contract
  $ truffle create test YourTestName         # scaffold a test

http://trufflesuite.com/docs

$
```

Create package.json so we can manage development dependencies for the project:

`npm init -y`

Here's the github repo <https://github.com/dappuniversity/eth-todo-list>

```  
{
  "name": "eth-todo-list",
  "version": "1.0.0",
  "description": "Blockchain Todo List Powered By Ethereum",
  "main": "truffle-config.js",
  "directories": {
    "test": "test"
  },
  "scripts": {
    "dev": "lite-server",
    "test": "echo \"Error: no test specified\" && sexit 1"
  },
  "author": "gregory@dappuniversity.com",
  "license": "ISC",
  "devDependencies": {
    "bootstrap": "4.1.3",
    "chai": "^4.1.2",
    "chai-as-promised": "^7.1.1",
    "chai-bignumber": "^2.0.2",
    "lite-server": "^2.3.0",
    "nodemon": "^1.17.3",
    "truffle": "5.5.2",
    "truffle-contract": "4.0.31"
  }
}
```

cat the above into package.json

Install the dependencies that we just added to package.json:

`$ npm install`

Here is our directory contents:

```
$ tree -I node_modules
.
├── contracts
│   └── Migrations.sol
├── migrations
│   └── 1_initial_migration.js
├── package-lock.json
├── package.json
├── test
└── truffle-config.js

3 directories, 5 files
$
```

Create a new file under contracts called TodoList.sol:

`$ cd contracts/`

`$ touch TodoList.sol`

Edit the file. I'm using Atom `atom .` with the language-solidity (0.1.1) and linter-solidity (0.7.1) packages installed.

In TodoList.sol ...

Declare the version of Solidity we want to use

`pragma solidity ^0.5.16;`

Declare the smart contract:

```
contract TodoList {
  
}
---

Start with a simple change that we can test to ensure everything is working; keep track of tasks in the todolist. Add a state variable.

```
contract TodoList {
  uint taskCount;
}
```

(Note that variables use _camel case_.)

State variables are actually written to the blockchain; they represent the state of this smart contract on the blockchain. These are like class variables in an object-oriented context, where the scope of the variable belongs to the entire smart contract.

Let's initialise it, and add the public keyword which indicates we can read the variable from the smart contract. The keyword provides a function for us that allows us to read the taskCount from the todolist.

```
pragma solidity ^0.5.16;

contract TodoList {
  uint public taskCount = 0;
}
```

Compile the smart contract

```
$ truffle compile

Compiling your contracts...
===========================
> Compiling ./contracts/TodoList.sol
> Artifacts written to /Users/andrewwhitehouse/code/eth-todo-list/build/contracts
> Compiled successfully using:

   - solc: 0.5.16+commit.9c3226ce.Emscripten.clang
Andrews-MacBook-Pro:eth-todo-list andrewwhitehouse$
```

```
$ tree build
build
└── contracts
    ├── Migrations.json
    └── TodoList.json

1 directory, 2 files
$
```

In order to deploy to the local blockchain we need to update the truffle-config.js using [code](https://github.com/dappuniversity/eth-todo-list/blob/master/truffle-config.js) from the repository.

```
module.exports = {
  networks: {
    development: {
      host: "127.0.0.1",
      port: 7545,
      network_id: "*" // Match any network id
    }
  },
  solc: {
    optimizer: {
      enabled: true,
      runs: 200
    }
  }
}
```

This defines a development network that is connected to Ganache.

Check that Ganache is running and that the RPC server details match the values in truffle-config.js.

add image

Now let's create a migration file in order to get the smart contract onto the blockchain.

You have an initial file in the migrations directory:

```
$ tree migrations/
migrations/
└── 1_initial_migration.js

0 directories, 1 file
$
```

Copy the contents into a new file `2_deploy_contracts.js`

```

```

Whenever you deploy a contract to the blockchain you're changing its state. Migrations allow you to manage this, and the file naming (with an initial number) determines the order in which the migrations are run.

```
const TodoList = artifacts.require("TodoList");

module.exports = function (deployer) {
  deployer.deploy(TodoList);
};
```

Now run the migration

`$ truffle migrate`

```
$ truffle migrate
This version of µWS is not compatible with your Node.js build:

Error: node-loader:
Error: Module did not self-register: '/Users/andrewwhitehouse/.nvm/versions/node/v12.16.1/lib/node_modules/truffle/node_modules/ganache/dist/node/3wfpWiF8.node'.
Falling back to a NodeJS implementation; performance may be degraded.




Compiling your contracts...
===========================
> Everything is up to date, there is nothing to compile.


Starting migrations...
======================
> Network name:    'development'
> Network id:      5777
> Block gas limit: 6721975 (0x6691b7)


. . .


2_deploy_contracts.js
=====================

   Replacing 'Migrations'
   ----------------------
   > transaction hash:    0x0568e44a50b8ba9ca6aeb8bc609fd01c84d47cbe89f6be1df013d2f6518a9c58
   > Blocks: 0            Seconds: 0
   > contract address:    0x34724557c87Cec24a699CBB6d36e0D2B81d1875A
   > block number:        3
   > block timestamp:     1645960502
   > account:             0xD51453Cc710A4623fB912fcde03C40B6ADff38A5
   > balance:             99.98919606
   > gas used:            248842 (0x3cc0a)
   > gas price:           20 gwei
   > value sent:          0 ETH
   > total cost:          0.00497684 ETH
   > Saving migration to chain.
   > Saving artifacts
   -------------------------------------
   > Total cost:          0.00497684 ETH

Summary
=======
> Total deployments:   2
> Final cost:          0.00995368 ETH

$
```

After deployment the balance of the first account has reduced by a little. That's because deploying contracts to the blockchain actually costs _gas_. Truffle by defaut uses the first account inside the wallet to pay those fees.

Now let's use the Truffle console to retrieve the contract.

`$ truffle console`

(got an error)

`nvm use v14.17.5`	
	
`npm rebuild`

`npm install -g truffle`

`truffle compile`

`truffle migrate`

`truffle console`

```
$ truffle console
truffle(development)> todoList = await TodoList.deployed()
Uncaught:
Error: TodoList has not been deployed to detected network (network/artifact mismatch)
    at processTicksAndRejections (internal/process/task_queues.js:95:5)
    at Function.deployed (/Users/andrewwhitehouse/.nvm/versions/node/v14.17.5/lib/node_modules/truffle/build/webpack:/packages/contract/lib/contract/constructorMe
thods.js:83:1)
    at Object.checkNetworkArtifactMatch (/Users/andrewwhitehouse/.nvm/versions/node/v14.17.5/lib/node_modules/truffle/build/webpack:/packages/contract/lib/utils/i
ndex.js:256:1)
truffle(development)>
```

Run 

`truffle migrate --reset`

Truffle console still doesn't work.

https://stackoverflow.com/questions/48694192/contract-has-not-been-deployed-to-detected-network-network-artifact-mismatch-o

Looks like I had an issue with my deployment, and the migration not being run again. I created another one and it worked.

```
truffle(development)> todoList = await TodoList.deployed()
undefined
truffle(development)>
```

```
truffle(development)> todoList = await TodoList.deployed()
undefined
truffle(development)> todoList.address
'0x6314Cc6dF4703364e54a44c6c0a9429BEcf55185'
truffle(development)> taskCount = await todoList.taskCount()
undefined
truffle(development)> taskCount.toNumber()
0
truffle(development)>
```	
	
	
	
	
	For later
	
	
---	
	
Related

https://www.youtube.com/watch?v=ipwxYa-F1uY	
	

Notes:

https://cointelegraph.com/news/bragging-rights-twitter-previews-verification-badge-for-nft-profile-pics

You can use NFTs as a verified profile picture (to prove that you own it).

https://www.youtube.com/watch?v=_qaTU0BCg54

NFT replaces the blue checkmark.

Look at tycoon.eth Twitter account

NFT says "I am 0xtycoon on Twitter" and Twitter points back to the NFT.

Your wallet has a public key and a private key.







