<!--

## Target platform dependencies

The sel4-config mechanism for importing the seL4 kernel configuration
only dynamically sets features when ***compiling*** Rust code.
Target-platform dependencies like device drivers are handled by passing a
top-level feature through the cargo command line to effect the cargo dependency process.
For example, in `DebugConsole/cantrip-debug-console/Cargo.toml` configuration of the
platform-specific UART support for the command line interpreter is done with:

-->

## 目标平台依赖

sel4-config机制用于导入seL4内核配置，只有在***编译***Rust代码时才会动态设置功能。
目标平台依赖项（如设备驱动程序）通过将顶层功能通过cargo命令行传递来处理cargo依赖项进程。
例如，在`DebugConsole/cantrip-debug-console/Cargo.toml`中，针对命令行解释器的平台特定UART支持的配置是这样完成的：

```
[features]
default = [
    "autostart_support",
]
autostart_support = ["default-uart-client"]
# Target platform support
CONFIG_PLAT_BCM2837 = []
CONFIG_PLAT_SPARROW = ["cantrip-uart-client"]
```
<!--
The CONFIG_PLAT_* features mirror the seL4 kernel config parameters and can be
used to select an optional dependency:
-->

CONFIG_PLAT_*功能与seL4内核配置参数相对应，可用于选择可选依赖项：

```
[dependencies]
...
default-uart-client = { path = "../default-uart-client", optional = true }
...
cantrip-uart-client = { path = "../cantrip-uart-client", optional = true }
```
<!--
The platform feature is injected into the build process in *build/cantrip.mk* with:
-->
平台功能通过*build/cantrip.mk*中的以下内容注入到构建过程中：

```
cmake ... -DRUST_GLOBAL_FEATURES=${CONFIG_PLATFORM} ...
```
<!--
In addition to including platform-dependencies in the build process they
may also need to be included in the CAmkES assembly; this done by having
the `system.camkes` file platform-specific.
For example, `platforms/sparrow/system.camkes` plumbs the UARTDriver,
MlCoordinator, MailboxDriver, and TimerService components.

Some system services like the SDKRuntime are prepared for conditional inclusion
of dependent services;
e.g. if no MlCoordinator seervice is present all model-related SDK calls
returns SDKError::NoPlatformSupport.
This is done so applications have a stable ABI.

### [Next Section: Testing](Testing.md)
-->

除了在构建过程中包含平台依赖项，它们还可能需要包含在CAmkES组装中；这通过使`system.camkes`文件特定于平台来完成。
例如，`platforms/sparrow/system.camkes`连接了UARTDriver、MlCoordinator、MailboxDriver和TimerService组件。

一些系统服务（如SDKRuntime）已准备好条件包含依赖服务；
例如，如果不存在MlCoordinator服务，则所有与模型相关的SDK调用都会返回SDKError::NoPlatformSupport。
这样做是为了让应用程序拥有稳定的ABI。

### [下一章: 测试支持](Testing.md)