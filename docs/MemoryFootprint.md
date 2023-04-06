<!--
## Memory footprint

A release build for Sparrow fits in ~1.5MiB of memory and boots to
a running system in 4MiB.
The boostrap mechanism (using capDL and the rootserver) actually
requires ~2x the idle memory footprint to reach a running state
(due to the overhead of the rootserver instantiating the system).
The kmem.sh script can be used to analyze memory use. System
services should easily fit in 1MiB but due to CAmkES overhead
(e.g. per-thread cost) are significantly bloated. We use kmem and the
[bloaty tool](https://github.com/google/bloaty) to evaluate memory use.

To reduce memory use to <1MiB we are replacing the
CAmkES' runtime by a native Rust framework.
This should also improve performance and robustness by
extending the scope of the borrow checker and enabling the optimizer
to work across C <> Rust runtime boundaries that are a byproduct
of the CAmkES C-based implementation.
The RPC mechanism used for
communication between applications and the *SDKRuntime* is a prototype
of a native Rust implementation that demonstrates where we're headed.

### [Next Section: CantripOS capDL rootserver application](CantripRootserver.md)
-->

## 内存占用

Sparrow的发布构建适合占用约1.5MiB的内存，并在4MiB的内存中引导到运行系统。
引导机制（使用capDL和rootserver）实际上需要~2倍的空闲内存占用才能达到运行状态（因为rootserver实例化系统的开销）。
kmem.sh脚本可用于分析内存使用情况。系统服务应该轻松适应1MiB的内存空间，但由于CAmkES开销（例如，每个线程的成本），它们显著膨胀。
我们使用kmem和 [bloaty 工具](https://github.com/google/bloaty) 来评估内存使用情况。

为了将内存使用减少到<1MiB，我们正在将CAmkES的运行时替换为本机Rust框架。
这也应通过扩展借用检查器的范围并使优化器能够跨越CAmkES C <> Rust运行时边界工作来提高性能和鲁棒性，这是CAmkES基于C的实现的副产品。
用于应用程序和*SDKRuntime*之间通信的RPC机制是原生Rust实现的原型，展示了我们的发展方向。

### [下一章: CantripOS的capDL根服务器应用程序](CantripRootserver.md)