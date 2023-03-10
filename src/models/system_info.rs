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
pub struct SystemInfo {
    /// Unique identifier of the daemon.  <p><br /></p>  > **Note**: The format of the ID itself is not part of the API, and > should not be considered stable. 
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    /// Total number of containers on the host.
    #[serde(rename = "Containers", skip_serializing_if = "Option::is_none")]
    pub containers: Option<i32>,
    /// Number of containers with status `\"running\"`. 
    #[serde(rename = "ContainersRunning", skip_serializing_if = "Option::is_none")]
    pub containers_running: Option<i32>,
    /// Number of containers with status `\"paused\"`. 
    #[serde(rename = "ContainersPaused", skip_serializing_if = "Option::is_none")]
    pub containers_paused: Option<i32>,
    /// Number of containers with status `\"stopped\"`. 
    #[serde(rename = "ContainersStopped", skip_serializing_if = "Option::is_none")]
    pub containers_stopped: Option<i32>,
    /// Total number of images on the host.  Both _tagged_ and _untagged_ (dangling) images are counted. 
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<i32>,
    /// Name of the storage driver in use.
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Information specific to the storage driver, provided as \"label\" / \"value\" pairs.  This information is provided by the storage driver, and formatted in a way consistent with the output of `docker info` on the command line.  <p><br /></p>  > **Note**: The information returned in this field, including the > formatting of values and labels, should not be considered stable, > and may change without notice. 
    #[serde(rename = "DriverStatus", skip_serializing_if = "Option::is_none")]
    pub driver_status: Option<Vec<Vec<String>>>,
    /// Root directory of persistent Docker state.  Defaults to `/var/lib/docker` on Linux, and `C:\\ProgramData\\docker` on Windows. 
    #[serde(rename = "DockerRootDir", skip_serializing_if = "Option::is_none")]
    pub docker_root_dir: Option<String>,
    #[serde(rename = "Plugins", skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Box<crate::models::PluginsInfo>>,
    /// Indicates if the host has memory limit support enabled.
    #[serde(rename = "MemoryLimit", skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<bool>,
    /// Indicates if the host has memory swap limit support enabled.
    #[serde(rename = "SwapLimit", skip_serializing_if = "Option::is_none")]
    pub swap_limit: Option<bool>,
    /// Indicates if the host has kernel memory limit support enabled.  <p><br /></p>  > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated > `kmem.limit_in_bytes`. 
    #[serde(rename = "KernelMemory", skip_serializing_if = "Option::is_none")]
    pub kernel_memory: Option<bool>,
    /// Indicates if the host has kernel memory TCP limit support enabled.  Kernel memory TCP limits are not supported when using cgroups v2, which does not support the corresponding `memory.kmem.tcp.limit_in_bytes` cgroup. 
    #[serde(rename = "KernelMemoryTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<bool>,
    /// Indicates if CPU CFS(Completely Fair Scheduler) period is supported by the host. 
    #[serde(rename = "CpuCfsPeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_period: Option<bool>,
    /// Indicates if CPU CFS(Completely Fair Scheduler) quota is supported by the host. 
    #[serde(rename = "CpuCfsQuota", skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_quota: Option<bool>,
    /// Indicates if CPU Shares limiting is supported by the host. 
    #[serde(rename = "CPUShares", skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<bool>,
    /// Indicates if CPUsets (cpuset.cpus, cpuset.mems) are supported by the host.  See [cpuset(7)](https://www.kernel.org/doc/Documentation/cgroup-v1/cpusets.txt) 
    #[serde(rename = "CPUSet", skip_serializing_if = "Option::is_none")]
    pub cpu_set: Option<bool>,
    /// Indicates if the host kernel has PID limit support enabled.
    #[serde(rename = "PidsLimit", skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<bool>,
    /// Indicates if OOM killer disable is supported on the host.
    #[serde(rename = "OomKillDisable", skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Indicates IPv4 forwarding is enabled.
    #[serde(rename = "IPv4Forwarding", skip_serializing_if = "Option::is_none")]
    pub ipv4_forwarding: Option<bool>,
    /// Indicates if `bridge-nf-call-iptables` is available on the host.
    #[serde(rename = "BridgeNfIptables", skip_serializing_if = "Option::is_none")]
    pub bridge_nf_iptables: Option<bool>,
    /// Indicates if `bridge-nf-call-ip6tables` is available on the host.
    #[serde(rename = "BridgeNfIp6tables", skip_serializing_if = "Option::is_none")]
    pub bridge_nf_ip6tables: Option<bool>,
    /// Indicates if the daemon is running in debug-mode / with debug-level logging enabled. 
    #[serde(rename = "Debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    /// The total number of file Descriptors in use by the daemon process.  This information is only returned if debug-mode is enabled. 
    #[serde(rename = "NFd", skip_serializing_if = "Option::is_none")]
    pub nfd: Option<i32>,
    /// The  number of goroutines that currently exist.  This information is only returned if debug-mode is enabled. 
    #[serde(rename = "NGoroutines", skip_serializing_if = "Option::is_none")]
    pub n_goroutines: Option<i32>,
    /// Current system-time in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds. 
    #[serde(rename = "SystemTime", skip_serializing_if = "Option::is_none")]
    pub system_time: Option<String>,
    /// The logging driver to use as a default for new containers. 
    #[serde(rename = "LoggingDriver", skip_serializing_if = "Option::is_none")]
    pub logging_driver: Option<String>,
    /// The driver to use for managing cgroups. 
    #[serde(rename = "CgroupDriver", skip_serializing_if = "Option::is_none")]
    pub cgroup_driver: Option<CgroupDriver>,
    /// The version of the cgroup. 
    #[serde(rename = "CgroupVersion", skip_serializing_if = "Option::is_none")]
    pub cgroup_version: Option<CgroupVersion>,
    /// Number of event listeners subscribed.
    #[serde(rename = "NEventsListener", skip_serializing_if = "Option::is_none")]
    pub n_events_listener: Option<i32>,
    /// Kernel version of the host.  On Linux, this information obtained from `uname`. On Windows this information is queried from the <kbd>HKEY_LOCAL_MACHINE\\\\SOFTWARE\\\\Microsoft\\\\Windows NT\\\\CurrentVersion\\\\</kbd> registry value, for example _\"10.0 14393 (14393.1198.amd64fre.rs1_release_sec.170427-1353)\"_. 
    #[serde(rename = "KernelVersion", skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// Name of the host's operating system, for example: \"Ubuntu 16.04.2 LTS\" or \"Windows Server 2016 Datacenter\" 
    #[serde(rename = "OperatingSystem", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// Version of the host's operating system  <p><br /></p>  > **Note**: The information returned in this field, including its > very existence, and the formatting of values, should not be considered > stable, and may change without notice. 
    #[serde(rename = "OSVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Generic type of the operating system of the host, as returned by the Go runtime (`GOOS`).  Currently returned values are \"linux\" and \"windows\". A full list of possible values can be found in the [Go documentation](https://golang.org/doc/install/source#environment). 
    #[serde(rename = "OSType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Hardware architecture of the host, as returned by the Go runtime (`GOARCH`).  A full list of possible values can be found in the [Go documentation](https://golang.org/doc/install/source#environment). 
    #[serde(rename = "Architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The number of logical CPUs usable by the daemon.  The number of available CPUs is checked by querying the operating system when the daemon starts. Changes to operating system CPU allocation after the daemon is started are not reflected. 
    #[serde(rename = "NCPU", skip_serializing_if = "Option::is_none")]
    pub NCPU: Option<i32>,
    /// Total amount of physical memory available on the host, in bytes. 
    #[serde(rename = "MemTotal", skip_serializing_if = "Option::is_none")]
    pub mem_total: Option<i64>,
    /// Address / URL of the index server that is used for image search, and as a default for user authentication for Docker Hub and Docker Cloud. 
    #[serde(rename = "IndexServerAddress", skip_serializing_if = "Option::is_none")]
    pub index_server_address: Option<String>,
    #[serde(rename = "RegistryConfig", skip_serializing_if = "Option::is_none")]
    pub registry_config: Option<Box<crate::models::RegistryServiceConfig>>,
    /// User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, `GPU=UUID1`). 
    #[serde(rename = "GenericResources", skip_serializing_if = "Option::is_none")]
    pub generic_resources: Option<Vec<serde_json::Value>>,
    /// HTTP-proxy configured for the daemon. This value is obtained from the [`HTTP_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration. 
    #[serde(rename = "HttpProxy", skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<String>,
    /// HTTPS-proxy configured for the daemon. This value is obtained from the [`HTTPS_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration. 
    #[serde(rename = "HttpsProxy", skip_serializing_if = "Option::is_none")]
    pub https_proxy: Option<String>,
    /// Comma-separated list of domain extensions for which no proxy should be used. This value is obtained from the [`NO_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable.  Containers do not automatically inherit this configuration. 
    #[serde(rename = "NoProxy", skip_serializing_if = "Option::is_none")]
    pub no_proxy: Option<String>,
    /// Hostname of the host.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined labels (key/value metadata) as set on the daemon.  <p><br /></p>  > **Note**: When part of a Swarm, nodes can both have _daemon_ labels, > set through the daemon configuration, and _node_ labels, set from a > manager node in the Swarm. Node labels are not included in this > field. Node labels can be retrieved using the `/nodes/(id)` endpoint > on a manager node in the Swarm. 
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Indicates if experimental features are enabled on the daemon. 
    #[serde(rename = "ExperimentalBuild", skip_serializing_if = "Option::is_none")]
    pub experimental_build: Option<bool>,
    /// Version string of the daemon.  > **Note**: the [standalone Swarm API](/swarm/swarm-api/) > returns the Swarm version instead of the daemon  version, for example > `swarm/1.2.8`. 
    #[serde(rename = "ServerVersion", skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    /// URL of the distributed storage backend.   The storage backend is used for multihost networking (to store network and endpoint information) and by the node discovery mechanism.  <p><br /></p>  > **Deprecated**: This field is only propagated when using standalone Swarm > mode, and overlay networking using an external k/v store. Overlay > networks with Swarm mode enabled use the built-in raft store, and > this field will be empty. 
    #[serde(rename = "ClusterStore", skip_serializing_if = "Option::is_none")]
    pub cluster_store: Option<String>,
    /// The network endpoint that the Engine advertises for the purpose of node discovery. ClusterAdvertise is a `host:port` combination on which the daemon is reachable by other hosts.  <p><br /></p>  > **Deprecated**: This field is only propagated when using standalone Swarm > mode, and overlay networking using an external k/v store. Overlay > networks with Swarm mode enabled use the built-in raft store, and > this field will be empty. 
    #[serde(rename = "ClusterAdvertise", skip_serializing_if = "Option::is_none")]
    pub cluster_advertise: Option<String>,
    /// List of [OCI compliant](https://github.com/opencontainers/runtime-spec) runtimes configured on the daemon. Keys hold the \"name\" used to reference the runtime.  The Docker daemon relies on an OCI compliant runtime (invoked via the `containerd` daemon) as its interface to the Linux kernel namespaces, cgroups, and SELinux.  The default runtime is `runc`, and automatically configured. Additional runtimes can be configured by the user and will be listed here. 
    #[serde(rename = "Runtimes", skip_serializing_if = "Option::is_none")]
    pub runtimes: Option<::std::collections::HashMap<String, crate::models::Runtime>>,
    /// Name of the default OCI runtime that is used when starting containers.  The default can be overridden per-container at create time. 
    #[serde(rename = "DefaultRuntime", skip_serializing_if = "Option::is_none")]
    pub default_runtime: Option<String>,
    #[serde(rename = "Swarm", skip_serializing_if = "Option::is_none")]
    pub swarm: Option<Box<crate::models::SwarmInfo>>,
    /// Indicates if live restore is enabled.  If enabled, containers are kept running when the daemon is shutdown or upon daemon start if running containers are detected. 
    #[serde(rename = "LiveRestoreEnabled", skip_serializing_if = "Option::is_none")]
    pub live_restore_enabled: Option<bool>,
    /// Represents the isolation technology to use as a default for containers. The supported values are platform-specific.  If no isolation value is specified on daemon start, on Windows client, the default is `hyperv`, and on Windows server, the default is `process`.  This option is currently not used on other platforms. 
    #[serde(rename = "Isolation", skip_serializing_if = "Option::is_none")]
    pub isolation: Option<Isolation>,
    /// Name and, optional, path of the `docker-init` binary.  If the path is omitted, the daemon searches the host's `$PATH` for the binary and uses the first result. 
    #[serde(rename = "InitBinary", skip_serializing_if = "Option::is_none")]
    pub init_binary: Option<String>,
    #[serde(rename = "ContainerdCommit", skip_serializing_if = "Option::is_none")]
    pub containerd_commit: Option<Box<crate::models::Commit>>,
    #[serde(rename = "RuncCommit", skip_serializing_if = "Option::is_none")]
    pub runc_commit: Option<Box<crate::models::Commit>>,
    #[serde(rename = "InitCommit", skip_serializing_if = "Option::is_none")]
    pub init_commit: Option<Box<crate::models::Commit>>,
    /// List of security features that are enabled on the daemon, such as apparmor, seccomp, SELinux, user-namespaces (userns), and rootless.  Additional configuration options for each security feature may be present, and are included as a comma-separated list of key/value pairs. 
    #[serde(rename = "SecurityOptions", skip_serializing_if = "Option::is_none")]
    pub security_options: Option<Vec<String>>,
    /// Reports a summary of the product license on the daemon.  If a commercial license has been applied to the daemon, information such as number of nodes, and expiration are included. 
    #[serde(rename = "ProductLicense", skip_serializing_if = "Option::is_none")]
    pub product_license: Option<String>,
    /// List of custom default address pools for local networks, which can be specified in the daemon.json file or dockerd option.  Example: a Base \"10.10.0.0/16\" with Size 24 will define the set of 256 10.10.[0-255].0/24 address pools. 
    #[serde(rename = "DefaultAddressPools", skip_serializing_if = "Option::is_none")]
    pub default_address_pools: Option<Vec<crate::models::SystemInfoDefaultAddressPools>>,
    /// List of warnings / informational messages about missing features, or issues related to the daemon configuration.  These messages can be printed by the client as information to the user. 
    #[serde(rename = "Warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo {
            ID: None,
            containers: None,
            containers_running: None,
            containers_paused: None,
            containers_stopped: None,
            images: None,
            driver: None,
            driver_status: None,
            docker_root_dir: None,
            plugins: None,
            memory_limit: None,
            swap_limit: None,
            kernel_memory: None,
            kernel_memory_tcp: None,
            cpu_cfs_period: None,
            cpu_cfs_quota: None,
            cpu_shares: None,
            cpu_set: None,
            pids_limit: None,
            oom_kill_disable: None,
            ipv4_forwarding: None,
            bridge_nf_iptables: None,
            bridge_nf_ip6tables: None,
            debug: None,
            nfd: None,
            n_goroutines: None,
            system_time: None,
            logging_driver: None,
            cgroup_driver: None,
            cgroup_version: None,
            n_events_listener: None,
            kernel_version: None,
            operating_system: None,
            os_version: None,
            os_type: None,
            architecture: None,
            NCPU: None,
            mem_total: None,
            index_server_address: None,
            registry_config: None,
            generic_resources: None,
            http_proxy: None,
            https_proxy: None,
            no_proxy: None,
            name: None,
            labels: None,
            experimental_build: None,
            server_version: None,
            cluster_store: None,
            cluster_advertise: None,
            runtimes: None,
            default_runtime: None,
            swarm: None,
            live_restore_enabled: None,
            isolation: None,
            init_binary: None,
            containerd_commit: None,
            runc_commit: None,
            init_commit: None,
            security_options: None,
            product_license: None,
            default_address_pools: None,
            warnings: None,
        }
    }
}

/// The driver to use for managing cgroups. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CgroupDriver {
    #[serde(rename = "cgroupfs")]
    Cgroupfs,
    #[serde(rename = "systemd")]
    Systemd,
    #[serde(rename = "none")]
    None,
}

impl Default for CgroupDriver {
    fn default() -> CgroupDriver {
        Self::Cgroupfs
    }
}
/// The version of the cgroup. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CgroupVersion {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
}

impl Default for CgroupVersion {
    fn default() -> CgroupVersion {
        Self::_1
    }
}
/// Represents the isolation technology to use as a default for containers. The supported values are platform-specific.  If no isolation value is specified on daemon start, on Windows client, the default is `hyperv`, and on Windows server, the default is `process`.  This option is currently not used on other platforms. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Isolation {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "hyperv")]
    Hyperv,
    #[serde(rename = "process")]
    Process,
}

impl Default for Isolation {
    fn default() -> Isolation {
        Self::Default
    }
}

