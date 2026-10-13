# Boot path: `entry` to the main loop (SLUS_207.72)

Static analysis in Ghidra (GhidraMCP) plus a raw-ELF scan of `orig/SLUS_207.72`
(one PT_LOAD: file 0x1000 maps to VA 0x100000, filesz 0x3a4bf4, memsz 0x43eadc).
gp = `0x004a30f0`. The functions named below have been renamed in the Ghidra project.
Confidence: "sure" means the code matches the Sony SDK or the call shape is unique.
Names ending in `?` are best guesses.

The goal is to give a native runtime what it needs to HLE the EE kernel and the IOP.
That means the syscall table (section 2), the RPC server IDs and module order
(section 3), the threads, semaphores and interrupt handlers (sections 4 and 5),
and the GS/DMA and file-I/O contact points (sections 6 and 7).

---

## 1. entry, crt0, main

`entry` @ `0x00100008` (Sony crt0):

| step | addr | what |
|---|---|---|
| zero GPRs, FPRs, HI/LO/SA, fcsr | 0x100008-0x100118 | |
| clear BSS | 0x10011c | `sq zero` loop over `[0x004a4c00, 0x0053f115)`. `_end` = 0x53f115 |
| `InitMainThread` (`SetupThread`, syscall 0x3C) | 0x100174 | a0=gp `0x4a30f0`, a1=stack `0x01fe0000`, a2=stack size `0x20000`, a3=`_args` `0x4a5c00`, t0=root ra `0x1001d0`. Returned v0 becomes sp |
| `InitHeap` (`SetupHeap`, 0x3D) | 0x100190 | a0=`0x53f115` (`_end`), a1=-1 (heap runs to the stack) |
| `_InitSys` | jal `0x42c300` | libkernl kernel patches, see below |
| `FlushCache(0)` | jal `0x424020` | |
| `ei` | 0x1001a8 | |
| `main(argc=_args[0], argv=&_args[1])` | jal `0x31af80` **`ssx_main`** | |
| `exit(v0)` | j `0x42c6e8` | runs atexit (`0x42cd58`), then `_Exit` (syscall 4) |
| root `ra` stub | `0x1001d0` | `li v1,0x23; syscall` = ExitThread |

`_InitSys` @ `0x42c300` calls these in order. All are Sony libkernl kernel-patch code.
An HLE kernel can treat them as no-ops if `SetSyscall` / `0x5A` / `FindAddress` are benign:
- `0x42c0d8`: creates 2 semaphores (globals `0x455248` and `0x45524c`). These are libkernl internal locks.
- `0x42c1f0`: two `SetSyscall` calls, then `FindAddress` (0x83) scans of kernel RAM `0x80000000-0x80080000`.
- `0x42c7c8`: if `RCNT3_MODE & 0x100` is clear, it copies 0x740 bytes to `0x80076000` and 0x28 bytes to `0x82000` through hijacked syscall 0x5A, then `SetSyscall` rewires slots (ExecPS2/LoadExecPS2 patch).
- `0x424b78` **`libkernl_InitThreadHelper`**: creates a semaphore and a priority-0 helper thread `0x424aa0` (stack `0x52afa0`, size 0x400), starts it, then `ChangeThreadPriority(self, 1)`.
- `0x42c410`: if `0x42c3a8()` allows, copies 0x7a8 bytes to `0x80074000` (alarm patch; SetAlarm becomes syscall 0xFC/0xFD).
- `0x42cbd0`: copies 0x330 bytes to `0x80075000` and installs the TLB syscalls 0x55-0x59. It stores `GetEntryAddress(3)` in `0x456538`.

`ssx_main` @ `0x31af80`:
```
__main()                       0x40fc90  -> __do_global_ctors 0x40fbe0 (walks __CTOR_LIST__ @ 0x43ce38 backwards, runs once)
ssx_SystemInit()               0x31adb0
em = EA_MemAllocNamed(0x142c0, "PS2ExecutionMan")   0x317d70
PS2ExecMan ctor 0x317458(em); em->vtbl = 0x48db68
App_Run_MainLoop(g_App /*gp-0x2a74 -> 0x4a5b64*/, em, argc, argv)   0x316d60
```
`g_App` is set by `App_ctor` 0x316cd0 (base vtable `0x48dbe8`). That is called from
`SSXApp_ctor` 0x226830, which installs vtable **`0x47d9c8`** and is reached from the
static constructor `0x242eb8`. So the game App object exists before `main`. It is built by `__do_global_ctors`.

---

## 2. EE kernel syscall wrappers (complete scan)

Method: every `syscall` word (0x0000000C) in the PT_LOAD whose preceding 1-3
instructions load `$v1` with `addiu/ori v1,zero,N`. 168 sites in total. The canonical
libkernl stub block is `0x423980-0x424218`. Each stub is 16 bytes:
`li v1,N; syscall; jr ra; nop`. For these, **stub fn = syscall insn - 4**.
Many stubs have no Ghidra function because nothing references them. Those are marked `(no function)`.
Names follow the ps2sdk/Sony libkernl numbering. It differs from `docs/ps2-bible.md` in several places:
- 0x79/0x7A are `sceSifSetReg`/`sceSifGetReg`. They are not GetOsdConfigParam.
- 0x4C-0x4F are Get/SetGsH/VParam.
- 0x55-0x59 are the TLB calls installed by `_InitSys`.
- 0x5A/0x5B are RFU090/RFU091.

