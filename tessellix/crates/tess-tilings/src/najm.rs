//! Najm template tilings (Kaplan dissertation §3.6.1), expressed as
//! [`LatticeTiling`] data tables.
//!
//! The Najm construction (as generalized in the dissertation) associates a
//! regular polygon with every vertex of a wallpaper generating triangle
//! (the [p,q] orbifold triangle: angles π/p at A, π/2 at B, π/q at C).
//! The notation `([p,q]; m_A o_A, m_B o_B, m_C o_C; inflation)` puts an
//! n_A = p·m_A-gon at A, an n_B = 2·m_B-gon at B, and an n_C = q·m_C-gon
//! at C — a zero multiplier means no polygon there — sized by an inflation
//! rule and oriented so the designated ray (A→AB, B→BC, C→CA) meets the
//! named feature (`e` = edge midpoint, `v` = vertex); features lie every
//! π/n_v from that ray, alternating type. Reflecting the triangle in its
//! edges tiles the plane, and the polygons' union leaves hole tiles.
//!
//! This module implements the Euclidean generator — notation → circumradii
//! ([`NajmSpec::radii`], inflation cases 1–4) — plus the first two
//! substrate entries. The hole tiles (bowties, stars) come from
//! construction-specific formulas verified against the spike; a generic
//! hole decomposition is future work (items 22a/22c consume substrates).
//!
//! Two documented deviations from the dissertation text, each re-derived
//! independently (Euclidean C(x) = 2πx, E(x) = 1, T(x) = 2πx is linear,
//! so all inflation equations are linear):
//!
//! * **Case 3 (simultaneous pair)**: the extracted text reads
//!   `T(l) = T(x)/sin(π/n)`, missing a factor ½ — `l = x/sin(π/n)` would
//!   exceed the polygon's own circumradius `x/(2 sin(π/n))` and Fig. 3.21
//!   would not close. We solve `l = x/(2 sin(π/n))` (vertex extent) or
//!   `l = x/(2 tan(π/n))` (edge extent), which makes the two polygons'
//!   chords on the shared triangle edge coincide exactly (60-digit
//!   verification: agreement below 1e-55).
//! * **Edge extents** in cases 1/2: an edge midpoint at extent means the
//!   apothem, `l = r cos(π/n)`.

use tess_core::TilingCategory;
use tess_core::geometry::{Polygon, Vec2};
use tess_core::symmetry::{LatticeKind, WallpaperGroup};
use tess_core::view::ColorMode;

use crate::lattice::{similarity_map, Cell, LatticeTiling, SeedPattern, SIMILARITY_PARAMS};

/// Triangle vertex roles of the [p,q] generating triangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// Angle π/p; designated ray A→AB.
    A,
    /// Right angle; designated ray B→BC.
    B,
    /// Angle π/q; designated ray C→CA.
    C,
}

/// Which feature the designated ray meets (the `v`/`e` suffix).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    /// A polygon vertex lies on the ray.
    Vertex,
    /// An edge midpoint lies on the ray.
    Edge,
}

/// A triangle edge joining two nonzero polygon vertices (case 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diad {
    /// Edge AB.
    Ab,
    /// Edge AC (the hypotenuse).
    Ac,
    /// Edge BC.
    Bc,
}

impl Diad {
    fn verts(self) -> (Axis, Axis) {
        match self {
            Diad::Ab => (Axis::A, Axis::B),
            Diad::Ac => (Axis::A, Axis::C),
            Diad::Bc => (Axis::B, Axis::C),
        }
    }
}

/// One step of a sequential inflation list.
#[derive(Debug, Clone, Copy)]
pub enum Step {
    /// Pin a polygon's circumradius (the notation's scale lead, `A=0.6`).
    Fixed {
        /// Axis whose polygon is pinned.
        axis: Axis,
        /// Its circumradius, in units where |AC| = 1.
        radius: f64,
    },
    /// Cases 1/2: inflate until the tightest constraint binds — against
    /// every already-placed polygon along their shared triangle edge
    /// (case 1), and against the opposite triangle edge (case 2; A and C
    /// only, whose opposite-edge foot is B). Takes the minimum.
    Inflate(Axis),
}

