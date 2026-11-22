#!/usr/bin/env python3
"""M131 "Golden rhomboid-triangle" battery (Python stdlib only).

Bielefeld substitution tiling (discovered by Andritz, L.); rule source:
https://tilings.math.uni-bielefeld.de/substitution/golden-rhomboid-triangle/

Prototiles (unit scale):
  Y  equilateral triangle, side 1
  O  equilateral triangle, side phi
  R  60/120-degree rhomboid, sides 1 x phi            (area 2phi in Y-units)
Inflation lambda = phi.  Area law (exact, in units sqrt(3)/4):
  a_Y : a_O : a_R = 1 : phi^2 : 2phi,   phi^2 = phi + 1
  Y -> O                    (phi^2 * 1            = phi^2)
  O -> O + Y + R            (phi^2 + 1 + 2phi     = phi^4 = phi^2 * phi^2)
  R -> R + 2O               (2phi + 2phi^2        = 2phi*phi^2        )

Exact arithmetic model.  A point is the integer 4-tuple
  (a0, a1, b0, b1)  <=>  x = (a0 + a1*phi)/2,  y = (b0 + b1*phi)*sqrt(3)/2
so the coordinate ring is Q(phi) in each axis (full complex ring
Q(sqrt5, sqrt3)); phi acts linearly:  phi*(a0,a1) = (a1, a0+a1)  (since
phi^2 = phi + 1).  60-degree rotation (parity-protected, see header of
the Rust module):
  rot60(a0,a1,b0,b1) = ((a0-3b0)/2, (a1-3b1)/2, (a0+b0)/2, (a1+b1)/2)
All refinement offsets, areas, and overlap/containment decisions below
are exact integer arithmetic (orientation signs via an exact integer
test for the sign of u + v*phi).

Outputs (pinned for the Rust module):
  * the 18 orientation-class frames (SHAPES)
  * the 18 rule entries (RULES)
  * per-depth (r,o,y) counts d=0..12, class census + position checksum
    at d=12, exact shoelace totals
  * depth-2/3/4 pairwise + depth-12 spatial-hash verification:
    no proper crossings, no interior overlaps (collinear edge contacts
    must be opposite-sided), no T-junction angle gaps: the tiling is NOT
    face-to-face (post-293 repair #6: child vertices land mid-edge; each
    T-point must close exactly to pi on both sides of the passing edge,
    checked in integer units of pi/3)
  * cross-check of prototile/orientation densities against the Bielefeld
    patch.pdf fills (fills.json): 1525 Y : 4012 O : 2477 R, rhomboid
    direction-pair classes 1025/938/514
"""
import json, math, os, sys, time
from functools import cmp_to_key

t0 = time.time()
PHI_F = (1.0 + 5.0 ** 0.5) / 2.0
SQ3_F = 3.0 ** 0.5
E = sys.stdout
FILLS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "fills.json")

# ---------------------------------------------------------------------------
# exact arithmetic
# ---------------------------------------------------------------------------

def e_add(a, b):
    return (a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3])

def e_sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3])

def e_neg(a):
    return (-a[0], -a[1], -a[2], -a[3])

def e_scale(a, k):
    return (k * a[0], k * a[1], k * a[2], k * a[3])

def phi_mul(a):
    """a * phi in the 4-tuple ring (phi^2 = phi + 1)."""
    return (a[1], a[0] + a[1], a[3], a[2] + a[3])

def e_val(a):
    return (0.5 * (a[0] + a[1] * PHI_F), 0.5 * SQ3_F * (a[2] + a[3] * PHI_F))

def cmp_phi(u, v):
    """Exact sign of u + v*phi (integers) -> -1/0/+1."""
    if u == 0 and v == 0:
        return 0
    if v == 0:
        return 1 if u > 0 else -1
    if v < 0:
        return -cmp_phi(-u, -v)
    if u >= 0:
        return 1
    a = -u  # sign of a - v*phi, a > 0, v > 0
    t = 2 * a - v
    if t <= 0:
        return 1
    return 1 if t * t < 5 * v * v else -1

def cmp_e(a, b):
    """Exact lexicographic order (x then y) of two 4-tuple points."""
    c = cmp_phi(a[0] - b[0], a[1] - b[1])
    if c:
        return c
    return cmp_phi(a[2] - b[2], a[3] - b[3])

# pair = (u, v) <=> u + v*phi
def p_add(p, q):
    return (p[0] + q[0], p[1] + q[1])

def p_sub(p, q):
    return (p[0] - q[0], p[1] - q[1])

def p_mul(p, q):
    u, v = p
    u2, v2 = q
    return (u * u2 + v * v2, u * v2 + v * u2 + v * v2)

def p_neg(p):
    return (-p[0], -p[1])

def p_cmp(p, q):
    return cmp_phi(p[0] - q[0], p[1] - q[1])

def p_pow_phi(k):
    """phi^k as a pair (u, v), k >= 0."""
    r = (1, 0)
    for _ in range(k):
        r = p_mul(r, (0, 1))
    return r

# ---------------------------------------------------------------------------
# polygon helpers (exact)
# ---------------------------------------------------------------------------

