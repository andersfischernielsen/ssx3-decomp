// Differential-test harness around ps2-web-recomp's VU interpreter
// (snuri00/ps2-web-recomp, MIT; runtime/src/vu_micro.cpp compiled unmodified
// apart from a per-pair trace hook inserted by build.sh).
//
//   vu-ref <capture> <out> [--trace N] [--fast]
//
// Reads the SSX3_VU_CAPTURE records (format: crates/ssx3-ps2/src/vu/capture.rs),
// loads each program-start state, runs the reference interpreter to the E bit
// and writes, per record: u32 index, kicks, end ArchState, data memory, u64
// pairs. --trace N prints a per-pair trace of record N to stdout. --fast uses
// the reference's host-float path (default: its exact soft-float path,
// PS2_VU_EXACT).

#include "hw.hpp"
#include "gs/gif.hpp"
#include "gs/queue.hpp"
#include "ps2/state.hpp"
#include "ps2/system.hpp"
#include "vu_jit_web.hpp"
#include "vu_micro.hpp"

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <string>
#include <vector>

namespace ps2 {
Cpu g_cpu;
Options g_options;
std::uint8_t g_vu0_code[4096], g_vu0_mem[4096], g_vu1_code[16384], g_vu1_mem[16384];
}

static std::vector<std::vector<std::uint8_t>> g_kicks;

namespace ps2::gif {
void transfer(int, const std::uint8_t *data, std::uint32_t qwc) { g_kicks.emplace_back(data, data + qwc * 16); }
}
namespace ps2::gs::queue {
bool vu_active() { return false; }
Role &role() {
    static Role r = Role::Emulation;
    return r;
}
Ring &vu() {
    static Ring r;
    return r;
}
void Ring::post(Type, std::uint32_t, std::uint32_t, const void *, std::uint32_t) {}
}
namespace ps2::vu::jit {
int mode() { return 0; }
ProgramFn compile(const std::uint8_t *, std::uint32_t) { return nullptr; }
bool g_dry = false;
std::int64_t g_escape = -1;
std::vector<std::uint32_t> *g_kicks = nullptr;
}

// Trace hook (see build.sh / hook.hpp).
int g_trace_on = 0;
std::uint64_t g_pairs = 0;
void vu_trace_hook(std::uint32_t pc, const ps2::Vu0Regs &r) {
    ++g_pairs;
    if (!g_trace_on) return;
    std::printf("%04x vi", pc);
    for (int k = 1; k < 16; ++k) std::printf(" %04x", r.vi[k]);
    std::printf(" q %08x p %08x mac %04x st %03x clip %06x\n", r.q, r.p, r.mac, r.status, r.clip);
    for (int k = 1; k < 32; ++k) std::printf("  vf%02d %08x %08x %08x %08x\n", k, r.vf[k].w[0], r.vf[k].w[1], r.vf[k].w[2], r.vf[k].w[3]);
    std::printf("  acc  %08x %08x %08x %08x\n", r.acc.w[0], r.acc.w[1], r.acc.w[2], r.acc.w[3]);
}

struct Reader {
    const std::vector<std::uint8_t> &b;
    size_t pos = 0;
    bool ok = true;
    void take(void *dst, size_t n) {
        if (pos + n > b.size()) {
            ok = false;
            return;
        }
        std::memcpy(dst, b.data() + pos, n);
        pos += n;
    }
    std::uint32_t u32() { std::uint32_t v = 0; take(&v, 4); return v; }
    std::uint64_t u64() { std::uint64_t v = 0; take(&v, 8); return v; }
    std::uint16_t u16() { std::uint16_t v = 0; take(&v, 2); return v; }
    std::vector<std::uint8_t> bytes() {
        std::uint32_t n = u32();
        std::vector<std::uint8_t> v(n);
        if (n) take(v.data(), n);
        return v;
    }
};

struct Arch {
    std::uint32_t vf[32][4], acc[4];
    std::uint16_t vi[16];
    std::uint32_t q, p, i, r, mac, status, clip, tpc;
};

static void read_arch(Reader &rd, Arch &a) {
    for (auto &v : a.vf) for (auto &x : v) x = rd.u32();
    for (auto &x : a.acc) x = rd.u32();
    for (auto &x : a.vi) x = rd.u16();
    a.q = rd.u32(); a.p = rd.u32(); a.i = rd.u32(); a.r = rd.u32();
    a.mac = rd.u32(); a.status = rd.u32(); a.clip = rd.u32(); a.tpc = rd.u32();
}

static void put32(std::vector<std::uint8_t> &o, std::uint32_t v) { o.insert(o.end(), (std::uint8_t *)&v, (std::uint8_t *)&v + 4); }
static void put64(std::vector<std::uint8_t> &o, std::uint64_t v) { o.insert(o.end(), (std::uint8_t *)&v, (std::uint8_t *)&v + 8); }

