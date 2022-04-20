**Code Deployment**

There was a time when we deployed code to a machine running under the desk. 

When I worked for a payment systems company there was a single server powering the ATMs.

And then we started to move to data centres. And commodity hardware. And the Cloud.

So writing the code solution is part of what we need to deliver. We also need to make sure it is deployed reliably to somewhere that our users can access it.

There are various approaches to doing this, some more friendly than others. The discipline is known as DevOps.

A few years ago one one of our projects we experimented with writing a code wrapper around Amazon's Cloud SDK. Using a typed language like Java is very helpful because it gives you hints on the kinds of things you can do with a particular AWS resource, and the kind of parameters it takes. Understanding the data structures is key to understanding the myriad ways you can configure these environments. And the language wrapper helps.

Now Amazon has released the [CDK](https://aws.amazon.com/cdk/) which does the same thing.

I've been trying this out today. Here's the [tutorial](https://docs.aws.amazon.com/cdk/v2/guide/hello_world.html) which covers instructions for TypeScript, JavaScript, C#, Java and Python. The CDK also supports Go, and F#.

I followed the Java example.

`mkdir hello-cdk`

`cd hello-cdk`

`cdk init app --language java`
  
Check that the local code compiles.

`mvn compile -q` 

What stacks are currently deployed on Amazon for our account and region? (I previous ran `cdk bootstrap` to set this up.)
 
`cdk ls`
 
Update the local Java code to change the stack (this code maps to AWS Cloudformation scripts).

`cdk deploy`

Here's what the code looks like:

The "stack":

```
public class HelloCdkStack extends Stack {
    public HelloCdkStack(final App scope, final String id) {
        this(scope, id, null);
    }

    public HelloCdkStack(final App scope, final String id, final StackProps props) {
        super(scope, id, props);

        final Function hello = Function.Builder.create(this, "HelloHandler")
                .runtime(Runtime.NODEJS_14_X)
                .code(Code.fromAsset("lambda"))
                .handler("hello.handler")
                .build();

        LambdaRestApi.Builder.create(this, "Endpoint")
                .handler(hello)
                .build();
    }
}
```

The Lambda function (in JavaScript):

```
exports.handler = async function(event) {
    console.log("request:", JSON.stringify(event, undefined, 2));
    return {
        statusCode: 200,
        headers: { "Content-Type": "text/plain" },
        body: `Hello CDK! You've hit ${event.path}\n`
    };
}
```

The end point of the initial tutorial is to call the function through the AWS console.

![Screenshot 2022-04-20 at 17.57.36|690x388](upload://yyDaqVYCEmmnlfI0ftLYfnDi9Pd.png)

But I would like to be able to call it locally, which is where the [CDK Workshop](https://cdkworkshop.com/50-java/30-hello-cdk.html) comes in handy, with instructions on adding an API gateway which allows the lambda function to be accessed publicly.

And here's what we see:

```
. . .

[50%] success: Published b767421a09c8db032cc69f77f765adb72f5f238a4d8cd0cd433f58c686fabbc4:current_account-current_region
[100%] success: Published a53201b2fd5a6fb42ede8ef544e80350ca83c8f5e70b95d6fe431807a5bf0075:current_account-current_region
HelloCdkStack: creating CloudFormation changeset...

 ✅  HelloCdkStack

✨  Deployment time: 59.39s

Outputs:
HelloCdkStack.Endpoint8024A810 = https://zod7g2ow9i.execute-api.eu-west-2.amazonaws.com/prod/
Stack ARN:
arn:aws:cloudformation:eu-west-2:462920085534:stack/HelloCdkStack/2b1dd3d0-c0c8-11ec-b4fa-0aa6d6047c30

✨  Total time: 110.15s
```

```
$ curl https://zod7g2ow9i.execute-api.eu-west-2.amazonaws.com/prod/
Hello CDK! You've hit /
$ 
```

:tada: :tada: 

The thing about Java functions is that have a significant startup time, so I think it's interesting to keep exploring what the Rust equivalent would look like. (Maybe that's the comparison ... JavaScript vs Rust, or Java vs Rust? :thinking: )

I found some [instructions](https://hub.qovery.com/guides/tutorial/how-to-deploy-a-rust-rest-api-application-on-aws-with-ease/) on deploying Rust functions via Docker so that's going to be a hook for future investigation.

@Beaver 







