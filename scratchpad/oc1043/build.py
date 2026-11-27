import sys
sys.path.insert(0,'scratchpad/oc1043')
from gen import conv
docs={
'002aec58':("path_plat_get_track_status","""/// `int PATH_PlatGetTrackStatus(int slot, int* elapsed_ms, int* remaining_ms)`
/// (0x2aec58): reads one of the 24 sound-bank monitor slots (`D_004D3E98[slot]`),
/// locks the sound system and refreshes the slot's stream state (in-memory
/// streams: position from the stream header; streaming ones: each sub-request's
/// state), stores the two out values, and returns 0, -1 (no slot) or -2 (no
/// stream handle). Straight transcription of the generated body (labels kept)."""),
'002b1780':("music_player_create_stream","""/// `int MusicPlayer_CreateStream(SNDPLAYOPTS* opts, a1, a2, a3, t0)` (0x2b1780): pushes a
/// new bank-monitor instance on the music system's bank manager (default play
/// options, zeroed per-instance fields, delay 100/90/50, max volume 127, close on
/// end), copies `*opts` into its level slot, creates stream 2 and returns its
/// handle. Straight-line transcription of the generated body."""),
'002b3398':("music_player_dtor","""/// `void MusicPlayer_dtor(MusicPlayer* self, int flags)` (0x2b3398): resets the vtables of
/// the stream system's bank manager and sound system (rebased on `self` when
/// `flags == 0`), destroys the song configurations (virtual dtor, flag 3), frees
/// the MPF file and song instance, clears `gMusicSys`, then by `flags` runs the
/// sub-object destructors (bit 1) and `operator_delete` (bit 0). Transcription of
/// the generated body (labels kept)."""),
'002c1da8':("quad_tree_compact","""/// `void QuadTree_Compact(QuadTree* self, void** out, int* size, ctx)` (0x2c1da8,
/// `cQuadTree_exportTree`): allocates the node map and the output block, flattens the
/// tree into it, renumbers each node's children and byte-swaps the header and nodes
/// through the context's fix functions. Transcription of the generated body (labels
/// kept)."""),
'002d83b8':("avalanche_write_history_file","""/// `void Avalanche_WriteHistoryFile(bool direct, const char* name)` (0x2d83b8,
/// `cAvalanche::SaveData`): when the avalanche list exists, opens the course's
/// history file, writes the magic, the size and then every avalanche's nodes,
/// collisions and ground collisions, closes it and clears `cAvalanche_m_Save`.
/// Transcription of the generated body (labels kept)."""),
}
out=[]
for a,(fn,doc) in docs.items():
    out.append(conv(a,fn,doc,fn))
open('scratchpad/oc1043/funcs.rs','w').write('\n'.join(out))

import re
t=open('scratchpad/oc1043/funcs.rs').read()
t=re.sub(r"c\.pc = (0x[0-9a-f]+|t); call_indirect\(c, \1\);\n\s*if c\.pc != (0x[0-9a-f]+) \{ return Err\(Unwound\); \}",
         r"call_ptr(c, \1, \2)?;",t)
t=re.sub(r"c\.pc = (0x[0-9a-f]+); (ee::s\d+::\w+)\(c\);\n\s*if c\.pc != (0x[0-9a-f]+) \{ return Err\(Unwound\); \}",
         r"Callee { addr: \1, f: \2 }.call(c, \3)?;",t)
assert 'call_indirect' not in t, [l for l in t.split('\n') if 'call_indirect' in l][:3]
assert 'return Err' not in t
open('scratchpad/oc1043/funcs.rs','w').write(t)
