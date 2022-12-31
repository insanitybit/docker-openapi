/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.41) is used. For example, calling `/info` is the same as calling `/v1.41/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a [base64url encoded](https://tools.ietf.org/html/rfc4648#section-5) (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ``` 
 *
 * The version of the OpenAPI document: 1.41
 * 
 * Generated by: https://openapi-generator.tech
 */

/// HostConfig : Container configuration that depends on the host we are running on



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HostConfig {
    /// An integer value representing this container's relative CPU weight versus other containers. 
    #[serde(rename = "CpuShares", skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i32>,
    /// Memory limit in bytes.
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// Path to `cgroups` under which the container's `cgroup` is created. If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process. Cgroups are created if they do not already exist. 
    #[serde(rename = "CgroupParent", skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Block IO weight (relative weight).
    #[serde(rename = "BlkioWeight", skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<i32>,
    /// Block IO weight (relative device weight) in the form:  ``` [{\"Path\": \"device_path\", \"Weight\": weight}] ``` 
    #[serde(rename = "BlkioWeightDevice", skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<crate::models::ResourcesBlkioWeightDevice>>,
    /// Limit read rate (bytes per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ``` 
    #[serde(rename = "BlkioDeviceReadBps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<crate::models::ThrottleDevice>>,
    /// Limit write rate (bytes per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ``` 
    #[serde(rename = "BlkioDeviceWriteBps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<crate::models::ThrottleDevice>>,
    /// Limit read rate (IO per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ``` 
    #[serde(rename = "BlkioDeviceReadIOps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<crate::models::ThrottleDevice>>,
    /// Limit write rate (IO per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ``` 
    #[serde(rename = "BlkioDeviceWriteIOps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<crate::models::ThrottleDevice>>,
    /// The length of a CPU period in microseconds.
    #[serde(rename = "CpuPeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// Microseconds of CPU time that the container can get in a CPU period. 
    #[serde(rename = "CpuQuota", skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// The length of a CPU real-time period in microseconds. Set to 0 to allocate no time allocated to real-time tasks. 
    #[serde(rename = "CpuRealtimePeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to allocate no time allocated to real-time tasks. 
    #[serde(rename = "CpuRealtimeRuntime", skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`). 
    #[serde(rename = "CpusetCpus", skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems. 
    #[serde(rename = "CpusetMems", skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// A list of devices to add to the container.
    #[serde(rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::models::DeviceMapping>>,
    /// a list of cgroup rules to apply to the container
    #[serde(rename = "DeviceCgroupRules", skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    /// A list of requests for devices to be sent to device drivers. 
    #[serde(rename = "DeviceRequests", skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<crate::models::DeviceRequest>>,
    /// Kernel memory limit in bytes.  <p><br /></p>  > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated > `kmem.limit_in_bytes`. 
    #[serde(rename = "KernelMemory", skip_serializing_if = "Option::is_none")]
    pub kernel_memory: Option<i64>,
    /// Hard limit for kernel TCP buffer memory (in bytes).
    #[serde(rename = "KernelMemoryTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    /// Memory soft limit in bytes.
    #[serde(rename = "MemoryReservation", skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap. 
    #[serde(rename = "MemorySwap", skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100. 
    #[serde(rename = "MemorySwappiness", skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<i64>,
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    #[serde(rename = "NanoCpus", skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    /// Disable OOM Killer for the container.
    #[serde(rename = "OomKillDisable", skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used. 
    #[serde(rename = "Init", skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null` to not change. 
    #[serde(rename = "PidsLimit", skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    /// A list of resource limits to set in the container. For example:  ``` {\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048} ``` 
    #[serde(rename = "Ulimits", skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<crate::models::ResourcesUlimits>>,
    /// The number of usable CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last. 
    #[serde(rename = "CpuCount", skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// The usable percentage of the available CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last. 
    #[serde(rename = "CpuPercent", skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    /// Maximum IOps for the container system drive (Windows only)
    #[serde(rename = "IOMaximumIOps", skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    /// Maximum IO in bytes per second for the container system drive (Windows only). 
    #[serde(rename = "IOMaximumBandwidth", skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<i64>,
    /// A list of volume bindings for this container. Each volume binding is a string in one of these forms:  - `host-src:container-dest[:options]` to bind-mount a host path   into the container. Both `host-src`, and `container-dest` must   be an _absolute_ path. - `volume-name:container-dest[:options]` to bind-mount a volume   managed by a volume driver into the container. `container-dest`   must be an _absolute_ path.  `options` is an optional, comma-delimited list of:  - `nocopy` disables automatic copying of data from the container   path to the volume. The `nocopy` flag only applies to named volumes. - `[ro|rw]` mounts a volume read-only or read-write, respectively.   If omitted or set to `rw`, volumes are mounted read-write. - `[z|Z]` applies SELinux labels to allow or deny multiple containers   to read and write to the same volume.     - `z`: a _shared_ content label is applied to the content. This       label indicates that multiple containers can share the volume       content, for both reading and writing.     - `Z`: a _private unshared_ label is applied to the content.       This label indicates that only the current container can use       a private volume. Labeling systems such as SELinux require       proper labels to be placed on volume content that is mounted       into a container. Without a label, the security system can       prevent a container's processes from using the content. By       default, the labels set by the host operating system are not       modified. - `[[r]shared|[r]slave|[r]private]` specifies mount   [propagation behavior](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt).   This only applies to bind-mounted volumes, not internal volumes   or named volumes. Mount propagation requires the source mount   point (the location where the source directory is mounted in the   host operating system) to have the correct propagation properties.   For shared volumes, the source mount point must be set to `shared`.   For slave volumes, the mount must be set to either `shared` or   `slave`. 
    #[serde(rename = "Binds", skip_serializing_if = "Option::is_none")]
    pub binds: Option<Vec<String>>,
    /// Path to a file where the container ID is written
    #[serde(rename = "ContainerIDFile", skip_serializing_if = "Option::is_none")]
    pub container_id_file: Option<String>,
    #[serde(rename = "LogConfig", skip_serializing_if = "Option::is_none")]
    pub log_config: Option<Box<crate::models::HostConfigAllOfLogConfig>>,
    /// Network mode to use for this container. Supported standard values are: `bridge`, `host`, `none`, and `container:<name|id>`. Any other value is taken as a custom network's name to which this container should connect to. 
    #[serde(rename = "NetworkMode", skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// PortMap describes the mapping of container ports to host ports, using the container's port-number and protocol as key in the format `<port>/<protocol>`, for example, `80/udp`.  If a container's port is mapped for multiple protocols, separate entries are added to the mapping table. 
    #[serde(rename = "PortBindings", skip_serializing_if = "Option::is_none")]
    pub port_bindings: Option<::std::collections::HashMap<String, Vec<crate::models::PortBinding>>>,
    #[serde(rename = "RestartPolicy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<Box<crate::models::RestartPolicy>>,
    /// Automatically remove the container when the container's process exits. This has no effect if `RestartPolicy` is set. 
    #[serde(rename = "AutoRemove", skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Driver that this container uses to mount volumes.
    #[serde(rename = "VolumeDriver", skip_serializing_if = "Option::is_none")]
    pub volume_driver: Option<String>,
    /// A list of volumes to inherit from another container, specified in the form `<container name>[:<ro|rw>]`. 
    #[serde(rename = "VolumesFrom", skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
    /// Specification for mounts to be added to the container. 
    #[serde(rename = "Mounts", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<crate::models::Mount>>,
    /// A list of kernel capabilities to add to the container. Conflicts with option 'Capabilities'. 
    #[serde(rename = "CapAdd", skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    /// A list of kernel capabilities to drop from the container. Conflicts with option 'Capabilities'. 
    #[serde(rename = "CapDrop", skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<Vec<String>>,
    /// cgroup namespace mode for the container. Possible values are:  - `\"private\"`: the container runs in its own private cgroup namespace - `\"host\"`: use the host system's cgroup namespace  If not specified, the daemon default is used, which can either be `\"private\"` or `\"host\"`, depending on daemon version, kernel support and configuration. 
    #[serde(rename = "CgroupnsMode", skip_serializing_if = "Option::is_none")]
    pub cgroupns_mode: Option<CgroupnsMode>,
    /// A list of DNS servers for the container to use.
    #[serde(rename = "Dns", skip_serializing_if = "Option::is_none")]
    pub dns: Option<Vec<String>>,
    /// A list of DNS options.
    #[serde(rename = "DnsOptions", skip_serializing_if = "Option::is_none")]
    pub dns_options: Option<Vec<String>>,
    /// A list of DNS search domains.
    #[serde(rename = "DnsSearch", skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// A list of hostnames/IP mappings to add to the container's `/etc/hosts` file. Specified in the form `[\"hostname:IP\"]`. 
    #[serde(rename = "ExtraHosts", skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<String>>,
    /// A list of additional groups that the container process will run as. 
    #[serde(rename = "GroupAdd", skip_serializing_if = "Option::is_none")]
    pub group_add: Option<Vec<String>>,
    /// IPC sharing mode for the container. Possible values are:  - `\"none\"`: own private IPC namespace, with /dev/shm not mounted - `\"private\"`: own private IPC namespace - `\"shareable\"`: own private IPC namespace, with a possibility to share it with other containers - `\"container:<name|id>\"`: join another (shareable) container's IPC namespace - `\"host\"`: use the host system's IPC namespace  If not specified, daemon default is used, which can either be `\"private\"` or `\"shareable\"`, depending on daemon version and configuration. 
    #[serde(rename = "IpcMode", skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    /// Cgroup to use for the container.
    #[serde(rename = "Cgroup", skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<String>,
    /// A list of links for the container in the form `container_name:alias`. 
    #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// An integer value containing the score given to the container in order to tune OOM killer preferences. 
    #[serde(rename = "OomScoreAdj", skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i32>,
    /// Set the PID (Process) Namespace mode for the container. It can be either:  - `\"container:<name|id>\"`: joins another container's PID namespace - `\"host\"`: use the host's PID namespace inside the container 
    #[serde(rename = "PidMode", skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    /// Gives the container full access to the host.
    #[serde(rename = "Privileged", skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// Allocates an ephemeral host port for all of a container's exposed ports.  Ports are de-allocated when the container stops and allocated when the container starts. The allocated port might be changed when restarting the container.  The port is selected from the ephemeral port range that depends on the kernel. For example, on Linux the range is defined by `/proc/sys/net/ipv4/ip_local_port_range`. 
    #[serde(rename = "PublishAllPorts", skip_serializing_if = "Option::is_none")]
    pub publish_all_ports: Option<bool>,
    /// Mount the container's root filesystem as read only.
    #[serde(rename = "ReadonlyRootfs", skip_serializing_if = "Option::is_none")]
    pub readonly_rootfs: Option<bool>,
    /// A list of string values to customize labels for MLS systems, such as SELinux. 
    #[serde(rename = "SecurityOpt", skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    /// Storage driver options for this container, in the form `{\"size\": \"120G\"}`. 
    #[serde(rename = "StorageOpt", skip_serializing_if = "Option::is_none")]
    pub storage_opt: Option<::std::collections::HashMap<String, String>>,
    /// A map of container directories which should be replaced by tmpfs mounts, and their corresponding mount options. For example:  ``` { \"/run\": \"rw,noexec,nosuid,size=65536k\" } ``` 
    #[serde(rename = "Tmpfs", skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<::std::collections::HashMap<String, String>>,
    /// UTS namespace to use for the container.
    #[serde(rename = "UTSMode", skip_serializing_if = "Option::is_none")]
    pub uts_mode: Option<String>,
    /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled. 
    #[serde(rename = "UsernsMode", skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<String>,
    /// Size of `/dev/shm` in bytes. If omitted, the system uses 64MB. 
    #[serde(rename = "ShmSize", skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<i32>,
    /// A list of kernel parameters (sysctls) to set in the container. For example:  ``` {\"net.ipv4.ip_forward\": \"1\"} ``` 
    #[serde(rename = "Sysctls", skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<::std::collections::HashMap<String, String>>,
    /// Runtime to use with this container.
    #[serde(rename = "Runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// Initial console size, as an `[height, width]` array. (Windows only) 
    #[serde(rename = "ConsoleSize", skip_serializing_if = "Option::is_none")]
    pub console_size: Option<Vec<i32>>,
    /// Isolation technology of the container. (Windows only) 
    #[serde(rename = "Isolation", skip_serializing_if = "Option::is_none")]
    pub isolation: Option<Isolation>,
    /// The list of paths to be masked inside the container (this overrides the default set of paths). 
    #[serde(rename = "MaskedPaths", skip_serializing_if = "Option::is_none")]
    pub masked_paths: Option<Vec<String>>,
    /// The list of paths to be set as read-only inside the container (this overrides the default set of paths). 
    #[serde(rename = "ReadonlyPaths", skip_serializing_if = "Option::is_none")]
    pub readonly_paths: Option<Vec<String>>,
}

