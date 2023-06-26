# Cedar Authorization Service

## Big picture behind
This project was created as a part of a series of blog posts (this particular one) aimed at learning the Cedar policy language. It's a simple Rust server that evaluates Cedar policies and makes authorization decisions based on them. It's designed to be an additional resource for learning Cedar, complementing the official Cedar playground.

Please note that this is not an official Cedar playground. The official Cedar playground can be found [here](https://www.cedarpolicy.com/en/tutorial). The official playground is in a view-only mode for the Rust code, which means you can't edit the Rust code there. That's why I've decided to create this project: to provide a playground where I can play around with both the Cedar policies and the Rust code.

There's also another open-source project called [Cedar Agent](https://github.com/permitio/cedar-agent) that you might find useful. Cedar Agent is a service that evaluates Cedar policies.

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
