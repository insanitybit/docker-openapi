/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.41) is used. For example, calling `/info` is the same as calling `/v1.41/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a [base64url encoded](https://tools.ietf.org/html/rfc4648#section-5) (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ``` 
 *
 * The version of the OpenAPI document: 1.41
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExecConfig {
    /// Attach to `stdin` of the exec command.
    #[serde(rename = "AttachStdin", skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Attach to `stdout` of the exec command.
    #[serde(rename = "AttachStdout", skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Attach to `stderr` of the exec command.
    #[serde(rename = "AttachStderr", skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// Override the key sequence for detaching a container. Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`. 
    #[serde(rename = "DetachKeys", skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    /// Allocate a pseudo-TTY.
    #[serde(rename = "Tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// A list of environment variables in the form `[\"VAR=value\", ...]`. 
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Command to run, as a string or array of strings.
    #[serde(rename = "Cmd", skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// Runs the exec process with extended privileges.
    #[serde(rename = "Privileged", skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// The user, and optionally, group to run the exec process inside the container. Format is one of: `user`, `user:group`, `uid`, or `uid:gid`. 
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The working directory for the exec process inside the container. 
    #[serde(rename = "WorkingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

impl ExecConfig {
    pub fn new() -> ExecConfig {
        ExecConfig {
            attach_stdin: None,
            attach_stdout: None,
            attach_stderr: None,
            detach_keys: None,
            tty: None,
            env: None,
            cmd: None,
            privileged: None,
            user: None,
            working_dir: None,
        }
    }
}


