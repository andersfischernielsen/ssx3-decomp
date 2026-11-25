# Guest thread inventory

Measured on the gate replay (boot, menus, one race; 6000 frames) with
`SSX3_TRACE_SYSCALLS=1 SSX3_TRACE_THREADS=1 target/game/ssx3-headless 6000`
and `tools/thread_inventory.py` (plan: `../plans/guest-thread-removal.md`).
Thread ids are the kernel's: 1 is the main thread. The table lists seven
created threads (ids 2-8): "8 threads in a race" counts the main thread.

## What it shows

| id | thread | what it does in the replay |
| --- | --- | --- |
| 1 | main, `App_Run_MainLoop` (prio 1, later 100) | the frame pump: `PS2ExecMan_WaitFrame` (WaitSema/PollSema on sema 0x10), `PS2GraphicsMan_EndFrame` (sema 0x16/0x17), `EAThread_Yield` (RotateThreadReadyQueue, 82,852 calls), the `EAMutex`/`SND_Mutex` locks |
| 2 | `libkernl_IntrHelperThread` | waits once on sema 3 and never runs again |
| 3 | `EAFile_CdReadThread` | waits once on sema 0xd; native files serve reads without it |
| 4 | `libcdvd_CallbackThread` | waits once on sema 7 |
| 5 | `PS2ExecMan_WorkerThread` (prio 101) | waits on sema 0x11 only 8 times: it runs the long boot/load jobs (it creates the SND thread, `SND_IopInit`; `sceUsbKb*`, RPC binds) and takes the `EAMutex` locks |
| 6 | `PS2ExecMan_VSyncThread` (prio 99) | `WaitSema(0x12)` once per frame (signalled by `PS2ExecMan_VBlankSignal` in the VBlank-End ISR), then `App_VSyncTick` -> `SSXApp_Tick` (the game tick), then `PS2ExecMan_v05` signals the frame sema 0x10 |
| 7 | `Renderer_DmaKickThread` | waits once on sema 0x16 and never runs again (the native renderer bypasses the DMA kick) |
| 8 | `SND_EeThread` (prio 6) | `WaitSema(0x19)` about twice per frame (signalled from `SND_SifCmdHandler`, a SIF command ISR), then the sound pump under `SND_MutexLock` and the `EAMutex` locks |

A race frame is three threads and no other scheduling (frames 5500-5502 below):
VBlank-End ISR wakes thread 6 (the tick), which signals the frame sema and
hands to the main thread 1 (render, `DeferMan_SortBuckets`, EndFrame), and the
SND thread 8 runs once or twice for a sound pump. The CPU is idle in between.
Threads 2, 3, 4, 7 never run after boot; 5 runs the loading jobs.

Consequences for the plan:
- The threads that matter in the race are 1, 6 and 8. 6 is the easiest
  (ISR -> tick -> signal), 8 is ISR-driven too. The main thread's wait for the
  frame becomes "run the tick" (what thread 6 does).
- Locks (`EAMutex_Lock`/`SND_MutexLock`, hundreds of thousands per run) are
  semaphores that never block once only one flow of control remains: they can
  become counters, then no-ops.
- `EAThread_Yield` (RotateThreadReadyQueue) in thread 1: the yield loops need
  a look (what they wait for).
- Thread 5's loading jobs are the part with deep blocking (RPC binds to the
  IOP, CD reads); the boot/menu/load phases need their own trace pass.


Trace `/tmp/thr/full.txt`, 6006 frames.

## Threads

| id | entry | prio | stack | created by | note |
| --- | --- | --- | --- | --- | --- |
| 2 | `libkernl_IntrHelperThread` (0x424aa0) | 0 | 0x52afa0+0x400 | `libkernl_InitThreadHelper` (thread 1) | |
| 3 | `EAFile_CdReadThread` (0x3e3be0) | 12 | 0x51ac80+0x4000 | `EAFile_CdInit` (thread 1) | |
| 4 | `libcdvd_CallbackThread` (0x4009a8) | 11 | 0x51a480+0x800 | `sceCdInitEeCB` (thread 1) | |
| 5 | `PS2ExecMan_WorkerThread` (0x31ac60) | 101 | 0x6088d0+0x10000 | `PS2ExecMan_Init` (thread 1) | |
| 6 | `PS2ExecMan_VSyncThread` (0x31ac08) | 99 | 0x6048c0+0x4000 | `PS2ExecMan_Init` (thread 1) | |
| 7 | `Renderer_DmaKickThreadEntry` (0x382740) | 5 | 0x621540+0x1000 | `Renderer_Init` (thread 1) | |
| 8 | `SND_EeThread` (0x3c19a8) | 6 | 0x50cb20+0x6000 | `SND_IopInit` (thread 5) | |

