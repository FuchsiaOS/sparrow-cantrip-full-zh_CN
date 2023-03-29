<!--

## CantripOS target architecture dependencies

There are various areas in CantripOS where target architecture-specific support is required:

- *sel4-sys*: system call wrappers
- *cantrip-os-model*: capDL support for the cantrip-os-rootserver
- *cantrip-proc-manager/sel4bundle*: application construction
- *libcantrip*: application runtime support
- *build/platforms*: build support
- *apps/system/platforms*: CAmkES configuration

-->

## CantripOS 目标架构依赖

CantripOS 中有多个领域需要特定于目标架构的支持：

- *sel4-sys*: 系统调用包装器
- *cantrip-os-model*: 对 cantrip-os-rootserver 的 capDL 支持
- *cantrip-proc-manager/sel4bundle*: 应用程序构建
- *libcantrip*: 应用程序运行时支持
- *build/platforms*: 构建支持
- *apps/system/platforms*: CAmkES 配置

<!--

### sel4-sys

The sel4-sys crate provides interfaces to the seL4 kernel.
This is comprised of system call wrappers and related types & constants,
and support for state exported by the kernel during system startup
(e.g. the seL4_BootInfo provided by the kernel to the rootserver thread).

Much of sel4-sys's api's are generated at build time from XML specifications
in the seL4 kernel. Others are hardcoded by the crate.

The *arch* subdirectory holds code for each supported target archiecture:
aarch32 (ARM 32-bit), aarch64 (ARM 64-bit), riscv32 (RISC-V 32-bit),
riscv64 (RISC-V 64-bit), and x86* (not currently working and mostly ignored).
For example:

-->

### sel4-sys

sel4-sys 创建提供与 seL4 内核的接口。
它由系统调用包装器和相关类型和常量以及内核在系统启动期间导出的状态支持组成
（例如内核提供给 rootserver 线程的 seL4_BootInfo）。

sel4-sys 的许多 API 在构建时从 seL4 内核中的 XML 规范生成。其他的则由创建硬编码。

arch 子目录包含每个支持目标架构的代码：
aarch32（ARM 32 位）、aarch64（ARM 64 位）、riscv32（RISC-V 32 位）、
riscv64（RISC-V 64 位）和 x86*（目前不起作用且大多被忽略）。
例如：

```shell
$ ls arch
aarch32_mcs.rs     aarch32_no_mcs.rs  aarch32. rs        aarch64_mcs.rs     aarch64_no_mcs.rs
aarch64.rs         arm_generic.rs     riscv32_mcs.rs     riscv32_no_mcs.rs  riscv32.rs
riscv64_mcs.rs     riscv64_no_mcs.rs  riscv64.rs         riscv_generic.rs   syscall_common.rs
syscall_mcs.rs     syscall_no_mcs.rs  x86_64.rs          x86_generic.rs     x86.rs
```

<!--

The aarch64.rs file is included by the architecture-independent code.
It in turns includes either aarch64_mcs.rs or aarch64_no_mcs.rs depending
on whether the seL4 kernel is configured with or without MCS support.
Each of these files define arch-specific proc macros used by the syscall_*.rs
templates to fill-in syscall wrappers.

The arm_generic.rs file has definitions that present architecture-specific
definitions and api's using an architecture-independent naming convention.
For example, every architecture has an seL4_SmallPageObject that maps
to their "small page" (4K on many). Similarly there is an seL4_Page_Map
call that maps to seL4_ARM_Page_Map on ARM systems.

To add a new architecture (or fix something like x86) follow the
patterns for the riscv and aarch architectures. Testing/validation of
the syscall wrappers is done using the [sel4test system](Testing.md)
(`m sel4test+wrappers`).

-->

aarch64.rs 文件被体系结构无关的代码所包含。它又根据 seL4 内核的配置情况，
包含 aarch64_mcs.rs 或 aarch64_no_mcs.rs 文件。这些文件都定义了由 syscall_*.rs 模板使用的架构特定的过程宏来填充系统调用包装器。

arm_generic.rs 文件具有使用与体系结构无关的命名约定呈现的架构特定定义和 API。例如，
每个体系结构都有一个 seL4_SmallPageObject，它映射到它们的“小页面”（在许多系统上为 4K）。
类似地，有一个 seL4_Page_Map 调用，它映射到 ARM 系统上的 seL4_ARM_Page_Map。

要添加新的架构（或修复类似 x86 的问题），请遵循 riscv 和 aarch 架构的模式。
使用 [sel4test 系统](Testing.md)（m sel4test+wrappers）对系统调用包装器进行测试/验证。

<!--

### cantrip-os-model

Support for the capDL "Model" resides in the cantrip-os-model crate.
Architecture-dependent support is mostly to setup an seL4 thread's virtual
address space (VSpace) and to create architecture-specific seL4 objects
that back IRQ's and I/O interfaces.  Like sel4-sys there is an *arch*
directory with architecture-specific support. Unlike sel4-sys MCS support
is orthogonal; that logic is split out to a *feature* subdirectory.

To add a new architecture (or fix something like x86) follow the pattern
for a working architecture. Testing/validation is done by running simple
CAmkES test cases under [cantrip-os-rootserver](CantripRootserver.md).

-->

### cantrip-os-model

