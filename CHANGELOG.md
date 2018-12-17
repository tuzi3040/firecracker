# Changelog

##  [Unreleased]

### Added
- The `/logger` API has a new field called `options`. This is an array of
  strings that specify additional logging configurations. The only supported
  value is `LogDirtyPages`.
- When the `LogDirtyPages` option is configured via `PUT /logger`, a new metric
  called `memory.dirty_pages` is computed as the number of pages dirtied by the
  guest since the last time the metric was flushed. 

### Changed

- `PUT` requests on `/mmds` always return 204 on success.
- `PUT` operations on `/network-interfaces` API resources no longer accept 
  the previously required `state` parameter.
- The jailer starts with `--seccomp-level=2` (was previously 0) by default.
- Log messages use `anonymous-instance` as instance-id if no instance-id is set.

## [0.11.0]

### Added

- Apache-2.0 license
- Docs:
  - [charter](CHARTER.md)
  - [contribution guildelines](CONTRIBUTE.md)
  - [design](docs/design.md)
  - [getting started guide](docs/getting-started.md)
  - [security policy](SECURITY-POLICY.md)
  - [specifications](SPECIFICATION.md)
- **Experimental** vhost-based vsock implementation.

### Changed

- Improved MMDS network stack performance
- If the logging system is not yet initialized (via `PUT /logger`), log events
  are now sent to stdout/stderr.
- Moved the `instance_info_fails` metric under `get_api_requests`
- Improved [readme](README.md) and added links to more detailed information,
  now featured in subject-specific docs.

### Fixed
- Fixed bug in the MMDS network stack, that caused some RST packets to be sent
  without a destination.
- Fixed bug in `PATCH /drives`, whereby the ID in the path was not checked
  against the ID in the body.

## [0.10.1]

### Fixed

- The Swagger definition was corrected.

## [0.10.0]

### Added

- Each Firecracker process has an associated microVM Metadata Store (MMDS). Its
  contents can be configured using the `/mmds` API resource.

### Changed

- The boot source is specified only with the `kernel_image_path` and
  the optional parameter `boot_args`. All other fields are removed.
- The `path_on_host` property in the drive specification is now marked as
  *mandatory*.
- PATCH drive only allows patching/changing the `path_on_host` property.
- All PUT and PATCH requests return the status code 204.
- CPUID brand string (aka model name) now includes the host CPU frequency.
- API requests which add guest network interfaces have an additional parameter,
  `allow_mmds_requests` which defaults to `false`.
- Stopping the guest (e.g. using the `reboot` command) also terminates the
  Firecracker process. When the Firecracker process ends for any reason,
  (other than `kill -9`), metrics are flushed at the very end.
- On startup `jailer` closes all inherited file descriptors based on
  `sysconf(_SC_OPEN_MAX)` except input, output and error.
- The microVM ID prefixes each Firecracker log line. This ID also appears
  in the process `cmdline` so it's now possible to `ps | grep <ID>` for it.

## [0.9.0]

### Added

- Seccomp filtering is configured via the `--seccomp-level` jailer parameter.
- Firecracker logs the starting addresses of host memory areas provided as
  guest memory slots to KVM.
- The metric `panic_count` gets incremented to signal that a panic has
  occurred.
- Firecracker logs a backtrace when it crashes following a panic.
- Added basic instrumentation support for measuring boot time.

### Changed

- `StartInstance` is a synchronous API request (it used to be an asynchronous
  request).

### Fixed

- Ensure that fault messages sent by the API have valid JSON bodies.
- Use HTTP response code 500 for internal Firecracker errors, and 400 for user
  errors on InstanceStart.
- Serialize the machine configuration fields to the correct data types (as
  specified in the Swagger definition).
- NUMA node assignment is properly enforced by the jailer.
- The `is_root_device` and `is_read_only` properties are now marked as required
  in the Swagger definition of `Drive` object properties.

### Removed

- `GET` requests on the `/actions` API resource are no longer supported.
- The metrics associated with asynchronous actions have been removed.
- Remove the `action_id` parameter for `InstanceStart`, both from the URI and
  the JSON request body.

## [0.8.0]

### Added

- The jailer can now be configured to enter a preexisting network namespace,
  and to run as a daemon.
- Enabled PATCH operations on `/drives` resources.

## Changed

- The microVM `id` supplied to the jailer may now contain alphanumeric
  characters and hyphens, up to a maximum length of 64 characters.
- Replaced the `permissions` property of `/drives` resources with a boolean.
- Removed the `state` property of `/drives` resources.

