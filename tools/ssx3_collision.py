"""Build native type-1 WorldCollision resources from Tricky SSF meshes."""

from __future__ import annotations

import struct

from tricky_assets import FormatError


def _pad(data: bytearray, alignment: int) -> None:
    data.extend(b"\0" * (-len(data) % alignment))


def _split_mesh(mesh: dict) -> list[dict]:
    result = []
    remap = {}
    vertices = []
    triangles = []
    normals = []

    def flush() -> None:
        nonlocal remap, vertices, triangles, normals
        if triangles:
            result.append({"vertices": vertices, "triangles": triangles, "normals": normals})
        remap, vertices, triangles, normals = {}, [], [], []

    for triangle, normal in zip(mesh["triangles"], mesh["normals"]):
        new_indices = set(triangle) - remap.keys()
        if len(remap) + len(new_indices) > 256:
            flush()
        mapped = []
        for index in triangle:
            if index not in remap:
                remap[index] = len(vertices)
                vertices.append(mesh["vertices"][index])
            mapped.append(remap[index])
        triangles.append(mapped)
        normals.append(normal)
    flush()
    return result


def collision_only_parts(pbd: dict, models: list[dict], source: dict,
                         instances: list[int], mesh_counts: dict[int, int]) -> dict[int, int]:
    """Supply model parts for collision chunks beyond the source byte-index limit.

    Collide_VolumeObject consumes one collision mesh per non-null MDR geometry
    part, even when that part has no draw groups. More property records alone
    do not make it visit the additional meshes.
    """
    bindings = {}
    extra = {}
    for instance_id in instances:
        model_id = pbd["instances"][instance_id]["model_id"]
        prop = source["properties"][source["instance_properties"][instance_id]]
        mode, index = prop["collision_mode"], prop["collision_index"]
        bindings.setdefault(model_id, set()).add((mode, index))
        if mode != 1 or index < 0:
            continue
        source_count = len(source["collisions"][index]["meshes"])
        count = mesh_counts[index]
        if count <= source_count:
            continue
        # Every affected supplied prefab is one static root with one source
        # collision mesh. Preserve that root and append draw-free siblings.
        # A multi-part/animated prefab needs an explicit part/chunk mapping.
        objects = models[model_id]["objects"]
        if (source_count != 1 or len(objects) != 1 or
                objects[0].get("animation") is not None or
                objects[0]["parent"] != 0xffffffff or not objects[0]["meshes"]):
            raise FormatError("split collision needs an explicit model-part mapping")
        extra[model_id] = count - 1
    for model_id, additional in extra.items():
        for mode, index in bindings[model_id]:
            if mode != 1 or index < 0 or mesh_counts[index] != additional + 1:
                raise FormatError("shared model has incompatible split collision bindings")
    return extra


def pack_collision(collision: dict) -> bytes:
    """Preserve every face, splitting only to fit unsigned byte indices."""
    meshes = [part for mesh in collision["meshes"] for part in _split_mesh(mesh)]
    if len(meshes) > 65535:
        raise FormatError("WorldCollision submesh count exceeds its format")
    out = bytearray(16)
    for mesh in meshes:
        start = len(out)
        out.extend(b"\0" * 20)
        indices = len(out) - start
        out.extend(bytes(index for triangle in mesh["triangles"] for index in triangle))
        _pad(out, 4)
        boxes = len(out) - start
        # The native leaf collider reads one AABB for each run of 10 faces.
        for first in range(0, len(mesh["triangles"]), 10):
            points = [mesh["vertices"][index]
                      for triangle in mesh["triangles"][first:first + 10] for index in triangle]
            low = [min(point[axis] for point in points) for axis in range(3)]
            high = [max(point[axis] for point in points) for axis in range(3)]
            out.extend(struct.pack("<6f", *low, *high))
        _pad(out, 16)
        vertices = len(out) - start
        for vertex in mesh["vertices"]:
            out.extend(struct.pack("<4f", *vertex))
        normals = len(out) - start
        for normal in mesh["normals"]:
            out.extend(struct.pack("<4f", *normal))
        struct.pack_into("<2H4I", out, start,
                         len(mesh["triangles"]), len(mesh["vertices"]),
                         indices, boxes, vertices, normals)
    runtime_header = len(out)
    out.extend(struct.pack("<2I", 0xdeadbeef, 0xdeadbeef))
    _pad(out, 16)
    runtime_table = len(out)
    out.extend(struct.pack("<" + "I" * len(meshes), *([0xdeadbeef] * len(meshes))))
    struct.pack_into("<2H3I", out, 0, 1, len(meshes), 16,
                     runtime_header, runtime_table)
    return bytes(out)


def pack_sphere_tree(physics: dict) -> bytes:
    """Tricky's serialized body layout is also SSX 3's native body input."""
    bodies = physics["bodies"]
    out = bytearray(16)
    for body in bodies:
        out.extend(bytes.fromhex(body["raw_hex"]))
    runtime_header = len(out)
    out.extend(struct.pack("<2I", 0xdeadbeef, 0xdeadbeef))
    _pad(out, 16)
    runtime_bodies = len(out)
    out.extend(struct.pack("<I", 0xdeadbeef) * (len(bodies) * 0x140 // 4))
    struct.pack_into("<2H3I", out, 0, 3, len(bodies), 16,
                     runtime_header, runtime_bodies)
    return bytes(out)