Notes for the HLE:
- Alarms are **patched**. `SetAlarm` = 0xFC, `ReleaseAlarm` = 0xFD, `iSetAlarm` = -0xFE, `iReleaseAlarm` = -0xFF. The original 0x18/0x19/-0x1E/-0x1F are unused.
- `AddIntcHandler` and `AddDmacHandler` each have two stubs (`AddIntcHandler`/`AddIntcHandler2`). Both use v1 0x10 or 0x12.
- `-0x2F` (iGetThreadId) appears inline in `0x424c50` and `0x424d68`.
- The game calls `EnableIntc`/`DisableIntc`/`EnableDmac`/`DisableDmac` through wrappers `0x4248e8`/`0x424880`/`0x4249b8`/`0x424950` (renamed). The wrappers pick `_EnableIntc` or `_iEnableIntc` etc. at runtime.
- The full per-site table is in the **Appendix** at the end of this file.
- `0x42c078` **`DIntr`** / `0x42c0c0` **`EIntr`** are libkernl's DI/EI helpers. They use COP0 Status, not a syscall.


---

## 3. SIF / IOP

### Library entry points (renamed)
| fn | addr | notes |
|---|---|---|
| sceSifInitRpc | `0x426408` | calls `sceSifInitCmd` 0x425d38, registers cmd handlers 0x80000008/9/A/C, handshakes through `sceSifGetReg/SetReg(0x80000002)` |
| sceSifInitCmd | `0x425d38` | |
| sceSifAddCmdHandler | `0x426020` | |
| sceSifSendCmd | `0x4261b0` | |
| sceSifBindRpc | `0x426b48` | (client, sid, mode) |
| sceSifCallRpc | `0x426d18` | (client, fno, mode, send, ssize, recv, rsize, end_fn) |
| sceSifInitIopHeap | `0x42ab18` | binds 0x80000003 |
| sceSifAllocIopHeap | `0x42aba0` | iopheap fno 1 |
| sceSifLoadFileInit | `0x42b068` | binds 0x80000006 (loadfile) |
| sceSifLoadModule | `0x42ba68` | (path, argc, args) |
| sceSifSearchModuleByName | `0x42b6d0` | used with "dev9" |
| sceSifRebootIop | `0x42bf68` | (image path) |
| sceSifSyncIop | `0x42bf30` | |
| sceFsInit (fileio bind) | `0x427a90` | binds **0x80000001**. Client struct `0x52ee00` |
| sceOpen / sceClose / sceLseek / sceRead / sceWrite | `0x427d60` / `0x427fe8` / `0x428168` / `0x4283a0` / `0x428600` | fileio fno 0/1/4/2/3. Further fileio stubs `0x4288c0..0x42a950` use fno 5, 7, 0xa-0x19 |
| sceCdInit | `0x401718` | binds **0x80000592**, calls fno 0 with mode |
| sceCdMmode | `0x402450` | S-cmd client `0x453180` |
| sceCdSearchFile | `0x400ea8` | binds **0x80000597** |
| sceCdRead | `0x401df8` | N-cmd client `0x4527d0` (**0x80000595**), fno 1 |
| sceCdSync | `0x4013e8` | |
| sceCdReadClock / sceCdGetDiskType | `0x402520` / `0x402268` | S-cmd fno 1 / 3. Disk type 0x12 (PS2 CD) and 0x14 (PS2 DVD) are accepted |
| sceCdInitEeCB / sceCdCallback / sceCdPOffCallback | `0x400a78` / `0x4008a0` / `0x400d58` | |
| (cdvd) | `0x4011e0` binds 0x80000595. `0x4014f8` binds **0x80000593** (S-cmd). `0x4019f8` binds **0x8000059A**. `0x401bf0` binds **0x8000059C** (called as `f(0)` before disk-type queries, probably sceCdDiskReady). `0x401fd8` = N-cmd fno 4 into a 4 KB buffer |
| scePadInit | `0x3ff4a8` | binds **0x80000100** and **0x80000101** (client `0x526200`) |
| scePadPortOpen / scePadRead | `0x3ff708` / `0x3ffa58` | PortOpen is RPC fno 1. Read copies the DMA buffer (no RPC) |
| sceMcInit | `0x409940` | binds **0x80000400** (client `0x5271c0`) |
| SND_IopInit (EA SNDDRV client) | `0x3c1b80` | binds sid **0x00534E44** ('SND') on client `0x50b980`. Adds **SIF cmd handler 1** -> `0x3c1578` via `sceSifSetCmdBuffer` 0x425ff0. Allocates 4 KB of IOP heap. A fallback path loads `host0:` libsd/snddrv |

### Module load order (cold boot)
`ssx_SystemInit` 0x31adb0 -> `ssx_IopInit` 0x319610:
1. `sceSifInitRpc(0)`, `sceSifInitIopHeap()`
2. `ssx_IopReboot` 0x319820: `sceSifInitRpc`, `sceCdInit(0)`, `sceCdMmode(2 /*DVD*/)`,
   then **`sceSifRebootIop("cdrom0:\DATA\MODULES\DNAS271.IMG;1")`**. **DNAS271.IMG is the IOPRP image.**
   Then loop `sceSifSyncIop`, `sceSifInitRpc(0)`, `sceSifInitIopHeap()`.
3. `sceCdInit(0)`, `sceCdMmode(2)`, `0x427d28` (zeroes fileio state `0x52ee28`)
4. back in `ssx_SystemInit`, `ssx_LoadIopModule` 0x319718 (path built by `ssx_BuildModulePath` 0x319690 as
   `"cdrom0:\" + "DATA\MODULES\" + name + ";1"`, retry while `-0xCC`):
   **SIO2MAN.IRX, PADMAN.IRX, LIBSD.IRX, SNDDRV.IRX, MCMAN.IRX, MCSERV.IRX** (all argc 0, mandatory)
