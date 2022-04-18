Containers

I'm coming to the conclusion that:
- for the extra effort involved in writing Rust code there needs to be a particular benefit, because it is extra work
- blockchain applications are a good example
- I should focus on building blockchain appllications

Ethereum doesn't support Rust (with WebAssembly) fully yet. There are some other blockchains like Polkadot and Near. There are a few moving parts to these. There's also Solana which seems to be more mature. 

[_I can't decide whether to continue with Advent of Code all the way to the end, in Rust. I'm not convinced._]

So let's start with Solana.

To have a repeatable consistent way of running up the necessary pieces without filling our local machine with stuff, we can use _containers_, and Docker is a well established solution for this.

We create a Dockerfile:

```
FROM alpine:latest
MAINTAINER Andrew Whitehouse <you@example.com>

CMD ["echo", "Hello World!"]
```

Then build it:

`docker built -t solana-helloworld:0.0.1 .`

This will build the image:

```
REPOSITORY                                  TAG           IMAGE ID       CREATED         SIZE
solana-hello-world                          0.0.1         7911d578feb4   12 days ago     5.57MB
```

And then we can run it with:

`docker run 7911d578feb4`

```
$ docker run 7911d578feb4
Hello World!
$
```

Let's look at the Solana examples:

https://docs.solana.com/developing/on-chain-programs/examples

```
docker build -t solana-helloworld:0.0.1 .
```

```
docker run -it solana-helloworld:0.0.1
```

Here is the repository

```
https://github.com/andrewwhitehouse/solana-docker
```

```
# df -k
Filesystem     1K-blocks     Used Available Use% Mounted on
overlay         61252420 24833220  33278032  43% /
tmpfs              65536        0     65536   0% /dev
tmpfs            1017700        0   1017700   0% /sys/fs/cgroup
shm                65536        0     65536   0% /dev/shm
/dev/vda1       61252420 24833220  33278032  43% /etc/hosts
tmpfs            1017700        0   1017700   0% /proc/acpi
tmpfs            1017700        0   1017700   0% /sys/firmware
root@ce7ab66c2fdb:/home#
```

$ docker images
REPOSITORY          TAG       IMAGE ID       CREATED          SIZE
solana-helloworld   0.0.1     e8ee2446f4d4   34 seconds ago   2.66GB
redis               latest    ef47f3b6dc11   16 months ago    104MB
mongo               latest    3068f6bb852e   16 months ago    493MB
Andrews-MacBook-Pro:solana-docker andrewwhitehouse$ docker run -it solana-helloworld:0.0.1
root@b531c5e3e78e:/home# 


docker run -it solana-helloworld:0.0.1

```
$ solana config set --url localhost
$ solana-test-validator
```

```
docker exec -it b531c5e3e78e /bin/bash
```

`npm install`


```
root@b531c5e3e78e:/home/example-helloworld# npm install
npm WARN EBADENGINE Unsupported engine {
npm WARN EBADENGINE   package: 'helloworld@0.0.1',
npm WARN EBADENGINE   required: { node: '>=14.0.0' },
npm WARN EBADENGINE   current: { node: 'v12.22.9', npm: '8.5.1' }
npm WARN EBADENGINE }
npm WARN EBADENGINE Unsupported engine {
npm WARN EBADENGINE   package: 'yaml@2.0.0',
npm WARN EBADENGINE   required: { node: '>= 14' },
npm WARN EBADENGINE   current: { node: 'v12.22.9', npm: '8.5.1' }
npm WARN EBADENGINE }
npm WARN deprecated @types/yaml@1.9.7: This is a stub types definition. yaml provides its own type definitions, so you do not need this installed.

added 296 packages, and audited 297 packages in 23s

30 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
root@b531c5e3e78e:/home/example-helloworld# 
```

Change version to focal

`npm install`

`docker exec -it <blah> /bin/bash`

`npm run build:program-rust`

(takes a while)

`solana program deploy dist/program/helloworld.so`

```
# npm run start

> helloworld@0.0.1 start /home/example-helloworld
> ts-node src/client/main.ts

Let's say hello to a Solana account...
Connection to cluster established: http://localhost:8899 { 'feature-set': 1122441720, 'solana-core': '1.10.8' }
Failed to create keypair from CLI config file, falling back to new random keypair
Using account J3zJfPYzNnpFMwrmTosbYuKuYX2Qw95Ettu9jfz2AKF4 containing 0.00141872 SOL to pay for fees
Error: Failed to read program keypair at '/home/example-helloworld/dist/program/helloworld-keypair.json' due to error: TextEncoder is not defined. Program may need to be deployed with `solana program deploy dist/program/helloworld.so`
    at /home/example-helloworld/src/client/hello_world.ts:143:11
    at Generator.throw (<anonymous>)
    at rejected (/home/example-helloworld/src/client/hello_world.ts:31:65)
npm ERR! code ELIFECYCLE
npm ERR! errno 255
npm ERR! helloworld@0.0.1 start: `ts-node src/client/main.ts`
npm ERR! Exit status 255
npm ERR! 
npm ERR! Failed at the helloworld@0.0.1 start script.
npm ERR! This is probably not a problem with npm. There is likely additional logging output above.

npm ERR! A complete log of this run can be found in:
npm ERR!     /root/.npm/_logs/2022-04-17T19_42_49_757Z-debug.log
root@920ea58c33b0:/home/example-helloworld# 
```

What is this weird shit?

`docker build -t solana-helloworld:0.0.1 .`

`docker run solana-helloworld:0.0.1`

`docker ps` _container id_

`docker exec -it _container id_ /bin/bash`

