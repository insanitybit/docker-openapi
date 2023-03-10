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
pub struct ContainerInspectResponse {
    /// The ID of the container
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The time the container was created
    #[serde(rename = "Created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The path to the command being run
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The arguments to the command being run
    #[serde(rename = "Args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<crate::models::ContainerState>>,
    /// The container's image ID
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "ResolvConfPath", skip_serializing_if = "Option::is_none")]
    pub resolv_conf_path: Option<String>,
    #[serde(rename = "HostnamePath", skip_serializing_if = "Option::is_none")]
    pub hostname_path: Option<String>,
    #[serde(rename = "HostsPath", skip_serializing_if = "Option::is_none")]
    pub hosts_path: Option<String>,
    #[serde(rename = "LogPath", skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RestartCount", skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "MountLabel", skip_serializing_if = "Option::is_none")]
    pub mount_label: Option<String>,
    #[serde(rename = "ProcessLabel", skip_serializing_if = "Option::is_none")]
    pub process_label: Option<String>,
    #[serde(rename = "AppArmorProfile", skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<String>,
    /// IDs of exec instances that are running in the container.
    #[serde(rename = "ExecIDs", skip_serializing_if = "Option::is_none")]
    pub exec_ids: Option<Vec<String>>,
    #[serde(rename = "HostConfig", skip_serializing_if = "Option::is_none")]
    pub host_config: Option<Box<crate::models::HostConfig>>,
    #[serde(rename = "GraphDriver", skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<Box<crate::models::GraphDriverData>>,
    /// The size of files that have been created or changed by this container. 
    #[serde(rename = "SizeRw", skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<i64>,
    /// The total size of all the files in this container.
    #[serde(rename = "SizeRootFs", skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<i64>,
    #[serde(rename = "Mounts", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<crate::models::MountPoint>>,
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::ContainerConfig>>,
    #[serde(rename = "NetworkSettings", skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<Box<crate::models::NetworkSettings>>,
}

impl ContainerInspectResponse {
    pub fn new() -> ContainerInspectResponse {
        ContainerInspectResponse {
            id: None,
            created: None,
            path: None,
            args: None,
            state: None,
            image: None,
            resolv_conf_path: None,
            hostname_path: None,
            hosts_path: None,
            log_path: None,
            name: None,
            restart_count: None,
            driver: None,
            platform: None,
            mount_label: None,
            process_label: None,
            app_armor_profile: None,
            exec_ids: None,
            host_config: None,
            graph_driver: None,
            size_rw: None,
            size_root_fs: None,
            mounts: None,
            config: None,
            network_settings: None,
        }
    }
}