static void write_arch(std::vector<std::uint8_t> &o, const Arch &a) {
    for (auto &v : a.vf) for (auto x : v) put32(o, x);
    for (auto x : a.acc) put32(o, x);
    for (auto x : a.vi) o.insert(o.end(), (std::uint8_t *)&x, (std::uint8_t *)&x + 2);
    for (auto x : {a.q, a.p, a.i, a.r, a.mac, a.status, a.clip, a.tpc}) put32(o, x);
}

int main(int argc, char **argv) {
    if (argc < 3) {
        std::fprintf(stderr, "usage: vu-ref <capture> <out> [--trace N] [--fast] [--only N]\n");
        return 2;
    }
    long trace = -1, only = -1;
    bool fast = false;
    for (int k = 3; k < argc; ++k) {
        if (!std::strcmp(argv[k], "--trace") && k + 1 < argc) trace = std::atol(argv[++k]);
        else if (!std::strcmp(argv[k], "--only") && k + 1 < argc) only = std::atol(argv[++k]);
        else if (!std::strcmp(argv[k], "--fast")) fast = true;
    }
    if (!fast) setenv("PS2_VU_EXACT", "1", 1);
    std::ifstream f(argv[1], std::ios::binary);
    std::vector<std::uint8_t> buf((std::istreambuf_iterator<char>(f)), {});
    std::vector<std::uint8_t> out;
    Reader rd{buf};
    std::uint32_t index = 0;
    while (rd.pos < buf.size()) {
        if (rd.u32() != 0x52435556u || rd.u32() != 1) { std::fprintf(stderr, "bad record %u\n", index); return 1; }
        const std::uint32_t unit = rd.u32(), pc = rd.u32(), top = rd.u32(), itop = rd.u32();
        rd.u64(); rd.u64(); rd.u64();
        const std::uint32_t nseg = rd.u32();
        for (std::uint32_t s = 0; s < nseg; ++s) { rd.u32(); rd.u32(); }
        Arch start{};
        read_arch(rd, start);
        auto code = rd.bytes(), data = rd.bytes();
        // skip recorded outputs
        std::uint32_t nk = rd.u32();
        for (std::uint32_t k = 0; k < nk; ++k) { std::uint32_t n = rd.u32(); rd.pos += n * 16; }
        Arch dummy{};
        read_arch(rd, dummy);
        rd.bytes(); rd.u64(); rd.u64(); rd.u64();
        if (!rd.ok) { std::fprintf(stderr, "truncated record %u\n", index); break; }
        if (only >= 0 && index != (std::uint32_t)only && trace != (long)index) { ++index; continue; }

        ps2::Vu0Regs &r = ps2::vu::regs(unit);
        std::uint8_t *cm = unit ? ps2::g_vu1_code : ps2::g_vu0_code, *dm = unit ? ps2::g_vu1_mem : ps2::g_vu0_mem;
        std::memcpy(cm, code.data(), code.size());
        std::memcpy(dm, data.data(), data.size());
        for (int k = 0; k < 32; ++k) std::memcpy(r.vf[k].w, start.vf[k], 16);
        std::memcpy(r.acc.w, start.acc, 16);
        std::memcpy(r.vi, start.vi, 32);
        r.q = start.q; r.p = start.p; r.i = start.i; r.r = start.r & 0x7FFFFFu;
        r.mac = start.mac; r.status = start.status; r.clip = start.clip; r.tpc = start.tpc;
        ps2::vu::set_tops(top, itop);
        ps2::vu::code_changed(unit);
        g_kicks.clear();
        g_pairs = 0;
        g_trace_on = trace == (long)index;
        ps2::vu::run(unit, pc);
        g_trace_on = 0;

        Arch end{};
        for (int k = 0; k < 32; ++k) std::memcpy(end.vf[k], r.vf[k].w, 16);
        std::memcpy(end.acc, r.acc.w, 16);
        std::memcpy(end.vi, r.vi, 32);
        end.q = r.q; end.p = r.p; end.i = r.i; end.r = (r.r & 0x7FFFFFu) | 0x3F800000u;
        end.mac = r.mac; end.status = r.status; end.clip = r.clip; end.tpc = r.tpc;
        put32(out, index);
        put32(out, (std::uint32_t)g_kicks.size());
        for (auto &k : g_kicks) {
            put32(out, (std::uint32_t)(k.size() / 16));
            out.insert(out.end(), k.begin(), k.end());
        }
        write_arch(out, end);
        put32(out, (std::uint32_t)data.size());
        out.insert(out.end(), dm, dm + data.size());
        put64(out, g_pairs);
        ++index;
    }
    std::ofstream(argv[2], std::ios::binary).write((const char *)out.data(), out.size());
    std::fprintf(stderr, "vu-ref: %u records\n", index);
    return 0;
}
