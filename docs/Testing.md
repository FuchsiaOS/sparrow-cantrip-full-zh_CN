<!--

## Testing support

CantripOS testing support is a work in progress. Tests break down into
the following areas:

- *unit tests*: Rust cargo unit tests
- *sel4test*: seL4's sel4test framework
- *shell tests*: tests built into the CantripOS shell
- *application tests*: test applications (mostly to exercise the SDK)
- *robot tests*: automated tests that leverage the CantripOS shell

-->

## 测试支持

CantripOS的测试支持是一个正在进行中的工作。测试分为以下几个领域：

- *单元测试：Rust Cargo单元测试
- *sel4test：seL4的sel4test框架
- *shell测试：内置于CantripOS shell中的测试
- *应用测试：测试应用程序（主要是为了使用SDK）
- *机器人测试：利用CantripOS shell的自动化测试

<!--

### Unit tests

Unit tests excercise functional interfaces with tests that run on the
build system (aka the "host").
These are all cargo-based and meant to be fast enough to run as part of
a pre-submit process.
The available tests can be found with the hmm command or using tab
completion:

-->

### 单元测试

单元测试通过在构建系统（也称为“主机”）上运行的测试来执行功能接口。
这些都是基于Cargo的，旨在足够快以便于作为预提交过程的一部分运行。
可以使用hmm命令或使用tab键补全查找可用的测试：

``` shell
$ hmm cargo_test_<TAB>
cargo_test_cantrip                           cargo_test_cantrip_os_common_logger          cargo_test_cantrip_os_common_slot_allocator
cargo_test_cantrip_proc_interface            cargo_test_cantrip_proc_manager              cargo_test_debugconsole_zmodem
$ m cargo_test_cantrip
   ...
   Compiling memchr v2.5.0
   ...

running 5 tests
test tests::test_each_log_level_works ... ok
test tests::test_embedded_nul ... ok
test tests::test_formatting ... ok
test tests::test_max_log_level ... ok
test tests::test_too_long ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...

```

<!--
The main impediment to unit tests is structuring code so that
platform-independent code can be exercised in an isolated/host environment.
Expect the set of unit tests to grow as more code is structured with
unit testing in mind.

-->

单元测试的主要障碍在于构建代码，以便可以在隔离/主机环境中执行与平台无关的代码。
随着更多的代码以单元测试为目的进行结构化，预计单元测试的集合会增加。

<!--

### sel4test

sel4test is the seL4 kernel test facility that substitutes a test
harness for the usual rootserver and then automatically runs a suite
of tests that exercises system call api's and checks operational
correctness.
This facility is supported with two make targets:

-->

### sel4test

sel4test是seL4内核测试设施，它用测试测试哈尼斯替代通常的根服务器，
然后自动运行一组测试，以测试系统调用api并检查操作正确性。
此设施通过两个make目标进行支持：

``` shell
$ hmm sel4test

sel4test: (defined in build/platforms/sparrow/sim_sel4test.mk)
 C-based libsel4 syscall api wrappers. The result is run under Renode.

$ hmm sel4test+wrapper

sel4test+wrapper: (defined in build/platforms/sparrow/sim_sel4test.mk)
 crate wrapped with C shims. The result is run under Renode.
```

<!--

The first command runs the upstream seL4 test mechanism unchanged;
this mostly verifies the CantripOS kernel (which follows upstream
seL4 but has some non-trivial changes).
The second command runs the upstream test mechanism but using the Rust sel4-sys
crate with wrappers around the Rust implementations for use by C code;
this is mostly intended to exercise sel4-sys.

Note the sel4test target uses a debug build; this is consistent with
how upstream works. The sel4test+wrapper target however uses a release build of
the user mode pieces to reduce the space overhead of the Rust wrappers.

Not all target platforms may support the above make targets.

-->

第一个命令未更改上游seL4测试机制的运行方式；
这主要验证CantripOS内核（遵循上游seL4但具有一些非常规更改）。
第二个命令使用Rust sel4-sys crate作为包装器，用于C代码使用Rust实现，
主要用于测试sel4-sys。

