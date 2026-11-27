import re
docs = {
'd1.rs': ('spch_assign_variations', '0x003d_93c0', 'SPCH_AssignVariations', False, '''/// `int SPCH_AssignVariations(Phrase* phrase, SelectionSet* sel, int thr)`
/// (0x3d93c0): picks one sample variation per entry of a speech phrase. Every
/// entry's current choice (`+6`) starts at its first one (`+4`); a combination
/// is accepted when its sample-usage percentage (the sizes `SPCH_DecodeSampleEntry`
/// reports, times 100 over the global total at `gp+0x1748`) stays within `thr`
/// (0 = unchecked) and, for a phrase with sentence parts (`phrase[5]`),
/// `SPCH_BuildSentencePart` accepts it. Otherwise the choices advance like an
/// odometer (last entry first, each wrapping to its first choice); the search
/// ends with the accepted flag, or 0 once every combination was tried.
'''),
'seq.rs': ('lib_mpeg_sequence_header', '0x0040_3620', 'libmpeg_SequenceHeader', False, '''/// `libmpeg_SequenceHeader(dec)` (0x403620): parses an MPEG sequence header
/// from the bitstream: width/height (`dec+0x124/0x128`, `Error` when the width
/// field is 2801 or more), frame-rate and bit-rate words (`+0x134/0x138`), then
/// the intra and non-intra quantiser matrices (loaded from the stream through
/// the IPU, or the default tables), and the extension/user data. Then it
/// falls into the frame-buffer set-up of the decoder context at
/// `dec[0x858]->[0x40]`: macroblock dimensions (`+0x12c/0x130`) and, when the
/// picture size changed, the work-area allocations, `SetupFrameBuffers` and
/// the `SetPictureSize` calls (the last one a tail call).
'''),
'smb.rs': ('lib_mpeg_setup_macroblock', '0x0040_3e00', 'libmpeg_SetupMacroblock', False, '''/// `int libmpeg_SetupMacroblock(dec, mbAddr, b, flags, mtype)` (0x403e00): splits
/// the macroblock address by the picture width in macroblocks (`dec+0x12c`)
/// into column and row, waits for the IPU-side SPR DMA (`0x1000d400` bit 8),
/// and for a non-intra block (`flags & 1` clear) forms the predictions
/// (`Errorf` and 0 for a bad motion type 0 or above 3, else `FormPredictions`,
/// the DMA wait again, `FetchRefBlocks`). Then it records the per-buffer
/// entry of `dec[0x6b8..0x6c8 + cur * 0x140]`: done flag, two flags and the
/// macroblock's buffer address in the current frame (picture structure
/// `dec+0x174`: 3 frame, 2 the `+0x1e0` field buffer, else `+0x1d0`).
'''),
'spb.rs': ('lib_mpeg_setup_pred_block', '0x0040_4758', 'libmpeg_SetupPredBlock', True, '''/// `libmpeg_SetupPredBlock(dec, ref, a2, a3, t0, h, bx, by, dx, dy, fld, avg)`
/// (0x404758): appends the luma and the chroma prediction block descriptors
/// for one motion vector (`dx`, `dy` in half pixels, at the stack) to the
/// current buffer entry of `dec` (`dec[0x6bc + cur * 0x140]` holds the count
/// `k`): block address inside the reference picture, the fraction-dependent
/// row split (`+8`/`+0xc` rows before/after the macroblock edge), buffer
/// pointers, and the interpolation-routine selectors from the tables at
/// `0x453280`/`0x4532a0`. Leaf, 8 stack arguments.
'''),
'wbd.rs': ('lib_mpeg_wait_bdec', '0x0040_5b70', 'libmpeg_WaitBdec', False, '''/// `int libmpeg_WaitBdec(dec)` (0x405b70): waits for the IPU decode to finish.
/// Loops while the IPU-from DMA has quadwords queued (`0x1000b020`) and no
/// error is flagged in `IPU_CTRL` (`0x10002010`, ECD = bit 14), feeding the
/// stalled to-IPU DMA through the callback `dec[0x858]` (`DispatchCallback`,
/// command 1). With `dec[0x10a0]` set it aborts instead (interrupts off, DMA
/// registers cleared, `IPU_CTRL = 0x40000000`, returns 0). Otherwise it
/// stores `IPU_TOP` and the bit-position remainder (`dec+0x838/0x83c`) and,
/// when ECD is set, reports the error (`Error`, callbacks 2 and 3), resets the
/// DMA and returns 0; else 1.
'''),
}
order = ['d1.rs','seq.rs','smb.rs','spb.rs','wbd.rs']
out = []
over = []
for f in order:
    fn, addr, name, leaf, doc = docs[f]
    t = open(f).read()
    i = t.index('    guest(c, |c| {\n') + len('    guest(c, |c| {\n')
    j = t.rindex('\n    });\n}')
    b = t[i:j]
    b = re.sub(r'c\.pc = (0x[0-9a-f]+); (\w+)\(c\); return Ok\(\(\)\);', lambda m: f'c.pc = {m.group(1)};\n    ee::s15::{m.group(2)}(c);\n    return Ok(());', b)
    if leaf:
        b = b.replace('return Ok(());', 'return;')
        body = f'pub fn {fn}(c: &mut Cpu) {{\n{b}\n}}\n'
    else:
        body = f'pub fn {fn}(c: &mut Cpu) {{\n    guest(c, |c| {{\n{b}\n    }});\n}}\n'
    out.append(doc + body)
    over.append(f'    ({addr}, {fn}, "{name}"),')
open('funcs.rs','w').write('\n'.join(out))
open('over.rs','w').write('\n'.join(over))
