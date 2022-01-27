Notes from https://dev.to/nicholaschiasson/beginner-s-guide-to-running-rust-on-aws-lambda-277n

## Prerequisites

```
Andrews-MacBook-Pro:rust-book andrewwhitehouse$ aws --version
aws-cli/2.0.8 Python/3.8.5 Darwin/18.7.0 botocore/2.0.0dev12
Andrews-MacBook-Pro:rust-book andrewwhitehouse$ 
```

```
Andrews-MacBook-Pro:rust-book andrewwhitehouse$ nvm use v14.17.5
Now using node v14.17.5 (npm v6.14.14)
Andrews-MacBook-Pro:rust-book andrewwhitehouse$ node --version
v14.17.5
Andrews-MacBook-Pro:rust-book andrewwhitehouse$ 
```

`npm i -g aws-cdk`

```
Andrews-MacBook-Pro:rust-book andrewwhitehouse$ cdk --version
2.8.0 (build 8a5eb49)
Andrews-MacBook-Pro:rust-book andrewwhitehouse$
```

```
Andrews-MacBook-Pro:rust-book andrewwhitehouse$ rustc --version
rustc 1.57.0 (f1edd0429 2021-11-29)
Andrews-MacBook-Pro:rust-book andrewwhitehouse$
```

## Creating a Fresh CDK Project

```
Andrews-MacBook-Pro:~ andrewwhitehouse$ cd code
Andrews-MacBook-Pro:code andrewwhitehouse$ mkdir tutorial-rust-lambda
Andrews-MacBook-Pro:code andrewwhitehouse$ cd tutorial-rust-lambda/
Andrews-MacBook-Pro:tutorial-rust-lambda andrewwhitehouse$
```

```
cdk init app --language typescript
```

```
git init
git add .
git commit -m "Initial commit"
```

Update tsconfig.json.

`git diff`

```
@@ -21,10 +21,12 @@
     "strictPropertyInitialization": false,
     "typeRoots": [
       "./node_modules/@types"
-    ]
+    ],
+    "outDir": "dist"
   },
   "exclude": [
     "node_modules",
-    "cdk.out"
+    "cdk.out",
+    "dist"
   ]
 }
 ```

```
cdk bootstrap aws://462920085534/eu-west-2
```

## Writing a Lambda Function in Rust

```
cargo new lambda/hello
```

Add dependencies to lambda/hello/Cargo.toml

```
lambda_runtime = "0.3.0"
log = "0.4.14"
serde = "1.0.126"
simple_logger = "1.11.0"
tokio = "1.6.1"
```

```
pushd lambda/hello
```
Edit lambda/hello/src/main.rs

In Intellij, File > New Project from Existing Sources > select Cargo project

Typed in code. Fixed some typos.

```
cargo build
```

```
popd
```

```
npm i @aws-cdk/aws-lambda@1.107.0
```

edit `lib/tutorial-rust-lambda-stack.ts`

