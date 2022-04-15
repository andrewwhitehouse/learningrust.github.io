https://dev.to/nicholaschiasson/beginner-s-guide-to-running-rust-on-aws-lambda-277n

## Setup

Ensure [CDK](https://aws.amazon.com/cdk/) is installed.

```
$ cdk --version
2.8.0 (build 8a5eb49)
$ 
```

And [AWS CLI](https://aws.amazon.com/cli/).

```
$ aws --version
aws-cli/2.0.8 Python/3.8.5 Darwin/18.7.0 botocore/2.0.0dev12
$ 
```

And [NodeJs](https://nodejs.org/en/).

I'm using the [Node Version Manager](https://github.com/nvm-sh/nvm/blob/master/README.md).

```
$ nvm use v14.17.5
Now using node v14.17.5 (npm v6.14.14)
$
```

And [Rust](https://www.rust-lang.org/).

```
mkdir tutorial-rust-lambda
cd tutorial-rust-lambda
```

Initialize the project

```
cdk init app --language typescript
```

Save the clean project

```
git init
git add .
git commit -m "Initial commit."
```



