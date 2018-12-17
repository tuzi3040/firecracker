// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

extern crate libc;

use seccomp::{
    Error, SeccompAction, SeccompCmpOp, SeccompCondition, SeccompFilterContext, SeccompRule,
};

/// List of allowed syscalls, necessary for Firecracker to function correctly.
pub const ALLOWED_SYSCALLS: &[i64] = &[
    libc::SYS_read,
    libc::SYS_write,
    libc::SYS_open,
    libc::SYS_close,
    libc::SYS_stat,
    libc::SYS_fstat,
    libc::SYS_lseek,
    libc::SYS_mmap,
    libc::SYS_mprotect,
    libc::SYS_munmap,
    libc::SYS_brk,
    libc::SYS_rt_sigaction,
    libc::SYS_rt_sigprocmask,
    libc::SYS_rt_sigreturn,
    libc::SYS_ioctl,
    libc::SYS_readv,
    libc::SYS_writev,
    libc::SYS_pipe,
    libc::SYS_dup,
    libc::SYS_socket,
    libc::SYS_accept,
    libc::SYS_bind,
    libc::SYS_listen,
    libc::SYS_clone,
    libc::SYS_execve,
    libc::SYS_exit,
    libc::SYS_fcntl,
    libc::SYS_readlink,
    libc::SYS_sigaltstack,
    libc::SYS_prctl,
    libc::SYS_arch_prctl,
    libc::SYS_futex,
    libc::SYS_sched_getaffinity,
    libc::SYS_set_tid_address,
    libc::SYS_exit_group,
    libc::SYS_epoll_ctl,
    libc::SYS_epoll_pwait,
    libc::SYS_timerfd_create,
    libc::SYS_eventfd2,
    libc::SYS_epoll_create1,
    libc::SYS_getrandom,
];

// See /usr/include/x86_64-linux-gnu/sys/epoll.h
const EPOLL_CTL_ADD: u64 = 1;
const EPOLL_CTL_DEL: u64 = 2;

// See /usr/include/x86_64-linux-gnu/bits/fcntl-linux.h
const O_RDONLY: u64 = 0x00000000;
const O_RDWR: u64 = 0x00000002;
const O_NONBLOCK: u64 = 0x00004000;
const O_CLOEXEC: u64 = 0x02000000;
const F_GETFD: u64 = 1;
const F_SETFD: u64 = 2;
const F_SETFL: u64 = 4;
const FD_CLOEXEC: u64 = 1;

// See /usr/include/linux/futex.h
const FUTEX_WAIT: u64 = 0;
const FUTEX_WAKE: u64 = 1;
const FUTEX_REQUEUE: u64 = 3;
const FUTEX_PRIVATE_FLAG: u64 = 128;
const FUTEX_WAIT_PRIVATE: u64 = FUTEX_WAIT | FUTEX_PRIVATE_FLAG;
const FUTEX_WAKE_PRIVATE: u64 = FUTEX_WAKE | FUTEX_PRIVATE_FLAG;
const FUTEX_REQUEUE_PRIVATE: u64 = FUTEX_REQUEUE | FUTEX_PRIVATE_FLAG;

// See /usr/include/asm-generic/ioctls.h
const TCGETS: u64 = 0x5401;
const TCSETS: u64 = 0x5402;
const TIOCGWINSZ: u64 = 0x5413;
const FIOCLEX: u64 = 0x5451;
const FIONBIO: u64 = 0x5421;

