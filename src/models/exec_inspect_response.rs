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
pub struct ExecInspectResponse {
    #[serde(rename = "CanRemove", skip_serializing_if = "Option::is_none")]
    pub can_remove: Option<bool>,
    #[serde(rename = "DetachKeys", skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    #[serde(rename = "ExitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "ProcessConfig", skip_serializing_if = "Option::is_none")]
    pub process_config: Option<Box<crate::models::ProcessConfig>>,
    #[serde(rename = "OpenStdin", skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    #[serde(rename = "OpenStderr", skip_serializing_if = "Option::is_none")]
    pub open_stderr: Option<bool>,
    #[serde(rename = "OpenStdout", skip_serializing_if = "Option::is_none")]
    pub open_stdout: Option<bool>,
    #[serde(rename = "ContainerID", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// The system process ID for the exec process.
    #[serde(rename = "Pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
}

impl ExecInspectResponse {
    pub fn new() -> ExecInspectResponse {
        ExecInspectResponse {
            can_remove: None,
            detach_keys: None,
            ID: None,
            running: None,
            exit_code: None,
            process_config: None,
            open_stdin: None,
            open_stderr: None,
            open_stdout: None,
            container_id: None,
            pid: None,
        }
    }
}