/// How the constructor sizes the polygons (diss. §3.6.1 inflation cases).
#[derive(Debug, Clone, Copy)]
pub enum Inflation {
    /// Case 3: the two polygons touch across one common edge lying on the
    /// diad's triangle edge; closed form in the common edge length x.
    Pair(Diad),
    /// Case 4 ("ABC"): contacts on all three triangle edges simultaneously;
    /// solved by bisection on r_A (the BC gap decreases monotonically).
    Triple,
    /// Sequential list of [`Step`]s (may begin with a [`Step::Fixed`]).
    Order(&'static [Step]),
}

/// A full template notation `([p,q]; m_A o_A, m_B o_B, m_C o_C; inflation)`.
#[derive(Debug, Clone, Copy)]
pub struct NajmSpec {
    /// Orbifold [p,q]: angle π/p at A, π/q at C, right angle at B.
    pub p: u32,
    /// Orbifold second index.
    pub q: u32,
    /// Multipliers (m_A, m_B, m_C); 0 = no polygon at that vertex.
    /// m_B = 1 is forbidden (its "2-gon" is degenerate).
    pub mult: [u32; 3],
    /// Orientations (o_A, o_B, o_C) at the designated rays; the entry for
    /// a zero multiplier is unused.
    pub orient: [Feature; 3],
    /// Sizing rule.
    pub inflation: Inflation,
}

const PI: f64 = std::f64::consts::PI;

fn idx(v: Axis) -> usize {
    match v {
        Axis::A => 0,
        Axis::B => 1,
        Axis::C => 2,
    }
}

fn wrap_pi(mut a: f64) -> f64 {
    let tau = 2.0 * PI;
    while a > PI {
        a -= tau;
    }
    while a <= -PI {
        a += tau;
    }
    a
}

fn signed_area(pts: &[Vec2]) -> f64 {
    let mut a = 0.0;
    for i in 0..pts.len() {
        let p = pts[i];
        let q = pts[(i + 1) % pts.len()];
        a += p.x * q.y - q.x * p.y;
    }
    a * 0.5
}

impl NajmSpec {
    /// The generating triangle with hypotenuse AC = 1: A at the origin,
    /// C = (1, 0), B below the x-axis with the right angle there.
    pub fn triangle(p: u32) -> [Vec2; 3] {
        let a = PI / p as f64;
        [
            Vec2::ZERO,
            Vec2::new(a.cos(), -a.sin()) * a.cos(),
            Vec2::new(1.0, 0.0),
        ]
    }

    /// Side counts (n_A, n_B, n_C) = (p·m_A, 2·m_B, q·m_C); 0 = none.
    pub fn sides(&self) -> [u32; 3] {
        let mut n = [0; 3];
        if self.mult[0] > 0 {
            n[0] = self.p * self.mult[0];
        }
        if self.mult[1] > 0 {
            n[1] = 2 * self.mult[1];
        }
        if self.mult[2] > 0 {
            n[2] = self.q * self.mult[2];
        }
        n
    }

    /// Designated-ray target of an axis: A→B, B→C, C→A.
    fn designated(v: Axis) -> Axis {
        match v {
            Axis::A => Axis::B,
            Axis::B => Axis::C,
            Axis::C => Axis::A,
        }
    }

    /// Feature of P_v lying on the triangle edge v→toward. Features sit at
    /// k·(π/n_v) from the designated ray, alternating from o_v. Panics when
    /// the toward ray carries no feature (illegal notation).
    fn feature(&self, v: Axis, toward: Axis) -> Feature {
        let tri = Self::triangle(self.p);
        let ang = |w: Vec2| w.y.atan2(w.x);
        let ray = ang(tri[idx(toward)] - tri[idx(v)]);
        let des = ang(tri[idx(Self::designated(v))] - tri[idx(v)]);
        let pitch = PI / self.sides()[idx(v)] as f64;
        let t = wrap_pi(ray - des) / pitch;
        let k = t.round();
        assert!(
            (t - k).abs() < 1e-9,
            "triangle edge {v:?}->{toward:?} carries no feature of P_{v:?}"
        );
        let parity = ((k as i64) % 2 + 2) % 2;
        match (parity, self.orient[idx(v)]) {
            (0, o) => o,
            (_, Feature::Vertex) => Feature::Edge,
            (_, Feature::Edge) => Feature::Vertex,
        }
    }

    /// Extent coefficient of P_v on the edge v→toward: l = r_v · f, with
    /// f = 1 (vertex on the edge) or cos(π/n) (edge midpoint — the apothem;
    /// T(l) = T(r) cos(π/n) in the Euclidean plane).
    fn extent_f(&self, v: Axis, toward: Axis) -> f64 {
        match self.feature(v, toward) {
            Feature::Vertex => 1.0,
            Feature::Edge => (PI / self.sides()[idx(v)] as f64).cos(),
        }
    }

