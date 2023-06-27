# Cedar Authorization Service

## Big picture behind
This project was created as a part of a series of blog posts (this particular [one](https://dev.to/aws-builders/authorization-and-cedar-a-new-way-to-manage-permissions-part-ii-2bgb) aimed at learning the Cedar policy language. It's a simple Rust server that evaluates Cedar policies and makes authorization decisions based on them. It's designed to be an additional resource for learning Cedar, complementing the official Cedar playground.

Please note that this is not an official Cedar playground. The official Cedar playground can be found [here](https://www.cedarpolicy.com/en/tutorial). The official playground is in a view-only mode for the Rust code, which means you can't edit the Rust code there. That's why I've decided to create this project: to provide a playground where I can play around with both the Cedar policies and the Rust code.

There's also another open-source project called [Cedar Agent](https://github.com/permitio/cedar-agent) that you might find useful. Cedar Agent is a service that evaluates Cedar policies.

## What it is?

This Rust script is a simple HTTP server. The server has two main endpoints: `/health` and `/evaluate`. The `/health` endpoint is a simple health check that lets you know the server is running correctly.

The real action happens at the `/evaluate` endpoint. Here, you can send a POST request with a JSON payload containing the Cedar policies you want to evaluate, along with the attributes of the request you're checking. These attributes include the principal (the entity making the request), the action (what the principal is trying to do), and the resource (the object the action is being performed on). You can also optionally include a context, which is additional information that can be used in the decision.

Once the server receives your request, it gets to work evaluating the policies. It takes the Cedar policies and the attributes you've provided, and uses them to create an authorization request. This request is then evaluated against the policies. The server uses Cedar's policy language to make a fine-grained, attribute-based authorization decision.

Finally, the server returns the decision. If the policies permit the action for the given principal, resource, and context, the server will return a message saying "Authorization decision: allow.". If the policies deny the action, it will return a message saying "Authorization decision: deny.".

 Keep in mind that this is a dummy authorization service, and it should not be used in production environments. 
 
## Getting Started
This is a Rust server, so you'll need to have Rust and Cargo (the Rust package manager) installed on your machine to run it. If you don't have Rust and Cargo installed, you can download them from the [official Rust website](https://www.rust-lang.org/).

Once you have Rust and Cargo installed, you can run the server by navigating to the project directory in your terminal and typing `cargo run`. This will start the server, and you can then send HTTP requests to it to evaluate your Cedar policies.

The endpoint for evaluation of the policies is `http://localhost:8080/evaluate`. Make sure that the server is running.

## Examples

You can refer to the post here for explanations or you can use the examples below:

### With Permit Policy

```bash
curl -X POST -H "Content-Type: application/json" -d '{
    "policies": ["permit(principal == UserType::\"Customer\", action == Action::\"View\", resource == Resource::\"Product\");"],
    "resources": ["Resource::\"Product\""],
    "action": "Action::\"View\"",
    "principal": "UserType::\"Customer\"",
    "context": {}
}' "http://localhost:8080/evaluate"
```

### With Forbid Policy
```bash
curl -X POST -H "Content-Type: application/json" -d '{
    "policies": ["forbid(principal == UserType::\"Customer\", action == Action::\"Edit\", resource == Resource::\"Product\");"],
    "resources": ["Resource::\"Product\""],
    "action": "Action::\"Edit\"",
    "principal": "UserType::\"Customer\"",
    "context": {}
}' "http://localhost:8080/evaluate"
```

### With Context

```bash
curl -X POST -H "Content-Type: application/json" -d '{"policies": ["permit(principal == UserType::\"Customer\", action == Action::\"Purchase\", resource == Resource::\"Product\") when {context.membership == \"Premium\"};"],
    "resources":["Resource::\"Product\""],
    "action": "Action::\"Purchase\"",
    "principal": "UserType::\"Customer\"",
    "context": {"membership": "Premium"}
}' "http://localhost:8080/evaluate"
{"message":"Authorization decision: allow."}%
```

### With Multiple Policies

```bash
curl -X POST -H "Content-Type: application/json" -d '{
    "policies": [
        "permit(principal == UserType::\"Customer\", action == Action::\"View\", resource == Resource::\"Product\");",
        "forbid(principal == UserType::\"Customer\", action == Action::\"Edit\", resource == Resource::\"Product\");"
    ],
    "resources": ["Resource::\"Product\""],
    "action": "Action::\"Edit\"",
    "principal": "UserType::\"Customer\"",
    "context": {}
}' "http://localhost:8080/evaluate"
```
