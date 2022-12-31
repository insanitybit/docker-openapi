# docker-openapi
Models generated from the docker openapi spec, with minor modifications

OpenAPI generation for rust crates is far less than ideal, therefor some changes have been made:

1. The client has been removed. This crate *only* provides the data models.

2. Currently some fields aren't being generated with an idiomatic name. In the future I will change that
   so that they use appropriate casing. That will be a breaking change.

3. Some dependencies are removed or updated.

Over time I hope to have a stable build generation phase for the models. I may consider adding the
client at some point but it is so far from idiomatic at this point that it's not worth it - I'll be
manually implementing a client in another crate instead.