impl HostConfig {
    /// Container configuration that depends on the host we are running on
    pub fn new() -> HostConfig {
        HostConfig {
            cpu_shares: None,
            memory: None,
            cgroup_parent: None,
            blkio_weight: None,
            blkio_weight_device: None,
            blkio_device_read_bps: None,
            blkio_device_write_bps: None,
            blkio_device_read_i_ops: None,
            blkio_device_write_i_ops: None,
            cpu_period: None,
            cpu_quota: None,
            cpu_realtime_period: None,
            cpu_realtime_runtime: None,
            cpuset_cpus: None,
            cpuset_mems: None,
            devices: None,
            device_cgroup_rules: None,
            device_requests: None,
            kernel_memory: None,
            kernel_memory_tcp: None,
            memory_reservation: None,
            memory_swap: None,
            memory_swappiness: None,
            nano_cpus: None,
            oom_kill_disable: None,
            init: None,
            pids_limit: None,
            ulimits: None,
            cpu_count: None,
            cpu_percent: None,
            io_maximum_i_ops: None,
            io_maximum_bandwidth: None,
            binds: None,
            container_id_file: None,
            log_config: None,
            network_mode: None,
            port_bindings: None,
            restart_policy: None,
            auto_remove: None,
            volume_driver: None,
            volumes_from: None,
            mounts: None,
            cap_add: None,
            cap_drop: None,
            cgroupns_mode: None,
            dns: None,
            dns_options: None,
            dns_search: None,
            extra_hosts: None,
            group_add: None,
            ipc_mode: None,
            cgroup: None,
            links: None,
            oom_score_adj: None,
            pid_mode: None,
            privileged: None,
            publish_all_ports: None,
            readonly_rootfs: None,
            security_opt: None,
            storage_opt: None,
            tmpfs: None,
            uts_mode: None,
            userns_mode: None,
            shm_size: None,
            sysctls: None,
            runtime: None,
            console_size: None,
            isolation: None,
            masked_paths: None,
            readonly_paths: None,
        }
    }
}

/// cgroup namespace mode for the container. Possible values are:  - `\"private\"`: the container runs in its own private cgroup namespace - `\"host\"`: use the host system's cgroup namespace  If not specified, the daemon default is used, which can either be `\"private\"` or `\"host\"`, depending on daemon version, kernel support and configuration. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CgroupnsMode {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "host")]
    Host,
}

impl Default for CgroupnsMode {
    fn default() -> CgroupnsMode {
        Self::Private
    }
}
/// Isolation technology of the container. (Windows only) 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Isolation {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "hyperv")]
    Hyperv,
}

impl Default for Isolation {
    fn default() -> Isolation {
        Self::Default
    }
}
