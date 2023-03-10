/*
 * Docker Engine API
 *
 * The Engine API is an HTTP API served by Docker Engine. It is the API the Docker client uses to communicate with the Engine, so everything the Docker client can do can be done with the API.  Most of the client's commands map directly to API endpoints (e.g. `docker ps` is `GET /containers/json`). The notable exception is running containers, which consists of several API calls.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call. The body of the response will be JSON in the following format:  ``` {   \"message\": \"page not found\" } ```  # Versioning  The API is usually changed in each release, so API calls are versioned to ensure that clients don't break. To lock to a specific version of the API, you prefix the URL with its version, for example, call `/v1.30/info` to use the v1.30 version of the `/info` endpoint. If the API version specified in the URL is not supported by the daemon, a HTTP `400 Bad Request` error message is returned.  If you omit the version-prefix, the current version of the API (v1.41) is used. For example, calling `/info` is the same as calling `/v1.41/info`. Using the API without a version-prefix is deprecated and will be removed in a future release.  Engine releases in the near future should support this version of the API, so your client will continue to work even if it is talking to a newer Engine.  The API uses an open schema model, which means server may add extra properties to responses. Likewise, the server will ignore any extra query parameters and request body properties. When you write clients, you need to ignore additional properties in responses to ensure they do not break when talking to newer daemons.   # Authentication  Authentication for registries is handled client side. The client has to send authentication details to various endpoints that need to communicate with registries, such as `POST /images/(name)/push`. These are sent as `X-Registry-Auth` header as a [base64url encoded](https://tools.ietf.org/html/rfc4648#section-5) (JSON) string with the following structure:  ``` {   \"username\": \"string\",   \"password\": \"string\",   \"email\": \"string\",   \"serveraddress\": \"string\" } ```  The `serveraddress` is a domain/IP without a protocol. Throughout this structure, double quotes are required.  If you have already got an identity token from the [`/auth` endpoint](#operation/SystemAuth), you can just pass this instead of credentials:  ``` {   \"identitytoken\": \"9cbaf023786cd7...\" } ``` 
 *
 * The version of the OpenAPI document: 1.41
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Resources : A container's resources (cgroups config, ulimits, etc)



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Resources {
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
}

impl Resources {
    /// A container's resources (cgroups config, ulimits, etc)
    pub fn new() -> Resources {
        Resources {
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
        }
    }
}