// See /usr/include/linux/kvm.h
const KVM_GET_API_VERSION: u64 = 0xae00;
const KVM_CREATE_VM: u64 = 0xae01;
const KVM_CHECK_EXTENSION: u64 = 0xae03;
const KVM_GET_VCPU_MMAP_SIZE: u64 = 0xae04;
const KVM_CREATE_VCPU: u64 = 0xae41;
const KVM_SET_TSS_ADDR: u64 = 0xae47;
const KVM_CREATE_IRQCHIP: u64 = 0xae60;
const KVM_RUN: u64 = 0xae80;
const KVM_SET_MSRS: u64 = 0x4008ae89;
const KVM_SET_CPUID2: u64 = 0x4008ae90;
const KVM_SET_USER_MEMORY_REGION: u64 = 0x4020ae46;
const KVM_IRQFD: u64 = 0x4020ae76;
const KVM_CREATE_PIT2: u64 = 0x4040ae77;
const KVM_IOEVENTFD: u64 = 0x4040ae79;
const KVM_SET_REGS: u64 = 0x4090ae82;
const KVM_SET_SREGS: u64 = 0x4138ae84;
const KVM_SET_FPU: u64 = 0x41a0ae8d;
const KVM_SET_LAPIC: u64 = 0x4400ae8f;
const KVM_GET_SREGS: u64 = 0x8138ae83;
const KVM_GET_LAPIC: u64 = 0x8400ae8e;
const KVM_GET_SUPPORTED_CPUID: u64 = 0xc008ae05;

// See /usr/include/linux/if_tun.h
const TUNSETIFF: u64 = 0x400454ca;
const TUNSETOFFLOAD: u64 = 0x400454d0;
const TUNSETVNETHDRSZ: u64 = 0x400454d8;

// See /usr/include/asm-generic/mman-common.h and /usr/include/asm-generic/mman.h
const PROT_NONE: u64 = 0x0;
const PROT_READ: u64 = 0x1;
const PROT_WRITE: u64 = 0x2;
const MAP_SHARED: u64 = 0x01;
const MAP_PRIVATE: u64 = 0x02;
const MAP_ANONYMOUS: u64 = 0x20;
const MAP_NORESERVE: u64 = 0x4000;

// See /usr/include/x86_64-linux-gnu/bits/socket.h
const PF_LOCAL: u64 = 1;