5. `sceMcInit()` (0x80000400), `scePadInit(0)` (0x80000100/0x101), `sceCdReadClock` -> srand seed

The second stage is `InitLoadScreen_Enter` 0x232568 -> `SSXApp_BootLoad` 0x2282d0 -> **`ssx_LoadNetModules` 0x228ee0**.
In order (flag 1 = mandatory, otherwise "try"):
USBD.IRX (args 0x2b @0x47a998), USBKB.IRX (args), DEV9.IRX, then `ssx_InstallPowerOffHandler` 0x319930.
That function checks `sceSifSearchModuleByName("dev9")`; if found it creates a power-off thread 0x3198a0 and calls `sceCdPOffCallback(0x319910)`.
Then INET.IRX (args), NETCNF.IRX (args with "cdrom0:\..." icon paths), INETCTL.IRX (args), SMAP.IRX (try), PPP.IRX, PPPOE.IRX (try, args), SPDUART.IRX (if SMAP and PPPOE are ok).
`0x3e52b0(10)`, then if networking is present: MSIFRPC.IRX (`0x3197d0`), LIBNET.IRX, `0x40b130(0)`, DRTYSCKF.IRX, LGAUD.IRX (try, args), VOIPF.IRX (try).
If VOIPF loaded, `0x3f6a40` binds sid **0x237**; otherwise `0x3f6ab0`.
**SDRDRV.IRX on the disc is never referenced by name.**
Other RPC clients: `0x40c028` binds **0x80000211** (client 0x5290a0, fno 2-6, 8-slot device table, VBlankEnd handler `0x40cf88`; USB keyboard/headset family, unconfirmed).

---

## 4. Threads and semaphores

Priorities: the main thread starts at the InitMainThread default.
`libkernl_InitThreadHelper` drops it to **1**. `SND_IopInit` later sets it to **100** (`ChangeThreadPriority(GetThreadId(),100)`).
`PS2ExecMan_Init` reads the then-current priority P through `ReferThreadStatus`.

| creator | entry | prio | stack | purpose |
|---|---|---|---|---|
| `libkernl_InitThreadHelper` 0x424b78 | `0x424aa0` | 0 | 0x52afa0/0x400 | libkernl internal helper |
| `ssx_InstallPowerOffHandler` 0x319930 | `0x3198a0` | 1 | 0x538b10/0x2000 | power-off (waits sema) |
| `PS2ExecMan_Init` 0x31a6d8 | **`PS2ExecMan_WorkerThread` 0x31ac60** | P+1 | em+0x4040/0x10000 | runs queued jobs. Waits sema em+0x14044, which is signalled by `0x31acd8` |
| `PS2ExecMan_Init` | **`PS2ExecMan_VSyncThread` 0x31ac08** | P-1 | em+0x30/0x4000 | waits sema em+0x4034 (signalled from VBlank-End ISR). Calls `0x317500` -> `App_VSyncTick(g_App)` |
| `Renderer_Init` 0x375a40 | `0x382740` -> **`Renderer_DmaKickThread` 0x382760** | 5 | r+0x5ae0/0x1000 | kicks VIF1/GIF chain DMA per frame. Semas r+0x5ac8 (max 0x10, init 0) and r+0x5acc (mutex) |
| `SND_IopInit` 0x3c1b80 | `0x3c19a8` | 6 | 0x50cb20/0x6000 | EA SND driver pump |
| `sceCdInitEeCB` (from EAFile_CdInit) | libcdvd callback thread | 0xb | 0x51a480/0x800 | cd callback `0x3e3ad8` |
| `EAFile_CdInit` 0x3e4040 | **`EAFile_CdReadThread` 0x3e3be0** | 0xc | 0x51ac80/0x4000 | issues `sceCdRead`. Sema `0x519c4c` (max 0x20) |
| `0x3f5000` (via 0x3f3b18 <- 0x2558c0) | `0x3f44f8` | 10 | 0x523f00/0x2000 | SIF-cmd/DMA device client (handshake via `sceSifSendCmd(1,...)`; VBlankStart ISR `0x3f4410`) |
| `EAThread_Create` 0x3e50c8 | various | 0x61-0x67 (default 100) | caller | generic EA thread wrapper ('PS2T' tag) |

Frame semaphores in `PS2ExecutionMan` (em):
- em+0x0c and em+0x18 are created with default params. **em+0x18 is the "frame done / vsync" sema** that `PS2ExecMan_WaitFrame` 0x31aa58 blocks on, then drains with `PollSema`.
- em+0x14044 (max 0x400) feeds the worker thread.
- em+0x4034 (max 0x400) is the VBlank tick.

---

## 5. Interrupt handlers