capDL "Model" 的支持位于 cantrip-os-model crate 中。
架构相关的支持大部分是用于设置一个 seL4 线程的虚拟地址空间（VSpace），
并创建支持 IRQ 和 I/O 接口的架构特定的 seL4 对象。
与 sel4-sys 类似，*arch* 目录下有架构特定的支持。
不同于 sel4-sys，MCS 支持是正交的；该逻辑被拆分到 *feature* 子目录中。

要添加新的架构（或修复类似 x86 的问题），请遵循工作中架构的模式。
通过在 [cantrip-os-rootserver](CantripRootserver.md) 下运行简单的 CAmkES 测试用例进行测试/验证。

<!--

### cantrip-proc-manager/sel4bundle

The cantrip-proc-manager/sel4bundle module is similar to cantrip-os-model
except it constructs a CantripOS appllication from an sel4BundleImage
instead of a capDL Model. The realized seL4 thread may be limited in size
(e.g. on aarch64 a VSpace is constructed to support at most 2MiB of virtual
address space).
and has a fixed set of capabilties/objects provided to it.
Like cantrip-os-model there are *arch* and *feature* directories.

To add a new architecture follow the pattern for aarch64 or riscv32.
Testing is non-trivial with only printf-style debugging available unless
the simulator for the target-architecture supports GDB.

-->

### cantrip-proc-manager/sel4bundle

cantrip-proc-manager/sel4bundle 模块类似于 cantrip-os-model，
只是它从 sel4BundleImage 构造 CantripOS 应用程序，而不是从 capDL 模型中构造。
实现的 seL4 线程大小可能受到限制（例如，在 aarch64 上，构造了一个 VSpace 来支持最多 2MiB 的虚拟地址空间），
并且具有一组固定的能力/对象。与 cantrip-os-model 一样，有 *arch* 和 *feature* 目录。

要添加新的架构，请按照 aarch64 或 riscv32 的模式进行操作。
测试不太容易，除非目标架构的模拟器支持 GDB，否则只能使用类 printf 的调试方式。

<!--

### libcantrip

libbcantrip is the support code statically linked into each CantripOS
application. There is a libcantrip crate for Rust applications and a
library for C applications. Each version has an *arch* subdirectory
with a crt0.S file that handles startup work for an application.
The crt0 code is tightly coupled to the sel4bundle setup work and to
SDKRuntime/sdk-interface crate that implements RPC communication
between applications and the SDKRuntime.
Rather than provide a (potentially) stale explanation of how this
works, consult the code.

-->

### libcantrip

libcantrip是静态链接到每个CantripOS应用程序中的支持代码。
对于Rust应用程序，有一个libcantrip crate，对于C应用程序，有一个库。
每个版本都有一个*arch*子目录，其中包含一个crt0.S文件，用于处理应用程序的启动工作。
crt0代码与sel4bundle设置工作和实现应用程序与SDKRuntime/sdk-interface crate之间的RPC通信的SDKRuntime紧密耦合。
与其提供一个（可能）过时的解释，不如查看代码。

<!--

### build/platforms

The build system has platform-specific information in the *build/platforms*
directory; e.g. `build/platform/rpi3`. Most .mk files overide or augment
default settings. The `cantrip.mk` file must fill-in `CONFIG_PLATFORM` with
the identifier used by the seL4 kernel for the target platform.

The `cantrip_builtins.mk` file specifies which applications are included in the
builtins bundle embedded in a bootable image. The bundle can include both
binary applications and command ("repl") scripts. For platforms without
a UART driver it is useful to include an `autostart.repl` file that runs
applications to sanity-check system operation.

-->

### build/platforms

构建系统在`build/platforms`目录中具有特定于平台的信息；例如，*build/platform/rpi3*。
大多数.mk文件覆盖或增强默认设置。`cantrip.mk`文件必须使用seL4内核用于目标平台的标识符填写CONFIG_PLATFORM。

`cantrip_builtins.mk`文件指定了嵌入到可启动映像中的builtins bundle中包含哪些应用程序。
bundle可以包括二进制应用程序和命令（“repl”）脚本。对于没有UART驱动程序的平台，
包括一个autostart.repl文件非常有用，它运行应用程序以检查系统操作的正确性。

<!--

### apps/system/platforms

The build process leverages CAmkES to setup system services. The per-platform
CAmkES build glue is in `apps/system/platforms` with the target platform name
coming from the seL4 kernel. For example, the top-level "rpi3" platform
is setup to build a 64-bit system which seL4 converts to "bcm2387"; so the
CAmkES build glue is located in `apps/system/platforms/bcm2387`.

Each platform must have at least a *system.camkes* file to configure the
CAmkES assembly. Platform-specific components are specified with a `CMakeLists.txt`
file and any configuration overrides can be specified in an `easy-settings.cmake`
file.

-->

### apps/system/platforms

构建过程利用CAmkES设置系统服务。每个平台的CAmkES构建粘合剂在`apps/system/platforms`中，
目标平台名称来自seL4内核。例如，顶级“rpi3”平台设置为构建一个64位系统，seL4将其转换为“bcm2387”；
因此，CAmkES构建粘合剂位于`apps/system/platforms/bcm2387`。

每个平台必须至少有一个*system.camkes*文件来配置CAmkES装配。
平台特定的组件使用`CMakeLists.txt`文件指定，任何配置覆盖都可以在`easy-settings.cmake`文件中指定。

<!--

### [Next Section: Target Platform dependencies](PlatformDependencies.md)

-->

### [下一章: 目标平台依赖](PlatformDependencies.md)