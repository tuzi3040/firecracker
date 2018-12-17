# Firecracker Charter

## Mission

Our mission is to enable secure, multi-tenant, minimal-overhead
execution of container and function workloads.

## Tenets (unless you know better ones)

These tenets guide Firecracker's development:

1. **Built-In Security**: We provide compute security barriers that enable
   multi-tenant workloads, and cannot be mistakenly disabled by customers.
   Customer workloads are simultaneously considered sacred (shall not be
   touched) and malicious (shall be defended against).
1. **Light-weight Virtualization**: We focus on transient or stateless workloads
   over long-running or persistent workloads. Firecracker's hardware resources
   overhead is known and guaranteed.
1. **Minimalist in Features**: If it's not clearly required for our mission, we
   won't build it. We maintain a single implementation per capability.
1. **Compute Oversubscription**: All of the hardware compute resources exposed
   by Firecracker to guests can be securely oversubscribed.

## Contributions & Project Roles

All contributions must align with this charter and follow Firecracker's
[contribution process](CONTRIBUTING.md).

Firecracker [maintainers](MAINTAINERS.md) merge contributions into the
master branch and create Firecracker releases. Maintainers are also
subject to the mission and tenets outlined above. Anyone may submit
and review contributions.