`cd example-helloworld`

`npm install`

`npm run build:program-rust`

`solana program deploy dist/program/helloworld.so`

`npm run start`

```
# npm run start

> helloworld@0.0.1 start /home/example-helloworld
> ts-node src/client/main.ts

Let's say hello to a Solana account...
Connection to cluster established: http://localhost:8899 { 'feature-set': 1122441720, 'solana-core': '1.10.8' }
Failed to create keypair from CLI config file, falling back to new random keypair
Using account EVfjezL4ufuSVYDZWRoqS3uJgZPqRvkxcaawFvViJXCZ containing 0.00141872 SOL to pay for fees
Error: Failed to read program keypair at '/home/example-helloworld/dist/program/helloworld-keypair.json' due to error: TextEncoder is not defined. Program may need to be deployed with `solana program deploy dist/program/helloworld.so`
    at /home/example-helloworld/src/client/hello_world.ts:143:11
    at Generator.throw (<anonymous>)
    at rejected (/home/example-helloworld/src/client/hello_world.ts:31:65)
```

Stack Overflow <https://github.com/solana-labs/example-helloworld/issues/322>

```
$ docker exec -it 55d5b773ff3e /bin/bash
root@55d5b773ff3e:/home# node --version
v10.19.0
root@55d5b773ff3e:/home# 
```

Here's an article let's see if it works.

https://www.stewright.me/2021/03/install-nodejs-14-on-ubuntu-20-04/

More comprehensive

https://www.digitalocean.com/community/tutorials/how-to-install-node-js-on-ubuntu-20-04

Let's try it in the container first

curl -sL https://deb.nodesource.com/setup_14.x -o /tmp/nodesource_setup.sh

bash /tmp/nodesource_setup.sh

apt-get install -y nodejs

try again

docker build -t solana-helloworld:0.0.1 

```
$ docker images
REPOSITORY          TAG       IMAGE ID       CREATED              SIZE
solana-helloworld   0.0.1     97124a02e8a0   About a minute ago   2.08GB
<none>              <none>    f4ab73a17e8b   46 minutes ago       2.45GB
redis               latest    ef47f3b6dc11   16 months ago        104MB
mongo               latest    3068f6bb852e   16 months ago        493MB
Andrews-MacBook-Pro:solana-docker andrewwhitehouse$ 
```

docker run 971

```
$ docker run 971
Ledger location: test-ledger
Log: test-ledger/validator.log
Initializing...
Waiting for fees to stabilize 1...
Connecting...
Identity: SE8EAYPDMAFqcxL24CaQNMBwZSvLQhe2nrnSrZvbt1j
Genesis Hash: Bf5pC7zaDfSf17ms6C88t5CawybqFMKLukyS6KW6YPpT
Version: 1.10.8
Shred Version: 63851
Gossip Address: 127.0.0.1:1024
TPU Address: 127.0.0.1:1027
JSON RPC URL: http://127.0.0.1:8899
00:00:03 | Processed Slot: 1 | Confirmed Slot: 1 | Finalized Slot: 0 | Full Snapshot Slot: - | Incremental Snapshot Slot: - | Transactions: 0 | ◎500.000000000
00:00:03 | Processed Slot: 1 | Confirmed Slot: 1 | Finalized Slot: 0 | Full Snapshot Slot: - | Incremental Snapshot Slot: - | Transactions: 0 | ◎500.000000000
00:00:03 | Processed Slot: 2 | Confirmed Slot: 2 | Finalized Slot: 0 | Full Snapshot Slot: - | Incremental Snapshot Slot: - | Transactions: 1 | ◎499.999995000
00:00:04 | Processed Slot: 3 | Confirmed Slot: 3 | Finalized Slot: 0 | Full Snapshot Slot: - | Incremental Snapshot Slot: - | Transactions: 2 | ◎499.999990000
00:00:04 | Processed Slot: 3 | Confirmed Slot: 3 | Finalized Slot: 0 | Full Snapshot Slot: - | Incremental Snapshot Slot: - | Transactions: 2 | ◎499.999990000
00:00:04 | Processed Slot: 4 | Confirmed Slot: 4 | Finalized Slot: 0 | Full Snapshot Slot: - | Incremental Snapshot Slot: - | Transactions: 3 | ◎499.999985000
```

```
$ docker ps
CONTAINER ID   IMAGE     COMMAND                  CREATED          STATUS          PORTS      NAMES
f9fe8dad30cd   971       "solana-test-validat…"   54 seconds ago   Up 53 seconds   8899/tcp   determined_perlman
Andrews-MacBook-Pro:solana-docker andrewwhitehouse$ 
```

$ docker exec -it f9f /bin/bash
root@f9fe8dad30cd:/home# 

$ docker exec -it f9f /bin/bash
root@f9fe8dad30cd:/home# cd example-helloworld/
root@f9fe8dad30cd:/home/example-helloworld# node -version
node: bad option: -version
root@f9fe8dad30cd:/home/example-helloworld# node --version
v14.19.1
root@f9fe8dad30cd:/home/example-helloworld# npm install

npm run build:program-rust

```
cargo-build-bpf child: cargo +bpf build --target bpfel-unknown-unknown --release
   Compiling proc-macro2 v1.0.27
   Compiling syn v1.0.73
   Compiling serde_derive v1.0.136
   Compiling serde v1.0.136
error: linker `cc` not found
  |
  = note: No such file or directory (os error 2)

```

Run the commands again

Build / run / bash prompt

npm install

npm run build:program-rust

solana program deploy dist/program/helloworld.so

npm run start