## [0.7.0]

### Added

- Rate limiting functionality allows specifying an initial one time
  burst size.
- Firecracker can now boot from an arbitrary boot partition by specifying
  its unique id in the driver's API call.
- Block device rescan is triggered via a PUT `/actions` with the drive ID in
  the action body's `payload` field and the `action_type` field set to
  `BlockDeviceRescan`.

### Changed

- Removed `noapic` from the default guest kernel command line.
- The `action_id` parameter is no longer required for synchronous PUT requests
  to `/actions`.
- PUT requests are no longer allowed on `/drives` resources after the guest
  has booted.

### Fixed

- Fixed guest instance kernel loader to accelerate vCPUs launch and
  consequently guest kernel boot.
- Fixed network emulation to improve IO performance.

## [0.6.0]

### Added

- Firecracker uses two different named pipes to record human readable logs and
  metrics, respectively.

### Changed

- Seccomp filtering can be enabled via setting the `USE_SECCOMP` environment
  variable.
- It is possible to supply only a partial specification when attaching a rate
  limiter (i.e. just the bandwidth or ops parameter).
- Errors related to guest network interfaces are now more detailed.

### Fixed

- Fixed a bug that was causing Firecracker to panic whenever a `PUT` request
  was sent on an existing network interface.
- The `id` parameter of the `jailer` is required to be an RFC 4122-compliant
  UUID.
- Fixed an issue which caused the network RX rate limiter to be more
  restrictive than intended.
- API requests which contain unknown fields will generate an error.
- Fixed an issue related to high CPU utilization caused by improper `KVM PIT`
  configuration.
- It is now possible to create more than one network tun/tap interface inside a
  jailed Firecracker.

## [0.5.0]

### Added

- Added metrics for API requests, VCPU and device actions for the serial
  console (`UART`), keyboard (`i8042`), block and network devices. Metrics are
  logged every 60 seconds.
- A CPU features template for C3 is available, in addition to the one for T2.
- Seccomp filters restrict Firecracker from calling any other system calls than
  the minimum set it needs to function properly. The filters are enabled by
  setting the `USE_SECCOMP` environment variable to 1 before running
  Firecracker.
- Firecracker can be started by a new binary called `jailer`. The jailer takes
  as command line arguments a unique ID, the path to the Firecracker binary,
  the NUMA node that Firecracker will be assigned to and a `uid` and `gid` for
  Firecracker to run under. It sets up a `chroot` environment and a `cgroup`,
  and calls exec to morph into Firecracker.

### Changed

- In case of failure, the metrics and the panic location are logged before
  aborting.
- Metric values are reset with every flush.
- `CPUTemplate` is now called `CpuTemplate` in order to work seamlessly with
  the swagger code generator for Go.
- `firecracker-beta.yaml` is now called `firecracker.yaml`.

### Fixed

- Handling was added for several untreated KVM exit scenarios, which could have
  led to panic.
- Fixed a bug that caused Firecracker to crash when attempting to disable the
  `IA32_DEBUG_INTERFACE MSR` flag in the T2 CPU features.

### Removed

- Removed a leftover file generated by the logger unit tests.
- Removed `firecracker-v1.0.yaml`.

## [0.4.0]

### Added

- The CPU Template can be set with an API call on `PUT /machine-config`. The
  only available template is T2.
- Hyperthreading can be enabled/disabled with an API call on
  `PUT /machine-config`. By default, hyperthreading is disabled.
- Added boot time performance test (`tests/performance/test_boottime.py`).
- Added Rate Limiter for VirtIO/net and VirtIO/net devices. The Rate Limiter
  uses two token buckets to limit rate on bytes/s and ops/s. The rate limiter
  can be (optionally) configured per drive with a `PUT` on `/drives/{drive_id}`
  and per network interface with a `PUT` on `/network-interface/{iface_id}`.
- Implemented pre-boot PUT updates for `/boot-source`, `/drives`,
  `/network-interfaces` and `/vsock`.
- Added integration tests for `PUT` updates.

### Changed

- Moved the API definition (`swagger/firecracker-beta.yaml`) to the
  `api_server` crate.
- Removed `"console=ttyS0"` and added `"8250.nr_uarts=0"` to the default kernel
  command line to decrease the boot time.
- Changed the CPU topology to have all logical CPUs on a single socket.
- Removed the upper bound on CPU count as with musl there is no good way to get
  the total number of logical processors on a host.
- Build time tests now print the full output of commands.
- Disabled the Performance Monitor Unit and the Turbo Boost.
- Check the expected KVM capabilities before starting the VM.
- Logs now have timestamps.

