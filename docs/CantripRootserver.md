<!--
## CantripOS capDL rootserver application

The other main Rust piece of CantripOS is the rootserver application that is located in
*projects/capdl/cantrip-os-rootserver*. This depends on the *capdl* and *model*
submodules of *cantrip-os-common*. While it is possible to select either
cantrip-os-rootserver or the C-based capdl-loader-app with a CMake setting
in the CAmkES project's easy-settings.cmake file; e.g. `projects/cantrip/easy-settings.cmake` has:

```
#set(CAPDL_LOADER_APP "capdl-loader-app" CACHE STRING "")
set(CAPDL_LOADER_APP "cantrip-os-rootserver" CACHE STRING "")
```

using capdl-loader-app is not advised because it lacks important functionality
found only in cantrip-os-rootserver.

The most important differences between cantrip-os-rootserver and capdl-loader-app are:

- Support for reclaiming the rootserver's memory on exit.
- Support for CantripOS CAmkES features (e.g. MemoryManager, RTReply caps).
- Reduced memory consumption.

Otherwise cantrip-os-rootserver should provide the same functionality though
certain features are not tested (e.g. CONFIG_CAPDL_LOADER_STATIC_ALLOC)
and/or not well-tested (e.g. CONFIG_CAPDL_LOADER_CC_REGISTERS).

Beware that many of the cmake rootserver configuration parameters are not plumbed
through to the Rust code.  Its likely you will need to tweak features in the
Cargo.toml for cantrip-os-rootserver and/or cantrip-os-model (cantrip-os-common).

By default cantrip-os-rootserver prints information about the capDL specification
when it starts up. If you want verbose logging enable `LOG_DEBUG` or `LOG_TRACE`
in the Cargo.toml.

### [Next Section: Depending on CantripOS Rust crates](CrateDependencies.md)
-->

## CantripOS的capDL根服务器应用程序

CantripOS的另一个主要Rust组件是位于*projects/capdl/cantrip-os-rootserver*中的根服务器应用程序。
这取决于*cantrip-os-common*的*capdl*和*model*子模块。
虽然可以使用CMake设置在CAmkES项目的easy-settings.cmake文件中选择cantrip-os-rootserver或基于C的capdl-loader-app；
例如，`projects/cantrip/easy-settings.cmake`中有：

```
#set(CAPDL_LOADER_APP "capdl-loader-app" CACHE STRING "")
set(CAPDL_LOADER_APP "cantrip-os-rootserver" CACHE STRING "")
```

但不建议使用capdl-loader-app，因为它缺少仅在cantrip-os-rootserver中才有的重要功能。

cantrip-os-rootserver和capdl-loader-app之间最重要的区别是：

- 支持在退出时回收根服务器的内存。
- 支持CantripOS CAmkES特性（例如MemoryManager、RTReply caps）。
- 减少内存消耗。

否则，cantrip-os-rootserver应该提供相同的功能，
尽管某些功能未经测试（例如CONFIG_CAPDL_LOADER_STATIC_ALLOC）和/或未经充分测试（例如CONFIG_CAPDL_LOADER_CC_REGISTERS）。

请注意，许多cmake根服务器配置参数没有传递到Rust代码中。您可能需要在cantrip-os-rootserver和/或cantrip-os-model（cantrip-os-common）的Cargo.toml中调整功能。

默认情况下，cantrip-os-rootserver在启动时会打印有关capDL规范的信息。如果您想启用详细日志记录，请在Cargo.toml中启用LOG_DEBUG或LOG_TRACE。

### [下一章: 依赖CantripOS的Rust crates](CrateDependencies.md)