| site | call | cause/channel | handler | arg |
|---|---|---|---|---|
| `PS2ExecMan_Init` 0x31a8dc | AddIntcHandler2 | **3 = VBlank End** | `PS2ExecMan_VBlankEndIntc` 0x31a490 -> calls `PS2ExecMan_VBlankSignal` 0x31abd0, which does `iSignalSema(em+0x4034)` | 0x31abd0 |
| `Renderer_Init` 0x375dfc | AddIntcHandler2 | **2 = VBlank Start** | `Renderer_VBlankStartIntc` 0x3825c0 -> 0x3825f8: in state 4, waits N vblanks then `iSignalSema(r+0x5ac8)` | renderer |
| `Renderer_Init` 0x375e28 | AddDmacHandler2 | **ch1 = VIF1** | `Renderer_Vif1DmacHandler` 0x382650 -> 0x382688: state 1 -> 2 then `iSignalSema`, state 3 -> 4 | renderer |
| `SND_IopInit` 0x3c2170 | AddIntcHandler | 2 = VBlank Start (only when SND rate is -1) | `0x3c1980` | |
| `EATimer_Init` 0x3e4c50 | AddIntcHandler | **10 = TIMER1** (RCNT1 target, `EATimer_Init(60)`) | `0x3e4db8` | |
| `0x3f5000` 0x3f53a4 | AddIntcHandler2 | 2 = VBlank Start | `0x3f4410` | 0x51f0c0 |
| `0x40c028` 0x40c1c4 | AddIntcHandler | 3 = VBlank End | `0x40cf88` | |
| libmpeg `libmpeg_IpuCscDma` 0x408df0 | AddDmacHandler | **ch3 = IPU_FROM** | `0x408ca8` (temporary, removed after CSC) | |
| libmpeg `0x409088` | AddDmacHandler | (IPU, ch from arg) | `0x408fb0` | |
| `sceSifInitCmd` 0x425ec0 | AddDmacHandler | ch5 = SIF0 (per SDK; arg not constant-folded) | `0x426230` | |
| `0x361f98`/`0x362058` | AddIntc/AddDmacHandler | runtime args | (generic wrappers) | |

EnableIntc(3), EnableIntc(2), EnableDmac(1) and EnableIntc(10) are the boot-critical ones.
Writing `REG_DMAC_STAT = 0xa006` in `Renderer_Init` sets the CIS masks directly.

---

## 6. Graphics init and frame submit

There is no libgraph `sceGsSyncV`/`sceGsSwapDBuff`. EA's renderer drives the GS registers itself.
- The renderer singleton comes from `0x375a08`, reached from `SSXApp_Init`. `Renderer_Init` 0x375a40 does this:
  - libgraph-ish init: `0x3fdf88(0)` (GS param), `0x3ff060`, `0x3fe690`, `0x3febc0`, `0x3fdab0`, `0x3fdfb0(1)`
  - resets the DMAC: `D_STAT=0xa006`, `D1_CHCR=0x81`, `D2_CHCR=0x81`, `GIF_CTRL=1`, `VIF1_FBRST=9`, `VIF1_ERR=0`
  - sends 1 QW via VIF1 from `0x80000000`
  - creates the DMA kick thread and installs the ISRs above
- **`Renderer_SetDisplayMode` 0x37bd98** (w, psm, ..., interlace, omode, ffmode): allocates VRAM for the front, back and Z buffers. It waits for vsync with `GS_CSR=8` and polls bit 3, then calls **`sceGsResetGraph` 0x3fd910**(mode, inter, omode = 2 NTSC or 0x50 for 480p, ffmode). `sceGsResetGraph` does `GS_CSR=0x200`, `GsPutIMR(0xff00)`, **`SetGsCrt`** (stub 0x4239a0).
- **`Renderer_SetDisplayRegs` 0x382af0** is the flip. It picks the front buffer by frame parity (double buffering, FBP fields r[0x169e]/r[0x169f]), waits for VIF1/GIF/SPR-from idle, then writes **PMODE** (`|0xff20`), **SMODE2**, **DISPFB1**, **DISPLAY1** and BGCOLOR directly. It builds a GIF packet at `0x304ffcc0` (uncached) and sends it via `0x371940(pkt, qwc, 6)`.
- **Frame submit**: `Renderer_DmaKickThread` 0x382760. When the next buffer is ready it pushes a VIF1 FIFO word, sets `GIF_MODE=4`, waits `GS_CSR` FIELD, then **`D2(GIF) TADR=..., CHCR=0x105`** (chain) and **`D1(VIF1) TADR=..., CHCR=0x185`** (chain + TTE). So **both GIF ch2 and VIF1 ch1** are used: VIF1 carries geometry for VU1 (PATH1/2) and GIF carries PATH3. The ch1 completion ISR advances the state machine; VBlankStart releases the next flip after N vblanks (r+0x5ab8).
- **VSync wait for game logic**: `PS2ExecMan_WaitFrame` 0x31aa58 does `WaitSema(em+0x18)`.

---

## 7. Main loop, states, file layer

`App_Run_MainLoop` 0x316d60 (the main thread, never returns until quit):
```
em->Init(60)              vtbl+0x14 = PS2ExecMan_Init 0x31a6d8
g_App->Init(argc, argv)   vtbl+0x1c = SSXApp_Init 0x226900   (game vtbl 0x47d9c8)
em->Post(g_App->state)    vtbl+0x4c = 0x31acd8 (SignalSema to the worker)
while (!em->Quit())       vtbl+0x54 = 0x31ad00
    em->Begin()           +0x5c = 0x31a9d8
    EATimer_ServiceCallbacks(0)   0x3e5928 (16 slots; incl. 0x31ad20 cd tray poll, 0x3e4000)
    em->End()             +0x64 = 0x31aa18
    em->WaitFrame()       +0x24 = PS2ExecMan_WaitFrame 0x31aa58
    0x317328 -> 0x3174a8(em,1)  (runs pending state callback + em vtbl+0x74)
```
`App_VSyncTick` 0x317348 runs on the VSync thread. It calls `g_App->Tick()` (vtbl+0x3c = `SSXApp_Tick` 0x227f58)
as a fixed step, then em vtbl+0x2c (0x31aac8, signals frame sema).