请注意，sel4test目标使用调试构建；这与上游工作方式一致。
但是，sel4test+wrapper目标使用用户模式部分的释放构建，以减少Rust包装器的空间开销。

并非所有的目标平台都支持上述make目标。

<!--

### Shell tests

Shell tests refers to builtin commands in the DebugConsole that exercise parts
of the system.
By convention these have a "test_" prefix; e.g.

-->

### Shell 测试

Shell测试是指在DebugConsole中内置命令，用于测试系统的各个部分。
按照惯例，这些命令都以“test_”前缀开头，例如：

``` shell
CANTRIP> ?
...
test_alloc
test_alloc_error
test_mailbox
test_malloc
test_mfree
test_mlcancel
test_mlexecute
test_mlperiodic
test_obj_alloc
test_panic
test_timer_async
test_timer_blocking
test_timer_completed
CANTRIP> test_obj_alloc
32 bytes in-use, 63639264 bytes free, 32 bytes requested, 3342336 overhead
2 objs in-use, 2 objs requested
32 bytes in-use, 63639264 bytes free, 13648 bytes requested, 3342336 overhead
2 objs in-use, 11 objs requested
Batch alloc ok: ObjDescBundle { cnode: 43, depth: 7, objs: [ObjDesc { type_: seL4_TCBObject, count: 1, cptr: 0 }, ObjDesc { type_: seL4_EndpointObject, count: 2, cptr: 1 }, ObjDesc { type_: seL4_ReplyObject, count: 2, cptr: 3 }, ObjDesc { type_: seL4_SchedContextObject, count: 8, cptr: 5 }, ObjDesc { type_: seL4_RISCV_4K_Page, count: 10, cptr: 6 }] }
cantrip_object_alloc_in_cnode ok: ObjDescBundle { cnode: 43, depth: 5, objs: [ObjDesc { type_: seL4_TCBObject, count: 1, cptr: 0 }, ObjDesc { type_: seL4_EndpointObject, count: 1, cptr: 1 }, ObjDesc { type_: seL4_ReplyObject, count: 1, cptr: 2 }, ObjDesc { type_: seL4_SchedContextObject, count: 8, cptr: 3 }, ObjDesc { type_: seL4_RISCV_4K_Page, count: 2, cptr: 4 }] }
All tests passed!
```

<!--

Shell commands bloat a system image so are conditionally compiled in
(in particular release builds do not include any test commands).
Check `DebugConsole/cantrip-shell/Cargo.toml` for features named "TEST_*".
These control the set of test commands, some of which are platform-dependent.
Beware that some builtin tests may generate assertions that will kill the
console shell; e.g.

-->

Shell命令会增加系统映像的大小，因此它们是有条件编译的（特别是发布版本不包括任何测试命令）。
请查看`DebugConsole/cantrip-shell/Cargo.toml`文件中名为 "TEST_*" 的功能。
这些功能控制测试命令的集合，其中一些依赖于平台。
请注意，一些内置测试可能会生成断言，导致控制台shell崩溃，例如：

``` shell
CANTRIP> test_panic
panic::panicked at 'testing', cantrip-shell/src/test_panic.rs:34:5
```

<!--

### Application tests

There are several applications designed to exercise/test the SDKRuntime.
These are typically included in the builtins archive baked into a system image.
For example,

-->

### 应用测试

有几个应用程序旨在对 SDKRuntime 进行测试和验证。
这些应用程序通常包含在构建成的内置档案中，一起打包到系统映像中。
例如，

``` shell
CANTRIP> builtins
fibonacci.app 30852
hello.app 580
keyval.app 31040
logtest.app 26100
mltest.app 29832
mobilenet_v1_emitc_static.model 1001090
panic.app 24812
suicide.app 551
timer.app 3121
CANTRIP> install logtest.app
Collected 26100 bytes of data, crc32 8673c4b7
Application "logtest" installed
CANTRIP> start logtest
Bundle "logtest" started.
CANTRIP> [logtest]::logtest::ping!
[logtest]::DONE
stop logtest
Bundle "logtest" stopped.
CANTRIP> uninstall logtest
Bundle "logtest" uninstalled.
```

