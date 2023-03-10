/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.41) is used. For example, calling `/info` is the same as calling `/v1.41/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a [base64url encoded](https://tools.ietf.org/html/rfc4648#section-5) (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ``` 
 *
 * The version of the OpenAPI document: 1.41
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PluginConfig : The config of a plugin.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Docker Version used to create the plugin
    #[serde(rename = "DockerVersion", skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Documentation")]
    pub documentation: String,
    #[serde(rename = "Interface")]
    pub interface: Box<crate::models::PluginConfigInterface>,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Vec<String>,
    #[serde(rename = "WorkDir")]
    pub work_dir: String,
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::PluginConfigUser>>,
    #[serde(rename = "Network")]
    pub network: Box<crate::models::PluginConfigNetwork>,
    #[serde(rename = "Linux")]
    pub linux: Box<crate::models::PluginConfigLinux>,
    #[serde(rename = "PropagatedMount")]
    pub propagated_mount: String,
    #[serde(rename = "IpcHost")]
    pub ipc_host: bool,
    #[serde(rename = "PidHost")]
    pub pid_host: bool,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<crate::models::PluginMount>,
    #[serde(rename = "Env")]
    pub env: Vec<crate::models::PluginEnv>,
    #[serde(rename = "Args")]
    pub args: Box<crate::models::PluginConfigArgs>,
    #[serde(rename = "rootfs", skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<Box<crate::models::PluginConfigRootfs>>,
}

impl PluginConfig {
    /// The config of a plugin.
    pub fn new(description: String, documentation: String, interface: crate::models::PluginConfigInterface, entrypoint: Vec<String>, work_dir: String, network: crate::models::PluginConfigNetwork, linux: crate::models::PluginConfigLinux, propagated_mount: String, ipc_host: bool, pid_host: bool, mounts: Vec<crate::models::PluginMount>, env: Vec<crate::models::PluginEnv>, args: crate::models::PluginConfigArgs) -> PluginConfig {
        PluginConfig {
            docker_version: None,
            description,
            documentation,
            interface: Box::new(interface),
            entrypoint,
            work_dir,
            user: None,
            network: Box::new(network),
            linux: Box::new(linux),
            propagated_mount,
            ipc_host,
            pid_host,
            mounts,
            env,
            args: Box::new(args),
            rootfs: None,
        }
    }
}