### Fixed

- `testrun.sh` can run on platforms with more than one package manager by
  setting the package manager via a command line parameter (`-p`).
- Allow correct set up of multiple network-interfaces with auto-generated MAC.
- Fixed sporadic bug in VirtIO which was causing lost packages.
- Don't allow `PUT` requests with empty body on `/machine-config`.
- Deny `PUT` operations after the microvm boots (exception: the temporarily fix
  for live resize of block devices).

### Removed

- Removed examples crate. This used to have a Python example of starting
  Firecracker. This is replaced by `test_api.py` integration tests.
- Removed helper scripts for getting coverage and coding style errors. These
  were replaced by `test_coverage.py` and `test_style.py` test integration
  tests.
- Removed `--vmm-no-api` command line option. Firecracker can only be started
  via the API.

## [0.3.0]

### Added

- Users can interrogate the Machine Configuration (i.e. vcpu count and memory
  size) using a `GET` request on `/machine-config`.
- The logging system can be configured through the API using a `PUT` on
  `/logger`.
- Block devices support live resize by calling `PUT` with the same parameters
  as when the block was created.
- Release builds have Link Time Optimization (LTO) enabled.
- Firecracker is built with `musl`, resulting in a statically linked binary.
- More in-tree integration tests were added as part of the continuous
  integration system.

### Changed

- The vcpu count is enforced to `1` or an even number.
- The Swagger definition of rate limiters was updated.
- Syslog-enabled logs were replaced with a host-file backed mechanism.

### Fixed

- The host topology of the CPU and the caches is not leaked into the microvm
  anymore.
- Boot time was improved by advertising the availability of the TSC deadline
  timer.
- Fixed an issue which prevented Firecracker from working on 4.14 (or newer)
  host kernels.
- Specifying the MAC address for an interface through the API is optional.

### Removed

- Removed support for attaching vsock devices.
- Removed support for building Firecracker with glibc.

## [0.2.0]

### Added

- Users can now interrogate Instance Information (currently just instance
  state) through the API.

### Changed

- Renamed `api/swagger/all.yaml` to `api/swagger/firecracker-v1.0.yaml` which
  specifies targeted API support for Firecracker v1.0.
- Renamed `api/swagger/firecracker-v0.1.yaml` to
  `api/swagger/firecracker-beta.yaml` which specifies the currently supported
  API.
- Users can now enforce that an emulated block device is read-only via the API.
  To specify whether a block device is read-only or read-write, an extra
  "permissions" field was added to the Drive definition in the API. The root
  filesystem is automatically mounted in the guest OS as `ro`/`rw` according to
  the specified "permissions". It's the responsibility of the user to mount any
  other read-only block device as such within the guest OS.
- Users can now stop the guest VM using the API. Actions of type `InstanceHalt`
  are now supported via the API.

### Fixed

- Added support for `getDeviceID()` in `virtIO-block`. Without this, the guest
  Linux kernel would complain at boot time that the operation is unsupported.
- `stdin` control is returned to the Firecracker process when guest VM is
  inactive. Raw mode `stdin` is forwarded to the guest OS when guest VM is
  running.

### Removed

- Removed `api/swagger/actions.yaml`.
- Removed `api/swagger/devices.yaml`.
- Removed `api/swagger/firecracker-mvp.yaml`.
- Removed `api/swagger/limiters.yaml`.

## [0.1.1]

### Changed

- Users can now specify the MAC address of a guest network interface via the
  `PUT` network interface API request. Previously, the guest MAC address
  parameter was ignored.

### Fixed

- Fixed a guest memory allocation issue, which previously led to a potentially
  significant memory chunk being wasted.
- Fixed an issue which caused compilation problems, due to a compatibility
  breaking transitive dependency in the tokio suite of crates.

## [0.1.0]

### Added

- One-process virtual machine manager (one Firecracker per microVM).
- RESTful API running on a unix socket. The API supported by v0.1 can be found
  at `api/swagger/firecracker-v0.1.yaml`.
- Emulated keyboard (`i8042`) and serial console (`UART`). The microVM serial
  console input and output are connected to those of the Firecracker process
  (this allows direct console access to the guest OS).
- The capability of mapping an existing host tun-tap device as a VirtIO/net
  device into the microVM.
- The capability of mapping an existing host file as a GirtIO/block device into
  the microVM.
- The capability of creating a VirtIO/vsock between the host and the microVM.
- Default demand fault paging & CPU oversubscription.