<!--

Unlike a shell builtin an application that dies can just be stopped and unloaded.
Note multiple applications can be run simultaneously (depending on available
resources) to exercise concurrent use of the SDKRuntime.
For example,

-->

与 shell 内置命令不同，一个崩溃的应用程序只需要停止并卸载即可。
注意，多个应用程序可以同时运行（取决于可用资源），以测试 SDKRuntime 的并发使用。

例如，

``` shell
CANTRIP> install timer.app
Collected 31212 bytes of data, crc32 8d3381c0
Application "timer" installed
CANTRIP> start fibonacci
Bundle "fibonacci" started.
CANTRIP> [fibonacci]::fibonacci::[ 0]                    0  0
[fibonacci]::fibonacci::[ 1]                    1  100
[fibonacci]::fibonacci::[ 2]                    1  200
s[fibonacci]::fibonacci::[ 3]                    2  300
t[fibonacci]::fibonacci::[ 4]                    3  400
ar[fibonacci]::fibonacci::[ 5]                    5  500
t [fibonacci]::fibonacci::[ 6]                    8  600
tim[fibonacci]::fibonacci::[ 7]                   13  700
er[fibonacci]::fibonacci::[ 8]                   21  800

[fibonacci]::fibonacci::[ 9]                   34  900
...
Bundle "timer" started.
CANTRIP> [fibonacci]::fibonacci::[26]               121393  2600
[timer]::timer::sdk_timer_cancel returned Err(SDKInvalidTimer) with nothing running
[timer]::timer::sdk_timer_poll returned Ok(0) with nothing running
[timer]::timer::sdk_timer_oneshot returned Err(SDKNoSuchTimer) with an invalid timer id
[timer]::timer::Timer 0 started
[fibonacci]::fibonacci::[27]               196418  2700
[fibonacci]::fibonacci::[28]               317811  2800
[timer]::timer::Timer 0 completed
[timer]::timer::Timer 1 started
[fibonacci]::fibonacci::[29]               514229  2900

```

<!--

By default, platforms without an interactive shell include an
`autostart.repl` script in their builtins bundle that runs the available applications.
Systems that have an interactive command line have a *builtins.repl* file that does
the same thing and can be run with the "source" command; e.g.

-->

默认情况下，没有交互式Shell的平台会在其内置包中包含一个名为autostart.repl的脚本，
用于运行可用的应用程序。拥有交互式命令行的系统有一个名为builtins.repl的文件，
也可以使用"source"命令运行，执行与自动启动脚本相同的功能；例如：

```shell
CANTRIP> builtins
builtins.repl 642
...
CANTRIP> source builtins.repl
CANTRIP> install hello.app
Collected 640 bytes of data, crc32 877a95c1
Application "hello" installed
...
```

<!--

### Robot tests

Robot tests refer to system-level tests that treat the system as a black box.
Typically these are automated and used for regression testing.
The sel4test mechanism can be used for this purpose as can many of the shell
and application tests described above.
The scripts we use with [Renode's Robot framework](https://renode.readthedocs.io/en/latest/introduction/testing.html)
are included in the *sim/tests* directory.

-->

### 机器人测试

机器人测试是指将系统视为黑盒子进行的系统级测试。
通常这些测试都是自动化的，用于回归测试。
sel4test机制以及上面描述的许多shell和应用程序测试都可以用于此目的。
我们在sim/tests目录中包含了使用[Renode的机器人框架](https://renode.readthedocs.io/en/latest/introduction/testing.html)的脚本。

<!--

### [Next Section: Memory Footprint](MemoryFootprint.md)

-->

### [下一章: 内存占用](MemoryFootprint.md)
