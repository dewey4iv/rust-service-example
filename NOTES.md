# Service

The goal of this crate is to play with ways to make composable apis a little easier to work with.

### Goals

- Services should be composable (like Tower) to support middlewares
- We should try to avoid "God Objects" that need to know about all the different services. This makes it hard to test. Option<Service> isn't really better.
- We should be able to separate concerns and support a version of: HTTP -> Kernel -> UseCases -> Repository -> Database
- Services and middlewares should be able to be stateful (think KV cache in-memory)

### Ideas

- [ ] Tower like Servce
- [ ] Axum function handlers