/// The default context containing the white listed syscall rules required by `Firecracker` to
/// function.
pub fn default_context() -> Result<SeccompFilterContext, Error> {
    Ok(SeccompFilterContext::new(
        vec![
            (
                libc::SYS_accept,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_bind,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_close,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_dup,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_epoll_create1,
                (
                    0,
                    vec![SeccompRule::new(
                        vec![SeccompCondition::new(0, SeccompCmpOp::Eq, 0)?],
                        SeccompAction::Allow,
                    )],
                ),
            ),
            (
                libc::SYS_epoll_ctl,
                (
                    0,
                    vec![
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, EPOLL_CTL_ADD)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, EPOLL_CTL_DEL)?],
                            SeccompAction::Allow,
                        ),
                    ],
                ),
            ),
            (
                libc::SYS_epoll_pwait,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_eventfd2,
                (
                    0,
                    vec![SeccompRule::new(
                        vec![
                            SeccompCondition::new(0, SeccompCmpOp::Eq, 0)?,
                            SeccompCondition::new(1, SeccompCmpOp::Eq, 0)?,
                        ],
                        SeccompAction::Allow,
                    )],
                ),
            ),
            (
                libc::SYS_fcntl,
                (
                    0,
                    vec![
                        SeccompRule::new(
                            vec![
                                SeccompCondition::new(1, SeccompCmpOp::Eq, F_SETFL)?,
                                SeccompCondition::new(
                                    2,
                                    SeccompCmpOp::Eq,
                                    O_RDONLY | O_NONBLOCK | O_CLOEXEC,
                                )?,
                            ],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![
                                SeccompCondition::new(1, SeccompCmpOp::Eq, F_SETFD)?,
                                SeccompCondition::new(2, SeccompCmpOp::Eq, FD_CLOEXEC)?,
                            ],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, F_GETFD)?],
                            SeccompAction::Allow,
                        ),
                    ],
                ),
            ),
            (
                libc::SYS_fstat,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_futex,
                (
                    0,
                    vec![
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                FUTEX_WAIT_PRIVATE,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                FUTEX_WAKE_PRIVATE,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                FUTEX_REQUEUE_PRIVATE,
                            )?],
                            SeccompAction::Allow,
                        ),
                    ],
                ),
            ),
            (
                libc::SYS_ioctl,
                (
                    0,
                    vec![
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, TCSETS)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, TCGETS)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, TIOCGWINSZ)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                KVM_CHECK_EXTENSION,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_CREATE_VM)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                KVM_GET_API_VERSION,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                KVM_GET_SUPPORTED_CPUID,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                KVM_GET_VCPU_MMAP_SIZE,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                KVM_CREATE_IRQCHIP,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_CREATE_PIT2)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_CREATE_VCPU)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_IOEVENTFD)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_IRQFD)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                KVM_SET_TSS_ADDR,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                KVM_SET_USER_MEMORY_REGION,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, FIOCLEX)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, FIONBIO)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, TUNSETIFF)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, TUNSETOFFLOAD)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, TUNSETVNETHDRSZ)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_GET_LAPIC)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_GET_SREGS)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_RUN)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_SET_CPUID2)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_SET_FPU)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_SET_LAPIC)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_SET_MSRS)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_SET_REGS)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, KVM_SET_SREGS)?],
                            SeccompAction::Allow,
                        ),
                    ],
                ),
            ),
            (
                libc::SYS_listen,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_lseek,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_mmap,
                (
                    0,
                    vec![
                        SeccompRule::new(vec![], SeccompAction::Allow),
                        SeccompRule::new(
                            vec![
                                SeccompCondition::new(0, SeccompCmpOp::Eq, 0)?,
                                SeccompCondition::new(2, SeccompCmpOp::Eq, PROT_NONE)?,
                                SeccompCondition::new(
                                    3,
                                    SeccompCmpOp::Eq,
                                    MAP_PRIVATE | MAP_ANONYMOUS,
                                )?,
                                SeccompCondition::new(4, SeccompCmpOp::Eq, -1i64 as u64)?,
                                SeccompCondition::new(5, SeccompCmpOp::Eq, 0)?,
                            ],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![
                                SeccompCondition::new(0, SeccompCmpOp::Eq, 0)?,
                                SeccompCondition::new(2, SeccompCmpOp::Eq, PROT_READ)?,
                                SeccompCondition::new(3, SeccompCmpOp::Eq, MAP_SHARED)?,
                                SeccompCondition::new(5, SeccompCmpOp::Eq, 0)?,
                            ],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![
                                SeccompCondition::new(0, SeccompCmpOp::Eq, 0)?,
                                SeccompCondition::new(2, SeccompCmpOp::Eq, PROT_READ | PROT_WRITE)?,
                                SeccompCondition::new(3, SeccompCmpOp::Eq, MAP_SHARED)?,
                                SeccompCondition::new(5, SeccompCmpOp::Eq, 0)?,
                            ],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![
                                SeccompCondition::new(0, SeccompCmpOp::Eq, 0)?,
                                SeccompCondition::new(2, SeccompCmpOp::Eq, PROT_READ | PROT_WRITE)?,
                                SeccompCondition::new(
                                    3,
                                    SeccompCmpOp::Eq,
                                    MAP_SHARED | MAP_ANONYMOUS | MAP_NORESERVE,
                                )?,
                                SeccompCondition::new(4, SeccompCmpOp::Eq, -1i64 as u64)?,
                                SeccompCondition::new(5, SeccompCmpOp::Eq, 0)?,
                            ],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![
                                SeccompCondition::new(0, SeccompCmpOp::Eq, 0)?,
                                SeccompCondition::new(2, SeccompCmpOp::Eq, PROT_READ | PROT_WRITE)?,
                                SeccompCondition::new(
                                    3,
                                    SeccompCmpOp::Eq,
                                    MAP_PRIVATE | MAP_ANONYMOUS,
                                )?,
                                SeccompCondition::new(4, SeccompCmpOp::Eq, -1i64 as u64)?,
                                SeccompCondition::new(5, SeccompCmpOp::Eq, 0)?,
                            ],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![
                                SeccompCondition::new(0, SeccompCmpOp::Eq, 0)?,
                                SeccompCondition::new(2, SeccompCmpOp::Eq, PROT_READ | PROT_WRITE)?,
                                SeccompCondition::new(
                                    3,
                                    SeccompCmpOp::Eq,
                                    MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE,
                                )?,
                                SeccompCondition::new(4, SeccompCmpOp::Eq, -1i64 as u64)?,
                                SeccompCondition::new(5, SeccompCmpOp::Eq, 0)?,
                            ],
                            SeccompAction::Allow,
                        ),
                    ],
                ),
            ),
            (
                libc::SYS_mprotect,
                (
                    0,
                    vec![SeccompRule::new(
                        vec![SeccompCondition::new(
                            2,
                            SeccompCmpOp::Eq,
                            PROT_READ | PROT_WRITE,
                        )?],
                        SeccompAction::Allow,
                    )],
                ),
            ),
            (
                libc::SYS_munmap,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_open,
                (
                    0,
                    vec![
                        SeccompRule::new(vec![], SeccompAction::Allow),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, O_RDWR)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                O_RDWR | O_CLOEXEC,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                O_RDWR | O_NONBLOCK | O_CLOEXEC,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(1, SeccompCmpOp::Eq, O_RDONLY)?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                O_RDONLY | O_CLOEXEC,
                            )?],
                            SeccompAction::Allow,
                        ),
                        SeccompRule::new(
                            vec![SeccompCondition::new(
                                1,
                                SeccompCmpOp::Eq,
                                O_RDONLY | O_NONBLOCK | O_CLOEXEC,
                            )?],
                            SeccompAction::Allow,
                        ),
                    ],
                ),
            ),
            (
                libc::SYS_pipe,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_read,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_readlink,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_readv,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_socket,
                (
                    0,
                    vec![SeccompRule::new(
                        vec![SeccompCondition::new(0, SeccompCmpOp::Eq, PF_LOCAL)?],
                        SeccompAction::Allow,
                    )],
                ),
            ),
            (
                libc::SYS_stat,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_timerfd_settime,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_write,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
            (
                libc::SYS_writev,
                (0, vec![SeccompRule::new(vec![], SeccompAction::Allow)]),
            ),
        ]
        .into_iter()
        .collect(),
        SeccompAction::Trap,
    )?)
}