    /// Case-3 coefficient: with the common edge x, extent = g·x where
    /// g = 1/(2 sin(π/n)) (vertex) or 1/(2 tan(π/n)) (edge midpoint).
    fn half_g(&self, v: Axis, toward: Axis) -> f64 {
        let t = PI / self.sides()[idx(v)] as f64;
        match self.feature(v, toward) {
            Feature::Vertex => 0.5 / t.sin(),
            Feature::Edge => 0.5 / t.tan(),
        }
    }

    /// Solve the circumradii (n-indexed: [r_A, r_B, r_C]; 0 for a zero
    /// multiplier). Panics on an illegal or incomplete spec: every nonzero
    /// polygon must be sized exactly once by the inflation list.
    pub fn radii(&self) -> [f64; 3] {
        assert_ne!(self.mult[1], 1, "m_B = 1 is forbidden (degenerate 2-gon)");
        let tri = Self::triangle(self.p);
        let dist = |u: Axis, v: Axis| (tri[idx(v)] - tri[idx(u)]).length();
        let mut r = [0.0; 3];
        let mut placed = [false; 3];
        match self.inflation {
            Inflation::Pair(d) => {
                let (u, v) = d.verts();
                assert!(
                    self.mult[idx(u)] > 0 && self.mult[idx(v)] > 0,
                    "case 3 needs polygons at both vertices of the diad"
                );
                let x = dist(u, v) / (self.half_g(u, v) + self.half_g(v, u));
                for w in [u, v] {
                    r[idx(w)] = x / (2.0 * (PI / self.sides()[idx(w)] as f64).sin());
                    placed[idx(w)] = true;
                }
            }
            Inflation::Triple => {
                assert!(
                    self.mult.iter().all(|&m| m > 0),
                    "case 4 (ABC) needs all three polygons"
                );
                // B and C ride on A (case 1 along AB and AC); the BC gap
                // decreases monotonically in r_A: bisect it to zero.
                let gap = |ra: f64| -> f64 {
                    let rb = (dist(Axis::A, Axis::B) - ra * self.extent_f(Axis::A, Axis::B))
                        / self.extent_f(Axis::B, Axis::A);
                    let rc = (dist(Axis::A, Axis::C) - ra * self.extent_f(Axis::A, Axis::C))
                        / self.extent_f(Axis::C, Axis::A);
                    rb * self.extent_f(Axis::B, Axis::C)
                        + rc * self.extent_f(Axis::C, Axis::B)
                        - dist(Axis::B, Axis::C)
                };
                assert!(gap(0.0) > 0.0, "ABC: polygons already overlap at r_A = 0");
                let mut hi = 1.0;
                while gap(hi) > 0.0 {
                    hi *= 2.0;
                    assert!(hi < 1e6, "ABC: no closing solution found");
                }
                let mut lo = 0.0;
                for _ in 0..100 {
                    let m = 0.5 * (lo + hi);
                    if gap(m) > 0.0 {
                        lo = m;
                    } else {
                        hi = m;
                    }
                }
                let ra = 0.5 * (lo + hi);
                r[idx(Axis::A)] = ra;
                r[idx(Axis::B)] = (dist(Axis::A, Axis::B)
                    - ra * self.extent_f(Axis::A, Axis::B))
                    / self.extent_f(Axis::B, Axis::A);
                r[idx(Axis::C)] = (dist(Axis::A, Axis::C)
                    - ra * self.extent_f(Axis::A, Axis::C))
                    / self.extent_f(Axis::C, Axis::A);
                for w in [Axis::A, Axis::B, Axis::C] {
                    placed[idx(w)] = true;
                }
            }
            Inflation::Order(steps) => {
                for &st in steps {
                    match st {
                        Step::Fixed { axis, radius } => {
                            assert!(
                                self.mult[idx(axis)] > 0 && !placed[idx(axis)],
                                "Fixed names a nonzero polygon exactly once"
                            );
                            r[idx(axis)] = radius;
                            placed[idx(axis)] = true;
                        }
                        Step::Inflate(v) => {
                            assert!(
                                self.mult[idx(v)] > 0 && !placed[idx(v)],
                                "Inflate names a nonzero polygon exactly once"
                            );
                            let mut best = f64::INFINITY;
                            for w in [Axis::A, Axis::B, Axis::C] {
                                if placed[idx(w)] {
                                    // Case 1: touch P_w along the shared edge.
                                    best = best.min(
                                        (dist(v, w) - r[idx(w)] * self.extent_f(w, v))
                                            / self.extent_f(v, w),
                                    );
                                }
                            }
                            if v == Axis::A || v == Axis::C {
                                // Case 2: reach the opposite triangle edge
                                // (the perpendicular foot is B).
                                best = best.min(dist(v, Axis::B) / self.extent_f(v, Axis::B));
                            }
                            assert!(best.is_finite(), "nothing to inflate {v:?} against");
                            r[idx(v)] = best;
                            placed[idx(v)] = true;
                        }
                    }
                }
            }
        }
        for v in [Axis::A, Axis::B, Axis::C] {
            assert_eq!(
                placed[idx(v)],
                self.mult[idx(v)] > 0,
                "the inflation must size every nonzero polygon exactly once"
            );
        }
        r
    }
}

/// CCW regular n-gon with circumcenter `center`, radius `r`, first vertex
/// at angle `phase`.
fn ngon(center: Vec2, r: f64, n: usize, phase: f64) -> Polygon {
    let step = 2.0 * PI / n as f64;
    let pts: Vec<Vec2> = (0..n)
        .map(|k| {
            let a = phase + step * k as f64;
            center + Vec2::new(a.cos(), a.sin()) * r
        })
        .collect();
    Polygon::from_slice(&pts)
}

/// The concave equilateral hexagon ("bowtie") centered at the midpoint of
/// the 6-fold centers `a0`, `a1` (a 2-fold point of the [6,3] template):
/// S = dodecagon@a0 ∩ enneagon@Ct and S₂ = dodecagon@a1 ∩ enneagon@Ct
/// (the two shared 70° vertices), Q = the enneagon@Ct vertex on the ray
/// Ct→midpoint (reflex 220°), plus their mirrors across the line a0→a1.
/// Requires |a1 − a0| = √3 (a nearest 6-fold neighbor pair, so the
/// intermediate point Ct = a0 + rot(a1 − a0, +30°) is a 3-fold center at
/// distance 1 from both). All six edges have the template's common edge
/// length; returned CCW.
fn bowtie(a0: Vec2, a1: Vec2, r_a: f64, r_c: f64) -> Polygon {
    let d = a1 - a0;
    let u = d * (1.0 / d.length());
    let rot = |v: Vec2, ang: f64| {
        let (c, s) = (ang.cos(), ang.sin());
        Vec2::new(c * v.x - s * v.y, s * v.x + c * v.y)
    };
    let ct = a0 + rot(u, PI / 6.0);
    let s = a0 + rot(u, PI / 12.0) * r_a;
    let s2 = a1 + rot(u, 11.0 * PI / 12.0) * r_a;
    let bm = (a0 + a1) * 0.5;
    let w = bm - ct;
    let q = ct + w * (r_c / w.length());
    let refl = |p: Vec2| {
        let w = p - a0;
        a0 + u * (2.0 * u.dot(w)) - w
    };
    let mut pts = vec![s, q, s2, refl(s2), refl(q), refl(s)];
    if signed_area(&pts) < 0.0 {
        pts.reverse();
    }
    Polygon::from_slice(&pts)
}

/// Najm template tiling `([6,3]; 2e, 0, 3e; AC)`, wallpaper group p6m —
/// diss. §3.6.1 Fig. 3.21's substrate (dodecagons, enneagons, bowties).
///
/// Template: at A a dodecagon (n_A = 6·2), at C an enneagon (n_C = 3·3),
/// both oriented `e` (edge midpoint on the designated ray); m_B = 0. The
/// case-3 inflation on AC makes the two share their equal chords on the
/// hypotenuse: the common edge x solves ½x·(cot 15° + cot 20°) = |AC| = 1,
/// so r_A = x/(2 sin 15°) ≈ 0.59629392291900583 and
/// r_C = x/(2 sin 20°) ≈ 0.45123723482970227 (cot 15° = 2 + √3 exactly;
/// cot 20° is a cubic irrational, so this entry lives in floating point,
/// verified to 1e-55 at 60 working digits). Orientation parities then read
/// off automatically: A has edge midpoints on AB **and** AC, C has the
/// midpoint on CA and — because m_C = 3 is odd — a vertex on CB.
///
/// World scale (the figure's): |AC| = 1, 6-fold centers on the hexagonal
/// lattice Λ = span{a1 = (√3, 0), a2 = (√3/2, 3/2)}, |det| = 3√3/2. The
/// primitive cell holds 1 dodecagon (origin; vertices at 15° + 30°, edge
/// midpoints at 30° — every chord faces an enneagon at distance 1 or a
/// bowtie), 2 enneagons, and 3 bowties (the 2-fold points a1/2, a2/2,
/// (a2 − a1)/2).
///
/// The two enneagon classes carry **different absolute orientations** —
/// the orientation rule applies per triangle copy: at C0 = (√3/2, 1/2) the
/// partner A bears 210°, phase 190° (vertex on ray CB at 270°); at
/// C1 = (√3, 1) the partner is A = a1, bearing 270°, phase 250°. The
/// rot180 about the 2-fold point (√3/2, 0) maps the first tile set onto
/// the second translated by −a2 exactly (190° + 180° ≡ 250° mod 40°).
///
/// Census per cell: areas 3·r_A² + 2·(9/2)·r_C²·sin 40° + 3·A_bowtie =
/// 3√3/2 = |det| (exact to 1e-50); 12 vertices 150.140.70 (the S points:
/// dodecagon + enneagon + bowtie) and 6 vertices 140.220 (the Q points:
/// enneagon angle against a bowtie reflex). Bowties are equilateral with
/// angles {70°, 70°, 70°, 70°, 220°, 220°}.
///
/// Pattern seeds by polygon class: dodecagons 0, enneagons 1, bowties 2.
pub fn najm_63_2e_0_3e_ac() -> LatticeTiling {
    let spec = NajmSpec {
        p: 6,
        q: 3,
        mult: [2, 0, 3],
        orient: [Feature::Edge, Feature::Vertex, Feature::Edge],
        inflation: Inflation::Pair(Diad::Ac),
    };
    let rr = spec.radii();
    let (r_a, r_c) = (rr[0], rr[2]);
    let s3 = 3.0f64.sqrt();
    let a1 = Vec2::new(s3, 0.0);
    let a2 = Vec2::new(s3 / 2.0, 1.5);

    // Dodecagon at the origin: vertices 15° + 30k, so the edge midpoints
    // (30k) face the six enneagons at distance 1 (rays 30° + 60k) and the
    // six bowtie centers (rays 60k).
    let dodecagon = ngon(Vec2::ZERO, r_a, 12, PI / 12.0);
    // Enneagon at C0 = (√3/2, 1/2): partner A bears −150° (= 210°), edge
    // midpoint there → phase = −150° − 20°; a vertex then lies on ray CB.
    let c0 = Vec2::new(s3 / 2.0, 0.5);
    let c0a = Vec2::ZERO - c0;
    let enne0 = ngon(c0, r_c, 9, c0a.y.atan2(c0a.x) - PI / 9.0);
    // Enneagon at C1 = (√3, 1): partner A = a1 bears −90° (= 270°) →
    // phase = −90° − 20° = 250° (NOT a translate of enne0's orientation).
    let c1 = Vec2::new(s3, 1.0);
    let c1a = a1 - c1;
    let enne1 = ngon(c1, r_c, 9, c1a.y.atan2(c1a.x) - PI / 9.0);

    let mut cells = vec![
        Cell {
            poly: dodecagon,
            seed: 0,
        },
        Cell {
            poly: enne0,
            seed: 1,
        },
        Cell {
            poly: enne1,
            seed: 1,
        },
    ];
    for nb in [a1, a2, a2 - a1] {
        cells.push(Cell {
            poly: bowtie(Vec2::ZERO, nb, r_a, r_c),
            seed: 2,
        });
    }

    LatticeTiling {
        id: "najm-63-2e-0-3e-ac",
        name: "Najm template ([6,3]; 2e, 0, 3e; AC)",
        description: "Najm-template substrate (Kaplan diss. §3.6.1, Fig. 3.21): regular dodecagons and enneagons sharing equal chords, case-3 AC inflation x = 2/(cot 15° + cot 20°), plus concave equilateral bowties (70°/220°) at the 2-fold points; 1 dodecagon + 2 enneagons + 3 bowties per cell, vertex figures 150.140.70 ×12 and 140.220 ×6; wallpaper group p6m.",
        lattice: [a1, a2],
        cells,
        offset: Vec2::ZERO,
        group: WallpaperGroup::P6m,
        lattice_kind: LatticeKind::Hexagonal,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}

/// Najm template tiling `([4,4]; 3e, 0, 0; A)`, wallpaper group p4m —
/// diss. §3.6.1 Fig. 3.22's second-row substrate (dodecagons + 4-point
/// stars), exact in Q(√2, √3).
///
/// Template: [4,4] triangle with a dodecagon (n_A = 4·3) at A, oriented
/// `e`; m_B = m_C = 0. Case-2 inflation (the order list `[A]` alone):
/// inflate A until its chords touch the opposite leg, r_A = |AB|/cos 15°
/// (unit |AC|), then rescale to leg |AB| = 1: r_A = 1/cos 15° = √6 − √2,
/// side s = 2·r_A·sin 15° = 4 − 2√3.
///
/// Dodecagon centers form the square lattice 2Z² (basis (2, 0), (0, 2));
/// neighbors across the odd grid lines share their full chord through the
/// 2-fold point B = (1, 0) (vertices at ±15° have x = r_A cos 15° = 1
/// exactly). The interstice at (1, 1) is one concave equilateral
/// 4-pointed star: tips on the axes at radius √3 − 1 (each a shared
/// dodecagon vertex, 60° tip angle) and reflex notches at the diagonal
/// dodecagon vertices, offset 2 − √3 (210° interior). All eight edges
/// have s = 4 − 2√3.
///
/// Census per cell: 1 dodecagon (area 24 − 12√3) + 1 star (area
/// 12√3 − 20) — sum 4 = |det| exactly; 4 vertices 60.150.150 (tips:
/// star + two dodecagons) and 4 vertices 150.210 (notch wrapped around a
/// dodecagon corner).
///
/// Pattern seeds by polygon class: dodecagons 0, stars 1.
pub fn najm_44_3e_0_0_a() -> LatticeTiling {
    let spec = NajmSpec {
        p: 4,
        q: 4,
        mult: [3, 0, 0],
        orient: [Feature::Edge, Feature::Vertex, Feature::Vertex],
        inflation: Inflation::Order(&[Step::Inflate(Axis::A)]),
    };
    // Solve at |AC| = 1, then rescale so the leg |AB| = cos 45° becomes 1:
    // r_A = 1/cos 15° = √6 − √2.
    let r_a = spec.radii()[0] / (PI / 4.0).cos();

    // Dodecagon at the origin; its ±15° vertices lie exactly on x = 1.
    let dodecagon = ngon(Vec2::ZERO, r_a, 12, PI / 12.0);
    // Star at C = (1, 1): tips at radius td = 1 − r_A sin 15° = √3 − 1 on
    // the axes, notches at offset q = 1 − r_A cos 45° = 2 − √3 diagonally.
    let td = 1.0 - r_a * (PI / 12.0).sin();
    let nq = 1.0 - r_a * (PI / 4.0).cos();
    let c = Vec2::new(1.0, 1.0);
    let mut star_pts = vec![
        c + Vec2::new(0.0, -td),
        c + Vec2::new(nq, -nq),
        c + Vec2::new(td, 0.0),
        c + Vec2::new(nq, nq),
        c + Vec2::new(0.0, td),
        c + Vec2::new(-nq, nq),
        c + Vec2::new(-td, 0.0),
        c + Vec2::new(-nq, -nq),
    ];
    if signed_area(&star_pts) < 0.0 {
        star_pts.reverse();
    }

    LatticeTiling {
        id: "najm-44-3e-0-0-a",
        name: "Najm template ([4,4]; 3e, 0, 0; A)",
        description: "Najm-template substrate (Kaplan diss. §3.6.1, Fig. 3.22 second row): regular dodecagons r_A = √6 − √2 sharing chords across the odd grid lines of the square lattice, with concave equilateral 4-pointed stars (side 4 − 2√3, tips 60°, reflex notches 210°) in the interstices; exact in Q(√2, √3), areas 24 − 12√3 and 12√3 − 20 per cell; wallpaper group p4m.",
        lattice: [Vec2::new(2.0, 0.0), Vec2::new(0.0, 2.0)],
        cells: vec![
            Cell {
                poly: dodecagon,
                seed: 0,
            },
            Cell {
                poly: Polygon::from_slice(&star_pts),
                seed: 1,
            },
        ],
        offset: Vec2::ZERO,
        group: WallpaperGroup::P4m,
        lattice_kind: LatticeKind::Square,
        category: TilingCategory::Islamic,
        secondary: &[],
        seed_pattern: SeedPattern::Uniform,
        params: &SIMILARITY_PARAMS,
        make_map: similarity_map,
        color_mode: ColorMode::Pattern,
    }
}