## Thread-relevant syscalls by site

thread, syscall, calling function, semaphore argument: count (first cycle).

| thread | syscall | caller | arg | count | first cycle |
| --- | --- | --- | --- | --- | --- |
| 0 | iSignalSema | `SND_SifCmdHandler` | 00000019 | 8133 | 290969520 |
| 0 | iSignalSema | `PS2ExecMan_VBlankSignal` | 00000012 | 5704 | 187376080 |
| 0 | iSignalSema | `PS2GraphicsMan_OnVBlankStart` | 00000016 | 5703 | 196804634 |
| 0 | iSignalSema | `EAFile_AlarmHandler` | 0000000d | 20 | 9829647 |
| 0 | iWakeupThread | `iWakeupThread` |  | 6 | 17462324596 |
| 1 | ChangeThreadPriority | `libkernl_InitThreadHelper` |  | 1 | 276989 |
| 1 | ChangeThreadPriority | `EAThread_InitMain` |  | 1 | 708441 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffcae0 | 15 | 36781215 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffcac0 | 15 | 36781820 |
| 1 | CreateSema | `EAFile_DelayTicks` | 01ffcbe0 | 15 | 36782964 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffcb40 | 15 | 45762629 |
| 1 | CreateSema | `sceSifCallRpc` | 01fffa10 | 7 | 658589 |
| 1 | CreateSema | `libcdvd_SemaInit` | 01fffa70 | 4 | 658914 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffd4a0 | 3 | 18814370 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffd480 | 3 | 18814975 |
| 1 | CreateSema | `EAFile_DelayTicks` | 01ffd5a0 | 3 | 18816119 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffd500 | 3 | 27795784 |
| 1 | CreateSema | `sceSifBindRpc` | 01fffba0 | 2 | 667183 |
| 1 | CreateSema | `sceSifCallRpc` | 01fffb90 | 2 | 667663 |
| 1 | CreateSema | `sceMcInit` | 01fffc20 | 2 | 702363 |
| 1 | CreateSema | `sceSifBindRpc` | 01fffc10 | 2 | 703384 |
| 1 | CreateSema | `sceSifBindRpc` | 01ffe8e0 | 2 | 817558 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffe890 | 2 | 818982 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffe8f0 | 2 | 819548 |
| 1 | CreateSema | `PS2ExecMan_Init` | 01fffc00 | 2 | 181381800 |
| 1 | CreateSema | `sceFs_FdSemaInit` | 01fff930 | 2 | 181383181 |
| 1 | CreateSema | `Renderer_Init` | 01fffaf0 | 2 | 181461127 |
| 1 | CreateSema | `libkernl_InitMallocSema` | 01fffd00 | 1 | 276691 |
| 1 | CreateSema | `libkernl_InitMallocSema` | 01fffd20 | 1 | 276695 |
| 1 | CreateSema | `libkernl_InitThreadHelper` | 01fffd00 | 1 | 276944 |
| 1 | CreateSema | `sceSifBindRpc` | 01fffc00 | 1 | 657593 |
| 1 | CreateSema | `sceSifBindRpc` | 01fffa60 | 1 | 658120 |
| 1 | CreateSema | `sceSifBindRpc` | 01fffa80 | 1 | 659227 |
| 1 | CreateSema | `sceSifCallRpc` | 01fffa70 | 1 | 659707 |
| 1 | CreateSema | `sceSifBindRpc` | 01fffae0 | 1 | 665634 |
| 1 | CreateSema | `sceSifBindRpc` | 01fffb80 | 1 | 666108 |
| 1 | CreateSema | `sceSifCallRpc` | 01fffb30 | 1 | 666577 |
| 1 | CreateSema | `sceSifBindRpc` | 01fffa10 | 1 | 671543 |
| 1 | CreateSema | `sceSifCallRpc` | 01fff9c0 | 1 | 671958 |
| 1 | CreateSema | `sceSifCallRpc` | 01fffb50 | 1 | 702987 |
| 1 | CreateSema | `sceSifCallRpc` | 01fffba0 | 1 | 708080 |
| 1 | CreateSema | `EAMutex_Init` | 01fffc70 | 1 | 708462 |
| 1 | CreateSema | `EAMutex_Init` | 01fffc50 | 1 | 709103 |
| 1 | CreateSema | `EAMutex_Init` | 01fffbb0 | 1 | 763062 |
| 1 | CreateSema | `sceSifBindRpc` | 01ffe8c0 | 1 | 816474 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffe870 | 1 | 816943 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffe8d0 | 1 | 818038 |
| 1 | CreateSema | `sceSifBindRpc` | 01ffe8b0 | 1 | 821230 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffe8a0 | 1 | 821818 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffe820 | 1 | 848521 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffe800 | 1 | 849126 |
| 1 | CreateSema | `EAFile_DelayTicks` | 01ffe920 | 1 | 850270 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffe880 | 1 | 9829935 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffde60 | 1 | 9830571 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffde40 | 1 | 9831176 |
| 1 | CreateSema | `EAFile_DelayTicks` | 01ffdf60 | 1 | 9832320 |
| 1 | CreateSema | `sceSifCallRpc` | 01ffdec0 | 1 | 18811985 |
| 1 | CreateSema | `EAFile_CdInit` | 01ffea10 | 1 | 181378876 |
| 1 | CreateSema | `EAMutex_Init` | 01fffc40 | 1 | 181380177 |
| 1 | CreateSema | `PS2ExecMan_Init` | 01fffbc0 | 1 | 181381698 |
| 1 | CreateSema | `PS2ExecMan_Init` | 01fffbe0 | 1 | 181381749 |
| 1 | CreateSema | `sceFs_SemaInit` | 01fff9a0 | 1 | 181382668 |
| 1 | CreateSema | `sceSifBindRpc` | 01fff900 | 1 | 181382884 |
| 1 | CreateSema | `sceSifCallRpc` | 01fff8b0 | 1 | 181383608 |
| 1 | CreateSema | `sceSifCallRpc` | 01fffaa0 | 1 | 181385044 |
| 1 | CreateSema | `sceSifCallRpc` | 01fffb00 | 1 | 187213434 |
| 1 | CreateSema | `EAMutex_Init` | 01ff7410 | 1 | 17686100819 |
| 1 | DeleteSema | `sceSifCallRpc` | 0000000d | 67 | 817196 |
| 1 | DeleteSema | `EAFile_DelayTicks` | 0000000d | 20 | 9829651 |
| 1 | DeleteSema | `sceSifCallRpc` | 00000008 | 10 | 659960 |
| 1 | DeleteSema | `sceSifBindRpc` | 00000008 | 5 | 659480 |
| 1 | DeleteSema | `sceSifBindRpc` | 0000000d | 4 | 816727 |
| 1 | DeleteSema | `sceSifBindRpc` | 0000000a | 3 | 702795 |
| 1 | DeleteSema | `sceSifCallRpc` | 0000000a | 3 | 703240 |
| 1 | DeleteSema | `sceSifBindRpc` | 00000004 | 2 | 657846 |
| 1 | DeleteSema | `sceSifCallRpc` | 00000016 | 2 | 181383861 |
| 1 | DeleteSema | `sceSifCallRpc` | 00000004 | 1 | 658842 |
| 1 | DeleteSema | `sceSifBindRpc` | 00000014 | 1 | 181383137 |
| 1 | DeleteSema | `sceSifCallRpc` | 00000018 | 1 | 187213687 |
| 1 | DeleteSema | `EAMutex_Destroy` | 0000001b | 1 | 17638869574 |
| 1 | DisableIntc | `DisableIntc` |  | 4 | 181382018 |
| 1 | EnableIntc | `EnableIntc` |  | 5 | 761409 |
| 1 | PollSema | `PS2ExecMan_WaitFrame` | 00000010 | 5952 | 187384092 |
| 1 | PollSema | `libcdvd_NCmdPreCheck` | 00000004 | 41 | 821048 |
| 1 | PollSema | `libcdvd_SCmdPreCheck` | 00000005 | 28 | 659054 |
| 1 | PollSema | `sceMcGetInfo` | 00000008 | 23 | 482600542 |
| 1 | PollSema | `sceCdDiskReady` | 00000006 | 22 | 818365 |
| 1 | PollSema | `sceMcGetDir` | 00000008 | 9 | 497361784 |
| 1 | PollSema | `sceMcOpen` | 00000008 | 3 | 521965108 |
| 1 | PollSema | `sceMcRead` | 00000008 | 3 | 526881602 |
| 1 | PollSema | `sceMcClose` | 00000008 | 3 | 531802030 |
| 1 | RotateThreadReadyQueue | `EAThread_Yield` |  | 82852 | 2219429629 |
| 1 | SetAlarm | `EAFile_DelayTicks` |  | 20 | 850274 |
| 1 | SignalSema | `SND_MutexUnlock` | 00000018 | 99493 | 290499086 |
| 1 | SignalSema | `EAMutex_Unlock` | 0000000b | 16448 | 710563 |
| 1 | SignalSema | `PS2GraphicsMan_EndFrame` | 00000017 | 5947 | 192324236 |
| 1 | SignalSema | `PS2GraphicsMan_EndFrame` | 00000016 | 5947 | 192324240 |
| 1 | SignalSema | `EAMutex_Unlock` | 0000001b | 3600 | 2642525025 |
| 1 | SignalSema | `EAMutex_Unlock` | 0000000c | 1560 | 187207572 |
| 1 | SignalSema | `EAMutex_Unlock` | 0000001a | 819 | 12355164416 |
| 1 | SignalSema | `EAMutex_Unlock` | 0000001c | 426 | 17643959836 |
| 1 | SignalSema | `EAMutex_Unlock` | 0000000e | 113 | 9914769130 |
| 1 | SignalSema | `sceMcSync` | 00000008 | 41 | 482601354 |
| 1 | SignalSema | `sceCdDiskReady` | 00000006 | 22 | 819259 |
| 1 | SignalSema | `libcdvd_NCmdDiskReady` | 00000004 | 20 | 849392 |
| 1 | SignalSema | `sceCdGetError` | 00000005 | 20 | 9830201 |
| 1 | SignalSema | `PS2ExecMan_v09` | 00000011 | 7 | 187212846 |
| 1 | SignalSema | `sceOpen` | 00000013 | 6 | 181384002 |
| 1 | SignalSema | `sceCdMmode` | 00000005 | 3 | 659982 |
| 1 | SignalSema | `sceCdReadClock` | 00000005 | 2 | 708355 |
| 1 | SignalSema | `__malloc_unlock` | 00000001 | 2 | 708905 |
| 1 | SignalSema | `sceCdGetDiskType` | 00000005 | 2 | 819814 |
| 1 | SignalSema | `sceMcInit` | 00000008 | 1 | 703256 |
| 1 | SignalSema | `sceCdGetToc` | 00000004 | 1 | 822519 |
| 1 | SignalSema | `sceFsInit` | 00000014 | 1 | 181383442 |
| 1 | SignalSema | `sceCdTrayReq` | 00000005 | 1 | 187213716 |
| 1 | StartThread | `PS2ExecMan_Init` |  | 2 | 181381858 |
| 1 | StartThread | `libkernl_InitThreadHelper` |  | 1 | 276981 |
| 1 | StartThread | `EAFile_CdInit` |  | 1 | 181378884 |
| 1 | StartThread | `sceCdInitEeCB` |  | 1 | 181378963 |
| 1 | StartThread | `Renderer_Init` |  | 1 | 181461425 |
| 1 | WaitSema | `SND_MutexLock` | 00000018 | 99493 | 290498924 |
| 1 | WaitSema | `EAMutex_Lock` | 0000000b | 16448 | 709509 |
| 1 | WaitSema | `PS2ExecMan_WaitFrame` | 00000010 | 5951 | 187214093 |
| 1 | WaitSema | `PS2GraphicsMan_EndFrame` | 00000017 | 5947 | 192324232 |
| 1 | WaitSema | `EAMutex_Lock` | 0000001b | 3600 | 2642524970 |
| 1 | WaitSema | `EAMutex_Lock` | 0000000c | 1560 | 187207495 |
| 1 | WaitSema | `EAMutex_Lock` | 0000001a | 819 | 12355164361 |
| 1 | WaitSema | `EAMutex_Lock` | 0000001c | 426 | 17643959781 |
| 1 | WaitSema | `EAMutex_Lock` | 0000000e | 113 | 9914769083 |
| 1 | WaitSema | `sceSifCallRpc` | 0000000d | 67 | 817192 |
| 1 | WaitSema | `EAFile_DelayTicks` | 0000000d | 20 | 850278 |
| 1 | WaitSema | `sceSifCallRpc` | 00000008 | 10 | 659956 |
| 1 | WaitSema | `sceFs_Lock` | 00000013 | 6 | 181382675 |
| 1 | WaitSema | `sceSifBindRpc` | 00000008 | 5 | 659476 |
| 1 | WaitSema | `sceSifBindRpc` | 0000000d | 4 | 816723 |
| 1 | WaitSema | `sceSifBindRpc` | 0000000a | 3 | 702791 |
| 1 | WaitSema | `sceSifCallRpc` | 0000000a | 3 | 703236 |
| 1 | WaitSema | `sceSifBindRpc` | 00000004 | 2 | 657842 |
| 1 | WaitSema | `__malloc_lock` | 00000001 | 2 | 708504 |
| 1 | WaitSema | `sceSifCallRpc` | 00000016 | 2 | 181383857 |
| 1 | WaitSema | `sceSifCallRpc` | 00000004 | 1 | 658838 |
| 1 | WaitSema | `sceMcInit` | 00000008 | 1 | 702408 |
| 1 | WaitSema | `sceSifBindRpc` | 00000014 | 1 | 181383133 |
| 1 | WaitSema | `sceFsInit` | 00000014 | 1 | 181383192 |
| 1 | WaitSema | `sceSifCallRpc` | 00000018 | 1 | 187213683 |
| 1 | iSignalSema | `SND_SifCmdHandler` | 00000019 | 1072 | 335016339 |
| 1 | iSignalSema | `sceSifRpc_RequestEnd` | 0000000d | 71 | 816683 |
| 1 | iSignalSema | `libcdvd_CallbackCmdHandler` | 00000004 | 20 | 850144 |
| 1 | iSignalSema | `sceSifRpc_RequestEnd` | 00000008 | 15 | 659436 |
| 1 | iSignalSema | `PS2ExecMan_VBlankSignal` | 00000012 | 12 | 181382364 |
| 1 | iSignalSema | `sceSifRpc_RequestEnd` | 0000000a | 6 | 702751 |
| 1 | iSignalSema | `PS2GraphicsMan_OnVBlankStart` | 00000016 | 4 | 2223892027 |
| 1 | iSignalSema | `sceSifRpc_RequestEnd` | 00000004 | 3 | 657802 |
| 1 | iSignalSema | `sceSifRpc_RequestEnd` | 00000016 | 2 | 181383818 |
| 1 | iSignalSema | `sceSifRpc_RequestEnd` | 00000014 | 1 | 181383093 |
| 1 | iSignalSema | `sceSifRpc_RequestEnd` | 00000018 | 1 | 187213644 |
| 2 | WaitSema | `libkernl_IntrHelperThread` | 00000003 | 1 | 277021 |
| 3 | WaitSema | `EAFile_CdReadThread` | 0000000d | 1 | 181378908 |
| 4 | WaitSema | `libcdvd_CallbackThread` | 00000007 | 1 | 181378993 |
| 5 | ChangeThreadPriority | `SND_IopInit` |  | 1 | 272090410 |
| 5 | ChangeThreadPriority | `SSXApp_SetMainThreadPriority` |  | 1 | 290498410 |
| 5 | CreateSema | `sceSifCallRpc` | 00617ed0 | 14 | 192337727 |
| 5 | CreateSema | `sceSifCallRpc` | 00617ec0 | 2 | 241545986 |
| 5 | CreateSema | `sceSifCallRpc` | 00618020 | 1 | 192351404 |
| 5 | CreateSema | `sceSifBindRpc` | 00618100 | 1 | 241569275 |
| 5 | CreateSema | `sceSifCallRpc` | 00618000 | 1 | 241569763 |
| 5 | CreateSema | `sceSifBindRpc` | 00617f90 | 1 | 241570745 |
| 5 | CreateSema | `sceUsbKbInit` | 00618090 | 1 | 241571020 |
| 5 | CreateSema | `SND_MutexInit` | 0050ace0 | 1 | 272086037 |
| 5 | CreateSema | `SND_IopInit` | 0050acb0 | 1 | 272089445 |
| 5 | CreateSema | `sceSifBindRpc` | 00617f10 | 1 | 272089590 |
| 5 | CreateSema | `sceSifBindRpc` | 00617f40 | 1 | 272094197 |
| 5 | CreateSema | `sceSifCallRpc` | 00617e70 | 1 | 272094956 |
| 5 | CreateSema | `EAMutex_Init` | 00617e10 | 1 | 274974161 |
| 5 | CreateSema | `EAMutex_Init` | 00617400 | 1 | 2150597554 |
| 5 | CreateSema | `EAMutex_Init` | 00618430 | 1 | 17322346729 |
| 5 | CreateSema | `sceSifCallRpc` | 006177c0 | 1 | 17497705617 |
| 5 | DeleteSema | `sceSifCallRpc` | 00000018 | 17 | 192337980 |
| 5 | DeleteSema | `sceSifBindRpc` | 00000018 | 2 | 241569528 |
| 5 | DeleteSema | `sceSifBindRpc` | 0000001a | 2 | 272089843 |
| 5 | DeleteSema | `sceSifCallRpc` | 0000001a | 2 | 272090316 |
| 5 | DeleteSema | `sceUsbKbInit` | 00000018 | 1 | 241571584 |
| 5 | DeleteSema | `sceSifCallRpc` | 0000001d | 1 | 17497705870 |
| 5 | DisableIntc | `DisableIntc` |  | 1 | 272094533 |
| 5 | EnableIntc | `EnableIntc` |  | 1 | 272094588 |
| 5 | RotateThreadReadyQueue | `EAThread_Yield` |  | 39 | 17328559272 |
| 5 | SetAlarm | `THREAD_yieldticks` |  | 7 | 192402382 |
| 5 | SignalSema | `EAMutex_Unlock` | 0000000b | 5836 | 241570542 |
| 5 | SignalSema | `EAMutex_Unlock` | 0000000c | 5246 | 241572263 |
| 5 | SignalSema | `EAMutex_Unlock` | 0000000e | 660 | 17456429232 |
| 5 | SignalSema | `EAMutex_Unlock` | 0000001c | 627 | 17322760786 |
| 5 | SignalSema | `SND_MutexUnlock` | 00000018 | 26 | 272086348 |
| 5 | SignalSema | `sceUsbKbSync` | 00000018 | 1 | 241571568 |
| 5 | SleepThread | `THREAD_yieldticks` |  | 7 | 192402390 |
| 5 | StartThread | `SND_IopInit` |  | 1 | 272090377 |
| 5 | WaitSema | `EAMutex_Lock` | 0000000b | 5836 | 241570134 |
| 5 | WaitSema | `EAMutex_Lock` | 0000000c | 5246 | 241572186 |
| 5 | WaitSema | `EAMutex_Lock` | 0000000e | 660 | 17456429185 |
| 5 | WaitSema | `EAMutex_Lock` | 0000001c | 627 | 17322760731 |
| 5 | WaitSema | `SND_MutexLock` | 00000018 | 26 | 272086082 |
| 5 | WaitSema | `sceSifCallRpc` | 00000018 | 17 | 192337976 |
| 5 | WaitSema | `PS2ExecMan_WorkerThread` | 00000011 | 8 | 187214110 |
| 5 | WaitSema | `sceSifBindRpc` | 00000018 | 2 | 241569524 |
| 5 | WaitSema | `sceSifBindRpc` | 0000001a | 2 | 272089839 |
| 5 | WaitSema | `sceSifCallRpc` | 0000001a | 2 | 272090312 |
| 5 | WaitSema | `sceUsbKbGetInfo` | 00000018 | 1 | 241571055 |
| 5 | WaitSema | `sceUsbKbSync` | 00000018 | 1 | 241571527 |
| 5 | WaitSema | `sceSifCallRpc` | 0000001d | 1 | 17497705866 |
| 5 | iSignalSema | `SND_SifCmdHandler` | 00000019 | 75 | 272095274 |
| 5 | iSignalSema | `PS2ExecMan_VBlankSignal` | 00000012 | 53 | 246417824 |
| 5 | iSignalSema | `PS2GraphicsMan_OnVBlankStart` | 00000016 | 51 | 246005832 |
| 5 | iSignalSema | `sceSifRpc_RequestEnd` | 00000018 | 20 | 192337937 |
| 5 | iSignalSema | `sceSifRpc_RequestEnd` | 0000001a | 4 | 272089799 |
| 5 | iSignalSema | `sceSifRpc_RequestEnd` | 0000001d | 1 | 17497705827 |
| 6 | CreateSema | `sceSifCallRpc` | 006083c0 | 4 | 187378090 |
| 6 | CreateSema | `sceSifCallRpc` | 00608360 | 2 | 182456375 |
| 6 | DeleteSema | `sceSifCallRpc` | 00000018 | 6 | 182456628 |
| 6 | SignalSema | `PS2ExecMan_v05` | 00000010 | 5952 | 187384072 |
| 6 | WaitSema | `PS2ExecMan_VSyncThread` | 00000012 | 5965 | 181381981 |
| 6 | WaitSema | `sceSifCallRpc` | 00000018 | 6 | 182456624 |
| 6 | iSignalSema | `SND_SifCmdHandler` | 00000019 | 18 | 2332550148 |
| 6 | iSignalSema | `sceSifRpc_RequestEnd` | 00000018 | 6 | 182456585 |
| 6 | iWakeupThread | `iWakeupThread` |  | 1 | 241508280 |
| 7 | WaitSema | `Renderer_DmaKickThread` | 00000016 | 1 | 181461499 |
| 8 | SignalSema | `SND_MutexUnlock` | 00000018 | 9297 | 275246752 |
| 8 | SignalSema | `EAMutex_Unlock` | 0000001b | 3458 | 2672341539 |
| 8 | SignalSema | `EAMutex_Unlock` | 0000001a | 672 | 12386355535 |
| 8 | SignalSema | `EAMutex_Unlock` | 0000000c | 328 | 2769859130 |
| 8 | WaitSema | `SND_EeThread` | 00000019 | 9299 | 272090402 |
| 8 | WaitSema | `SND_MutexLock` | 00000018 | 9297 | 275240933 |
| 8 | WaitSema | `EAMutex_Lock` | 0000001b | 3458 | 2672341504 |
| 8 | WaitSema | `EAMutex_Lock` | 0000001a | 672 | 12386355500 |
| 8 | WaitSema | `EAMutex_Lock` | 0000000c | 328 | 2769859053 |
| 8 | iSignalSema | `PS2ExecMan_VBlankSignal` | 00000012 | 195 | 2140661744 |
| 8 | iSignalSema | `PS2GraphicsMan_OnVBlankStart` | 00000016 | 189 | 1766321332 |

