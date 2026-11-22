"""Read Tricky PS2 prefab hierarchies, cubic tracks and quantized meshes.

Format reference: SSX-Library BDHandler's PS2 model reader. This module
decodes source assets; it does not execute source animations or gameplay.
"""

from __future__ import annotations

import math
import struct

from tricky_assets import FormatError


def _extent(data: bytes, start: int, size: int, end: int | None = None) -> None:
    if start < 0 or size < 0 or start + size > (len(data) if end is None else end):
        raise FormatError("Tricky prefab data extends beyond its declared extent")


def _align16(position: int) -> int:
    return (position + 15) & ~15


def _mesh_chunk(data: bytes, start: int, end: int, scale: tuple) -> tuple[dict, int]:
    _extent(data, start, 80, end)
    strips = struct.unpack_from("<I", data, start + 48)[0]
    vertices = struct.unpack_from("<I", data, start + 56)[0]
    if strips > 256 or not 0 < vertices <= 256:
        raise FormatError("Tricky mesh chunk has unsupported strip/vertex counts")
    pos = start + 80
    _extent(data, pos, strips * 2, end)
    raw_strips = struct.unpack_from("<" + "h" * strips, data, pos)
    if any(length < 9 or length % 3 for length in raw_strips):
        raise FormatError("Tricky mesh chunk has an invalid strip length")
    strip_lengths = [length // 3 for length in raw_strips]
    if sum(strip_lengths) != vertices:
        raise FormatError("Tricky mesh strips do not cover its vertices")
    pos = _align16(pos + 2 * strips) + 64
    _extent(data, pos, vertices * 4, end)
    uv_values = struct.unpack_from("<" + "h" * (vertices * 2), data, pos)
    uv = [list(value / 4096 for value in uv_values[index:index + 2])
          for index in range(0, len(uv_values), 2)]
    pos = _align16(pos + vertices * 4) + 48
    _extent(data, pos, vertices * 6, end)
    normal_values = struct.unpack_from("<" + "h" * (vertices * 3), data, pos)
    normals = [list(value / 32768 for value in normal_values[index:index + 3])
               for index in range(0, len(normal_values), 3)]
    pos = _align16(pos + vertices * 6) + 16
    _extent(data, pos, vertices * 6, end)
    xyz_values = struct.unpack_from("<" + "h" * (vertices * 3), data, pos)
    positions = [[xyz_values[index + axis] * scale[axis] / 32768 for axis in range(3)]
                 for index in range(0, len(xyz_values), 3)]
    pos = _align16(pos + vertices * 6) + 16
    _extent(data, pos, 0, end)
    triangles = []
    base = 0
    for length in strip_lengths:
        for index in range(2, length):
            triangle = ([base + index - 2, base + index - 1, base + index]
                        if index % 2 == 0 else
                        [base + index - 1, base + index - 2, base + index])
            if len({tuple(positions[vertex]) for vertex in triangle}) == 3:
                triangles.append(triangle)
        base += length
    return {"positions": positions, "normals": normals, "uv": uv,
            "strip_lengths": strip_lengths, "triangles": triangles}, pos


def _object_animation(data: bytes, start: int, end: int) -> dict:
    """Read the six-axis cubic tracks, preserving authored coefficients/time.

    Tricky offsets are relative to the animation descriptor. Native MDR uses
    the same channel/curve records with offsets relative to its model payload;
    ssx3_models performs that relocation when writing the target resource.
    """
    _extent(data, start, 36, end)
    base = struct.unpack_from("<6f3I", data, start)
    mask, count, table = base[6:]
    if mask & ~0x3f or count != bin(mask).count("1"):
        raise FormatError("Tricky animation channel mask/count disagree")
    _extent(data, start + table, count * 4, end)
    channels = []
    for index in range(count):
        relative = struct.unpack_from("<I", data, start + table + index * 4)[0]
        channel = start + relative
        _extent(data, channel, 8, end)
        segments, segment_offset = struct.unpack_from("<2I", data, channel)
        # The source reader consumes the records following this header.
        # Preserve and inspect the offset rather than inventing key samples.
        _extent(data, channel + segment_offset, segments * 24, end)
        curves = [list(struct.unpack_from("<6f", data, channel + segment_offset + i * 24))
                  for i in range(segments)]
        if not all(math.isfinite(value) for curve in curves for value in curve):
            raise FormatError("Tricky animation contains a non-finite curve")
        channels.append(curves)
    if not all(math.isfinite(value) for value in base[:6]):
        raise FormatError("Tricky animation base transform is not finite")
    return {"base_position": list(base[:3]), "base_rotation": list(base[3:6]),
            "mask": mask, "channels": channels}


def parse_prefabs(data: bytes, counts: dict, offsets: dict) -> list[dict]:
    material_blocks = []
    pos = offsets["material_blocks"]
    for _ in range(counts["material_blocks"]):
        _extent(data, pos, 4)
        count = struct.unpack_from("<I", data, pos)[0]
        pos += 4
        _extent(data, pos, 4 * count)
        block = list(struct.unpack_from("<" + "I" * count, data, pos))
        if any(material >= counts["materials"] for material in block):
            raise FormatError("Tricky prefab material block references a missing material")
        material_blocks.append(block)
        pos += 4 * count
    _extent(data, offsets["model_pointers"], counts["models"] * 4)
    models = []
    for index in range(counts["models"]):
        relative = struct.unpack_from("<I", data, offsets["model_pointers"] + index * 4)[0]
        start = offsets["models"] + relative
        _extent(data, start, 56)
        header = struct.unpack_from("<5If3f5I", data, start)
        size, object_count, object_offset, material_block, flags = header[:5]
        scale = tuple(value if value != 0 else 1.0 for value in header[6:9])
        _extent(data, start, size)
        model_end = start + size
        _extent(data, start + object_offset, object_count * 24, model_end)
        if material_block >= len(material_blocks) or not all(math.isfinite(v) for v in scale):
            raise FormatError("Tricky prefab has invalid material block or scale")
        objects = []
        for object_index in range(object_count):
            entry = start + object_offset + object_index * 24
            parent, high, medium, low, animation, matrix_offset = struct.unpack_from("<6I", data, entry)
            if parent != 0xffffffff and parent >= object_count:
                raise FormatError("Tricky prefab hierarchy references a missing parent")
            matrix = None
            if matrix_offset != 0xffffffff:
                _extent(data, entry + matrix_offset, 64, model_end)
                matrix = list(struct.unpack_from("<16f", data, entry + matrix_offset))
                if not all(math.isfinite(v) for v in matrix):
                    raise FormatError("Tricky prefab has a non-finite part transform")
            meshes = []
            bounds = None
            if high not in (0, 0xffffffff):
                geometry = entry + high
                _extent(data, geometry, 44, model_end)
                geometry_size = struct.unpack_from("<I", data, geometry)[0]
                bounds = list(struct.unpack_from("<6f", data, geometry + 4))
                mesh_count = struct.unpack_from("<I", data, geometry + 32)[0]
                _extent(data, geometry, geometry_size, model_end)
                mesh_table = geometry + 44 + 4 * mesh_count
                _extent(data, mesh_table, 28 * mesh_count, geometry + geometry_size)
                for mesh_index in range(mesh_count):
                    words = struct.unpack_from("<7I", data, mesh_table + mesh_index * 28)
                    _, material_slot, mesh_size, mesh_start, *lod_lengths = words
                    if material_slot >= len(material_blocks[material_block]):
                        raise FormatError("Tricky mesh references a missing prefab material")
                    mesh_pos = offsets["mesh_data"] + mesh_start
                    mesh_end = mesh_pos + mesh_size
                    _extent(data, mesh_pos, mesh_size)
                    chunks = []
                    while True:
                        chunk, mesh_pos = _mesh_chunk(data, mesh_pos, mesh_end, scale)
                        chunks.append(chunk)
                        if mesh_pos + 32 > mesh_end or data[mesh_pos + 31] != 0x6c:
                            break
                    meshes.append({"index": mesh_index,
                                   "material": material_blocks[material_block][material_slot],
                                   "chunks": chunks, "source_lod_lengths": lod_lengths})
            objects.append({"index": object_index, "parent": parent,
                            "matrix": matrix, "meshes": meshes, "bounds": bounds,
                            "animation_offset": animation,
                            "animation": (_object_animation(data, entry + animation, model_end)
                                          if animation not in (0, 0xffffffff) else None),
                            "source_medium_offset": medium, "source_low_offset": low})
        models.append({"index": index, "scale": scale, "flags": flags,
                       "animation_time": header[5], "objects": objects,
                       "source_vertex_count": header[10],
                       "source_strip_count": header[11]})
    return models
