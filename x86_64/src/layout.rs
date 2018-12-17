// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//
// Portions Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the THIRD-PARTY file.

// Magic addresses used to lay out x86_64 VMs.

// The 'zero page', a.k.a linux kernel bootparams.
pub const ZERO_PAGE_START: usize = 0x7000;
// Initial stack for the boot CPU.
pub const BOOT_STACK_START: usize = 0x8000;
pub const BOOT_STACK_POINTER: usize = 0x8ff0;
// Initial pagetables.
pub const PML4_START: usize = 0x9000;
pub const PDPTE_START: usize = 0xa000;
pub const PDE_START: usize = 0xb000;
// Kernel command line.
pub const CMDLINE_START: usize = 0x20000;
pub const CMDLINE_MAX_SIZE: usize = 0x10000;
// MPTABLE, describing VCPUS.
pub const MPTABLE_START: usize = 0x9fc00;
// Where BIOS/VGA magic would live on a real PC.
pub const EBDA_START: u64 = 0x9fc00;
// 1MB.  We don't put anything above here except the kernel itself.
pub const HIMEM_START: usize = 0x100000;
