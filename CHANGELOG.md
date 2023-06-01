# Changelog

## 0.7.0-rc.1

This update refactors all of the clients and changes the way responses are handled. Previously methods that make calls to a third party service would return an `anyhow::Result<T>` over some structured interpretation of the response body (i.e. a deserialized struct, a unit type, or a raw string). Methods now return a `Result<Response<T>, ClientError>` where a `Response` contains the body type that was previously returned along with the status code and headers that were sent from the third party service. `ClientError` now provides a more structure error type than the previous `anyhow` error.

### Breaking Changes

* Methods now return `Result<Response<T>, ClientError>` instead of `anyhow::Result<T>`. The body can be accessed via `response.body`.
* Structured errors are now returned instead of `anyhow::Error`.
* Redirects are no longer followed by default. Previously if a 3xx response code was returned by a service, the inner client would follow it. This broke the ability for a service to specify a meaningful 3xx response. Methods that receive 3xx responses will now return them immediately and not attempt to resolve them.