def rot60(a):
    a0, a1, b0, b1 = a
    assert (a0 - 3 * b0) % 2 == 0 and (a1 - 3 * b1) % 2 == 0
    assert (a0 + b0) % 2 == 0 and (a1 + b1) % 2 == 0
    return ((a0 - 3 * b0) // 2, (a1 - 3 * b1) // 2, (a0 + b0) // 2, (a1 + b1) // 2)

def rot60_pts(pts):
    return [rot60(p) for p in pts]

def rot_k(k, pts):
    for _ in range(k % 6):
        pts = rot60_pts(pts)
    return pts

def mir(pts):
    return [(p[0], p[1], -p[2], -p[3]) for p in pts]

def norm_key(pts):
    # exact translation normalization: subtract the exact-min vertex,
    # then sort the set by the exact lexicographic key
    best = pts[0]
    for p in pts[1:]:
        if cmp_e(p, best) < 0:
            best = p
    shifted = [e_sub(p, best) for p in pts]
    shifted.sort(key=cmp_to_key(lambda a, b: cmp_e(a, b)))
    return tuple(shifted)

def shoelace(pts):
    """S with area = (sqrt(3)/4) * (u + v*phi), S = (u,v) = shoelace sum."""
    s = (0, 0)
    n = len(pts)
    for i in range(n):
        a = pts[i]
        b = pts[(i + 1) % n]
        s = p_add(s, p_sub(p_mul((a[0], a[1]), (b[2], b[3])),
                           p_mul((b[0], b[1]), (a[2], a[3]))))
    return s

def edge_len2(p, q):
    """|q-p|^2 * 4 as a pair (u,v) (exact).  The y-axis carries a sqrt(3)
    factor (y = (b0+b1*phi)*sqrt(3)/2), so |dy|^2 contributes 3*|dy_pair|^2."""
    dx = (q[0] - p[0], q[1] - p[1])
    dy = (q[2] - p[2], q[3] - p[3])
    u = dx[0] ** 2 + dx[1] ** 2 + 3 * dy[0] ** 2 + 3 * dy[1] ** 2
    v = 2 * dx[0] * dx[1] + dx[1] ** 2 + 6 * dy[0] * dy[1] + 3 * dy[1] ** 2
    return (u, v)

def cross(p, q):
    """2D cross product of vectors p, q -> pair (u,v), exact."""
    return p_sub(p_mul((p[0], p[1]), (q[2], q[3])),
                 p_mul((q[0], q[1]), (p[2], p[3])))

def orient(a, b, c):
    """Exact sign of cross(b-a, c-a): -1/0/+1."""
    return p_cmp(cross((b[0] - a[0], b[1] - a[1], b[2] - a[2], b[3] - a[3]),
                       (c[0] - a[0], c[1] - a[1], c[2] - a[2], c[3] - a[3])), (0, 0))

def dot4(u, v):
    """Exact dot product of 4-tuples u, v as a pair (U, V) = U + V*phi,
    equal to 4x the Euclidean dot product (coordinates carry the 1/2
    factor, the y-axis the sqrt(3)/2 factor):
    4*dot = pair(ax)*pair(bx) + 3*pair(ay)*pair(by)."""
    dx = p_mul((u[0], u[1]), (v[0], v[1]))
    dy = p_mul((u[2], u[3]), (v[2], v[3]))
    return p_add(dx, (3 * dy[0], 3 * dy[1]))

def in_convex_closed(pts, p):
    """p in convex CCW polygon (closed).  pts must be CCW (positive area)."""
    n = len(pts)
    pos = 0
    for i in range(n):
        o = orient(pts[i], pts[(i + 1) % n], p)
        if o < 0:
            return False
        if o > 0:
            pos = 1
    return True  # all >= 0 (collinear-on-boundary allowed)

def in_convex_strict(pts, p):
    """p strictly inside convex CCW polygon."""
    n = len(pts)
    for i in range(n):
        if orient(pts[i], pts[(i + 1) % n], p) <= 0:
            return False
    return True

def proper_cross(a1, a2, b1, b2):
    """Proper crossing of open segments a1a2 and b1b2 (exact)."""
    o1 = orient(a1, a2, b1)
    o2 = orient(a1, a2, b2)
    if o1 == 0 or o2 == 0:
        return False
    o3 = orient(b1, b2, a1)
    o4 = orient(b1, b2, a2)
    if o3 == 0 or o4 == 0:
        return False
    return (o1 > 0) != (o2 > 0) and (o3 > 0) != (o4 > 0)

def vertex_strictly_on_seg(v, p, q):
    """v collinear with pq and strictly between (T-junction witness)."""
    if orient(p, q, v) != 0:
        return False
    # between via x (or y for vertical) coordinate, exact
    if p[0] != q[0] or p[1] != q[1]:
        lo, hi = (p, q) if cmp_e(p, q) < 0 else (q, p)
    else:
        return False
    return cmp_e(lo, v) < 0 and cmp_e(v, hi) < 0

def seg_overlap_full(a1, a2, b1, b2):
    """For two collinear closed segments, return
       'none' | 'point' | 'full-a' | 'full-b' | 'partial' (a T-junction
       overlap: a positive-length overlap that is a proper subsegment of
       both, or of exactly one)."""
    if orient(a1, a2, b1) != 0 or orient(a1, a2, b2) != 0:
        return 'none'
    # order along the line by exact lexicographic key
    a1o, a2o = (a1, a2) if cmp_e(a1, a2) < 0 else (a2, a1)
    b1o, b2o = (b1, b2) if cmp_e(b1, b2) < 0 else (b2, b1)
    # post-293 repair #7: lo is the LATER start, hi the EARLIER end
    # (original computed lo = min of the starts -> 6 of 7 synthetic
    # containment/touch cases misclassified; see NOTES.md)
    lo = a1o if cmp_e(a1o, b1o) > 0 else b1o
    hi = a2o if cmp_e(a2o, b2o) < 0 else b2o
    if cmp_e(lo, hi) > 0:
        return 'none'
    if lo == hi:
        return 'point'
    a_full = (lo == a1o and hi == a2o)
    b_full = (lo == b1o and hi == b2o)
    if a_full and b_full:
        return 'full-a'
    if a_full:
        return 'full-a'
    if b_full:
        return 'full-b'
    return 'partial'

def edges_of(pts):
    n = len(pts)
    return [(pts[i], pts[(i + 1) % n]) for i in range(n)]

# ---------------------------------------------------------------------------
# the 18 orientation classes
# ---------------------------------------------------------------------------

Y0 = [(0, 0, 0, 0), (2, 0, 0, 0), (1, 0, 1, 0)]
O0 = [(0, 0, 0, 0), (0, 2, 0, 0), (0, 1, 0, 1)]
R0 = [(0, 0, 0, 0), (2, 0, 0, 0), (2, 1, 0, 1), (0, 1, 0, 1)]

FRAMES = []
for k in range(6):
    FRAMES.append(rot_k(k, Y0))          # classes  0..5  (Y_k)
for k in range(6):
    FRAMES.append(rot_k(k, O0))          # classes  6..11 (O_k)
for k in range(3):
    FRAMES.append(rot_k(k, R0))          # classes 12..14 (R family A, k=0,1,2)
for k in range(3):
    FRAMES.append(rot_k(k, mir(R0)))     # classes 15..17 (R family B, k=0,1,2)

# normalize vertex order to CCW (the mirror family starts out CW)
for c in range(18):
    if p_cmp(shoelace(FRAMES[c]), (0, 0)) < 0:
        FRAMES[c] = list(reversed(FRAMES[c]))

# sanity: CCW orientation, areas, edge lengths
FAMILY = [(0, 5), (6, 11), (12, 17)]
AREA_WANT = {(0, 5): (2, 0), (6, 11): (2, 2), (12, 17): (0, 4)}   # shoelace S = 2*area
LEN_WANT = {(0, 5): [(4, 0)] * 3, (6, 11): [(4, 4)] * 3,
            (12, 17): [(4, 0), (4, 4), (4, 0), (4, 4)]}
for c, fr in enumerate(FRAMES):
    S = shoelace(fr)
    fam = (0, 5) if c <= 5 else ((6, 11) if c <= 11 else (12, 17))
    assert S == AREA_WANT[fam], (c, S)
    Ls = [edge_len2(fr[i], fr[(i + 1) % len(fr)]) for i in range(len(fr))]
    assert Ls == LEN_WANT[fam], (c, Ls)

# corner angles in units of pi/3 (exact).  Y/O frames: all 60 degrees.
# R frames: two 60 + two 120 degrees.  Exact test: cos^2 == 1/4  <=>
# 4*dot^2 == |w1|^2 * |w2|^2 (|.|^2 is the edge_len2 convention, 4x true);
# the sign of the dot picks 60 (+) vs 120 (-).
ANGLE_UNITS = []
for c, fr in enumerate(FRAMES):
    nfr = len(fr)
    units = []
    for i in range(nfr):
        w1 = e_sub(fr[i], fr[(i - 1) % nfr])
        w2 = e_sub(fr[(i + 1) % nfr], fr[i])
        d = dot4(w1, w2)
        l1 = edge_len2(fr[(i - 1) % nfr], fr[i])
        l2 = edge_len2(fr[i], fr[(i + 1) % nfr])
        assert p_cmp(p_mul((4, 0), p_mul(d, d)), p_mul(l1, l2)) == 0, \
            ("corner not exactly 60/120", c, i)
        # w1 points INTO the vertex, w2 out: angle(w1,w2) = pi - interior
        # angle, so dot > 0  <=>  interior angle 120 degrees (2 units)
        units.append(2 if p_cmp(d, (0, 0)) > 0 else 1)
    ANGLE_UNITS.append(units)
    if c < 12:
        assert units == [1] * nfr, (c, units)
    else:
        assert sorted(units) == [1, 1, 2, 2], (c, units)

# unanchored-set distinctness.  Y and O are equilateral triangles: a 120-degree
# rotation (rot_k with k=2 or 4) preserves the vertex SET, so frames {c0,c0+2,
# c0+4} of each family share one set -> 2 unanchored sets per family (orbits
# {0,2,4} and {1,3,5}).  The 6 R parallelogram frames have no 120-degree
# self-symmetry and stay 6 distinct.  Quotient Y/O by 120-degree rotation:
# 2 + 2 + 6 = 10 unanchored sets.  The 18 ANCHORED classes (SHAPES/RULES) are
# unchanged; conjugate() resolves a quotient key to the family representative
# and solves the exact lattice translation (vertex-sum / n, integral by
# parity protection).
KEY2CLS = {}
KEYFAM = {}
for c, fr in enumerate(FRAMES):
    if c < 6:       # Y_k: set orbits {0,2,4}, {1,3,5}
        rep = c % 2
    elif c < 12:    # O_k: same, shifted by 6
        rep = 6 + (c - 6) % 2
    else:           # R: 6 distinct
        rep = c
    k = norm_key(FRAMES[rep])
    fam = 'Y' if rep < 6 else ('O' if rep < 12 else 'R')
    if k in KEY2CLS:
        # orbit member of an already-registered set: expected for Y/O
        assert KEYFAM[k] == fam, ("cross-family set collision", c, rep, KEY2CLS[k])
        continue
    KEY2CLS[k] = rep
    KEYFAM[k] = fam
assert len(KEY2CLS) == 10, len(KEY2CLS)

# ---------------------------------------------------------------------------
# prototype dissections (children tile phi * parent exactly)
# ---------------------------------------------------------------------------
# class indices: Y_k = k, O_k = 6+k, R_A_k = 12+k, R_B_k = 15+k
PROTO = {
    0:  [(6, (0, 0, 0, 0))],                                    # Y0 -> O
    6:  [(6, (0, 0, 0, 0)), (0, (0, 1, 0, 1)), (15, (0, 1, 0, 1))],  # O0 -> O Y R_B0
    12: [(6, (0, 0, 0, 0)), (9, (0, 3, 0, 1)), (16, (0, 1, 0, 1))],  # R0 -> O0 O3 R_B1
}

def children_of(parent_cls, entries):
    pol = []
    for cc, off in entries:
        pol.append([e_add(off, v) for v in FRAMES[cc]])
    return pol

def verify_dissection(parent_cls, entries, label):
    """children tile phi * FRAMES[parent_cls] exactly (no overlap/gap)."""
    parent = FRAMES[parent_cls]
    inflated = [phi_mul(p) for p in parent]
    assert shoelace(inflated) == p_mul(shoelace(parent), (1, 1)), (label, "area law")
    ch = children_of(parent_cls, entries)
    # (a) child area sum
    tot = (0, 0)
    for poly in ch:
        tot = p_add(tot, shoelace(poly))
    assert tot == p_mul(shoelace(parent), (1, 1)), (label, "area sum", tot)
    # (b) each child inside the inflated parent (convex, vertex PIP)
    for poly in ch:
        for v in poly:
            assert in_convex_closed(inflated, v), (label, "child vertex outside parent", v)
    # (c) pairwise: no proper crossing, no vertex strictly inside, no
    #     T-junction, no partial collinear overlap
    m = len(ch)
    for i in range(m):
        for j in range(i + 1, m):
            for (a1, a2) in edges_of(ch[i]):
                for (b1, b2) in edges_of(ch[j]):
                    assert not proper_cross(a1, a2, b1, b2), (label, i, j, "proper crossing")
                    ov = seg_overlap_full(a1, a2, b1, b2)
                    assert ov in ('none', 'point', 'full-a', 'full-b'), (label, i, j, ov)
            for v in ch[i]:
                assert not in_convex_strict(ch[j], v), (label, i, j, "vertex strictly inside")
            for v in ch[j]:
                assert not in_convex_strict(ch[i], v), (label, i, j, "vertex strictly inside")
    # (d) boundary: the once-occurring directed edges partition the
    #     inflated parent boundary (edge-to-edge, no gap)
    from collections import Counter
    cnt = Counter()
    for poly in ch:
        for (a, b) in edges_of(poly):
            cnt[(a, b)] += 1
    boundary = [ab for ab, n in cnt.items() if n == 1]
    # each inflated parent edge must be partitioned by boundary edges
    pe = edges_of(inflated)
    for (P, Q) in pe:
        d = e_sub(Q, P)
        line_edges = []
        for (a, b) in boundary:
            if orient(P, Q, a) == 0 and orient(P, Q, b) == 0 and orient(a, b, P) == 0:
                # same line: must be co-oriented (a->b parallel to P->Q)
                t = cross(d, e_sub(b, a))
                assert p_cmp(t, (0, 0)) == 0, (label, "boundary edge skew")
                line_edges.append((a, b))
        if not line_edges:
            raise AssertionError((label, "parent edge uncovered", P, Q))
        # order along the line (exact pair ordering) and check they tile it exactly
        def pos_along(a):
            # projection parameter via x (or y), exact pair
            if d[0] != 0 or d[1] != 0:
                return (a[0] - P[0], a[1] - P[1])
            return (a[2] - P[2], a[3] - P[3])
        def along(t):
            # exact projection parameter of (t - P) along d (dot with d):
            # positive scalar multiple of the along-edge parameter for any
            # d direction, so the order is correct for -x/-y edges too
            r = e_sub(t, P)
            return p_add(p_mul((r[0], r[1]), (d[0], d[1])),
                         p_mul((3 * r[2], 3 * r[3]), (3 * d[2], 3 * d[3])))
        line_edges.sort(key=cmp_to_key(
            lambda ab, cd: p_cmp(along(ab[0]), along(cd[0]))))
        # exact sequential tiling: first starts at P, last ends at Q,
        # consecutive edges touch
        tot_len = (0, 0)
        prev = None
        for (a, b) in line_edges:
            va, vb = pos_along(a), pos_along(b)
            # edge must point along +d (co-oriented with the parent edge):
            # compare the sign of the edge's projection against d's own sign
            want = p_cmp((d[0], d[1]), (0, 0)) if (d[0] != 0 or d[1] != 0) else \
                   p_cmp((d[2], d[3]), (0, 0))
            sgn = p_cmp((e_sub(b, a)[0], e_sub(b, a)[1]), (0, 0)) if (d[0] != 0 or d[1] != 0) else \
                  p_cmp((e_sub(b, a)[2], e_sub(b, a)[3]), (0, 0))
            assert sgn == want, (label, "boundary edge anti-oriented")
            if prev is not None:
                assert va == prev, (label, "boundary gap/overlap on parent edge")
            prev = vb
            tot_len = p_add(tot_len, (vb[0] - va[0], vb[1] - va[1]))
        assert tot_len == (d[0], d[1]) if (d[0] != 0 or d[1] != 0) else tot_len == (d[2], d[3]), (label, "length mismatch")
    print(f"  {label}: OK  ({m} children)")

for pc in (0, 6, 12):
    verify_dissection(pc, PROTO[pc], f"proto {pc}")

# ---------------------------------------------------------------------------
# conjugation: all 18 rules
# ---------------------------------------------------------------------------

def make_T(k, mirror):
    def T(e):
        e2 = (e[0], e[1], -e[2], -e[3]) if mirror else e
        for _ in range(k % 6):
            e2 = rot60(e2)
        return e2
    return T

def conjugate(T, proto_entries):
    out = []
    for (cc0, off0) in proto_entries:
        Tset = [T(v) for v in FRAMES[cc0]]
        cc = KEY2CLS[norm_key(Tset)]
        # Tset = FRAMES[cc] + t exactly;  t = (sum_T - sum_F) / n
        n = len(Tset)
        sT = tuple(sum(v[i] for v in Tset) for i in range(4))
        sF = tuple(sum(v[i] for v in FRAMES[cc]) for i in range(4))
        t = []
        for i in range(4):
            dlt = sT[i] - sF[i]
            assert dlt % n == 0, ("translation not integral", cc0, cc, i)
            t.append(dlt // n)
        t = tuple(t)
        out.append((cc, e_add(T(off0), t)))
    return out

RULES = [None] * 18
for k in range(6):
    RULES[k] = conjugate(make_T(k, False), PROTO[0])      # Y_k
    RULES[6 + k] = conjugate(make_T(k, False), PROTO[6])  # O_k
for j in range(3):
    RULES[12 + j] = conjugate(make_T(j, False), PROTO[12])   # R_A_j
    RULES[15 + j] = conjugate(make_T(j, True), PROTO[12])    # R_B_j

# verify every rule dissects its parent exactly
for c in range(18):
    verify_dissection(c, RULES[c], f"rule {c}")

# ---------------------------------------------------------------------------
# growth from the seed R0 (class 12 at the origin)
# ---------------------------------------------------------------------------

def refine(tiles):
    out = []
    for (cls, pos) in tiles:
        p = phi_mul(pos)
        for (cc, off) in RULES[cls]:
            out.append((cc, e_add(p, off)))
    return out

def family_of(cls):
    return 2 if cls <= 5 else (1 if cls <= 11 else 0)  # Y, O, R

def census(tiles):
    r = o = y = 0
    for (cls, _) in tiles:
        f = family_of(cls)
        if f == 0:
            r += 1
        elif f == 1:
            o += 1
        else:
            y += 1
    return r, o, y

def class_counts(tiles):
    cc = [0] * 18
    for (cls, _) in tiles:
        cc[cls] += 1
    return cc

def pos_checksum(tiles):
    s = (0, 0, 0, 0)
    for (_, p) in tiles:
        s = e_add(s, p)
    return s

def shoelace_total(tiles):
    s = (0, 0)
    for (cls, pos) in tiles:
        s = p_add(s, shoelace(FRAMES[cls]))
    return s

# region(d) = phi^d * R0 (unrecentered), corners:
REGION_CORNERS = [(0, 0, 0, 0), (0, 0, 0, 0), (0, 0, 0, 0), (0, 0, 0, 0)]
def region_corners(d):
    return [phi_mul_phi_d(p, d) for p in R0]

def phi_mul_phi_d(p, d):
    e = p
    for _ in range(d):
        e = phi_mul(e)
    return e

def off_line_vertex(line_p, line_q, poly):
    """Sign of the side of line(line_p,line_q) that convex tile poly lies on
    (orient sign at the first off-line vertex; convexity makes it the same
    for all of them).  0 only for a degenerate line-lying tile."""
    for v in poly:
        o = orient(line_p, line_q, v)
        if o != 0:
            return o
    return 0

def opp_side(a1, a2, b1, b2, polyA, polyB):
    """True iff the two collinear tiles lie on OPPOSITE geometric sides of
    the shared line.  orient signs are relative to each edge's own
    direction, so the direction agreement of the two edges is folded in
    via the sign of their exact dot product."""
    si = off_line_vertex(a1, a2, polyA)
    sj = off_line_vertex(b1, b2, polyB)
    sd = p_cmp(dot4(e_sub(a2, a1), e_sub(b2, b1)), (0, 0))
    assert sd in (1, -1), ("parallel zero edge", a1, a2, b1, b2)
    return si * sj * sd < 0

def collect_t_incidences(polys, edges):
    """All T-junction witnesses (point, vertex_tile, passing_tile): a vertex
    of tile i strictly in the interior of an edge of tile j."""
    inc = []
    n = len(polys)
    for i in range(n):
        for j in range(i + 1, n):
            for v in polys[i]:
                for (b1, b2) in edges[j]:
                    if vertex_strictly_on_seg(v, b1, b2):
                        inc.append((v, i, j))
            for v in polys[j]:
                for (a1, a2) in edges[i]:
                    if vertex_strictly_on_seg(v, a1, a2):
                        inc.append((v, j, i))
    return inc

def check_t_point_closure(tiles, polys, edges, inc, label):
    """post-293 repair #6 -- T-junction angle closure.

    This substitution is NOT face-to-face: a tile vertex may land strictly
    in the interior of another tile's edge (a T-junction).  First
    appearance at depth 2 (5 points), 20 at d3, 72 at d4 -- inherent to the
    base rules (a child's phi-edge carries the parent's unit split points,
    see NOTES.md).  The tiling is still a valid (gap-free, overlap-free)
    partition, so the correct invariant replaces the old ntj==0 assert:
    at every T-point the angle must close EXACTLY on both sides of the
    passing edge's line -- the passing edge counts as a straight angle pi
    on its side, and every corner meeting at the point counts its exact
    60/120-degree angle (units pi/3: integer arithmetic).  Both sides
    must sum to exactly 3 units (pi).  A corner cone straddling the line
    would overlap the passing tile: hard failure."""
    if not inc:
        return 0
    vmap = {}
    for i, poly in enumerate(polys):
        for vi, v in enumerate(poly):
            vmap.setdefault(v, []).append((i, vi))
    bypt = {}
    for (p, i, j) in inc:
        bypt.setdefault(p, []).append((i, j))
    ntp2 = 0
    for p, ij in bypt.items():
        ntp2 += 1
        # passing edges (deduped: several vertex tiles may sit on the same
        # passing edge)
        passings = {}
        for (i, j) in ij:
            for (b1, b2) in edges[j]:
                if vertex_strictly_on_seg(p, b1, b2):
                    key = (j, (b1, b2) if cmp_e(b1, b2) < 0 else (b2, b1))
                    passings[key] = (b1, b2, j)
                    break
        (l1, l2, _) = next(iter(passings.values()))
        for (m1, m2, _) in passings.values():
            assert orient(l1, l2, m1) == 0 and orient(l1, l2, m2) == 0, \
                (label, "non-collinear passing edges at T-point", p)
        pset = {j for (b1, b2, j) in passings.values()}
        filling = {1: 0, -1: 0}
        for (b1, b2, j) in passings.values():
            s = off_line_vertex(l1, l2, polys[j])
            assert s in (1, -1), (label, "degenerate passing tile", p)
            filling[s] += 3
        for (k, vi) in vmap.get(p, []):
            assert k not in pset, (label, "corner and passing edge of one tile at T-point", p, k)
            poly = polys[k]
            nn = len(poly)
            s1 = orient(l1, l2, poly[(vi - 1) % nn])
            s2 = orient(l1, l2, poly[(vi + 1) % nn])
            if s1 == 0 and s2 == 0:
                raise AssertionError((label, "degenerate corner at T-point", p, k))
            if s1 == 0:
                side = s2
            elif s2 == 0:
                side = s1
            elif s1 == s2:
                side = s1
            else:
                raise AssertionError((label, "corner cone straddles T-line (overlap)", p, k))
            filling[side] += ANGLE_UNITS[tiles[k][0]][vi]
        assert filling[1] == 3 and filling[-1] == 3, \
            (label, "T-point angle not closed (pi/3 units per side)", p, filling)
    return ntp2

def check_no_overlap_pairs(tiles, label, tjunctions=True):
    """Full O(n^2) pairwise check: no proper crossings, no overlaps
    (collinear edge contacts require the tiles on OPPOSITE sides of the
    shared line; a same-side contact is an interior overlap), no vertex
    strictly inside another tile, and (optional) exact angle closure at
    every T-junction (post-293 repair #6: T-junctions are inherent to this
    substitution -- legal iff angle-closed, not forbidden)."""
    polys = [[e_add(pos, v) for v in FRAMES[cls]] for (cls, pos) in tiles]
    edges = [edges_of(p) for p in polys]
    n = len(tiles)
    ncross = ntp = ntj = 0
    for i in range(n):
        for j in range(i + 1, n):
            for (a1, a2) in edges[i]:
                for (b1, b2) in edges[j]:
                    if proper_cross(a1, a2, b1, b2):
                        ncross += 1
                    ov = seg_overlap_full(a1, a2, b1, b2)
                    if ov in ('partial', 'full-a', 'full-b'):
                        # collinear positive-length contact: legal only as a
                        # T-contact with the tiles on opposite sides of the
                        # shared line.  orient signs are per-edge-direction,
                        # so fold in the direction agreement of the two
                        # traversals (exact dot of the direction vectors).
                        si = off_line_vertex(a1, a2, polys[i])
                        sj = off_line_vertex(b1, b2, polys[j])
                        sd = p_cmp(dot4(e_sub(a2, a1), e_sub(b2, b1)), (0, 0))
                        if sd == 0 or si * sj * sd >= 0:
                            ntp += 1
            for v in polys[i]:
                if in_convex_strict(polys[j], v):
                    ntp += 1
            for v in polys[j]:
                if in_convex_strict(polys[i], v):
                    ntp += 1
            if tjunctions:
                for v in polys[i]:
                    for (b1, b2) in edges[j]:
                        if vertex_strictly_on_seg(v, b1, b2):
                            ntj += 1
                for v in polys[j]:
                    for (a1, a2) in edges[i]:
                        if vertex_strictly_on_seg(v, a1, a2):
                            ntj += 1
    assert ncross == 0, (label, "proper crossings", ncross)
    assert ntp == 0, (label, "overlaps/strict-inside", ntp)
    if tjunctions:
        inc = collect_t_incidences(polys, edges)
        tp = check_t_point_closure(tiles, polys, edges, inc, label)
        print(f"  {label}: T-points={tp} closed (post-293 #6), tj-incidences={ntj}",
              flush=True)
    return ntj

def check_in_region(tiles, d, label):
    reg = region_corners(d)
    for (cls, pos) in tiles:
        for v in FRAMES[cls]:
            assert in_convex_closed(reg, e_add(pos, v)), (label, "tile outside region")

def check_boundary(tiles, d, label):
    """The once-occurring directed edges partition the region boundary."""
    cnt = {}
    for (cls, pos) in tiles:
        for (a, b) in edges_of([e_add(pos, v) for v in FRAMES[cls]]):
            cnt[(a, b)] = cnt.get((a, b), 0) + 1
    boundary = [ab for ab, n in cnt.items() if n == 1]
    reg = region_corners(d)
    for (P, Q) in edges_of(reg):
        dd = e_sub(Q, P)
        le = []
        for (a, b) in boundary:
            if orient(P, Q, a) == 0 and orient(P, Q, b) == 0:
                le.append((a, b))
        assert le, (label, "uncovered region edge")
        def pos_along(a):
            if dd[0] != 0 or dd[1] != 0:
                return (a[0] - P[0], a[1] - P[1])
            return (a[2] - P[2], a[3] - P[3])
        def along(t):
            # exact projection parameter of (t - P) along dd: a positive
            # scalar multiple of the along-edge parameter for any direction
            r = e_sub(t, P)
            return p_add(p_mul((r[0], r[1]), (dd[0], dd[1])),
                         p_mul((3 * r[2], 3 * r[3]), (3 * dd[2], 3 * dd[3])))
        le.sort(key=cmp_to_key(lambda ab, cd: p_cmp(along(ab[0]), along(cd[0]))))
        tot = (0, 0)
        prev = None
        for (a, b) in le:
            va, vb = pos_along(a), pos_along(b)
            # co-oriented with the region edge (dominant-axis sign must
            # match dd's own sign; edges along -x are legitimate).  sgn is
            # the sign of the edge vector (b - a) on the dominant axis, the
            # same convention verify_dissection uses.
            want = p_cmp((dd[0], dd[1]), (0, 0)) if (dd[0] != 0 or dd[1] != 0) else \
                   p_cmp((dd[2], dd[3]), (0, 0))
            sgn = p_cmp((b[0] - a[0], b[1] - a[1]), (0, 0)) if (dd[0] != 0 or dd[1] != 0) else \
                  p_cmp((b[2] - a[2], b[3] - a[3]), (0, 0))
            assert sgn == want, (label, "anti-oriented boundary edge")
            if prev is not None:
                assert va == prev, (label, "boundary gap/overlap")
            prev = vb
            tot = p_add(tot, p_sub(vb, va))
        want = (dd[0], dd[1]) if (dd[0] != 0 or dd[1] != 0) else (dd[2], dd[3])
        assert tot == want, (label, "boundary length mismatch")

# ---------------------------------------------------------------------------
# spatial-hash overlap check (depth 12)
# ---------------------------------------------------------------------------

def bbox_val(cls, pos):
    fr = FRAMES[cls]
    pts = [e_val((pos[0] + v[0], pos[1] + v[1], pos[2] + v[2], pos[3] + v[3])) for v in fr]
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts]
    return (min(xs), max(xs), min(ys), max(ys))

def check_no_overlap_grid(tiles, label):
    """Grid-based pairwise exact check (same predicates as the O(n^2)
    check): no proper crossings, no overlaps (collinear contacts need
    opposite sides), no vertex strictly inside another tile, and exact
    angle closure at every T-junction (post-293 repair #6)."""
    import math
    cells = {}
    meta = []
    CELL = 1.0
    for idx, (cls, pos) in enumerate(tiles):
        meta.append((cls, pos, [e_add(pos, v) for v in FRAMES[cls]]))
        (x0, x1, y0, y1) = bbox_val(cls, pos)
        cx0, cx1 = int(math.floor(x0 / CELL)), int(math.floor(x1 / CELL))
        cy0, cy1 = int(math.floor(y0 / CELL)), int(math.floor(y1 / CELL))
        for cx in range(cx0, cx1 + 1):
            for cy in range(cy0, cy1 + 1):
                cells.setdefault((cx, cy), []).append(idx)
    ncross = ntp = ntj = 0
    nchecked = 0
    inc = []
    for idx in range(len(tiles)):
        (x0, x1, y0, y1) = bbox_val(tiles[idx][0], tiles[idx][1])
        cx0, cx1 = int(math.floor((x0 - 1.0) / CELL)), int(math.floor((x1 + 1.0) / CELL))
        cy0, cy1 = int(math.floor((y0 - 1.0) / CELL)), int(math.floor((y1 + 1.0) / CELL))
        cand = set()
        for cx in range(cx0, cx1 + 1):
            for cy in range(cy0, cy1 + 1):
                for j in cells.get((cx, cy), ()):
                    if j > idx:
                        cand.add(j)
        for j in cand:
            (clsA, posA, polyA) = meta[idx]
            (clsB, posB, polyB) = meta[j]
            # fast float bbox reject
            (ax0, ax1, ay0, ay1) = bbox_val(clsA, posA)
            (bx0, bx1, by0, by1) = bbox_val(clsB, posB)
            if ax1 < bx0 - 1e-12 or bx1 < ax0 - 1e-12 or ay1 < by0 - 1e-12 or by1 < ay0 - 1e-12:
                continue
            nchecked += 1
            eA = edges_of(polyA)
            eB = edges_of(polyB)
            for (a1, a2) in eA:
                for (b1, b2) in eB:
                    if proper_cross(a1, a2, b1, b2):
                        ncross += 1
                    ov = seg_overlap_full(a1, a2, b1, b2)
                    if ov in ('partial', 'full-a', 'full-b'):
                        si = off_line_vertex(a1, a2, polyA)
                        sj = off_line_vertex(b1, b2, polyB)
                        sd = p_cmp(dot4(e_sub(a2, a1), e_sub(b2, b1)), (0, 0))
                        if sd == 0 or si * sj * sd >= 0:
                            ntp += 1
            for v in polyA:
                if in_convex_strict(polyB, v):
                    ntp += 1
            for v in polyB:
                if in_convex_strict(polyA, v):
                    ntp += 1
            for v in polyA:
                for (b1, b2) in eB:
                    if vertex_strictly_on_seg(v, b1, b2):
                        ntj += 1
                        inc.append((v, idx, j))
            for v in polyB:
                for (a1, a2) in eA:
                    if vertex_strictly_on_seg(v, a1, a2):
                        ntj += 1
                        inc.append((v, j, idx))
    polys = [m[2] for m in meta]
    edges = [edges_of(p) for p in polys]
    tp = check_t_point_closure(tiles, polys, edges, inc, label)
    print(f"  depth 12 grid: {nchecked} candidate pairs, "
          f"cross={ncross} overlap={ntp} tj-inc={ntj} T-points={tp} ({time.time()-t0:5.1f}s)")
    assert ncross == 0 and ntp == 0, (label, ncross, ntp)

# grow
tiles = [(12, (0, 0, 0, 0))]
rows = []
counts_by_depth = []
census12 = None
checksum12 = None
sl_total12 = None
for d in range(0, 13):
    r, o, y = census(tiles)
    counts_by_depth.append((r, o, y))
    S = shoelace_total(tiles)
    want = p_mul((0, 4), p_pow_phi(2 * d))
    assert S == want, (d, S, want)
    check_in_region(tiles, d, f"depth {d} containment")
    if d in (2, 3, 4):
        check_no_overlap_pairs(tiles, f"depth {d}", tjunctions=True)
    if d == 4:
        check_boundary(tiles, d, "depth 4 boundary")
    if d == 12:
        census12 = class_counts(tiles)
        checksum12 = pos_checksum(tiles)
        sl_total12 = S
        print(f"  depth 12: {len(tiles)} tiles, verifying with spatial hash ...", flush=True)
        check_no_overlap_grid(tiles, "depth 12")
        check_boundary(tiles, d, "depth 12 boundary")
        print(f"  depth 12 census (18 classes): {census12}")
        print(f"  depth 12 pos-checksum={checksum12}  shoelace={sl_total12}")
    print(f"  d={d:2d}  r={r:6d} o={o:6d} y={y:6d}  total={len(tiles):6d}  "
          f"shoelace={S}  ({time.time()-t0:5.1f}s)", flush=True)
    if d < 12:
        tiles = refine(tiles)

# ---------------------------------------------------------------------------
# T-point configuration coverage (post-293 repair #6): every one of the 18
# classes refined twice (a single tile per seed); exact angle closure is
# verified at every T-point.  A T-point's local configuration is
# context-free -- it is determined by the rule of the passing tile (the
# substitution is context-free and a T-point descends to depth 2), so the
# 18 seeds cover every T-point configuration realizable at any depth.
# ---------------------------------------------------------------------------
passing_classes = set()
tj_total = 0
for c in range(18):
    t2 = refine(refine([(c, (0, 0, 0, 0))]))
    polys = [[e_add(pos, v) for v in FRAMES[cls]] for (cls, pos) in t2]
    edges = [edges_of(p) for p in polys]
    inc = collect_t_incidences(polys, edges)
    tp = check_t_point_closure(t2, polys, edges, inc, f"seed class {c}")
    for (p, i, j) in inc:
        passing_classes.add(t2[j][0])
    tj_total += tp
print(f"  18-seed T-config coverage: {tj_total} T-points closed, "
      f"passing classes = {sorted(passing_classes)}")

# ---------------------------------------------------------------------------
# cross-check against the Bielefeld patch.pdf fill census (fills.json,
# float data parsed from the PDF): prototile counts and rhomboid
# side-direction-pair classes
# ---------------------------------------------------------------------------
if os.path.exists(FILLS):
    with open(FILLS) as f:
        fills = json.load(f)
    ny = no_ = nr = 0
    paircnt = {(0, 60): 0, (0, 120): 0, (60, 120): 0}
    for fl in fills:
        pts = [tuple(map(float, q)) for q in fl["pts"]]
        n = len(pts)

        def ang(i, m):
            (x1, y1), (x2, y2) = pts[i], pts[(i + 1) % m]
            return math.degrees(math.atan2(y2 - y1, x2 - x1)) % 180.0

        if n == 3:
            L = min(math.dist(pts[i], pts[(i + 1) % 3]) for i in range(3))
            if abs(L - 1.0) < 0.05:
                ny += 1
            elif abs(L - PHI_F) < 0.05:
                no_ += 1
            else:
                raise AssertionError(("triangle size", L))
        elif n == 4:
            Ls = [math.dist(pts[i], pts[(i + 1) % 4]) for i in range(4)]
            longs = [i for i in range(4) if Ls[i] > 1.3]
            shorts = [i for i in range(4) if Ls[i] <= 1.3]
            assert len(longs) == 2 and len(shorts) == 2, ("rhombus sides", Ls)

            def snap(d):
                return min((0.0, 60.0, 120.0),
                           key=lambda c: min(abs(d - c), 180.0 - abs(d - c)))

            d1, d2 = snap(ang(longs[0], 4)), snap(ang(shorts[0], 4))
            key = (min(d1, d2), max(d1, d2))
            paircnt[key] += 1
            nr += 1
        else:
            raise AssertionError(("polygon size", n))
    print(f"  patch fills: Y={ny} O={no_} R={nr}  dir-pairs={paircnt}")
    assert (ny, no_, nr) == (1525, 4012, 2477), ("patch census", ny, no_, nr)
    assert sum(paircnt.values()) == nr
    # asymptotic ratios y:o:r -> 1/phi : phi : 1 (tolerance: the published
    # patch is not an exact depth patch)
    for got, want in ((ny, 1.0 / PHI_F), (no_, PHI_F), (nr, 1.0)):
        assert abs(got / nr - want) / want < 0.02, ("patch ratio", got, want)

print("battery OK", f"({time.time()-t0:.1f}s)")