## Context switches per frame

Frames 0..6006; in the second half: min 1, max 245 per frame.


## Schedule, frames 5500..5503

| frame | cycle | from | to | resumes in |
| --- | --- | --- | --- | --- |
| 5500 | 27033615343 | 0 | idle | x118 (idle ticks, no thread ready) |
| 5500 | 27035948985 | 0 | 8 | `WaitSema` |
| 5500 | 27036030844 | 8 | idle | `idle` |
| 5500 | 27036035817 | 0 | idle | x21 (idle ticks, no thread ready) |
| 5500 | 27036443637 | 0 | 6 | `WaitSema` |
| 5500 | 27036451311 | 6 | 1 | `WaitSema` |
| 5500 | 27037388045 | 1 | idle | `idle` |
| 5500 | 27037408045 | 0 | idle | x85 (idle ticks, no thread ready) |
| 5501 | 27039094713 | 0 | 8 | `WaitSema` |
| 5501 | 27039369301 | 8 | idle | `idle` |
| 5501 | 27039389301 | 0 | idle | x100 (idle ticks, no thread ready) |
| 5501 | 27041363752 | 0 | 6 | `WaitSema` |
| 5501 | 27041371426 | 6 | 1 | `WaitSema` |
| 5501 | 27042240449 | 1 | 8 | `WaitSema` |
| 5501 | 27042468065 | 8 | 1 | `DeferMan_SortBuckets` |
| 5501 | 27042517354 | 1 | idle | `idle` |
| 5501 | 27042517355 | 0 | idle | x144 (idle ticks, no thread ready) |
| 5502 | 27045386169 | 0 | 8 | `WaitSema` |
| 5502 | 27045550777 | 8 | idle | `idle` |
| 5502 | 27045570777 | 0 | idle | x37 (idle ticks, no thread ready) |
| 5502 | 27046283867 | 0 | 6 | `WaitSema` |
| 5502 | 27046291541 | 6 | 1 | `WaitSema` |
| 5502 | 27047197735 | 1 | idle | `idle` |
| 5502 | 27047217735 | 0 | idle | x57 (idle ticks, no thread ready) |