`SSXApp_Init` 0x226900 allocates PadCache, InputPad, LoadIndicator and 2DRender. It calls `0x375a08` (Renderer, which runs Renderer_Init via vtbl+0x14), `0x227bf0`, `0x369ff8`, `0x232328` and
**`App_SetNextState(PreStartScreen)`**.
State chain (`App_SetNextState` 0x316ef8 stores into App+4):
- **PreStartScreen** (ctor 0x232328, vtbl 0x47d938, update `PreStartScreen_Update` 0x232348)
- then **InitLoadScreen** (vtbl 0x47d8f0, enter `InitLoadScreen_Enter` 0x232568 -> `SSXApp_BootLoad` 0x2282d0 (net modules + mounting BIGs), update `InitLoadScreen_Update` 0x232588)
- then **StartScreen** (ctor `StartScreen_ctor` 0x232ae0) = the first front-end screen.

### File layer (EA "FILE" lib)
- Setup in `ssx_SystemInit`:
  - `EAFile_Startup(0x30, 500, 0x20)` 0x3dcbd8 -> `EAFile_Init` 0x3dcc88 -> **`EAFile_CdInit` 0x3e4040** (sceCdInit, Mmode, DiskReady, GetDiskType; walks the ISO9660 directory itself into a table; starts the CD read thread and the cd callback)
  - `EAFile_SetRootPath("cd:")` 0x3dde78 stores the root at `0x519af0`. Device mode `0x450c44`: 1 = cd, 2 = `host0:`