#[cfg(test)]
mod tests {
    extern crate libc;
    extern crate seccomp;

    #[test]
    #[cfg(target_env = "musl")]
    fn test_basic_seccomp() {
        assert!(
            seccomp::setup_seccomp(seccomp::SeccompLevel::Basic(super::ALLOWED_SYSCALLS)).is_ok()
        );
    }

    #[test]
    #[cfg(target_env = "musl")]
    fn test_advanced_seccomp() {
        // Sets up context with additional rules required by the test.
        let mut context = super::default_context().unwrap();
        assert!(context
            .add_rules(
                libc::SYS_exit,
                None,
                vec![seccomp::SeccompRule::new(
                    vec![],
                    seccomp::SeccompAction::Allow,
                )],
            )
            .is_ok());
        assert!(context
            .add_rules(
                libc::SYS_rt_sigprocmask,
                None,
                vec![seccomp::SeccompRule::new(
                    vec![],
                    seccomp::SeccompAction::Allow,
                )],
            )
            .is_ok());
        assert!(context
            .add_rules(
                libc::SYS_set_tid_address,
                None,
                vec![seccomp::SeccompRule::new(
                    vec![],
                    seccomp::SeccompAction::Allow,
                )],
            )
            .is_ok());
        assert!(context
            .add_rules(
                libc::SYS_sigaltstack,
                None,
                vec![seccomp::SeccompRule::new(
                    vec![],
                    seccomp::SeccompAction::Allow,
                )],
            )
            .is_ok());

        assert!(seccomp::setup_seccomp(seccomp::SeccompLevel::Advanced(context)).is_ok());
    }
}
