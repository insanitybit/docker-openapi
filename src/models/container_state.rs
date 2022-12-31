/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.41) is used. For example, calling `/info` is the same as calling `/v1.41/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a [base64url encoded](https://tools.ietf.org/html/rfc4648#section-5) (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ``` 
 *
 * The version of the OpenAPI document: 1.41
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ContainerState : ContainerState stores container's running state. It's part of ContainerJSONBase and will be returned by the \"inspect\" command. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ContainerState {
    /// String representation of the container state. Can be one of \"created\", \"running\", \"paused\", \"restarting\", \"removing\", \"exited\", or \"dead\". 
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Whether this container is running.  Note that a running container can be _paused_. The `Running` and `Paused` booleans are not mutually exclusive:  When pausing a container (on Linux), the freezer cgroup is used to suspend all processes in the container. Freezing the process requires the process to be running. As a result, paused containers are both `Running` _and_ `Paused`.  Use the `Status` field instead to determine if a container's state is \"running\". 
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    /// Whether this container is paused.
    #[serde(rename = "Paused", skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// Whether this container is restarting.
    #[serde(rename = "Restarting", skip_serializing_if = "Option::is_none")]
    pub restarting: Option<bool>,
    /// Whether this container has been killed because it ran out of memory. 
    #[serde(rename = "OOMKilled", skip_serializing_if = "Option::is_none")]
    pub oom_killed: Option<bool>,
    #[serde(rename = "Dead", skip_serializing_if = "Option::is_none")]
    pub dead: Option<bool>,
    /// The process ID of this container
    #[serde(rename = "Pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    /// The last exit code of this container
    #[serde(rename = "ExitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The time when this container was last started.
    #[serde(rename = "StartedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// The time when this container last exited.
    #[serde(rename = "FinishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(rename = "Health", skip_serializing_if = "Option::is_none")]
    pub health: Option<Box<crate::models::Health>>,
}

impl ContainerState {
    /// ContainerState stores container's running state. It's part of ContainerJSONBase and will be returned by the \"inspect\" command. 
    pub fn new() -> ContainerState {
        ContainerState {
            status: None,
            running: None,
            paused: None,
            restarting: None,
            oom_killed: None,
            dead: None,
            pid: None,
            exit_code: None,
            error: None,
            started_at: None,
            finished_at: None,
            health: None,
        }
    }
}

/// String representation of the container state. Can be one of \"created\", \"running\", \"paused\", \"restarting\", \"removing\", \"exited\", or \"dead\". 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "restarting")]
    Restarting,
    #[serde(rename = "removing")]
    Removing,
    #[serde(rename = "exited")]
    Exited,
    #[serde(rename = "dead")]
    Dead,
}

impl Default for Status {
    fn default() -> Status {
        Self::Created
    }
}