- Requests are queued and serviced by **`EAFile_ServiceQueue` 0x3ddfa8**. The op is in bits 20-23 of the request: 2/8 open, 3 close, 4 read, 5 write, 6 size.
- **Open "a.big|b"**: split at `'|'`. With no `'|'` the file is opened directly. Otherwise the code walks the mounted BIG list `0x519ae4` and calls **`EAFile_BigFindEntry` 0x3e2768**, which gives the offset and size inside the archive. A leading `'|'` means "search all mounted BIGs".
- BIGs are opened in `SSXApp_BootLoad` via `0x3ded50(path, 0, 100, &handle)`. The archives are music, music2, audio, speech, scdat, mdlps2, the 11 *txp.big files and rwrdps2. `0x317f38` = exists check.
- **`EAFile_PlatformOpen` 0x3e3208** calls `EAFile_ResolvePath` 0x3e3098 (adds the root, turns `\` into `/`, returns the device). On cd it calls **`EAFile_CdLookup` 0x3e4508**: upper-cases the name, hashes it (0x3e36f8), then bsearches the private ISO table. The handle is `0x01000000 | index`. On host it calls `sceOpen` (fileio).
- **Read**: `EAFile_PlatformRead` 0x3e3350. On cd it calls `EAFile_CdRead` 0x3e4648, which converts to LSN = entry.lsn + off/2048 and handles partial sectors. It signals the CD thread, which runs `EAFile_CdIssueRead` 0x3e3b00 -> **`sceCdRead(lsn, nsec, buf, mode@0x519c5c)`** + `sceCdSync(0)`, with 3 retries. On host the request is deferred to `sceRead`. Write uses `sceLseek` + `sceWrite`.

---

## 8. IPU / movies at boot

- Sony **libmpeg 2.7** ("PsIIlibmpeg 2700" @0x4531b8) sits around `0x402c30-0x409xxx`. `libmpeg_IpuCscDma` 0x408df0 drives IPU_FROM ch3 and IPU_CMD CSC. EA's player wraps it ("RCMP::mpegbuff", "MoviePlayer" objects of 0x44 bytes: ctor `0x2533c8`, open `0x2534a8`, play `0x253860`, update `0x253938`, stop `0x253418`).
- **`ssx_BootMoviesUpdate` 0x1a27a0** is driven by flag word gp-0x1724 (`uGpffffe8dc`):
  - bit0 plays `data\movies\eabig.mpc` (EA logo)
  - bit1 plays `thx.mpc` (*not on disc*)
  - bit2 plays `intro.mpc`, or `intro_dj.mpc` depending on a profile flag
  - Pad buttons 0x70/0x7a skip the intro.
  - When the movies finish it continues into the front-end (`0x19d250`, `0x1a0608`, net `0x263828`).
- On the disc, DATA/MOVIES holds ABC1/DBC2/EBC3(+WS), EABIG, INTRO, INTRO_DJ and MTNALIVE (.MPC).

---

## Appendix: syscall site table

| syscall insn | stub fn | v1 | name | Ghidra name |
|---|---|---|---|---|
| 0x100178 | inline in entry | 0x3c | SetupThread (=InitMainThread) | entry |
| 0x100194 | inline in entry | 0x3d | SetupHeap (=InitHeap) | entry |
| 0x1001d4 | 0x1001d0 (crt0 ExitThread stub, root ra) | 0x23 | ExitThread | (no function) |
| 0x423984 | 0x423980 | 0x0 | RFU000_FullReset | (no function) |
| 0x423994 | 0x423990 | 0x1 | ResetEE | (no function) |
| 0x4239a4 | 0x4239a0 | 0x2 | SetGsCrt | SetGsCrt |
| 0x4239b4 | 0x4239b0 | 0x3 | RFU003 | (no function) |
| 0x4239c4 | 0x4239c0 | 0x4 | Exit | _Exit |
| 0x4239d4 | 0x4239d0 | 0x5 | _ExceptionEpilogue (RFU005) | (no function) |
| 0x4239e4 | 0x4239e0 | 0x6 | LoadExecPS2 | _LoadExecPS2 |
| 0x4239f4 | 0x4239f0 | 0x7 | ExecPS2 | (no function) |
| 0x423a04 | 0x423a00 | 0x8 | RFU008 | (no function) |
| 0x423a14 | 0x423a10 | 0x9 | RFU009 | (no function) |
| 0x423a24 | 0x423a20 | 0xa | AddSbusIntcHandler | (no function) |
| 0x423a34 | 0x423a30 | 0xb | RemoveSbusIntcHandler | (no function) |
| 0x423a44 | 0x423a40 | 0xc | Interrupt2Iop | (no function) |
| 0x423a54 | 0x423a50 | 0xd | SetVTLBRefillHandler | SetVTLBRefillHandler |
| 0x423a64 | 0x423a60 | 0xe | SetVCommonHandler | (no function) |
| 0x423a74 | 0x423a70 | 0xf | SetVInterruptHandler | (no function) |
| 0x423a84 | 0x423a80 | 0x10 | AddIntcHandler | AddIntcHandler |
| 0x423a94 | 0x423a90 | 0x10 | AddIntcHandler | AddIntcHandler |
| 0x423aa4 | 0x423aa0 | 0x11 | RemoveIntcHandler | RemoveIntcHandler |
| 0x423ab4 | 0x423ab0 | 0x12 | AddDmacHandler | AddDmacHandler |
| 0x423ac4 | 0x423ac0 | 0x12 | AddDmacHandler | AddDmacHandler |
| 0x423ad4 | 0x423ad0 | 0x13 | RemoveDmacHandler | RemoveDmacHandler |
| 0x423ae4 | 0x423ae0 | 0x14 | _EnableIntc | _EnableIntc |
| 0x423af4 | 0x423af0 | 0x15 | _DisableIntc | _DisableIntc |
| 0x423b04 | 0x423b00 | 0x16 | _EnableDmac | _EnableDmac |
| 0x423b14 | 0x423b10 | 0x17 | _DisableDmac | _DisableDmac |
| 0x423b24 | 0x423b20 | 0xfc | SetAlarm (patched 0xFC) | SetAlarm |
| 0x423b34 | 0x423b30 | 0xfd | ReleaseAlarm (patched 0xFD) | ReleaseAlarm |
| 0x423b44 | 0x423b40 | -0x1a | _iEnableIntc | _iEnableIntc |
| 0x423b54 | 0x423b50 | -0x1b | _iDisableIntc | _iDisableIntc |
| 0x423b64 | 0x423b60 | -0x1c | _iEnableDmac | _iEnableDmac |
| 0x423b74 | 0x423b70 | -0x1d | _iDisableDmac | _iDisableDmac |
| 0x423b84 | 0x423b80 | -0xfe | iSetAlarm (patched -0xFE) | iSetAlarm |
| 0x423b94 | 0x423b90 | -0xff | iReleaseAlarm (patched -0xFF) | (no function) |
| 0x423ba4 | 0x423ba0 | 0x20 | CreateThread | CreateThread |
| 0x423bb4 | 0x423bb0 | 0x21 | DeleteThread | DeleteThread |
| 0x423bc4 | 0x423bc0 | 0x22 | StartThread | _StartThread |
| 0x423bd4 | 0x423bd0 | 0x23 | ExitThread | (no function) |
| 0x423be4 | 0x423be0 | 0x24 | ExitDeleteThread | ExitDeleteThread |
| 0x423bf4 | 0x423bf0 | 0x25 | TerminateThread | TerminateThread |
| 0x423c04 | 0x423c00 | -0x26 | iTerminateThread | iTerminateThread |
| 0x423c14 | 0x423c10 | 0x27 | DisableDispatchThread | (no function) |
| 0x423c24 | 0x423c20 | 0x28 | EnableDispatchThread | (no function) |
| 0x423c34 | 0x423c30 | 0x29 | ChangeThreadPriority | ChangeThreadPriority |
| 0x423c44 | 0x423c40 | -0x2a | iChangeThreadPriority | iChangeThreadPriority |
| 0x423c54 | 0x423c50 | 0x2b | RotateThreadReadyQueue | RotateThreadReadyQueue |
| 0x423c64 | 0x423c60 | -0x2c | iRotateThreadReadyQueue | (no function) |
| 0x423c74 | 0x423c70 | 0x2d | ReleaseWaitThread | (no function) |
| 0x423c84 | 0x423c80 | -0x2e | iReleaseWaitThread | (no function) |
| 0x423c94 | 0x423c90 | 0x2f | GetThreadId | GetThreadId |
| 0x423ca4 | 0x423ca0 | 0x30 | ReferThreadStatus | ReferThreadStatus |
| 0x423cb4 | 0x423cb0 | -0x31 | iReferThreadStatus | iReferThreadStatus |
| 0x423cc4 | 0x423cc0 | 0x32 | SleepThread | SleepThread |
| 0x423cd4 | 0x423cd0 | 0x33 | WakeupThread | WakeupThread |
| 0x423ce4 | 0x423ce0 | -0x34 | iWakeupThread | _iWakeupThread |
| 0x423cf4 | 0x423cf0 | 0x35 | CancelWakeupThread | (no function) |
| 0x423d04 | 0x423d00 | -0x36 | iCancelWakeupThread | (no function) |
| 0x423d14 | 0x423d10 | 0x37 | SuspendThread | SuspendThread |
| 0x423d24 | 0x423d20 | -0x38 | iSuspendThread | _iSuspendThread |
| 0x423d34 | 0x423d30 | 0x39 | ResumeThread | (no function) |
| 0x423d44 | 0x423d40 | -0x3a | iResumeThread | (no function) |
| 0x423d54 | 0x423d50 | 0x3b | JoinThread | (no function) |
| 0x423d64 | 0x423d60 | 0x3c | SetupThread (=InitMainThread) | (no function) |
| 0x423d74 | 0x423d70 | 0x3d | SetupHeap (=InitHeap) | (no function) |
| 0x423d84 | 0x423d80 | 0x3e | EndOfHeap | EndOfHeap |
| 0x423d94 | 0x423d90 | 0x3f | RFU063 | (no function) |
| 0x423da4 | 0x423da0 | 0x40 | CreateSema | CreateSema |
| 0x423db4 | 0x423db0 | 0x41 | DeleteSema | DeleteSema |
| 0x423dc4 | 0x423dc0 | 0x42 | SignalSema | SignalSema |
| 0x423dd4 | 0x423dd0 | -0x43 | iSignalSema | iSignalSema |
| 0x423de4 | 0x423de0 | 0x44 | WaitSema | WaitSema |
| 0x423df4 | 0x423df0 | 0x45 | PollSema | PollSema |
| 0x423e04 | 0x423e00 | -0x46 | iPollSema | (no function) |
| 0x423e14 | 0x423e10 | 0x47 | ReferSemaStatus | (no function) |
| 0x423e24 | 0x423e20 | -0x48 | iReferSemaStatus | (no function) |
| 0x423e34 | 0x423e30 | 0x49 | iDeleteSema | (no function) |
| 0x423e44 | 0x423e40 | 0x4a | SetOsdConfigParam | SetOsdConfigParam |
| 0x423e54 | 0x423e50 | 0x4b | GetOsdConfigParam | GetOsdConfigParam |
| 0x423e64 | 0x423e60 | 0x4c | GetGsHParam | (no function) |
| 0x423e74 | 0x423e70 | 0x4d | GetGsVParam | (no function) |
| 0x423e84 | 0x423e80 | 0x4e | SetGsHParam | (no function) |
| 0x423e94 | 0x423e90 | 0x4f | SetGsVParam | (no function) |
| 0x423ea4 | 0x423ea0 | 0x50 | CreateEventFlag | (no function) |
| 0x423eb4 | 0x423eb0 | 0x51 | DeleteEventFlag | (no function) |
| 0x423ec4 | 0x423ec0 | 0x52 | SetEventFlag | (no function) |
| 0x423ed4 | 0x423ed0 | -0x53 | iSetEventFlag | (no function) |
| 0x423ee4 | 0x423ee0 | 0x54 | ClearEventFlag | (no function) |
| 0x423ef4 | 0x423ef0 | -0x55 | iClearEventFlag | (no function) |
| 0x423f04 | 0x423f00 | 0x56 | WaitEventFlag | (no function) |
| 0x423f14 | 0x423f10 | 0x57 | PollEventFlag | (no function) |
| 0x423f24 | 0x423f20 | -0x58 | iPollEventFlag | (no function) |
| 0x423f34 | 0x423f30 | 0x59 | ReferEventFlagStatus | (no function) |
| 0x423f44 | 0x423f40 | -0x5a | iReferEventFlagStatus | (no function) |
| 0x423f54 | 0x423f50 | 0x5b | GetEntryAddress / GetSyscallHandler (RFU091) | (no function) |
| 0x423f64 | 0x423f60 | 0x5c | EnableIntcHandler | (no function) |
| 0x423f74 | 0x423f70 | -0x5c | iEnableIntcHandler | (no function) |
| 0x423f84 | 0x423f80 | 0x5d | DisableIntcHandler | (no function) |
| 0x423f94 | 0x423f90 | -0x5d | iDisableIntcHandler | (no function) |
| 0x423fa4 | 0x423fa0 | 0x5e | EnableDmacHandler | (no function) |
| 0x423fb4 | 0x423fb0 | -0x5e | iEnableDmacHandler | (no function) |
| 0x423fc4 | 0x423fc0 | 0x5f | DisableDmacHandler | (no function) |
| 0x423fd4 | 0x423fd0 | -0x5f | iDisableDmacHandler | (no function) |
| 0x423fe4 | 0x423fe0 | 0x60 | KSeg0 | (no function) |
| 0x423ff4 | 0x423ff0 | 0x61 | EnableCache | (no function) |
| 0x424004 | 0x424000 | 0x62 | DisableCache | (no function) |
| 0x424014 | 0x424010 | 0x63 | GetCop0 | (no function) |
| 0x424024 | 0x424020 | 0x64 | FlushCache | FlushCache |
| 0x424034 | 0x424030 | 0x66 | CpuConfig | (no function) |
| 0x424044 | 0x424040 | -0x67 | iGetCop0 | (no function) |
| 0x424054 | 0x424050 | -0x68 | iFlushCache | iFlushCache |
| 0x424064 | 0x424060 | -0x6a | iCpuConfig | (no function) |
| 0x424074 | 0x424070 | 0x6b | sceSifStopDma | sceSifStopDma |
| 0x424084 | 0x424080 | 0x6c | SetCPUTimerHandler | (no function) |
| 0x424094 | 0x424090 | 0x6d | SetCPUTimer | (no function) |
| 0x4240a4 | 0x4240a0 | 0x6e | SetOsdConfigParam2 | (no function) |
| 0x4240b4 | 0x4240b0 | 0x6f | GetOsdConfigParam2 | GetOsdConfigParam2 |
| 0x4240c4 | 0x4240c0 | 0x70 | GsGetIMR | (no function) |
| 0x4240d4 | 0x4240d0 | -0x70 | iGsGetIMR | (no function) |
| 0x4240e4 | 0x4240e0 | 0x71 | GsPutIMR | GsPutIMR |
| 0x4240f4 | 0x4240f0 | -0x71 | iGsPutIMR | (no function) |
| 0x424104 | 0x424100 | 0x72 | SetPgifHandler | (no function) |
| 0x424114 | 0x424110 | 0x73 | SetVSyncFlag | SetVSyncFlag |
| 0x424124 | 0x424120 | 0x74 | SetSyscall | (no function) |
| 0x424134 | 0x424130 | 0x75 | _print | (no function) |
| 0x424144 | 0x424140 | 0x76 | sceSifDmaStat | sceSifDmaStat |
| 0x424154 | 0x424150 | -0x76 | isceSifDmaStat | isceSifDmaStat |
| 0x424164 | 0x424160 | 0x77 | sceSifSetDma | sceSifSetDma |
| 0x424174 | 0x424170 | -0x77 | isceSifSetDma | isceSifSetDma |
| 0x424184 | 0x424180 | 0x78 | sceSifSetDChain | sceSifSetDChain |
| 0x424194 | 0x424190 | -0x78 | isceSifSetDChain | isceSifSetDChain |
| 0x4241a4 | 0x4241a0 | 0x79 | sceSifSetReg | sceSifSetReg |
| 0x4241b4 | 0x4241b0 | 0x7a | sceSifGetReg | sceSifGetReg |
| 0x4241c4 | 0x4241c0 | 0x7b | ExecOSD | _ExecOSD |
| 0x4241d4 | 0x4241d0 | 0x7c | Deci2Call | Deci2Call |
| 0x4241e4 | 0x4241e0 | 0x7d | PSMode | (no function) |
| 0x4241f4 | 0x4241f0 | 0x7e | MachineType | (no function) |
| 0x424204 | 0x424200 | 0x7f | GetMemorySize | GetMemorySize |
| 0x424214 | 0x424210 | 0x82 | _InitTLB | _InitTLB |
| 0x424c60 | inside 0x424c50 | -0x2f | iGetThreadId (inline in libkernl helper) | FUN_00424c50 |
| 0x424d78 | inside 0x424d68 | -0x2f | iGetThreadId (inline in libkernl helper) | FUN_00424d68 |
| 0x42c124 | inside 0x42c120 | 0x5a | (slot 0x5A, temporarily hijacked by SetSyscall: kernel-mode memcpy trampoline) | FUN_0042c120 |
| 0x42c1ac | inside 0x42c1a8 | 0x83 | FindAddress (scan kernel RAM for a word; used by _InitSys patcher) | FUN_0042c1a8 |
| 0x42c2f4 | inside 0x42c2f0 | 0x74 | SetSyscall | RFU116_SetSyscall |
| 0x42c344 | inside 0x42c340 | 0x74 | SetSyscall | RFU116_SetSyscall |
| 0x42c354 | inside 0x42c350 | 0x5a | (slot 0x5A, temporarily hijacked by SetSyscall: kernel-mode memcpy trampoline) | FUN_0042c350 |
| 0x42c39c | inside 0x42c398 | 0x5b | GetEntryAddress / GetSyscallHandler (RFU091) | RFU091 |
| 0x42c4c4 | inside 0x42c4c0 | 0x5a | (slot 0x5A, temporarily hijacked by SetSyscall: kernel-mode memcpy trampoline) | FUN_0042c4c0 |
| 0x42c504 | inside 0x42c500 | 0x74 | SetSyscall | RFU116_SetSyscall |
| 0x42c764 | inside 0x42c760 | 0x74 | SetSyscall | RFU116_SetSyscall |
| 0x42c774 | inside 0x42c770 | 0x5a | (slot 0x5A, temporarily hijacked by SetSyscall: kernel-mode memcpy trampoline) | FUN_0042c770 |
| 0x42c7bc | inside 0x42c7b8 | 0x5b | GetEntryAddress / GetSyscallHandler (RFU091) | RFU091 |
| 0x42cb6c | inside 0x42cb68 | 0x5a | (slot 0x5A, temporarily hijacked by SetSyscall: kernel-mode memcpy trampoline) | FUN_0042cb68 |
| 0x42cbb4 | inside 0x42cbb0 | 0x5b | GetEntryAddress / GetSyscallHandler (RFU091) | RFU091 |
| 0x42cbc4 | inside 0x42cbc0 | 0x74 | SetSyscall | RFU116_SetSyscall |
| 0x42cc9c | inline | 0x55 | PutTLBEntry | (no function) |
| 0x42ccac | inline | -0x55 | iPutTLBEntry | (no function) |
| 0x42ccbc | inside 0x42ccb8 | 0x56 | _SetTLBEntry | _SetTLBEntry (renamed; was RFU086_WaitEvnetFlag) |
| 0x42ccfc | inline | -0x56 | iSetTLBEntry | (no function) |
| 0x42cd0c | inline | 0x57 | GetTLBEntry | (no function) |
| 0x42cd1c | inline | -0x57 | iGetTLBEntry | (no function) |
| 0x42cd2c | inline | 0x58 | ProbeTLBEntry | (no function) |
| 0x42cd3c | inline | -0x58 | iProbeTLBEntry | (no function) |
| 0x42cd4c | inline | 0x59 | ExpandScratchPad | (no function) |
| 0x42d0d8 | inline | -0x54 | (kernel-patch trampoline epilogue, not a user wrapper) | (no function) |
| 0x456168 | inline | -0x8 | (kernel-patch payload copied to 0x82000, not a user wrapper) | (no function) |
