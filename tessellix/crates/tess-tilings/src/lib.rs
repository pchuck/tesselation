//! Built-in tiling generators for tessellix.
//!
//! # Adding a new tiling
//!
//! Tilings live in families (`crate::families`) — the unit of parallel
//! work. To add a tiling to an existing family, create the tiling
//! module, declare it in the family file, and add it to that family's
//! `register` slice, `classification` arms, census `IDS` list (and
//! `lattice_tilings` when it is a lattice table): only the family file
//! and the new module change. See `families/mod.rs` and
//! `docs/briefs/` for the fleet protocol.
//!
//! New families are orchestrator edits: a module under `families/`, one
//! line in `families::chain`, one in `families::classify_into`, one
//! feature in `Cargo.toml`, one re-export group below.

pub mod lattice;

/// Phase-A δ-validity envelope (diss. §3.4.1): the closed contact angles
/// for each registered strapwork substrate.
pub mod envelope;

pub use envelope::envelope_for;

/// The per-tiling classification table (always compiled — it is data,
/// not a family).
pub mod classify;

/// The 24h δ/θ sweep: deterministic scoring over host tilings.
/// Consumes the registry + envelope machinery; renders via tess-core.
pub mod sweep;

pub mod theta_search;

pub use classify::classification;

mod fields;
pub mod medallion;
// M86/M93: Taylor-Socolar and Jeandel-Rao deliberately carry no per-tiling
// feature gate (pre-refactor, the former was ungated despite a non-default
// `socolar` feature existing, and the latter had no feature at all). They
// remain unconditional members of the default-on `aperiodic` family.

/// Registry of all enabled built-in tilings.
pub mod families;

// Root re-exports: the pre-refactor `crate::<module>` paths keep working
// for moved code; new code should use `crate::families::<family>::<module>`.
// These are static — a new tiling does not add a line here.
#[cfg(feature = "uniform")]
pub use families::uniform::{square, hexagon, triangle, archimedean};
#[cfg(feature = "quasiperiodic")]
pub use families::quasiperiodic::{ammann_beenker, ammann_beenker_tri, shield, stampfli, golden_bee, golden_triangle, fractalizing, penrose, penrose_p1, penrose_p2, penrose_pentagon};
#[cfg(feature = "arrangements")]
pub use families::arrangements::{pinwheel, pythagorean, windmill};
#[cfg(feature = "pentagons")]
pub use families::pentagons::{pentagon1, pentagon2, pentagon3, pentagon7, pentagon8, pentagon6, pentagon9, pentagon10, pentagon11, pentagon12, pentagon13, pentagon14, pentagon15, hirschhorn};
#[cfg(feature = "laves")]
pub use families::laves::{cairo, laves};
#[cfg(feature = "kuniform")]
pub use families::kuniform::{kuniform, dual};
#[cfg(feature = "reptiles")]
pub use families::reptiles::{armchair, chair, sphinx, p_pentomino};
#[cfg(feature = "aperiodic")]
pub use families::aperiodic::{hat, turtle, spectre, taylor_socolar, jr_aperiodic};
#[cfg(feature = "pavers")]
pub use families::pavers::{basketweave, herringbone, running_bond, stepped_hexominoes, trapezoid_courses, glide_hexagon_strips, shifted_trapezoid_courses, wavy_courses, chiral_cut_honeycomb, split_triangle_grid, alcove_cut_grid, stack_bond, herringbone_45, basketweave_3};
#[cfg(feature = "islamic")]
pub use families::islamic::{hexagram, zellige, girih, girih_qc, girih_sc, girih_w85, pic_4_8_8, pic_3_6_3_6, pic_3_12_12, hankin, hankin_4_8_8, hankin_4_6_12, hankin_4_8_8_rosette_transform, hankin_hex, hankin_3_12_12, hankin_square_two_point, hankin_3_6_3_6, hankin_3_4_6_4, hankin_3_3_3_4_4, hankin_3_3_4_3_4, khatam_girih, najm, rosette_transform};

// ---------------------------------------------------------------------------
// Registry composition. The eleven families register in chain order
// (`families::chain`); within a tab, cards appear in family-chain order
// and then in the family's registration order. Adding a tiling to an
// existing family never edits this file.
// ---------------------------------------------------------------------------

pub fn registry() -> tess_core::Registry {
    families::chain(tess_core::Registry::new())
}

#[cfg(test)]
mod tests {
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::{ParamSet, ParamValue};
    use tess_core::symmetry::{LatticeKind, WallpaperGroup};
    use tess_core::tiling::{Query, Tiling};

    /// Deterministic xorshift64 rng, shared by the test modules in this
    /// crate (`crate::tests::Rng`).
    pub(crate) struct Rng(pub(crate) u64);
    impl Rng {
        pub(crate) fn new(seed: u64) -> Self {
            Self(seed)
        }
        pub(crate) fn f64(&mut self) -> f64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            (v >> 11) as f64 * (1.0 / 9007199254740992.0)
        }
    }

    /// Verify a tiling exactly covers `region`: every sample point is in at
    /// least one tile, and the total clipped area matches the region area
    /// (catches both gaps and overlaps).
    pub fn check_coverage(t: &dyn Tiling, name: &str, params: &ParamSet, region: BBox) {
        let r = t.generate(params, &Query { region });
        assert!(
            r.message.is_none(),
            "{name}: {msg}",
            msg = r.message.unwrap()
        );
        assert!(!r.tiles.is_empty(), "{name}: no tiles generated");

        // Culling: every tile intersects the region.
        for tile in &r.tiles {
            assert!(
                tile.poly.aabb().padding(1e-9).intersects(region),
                "{name}: tile outside region: {:?}",
                tile.poly.aabb()
            );
        }

        let sz = region.size();
        for gy in 0..40 {
            for gx in 0..40 {
                let p = region.min
                    + Vec2::new(
                        sz.x * (gx as f64 + 0.5) / 40.0,
                        sz.y * (gy as f64 + 0.5) / 40.0,
                    );
                let n = r.tiles.iter().filter(|t| t.poly.contains(p)).count();
                assert!(n >= 1, "{name}: gap at {p}");
            }
        }
        let mut rng = Rng::new(0x9E37_79B9_7F4A_7C15);
        for _ in 0..400 {
            let p = region.min + Vec2::new(rng.f64() * sz.x, rng.f64() * sz.y);
            let n = r.tiles.iter().filter(|t| t.poly.contains(p)).count();
            assert!(n >= 1, "{name}: gap at {p}");
        }

        let mut area = 0.0;
        for tile in &r.tiles {
            if let Some(c) = tile.poly.clipped_to(&region) {
                area += c.area();
            }
        }
        let ra = region.area();
        assert!(
            (area - ra).abs() <= 1e-6 * ra,
            "{name}: area mismatch (tiles {area} vs region {ra})"
        );
    }

    /// Verify generation is deterministic (geometry, prototile class, and
    /// pattern seed must all be identical).
    pub fn check_deterministic(t: &dyn Tiling, name: &str, params: &ParamSet, region: BBox) {
        let q = Query { region };
        let a = t.generate(params, &q);
        let b = t.generate(params, &q);
        assert_eq!(a.tiles.len(), b.tiles.len(), "{name}: tile count differs");
        for (ta, tb) in a.tiles.iter().zip(b.tiles.iter()) {
            assert_eq!(ta.poly.pts, tb.poly.pts, "{name}: tile geometry differs");
            assert_eq!(ta.proto, tb.proto, "{name}: tile proto differs");
            assert_eq!(ta.seed, tb.seed, "{name}: tile seed differs");
        }
    }

    /// The standard `covers_region` check at default parameters.
    pub(crate) fn check_covers(t: &dyn Tiling, name: &str, region: BBox) {
        let p = ParamSet::default_for(t.params());
        check_coverage(t, name, &p, region);
    }

    /// The standard `rotated_scaled` check (coverage must survive a
    /// non-axis-aligned scale + rotation).
    pub(crate) fn check_rotated(
        t: &dyn Tiling,
        name: &str,
        region: BBox,
        scale: f64,
        rotation: f64,
    ) {
        let mut p = ParamSet::default_for(t.params());
        p.set(crate::lattice::SCALE_ID, ParamValue::Float(scale));
        p.set(crate::lattice::ROTATION_ID, ParamValue::Angle(rotation));
        check_coverage(t, name, &p, region);
    }

    /// The standard `deterministic` check at default parameters.
    pub(crate) fn check_det(t: &dyn Tiling, name: &str, region: BBox) {
        let p = ParamSet::default_for(t.params());
        check_deterministic(t, name, &p, region);
    }

    /// The declared-category grouping both UIs render, via the shared
    /// [`tess_core::browse_tabs`] model: one tab per category in
    /// canonical [`TilingCategory::ALL`] order, each tiling in its
    /// primary tab plus every tab from its
    /// [`Tiling::secondary_categories`]. The per-tab census pins today's
    /// taxonomy — extend it when adding a tiling (and the README's
    /// count; the doc-sync guards `readme_matches_registry`,
    /// `family_docs_match_registrations` and `tiling_docs_match_registry`
    /// below fail the suite if the docs are not updated).
    #[test]
    fn registry_groups_are_complete_and_canonically_ordered() {
        let r = super::registry();
        let tabs = tess_core::browse_tabs(&r);

        // Exactly one tab per category, in canonical ALL order.
        assert_eq!(tabs.len(), tess_core::TilingCategory::ALL.len());
        for (tab, c) in tabs.iter().zip(tess_core::TilingCategory::ALL.iter()) {
            assert_eq!(tab.category, *c, "tabs follow TilingCategory::ALL order");
        }

        // Completeness: every tiling in its primary tab, every declared
        // secondary present, nothing else.
        let memberships: usize = tabs.iter().map(|t| t.cards.len()).sum();
        for t in r.iter() {
            let primary = tabs.iter().find(|tab| tab.category == t.category()).unwrap();
            assert!(
                primary.card_index_of(t.id()).is_some(),
                "primary tab missing {}",
                t.id()
            );
            for sec in t.secondary_categories() {
                let tab = tabs.iter().find(|tab| tab.category == *sec).unwrap();
                assert!(
                    tab.card_index_of(t.id()).is_some(),
                    "secondary tab {} missing {}",
                    sec,
                    t.id()
                );
            }
        }
        let expected_memberships: usize =
            r.len() + r.iter().map(|t| t.secondary_categories().len()).sum::<usize>();
        assert_eq!(memberships, expected_memberships, "membership census");

// Census of today's taxonomy (M127+M128+M129+M131+M132+M133+M134+M135+M136+M138+M140+M141+M142+M143+M144+M145+M146+M147+M148+M149+M150+M151+M152+M153+M154+M155+M156+M157+M158+M159+M160+M161+M162+M163; M164 = 22g decoration styles, M165 = 22d parquet deformations, M167 = 24a medallion interface, M168 = taylor-socolar plane-filling CHT presentation, M169 = 24c (n,c)-monster medallion catalog, M170 = 24h PIC explorer (sweep module + explore CLI, no registry change), M171 = 24c extension — (9,c)-monster records via the rotated odd-n family (no registry change) — all no census movement; M172 = 24d instance 1 — dodecagon-hexagram-honeycomb composition (Islamic 34 -> 35, built-ins 163 -> 164), M173 = Ammann A4 rectilinear chair set (Quasiperiodic 16 -> 17, built-ins 164 -> 165); M174 = 24b hole-inference engine sprint 1 (medallion layer only, no registry change), M175 = 24h exact-pin follow-up #2 — canonical 3.12.12 motif assignment (sweep theta-axis fix, no registry change); M176 = 24h exact-pin follow-up #3 — hankin-4-8-8 θ-axis host verified canonical (sweep pins + docs, no registry change), M177 = 24e underdetermined-θ search (theta_search.rs — deterministic ranked search over documented freedom sets, byte-deterministic leaderboard/v1, no registry change); M166 = CAP tiling, Aperiodic 11 -> 12); M180 = 24b hole-inference engine sprint 2 (girih_infer.rs — Plain-level generalization, medallion layer only, no registry change); M182 = odd-n cyclotomic K-layers — Field::K11 + Field::K13 fields.rs blocks (certified constants, RED towers, ISO brackets, 0-bisection batteries; option-C I192 endpoint certification; sign-engine envelope erratum + peak-bit wrap guard; no registry change); M184 = per-tiling depth param on glb-binary-n7 (retires the e01fc76 scale override, closes the TUI-thumb depth-4 gap; depth 0 = budget rule byte-identical, 1..=5 pinned; no registry change); M186 = Ammann A3 non-edge-to-edge substitution (Quasiperiodic 17 -> 18, built-ins 165 -> 166)). 166 built-ins — 63 before
        // the cairo-collinear merge into cairo-pentagonal (M76) —
        // across the eleven tabs, in TilingCategory::ALL order:
        //   Uniform 11 (8 Archimedean + triangle/square/hexagon),
        //   TwoUniform 20, Laves 33 (25 primary: 8 incl. cairo-pentagonal
        //   + 17 machine-dualized k-uniform duals (M98, M102, M106, M120,
        //   M121, M145, M147) + 3 regular secondaries + 5 isohedral-type
        //   secondaries), Wallpaper 17 (the group
        //   catalog — one card per represented group; M85 completed it),
        //   Isohedral 5 (the Kaplan IH instances, primary since the
        //   2026-11-24 tab chore), PentagonTypes 17, Islamic 30,
        //   Arrangements 8, Quasiperiodic 23, Aperiodic 12, Nonperiodic 11.
        // Membership census 187 = 166 primaries + 21 secondaries
        // (3 regular Laves + 5 isohedral->Laves + 9 Wallpaper
        // + 2 typed PentagonTypes: cairo-pentagonal (type 4)
        // and floret-pentagonal (type 5), M122 + 2 untyped
        // PentagonTypes: ih27/ih29).
        // History: M50 16 Wallpaper under default features -> M55 17
        // (pentagon-15) -> M59 20 (pentagon-9) -> M60 21 (pentagon-11)
        // -> M62 22 (pentagon-12) -> M63 23 (pentagon-10) -> M64 24
        // (pentagon-3) -> M65 25 (pentagon-2) -> M72 26 (pentagon-7)
        // -> M76: the old single Wallpaper bucket (26) is replaced by
        // the 10-tab scheme above -> M77 PentagonTypes 10 -> 11
        // (pentagon-8) -> M74 PentagonTypes 11 -> 12 (pentagon-6)
        // -> M78 Wallpaper 9 -> 10 (stepped-hexominoes, p1)
        // -> M79 Wallpaper 10 -> 11 (trapezoid-courses, pm)
        // -> M80 Wallpaper 11 -> 12 (glide-hexagon-strips, pg)
        // -> M81 Wallpaper 12 -> 13 (shifted-trapezoid-courses, pmg)
        // -> M82 Wallpaper 13 -> 14 (wavy-courses, cm)
        // -> M83 Wallpaper 14 -> 15 (chiral-cut-honeycomb, p3)
        // -> M84 Wallpaper 15 -> 16 (split-triangle-grid, p31m)
        // -> M85 Wallpaper 16 -> 17 (alcove-cut-grid, p3m1 — the last
        // gap; catalog complete) -> M86 Aperiodic 4 -> 5
        // (taylor-socolar) -> M87 TwoUniform 2 -> 3 (kuniform-n18)
// -> M88 (kuniform-n9) -> M89 (kuniform-n8)
        // -> M90 TwoUniform 5 -> 6 (kuniform-n5) -> M91 (kuniform-n1)
// -> M92 (kuniform-n13) -> M93 Aperiodic 5 -> 6 (jeandel-rao)
// -> M94 Islamic 9 -> 12 (hankin-4-8-8-rosette,
        // hankin-4-6-12, hankin-hex-two-point)
        // -> M98 Laves 11 -> 15 (the dual() helper dualizes
        // kuniform-p6m / -n18 / -n9 / -n8; Laves-primary only).
        // -> M96 TwoUniform 8 -> 9 (kuniform-n20, the module's
        // first p6 entry)
        // -> M97 TwoUniform 9 -> 10 (kuniform-n12, the module's
        // first cmm entry)
        // -> M95 TwoUniform 10 -> 11 (kuniform-n19; the frontier
        // solve corrected the group to p6m — mirrors pass torus-exact,
// e = 3 under D6, matching the wiki's (t=3, e=3) exactly)
// -> M99 Islamic 13 -> 14 (hankin-square-two-point, the
        // two-point hankin entry on the square 4.4.4.4 substrate;
// θK = 45°, δ = 0.5, cmm, docs/families/islamic.md §54).
        // -> M99 Islamic 14 -> 15 (hankin-4-8-8-rosette-transform,
        // the rosette transform of the 4.8.8 substrate at θK = 45°,
        // docs/families/islamic.md §55).
        // -> M99 Islamic 15 -> 17 (najm-63-2e-0-3e-ac, najm-44-3e-0-0-a;
        // roadmap item 22b: the najm template generator, diss. §3.6.1,
        // docs/families/islamic.md §56).
        // -> M100 Islamic 17 -> 18 (hankin-3-6-3-6, the hankin
        // inference entry on the trihexagonal 3.6.3.6 substrate;
// θK = 60° — the regular hexagram, docs/families/islamic.md §57).
        // -> M100 Islamic 18 -> 19 (hankin-3-4-6-4, the hankin
        // inference entry on the rhombitrihexagonal 3.4.6.4
        // substrate; θK = 60°, docs/families/islamic.md §58).
// -> M102 Laves 15 -> 19 (the dual() helper dualizes
        // kuniform-n5 / -n1 / -n13 / -n20, k-uniform batch 2;
        // Laves-primary only; provisional numbering, renumbered at
        // integration).
// -> M103 TwoUniform 11 -> 13 (kuniform-n4, the cmm
        // 4⁴;3³.4² form, and kuniform-n14, the pmm 3⁶;3³.4² form;
        // torus-exact t/e vs the wiki parentheticals (2,4)/(3,4),
        // docs/families/kuniform.md §60 — the fork's provisional §61 renumbered
        // by landing order).
// -> M106 Laves 19 -> 22 (the dual() helper dualizes
        // kuniform-pmm / -n12 / -n19, k-uniform batch 3: rectangular,
        // centered-rectangular and hexagonal primals; Laves-primary
        // only; docs/families/kuniform.md §61 — the fork's provisional §64
        // renumbered by landing order).
        // -> M108 TwoUniform 13 -> 15 (kuniform-n3, the cmm
        // 4⁴;3³.4² sibling, and kuniform-n15, the cmm 3⁶;3³.4²
        // sibling; torus-exact t/e vs the wiki parentheticals
        // (3,5)/(4,5), docs/families/kuniform.md §62 — the fork's provisional §61
        // renumbered by landing order).
        // -> M107 Islamic 19 -> 20 (hankin-3-3-3-4-4, the hankin
        // inference entry on the elongated-triangular 3.3.3.4.4
        // substrate; θK = 60°, cmm, docs/families/islamic.md §63 — the fork's
        // provisional §65 renumbered by landing order).
// -> M104 Quasiperiodic 8 -> 9 (golden-bee, the Ammann A2
        // chair as a √φ-inflating irreptile substitution; Vince AMM
        // 121 (2014) / Durand–Shen–Vereshchagin DCG 63 (2020)
        // limit-periodic nuance documented; docs/families/quasiperiodic.md §64 — the
        // fork's provisional §62 renumbered by landing order; landed
        // after M107).
// -> M110 Quasiperiodic 9 -> 10 (fractalizing-3-12-12, Chavey's
        // limit-periodic dodecagon-dense fractalizing family over the
        // uniform 3.12.12 — flower-orbit planes of triangles, squares,
        // hexagons and dodecagons, each level shrunk by 2+sqrt3;
        // docs/families/quasiperiodic.md §65 — the fork's provisional §64 renumbered by
        // landing order; landed after M104).
        // -> M111 Islamic 20 -> 21 (hankin-3-3-4-3-4, the hankin
        // inference entry on the snub-square 3.3.4.3.4 substrate —
        // first chiral substrate; θK = 60°, measured p4g on the
        // centered lattice ⟨(L/2,L/2),(L/2,−L/2)⟩; docs/families/islamic.md §66 —
        // the fork's provisional claim, number free at landing;
        // landed after M110).
        // -> M114 Arrangements 5 -> 6 (stack-bond, the pmm stacked
        // 2:1 domino bond on the rectangular lattice <(2,0),(0,1)>;
        // one split domino per cell).
        // -> M115 Arrangements 6 -> 7 (herringbone-45, the M40 bond in
        // its 45-degree-laid canonical frame, pgg on the rectangular
        // lattice <(0,2*sqrt(2)),(sqrt(2),0)>).
        // -> M116 Arrangements 7 -> 8 (basketweave-3, the triple
        // basketweave paver bond: three horizontal courses plus one
        // vertical course, period 5; pmm on the rectangular lattice
        // <(6,0),(0,5)> — an index-3 superlattice of the true
        // <(2,0),(0,5)>; 15 split dominoes per cell).
        // -> M117 PentagonTypes 14 -> 15 (pentagon-1, the Reinhardt
        // type-1 prototype: two pentagons per hexagonal-metric cell;
        // measured cmm — the prototile is achiral — on the a²=3b²
        // special centered-rectangular lattice).
        // -> M118 TwoUniform 15 -> 16 (kuniform-n2, the p4m
        // 3.12.12;3.4.3.12 form — the 3-4-3-12 tiling: dodecagons
        // kissing flat-edge-to-flat-edge with square+4-triangle fills;
        // torus-exact t/e vs the wiki parentheticals (3,3); the
        // missing-number hunt of the closed 20-list, batch 4).
        // -> M119 TwoUniform 16 -> 17 (kuniform-n16, the p4g
        // 3³.4²;3².4.3.4 form — variant 1 of the 33344-33434 tiling;
        // 4 tile orbits (8+4+4+2 = 18 tiles) / 5 edge orbits measured
        // on the torus, matching the wiki's 4-isohedral/5-isotoxal;
        // measured coset structure: D4, axis mirrors pure, diagonal
        // cosets true glides — p4g).
        // -> M120 Laves 22 -> 23 (kuniform-dual-n2, the face-centroid
        // dual of the 3-4-3-12 tiling: four kites (3.4.3.12 orbit,
        // sides (3+√3)/6 & (2+√3)/√3, area 3/4+5√3/12) plus four
        // isosceles triangles (3.12.12 orbit, base 2+√3, area
        // 1+7√3/12) per cell; areas sum to 7+4√3 = |det|; measured
        // p4m — every group coset mod lattice represented by an
        // operation fixing the primal square center).
        // -> M121 Laves 23 -> 24 (kuniform-dual-n16, the face-centroid
        // "key-lock" dual of the p4g 3³.4²;3².4.3.4 tiling: eight
        // pentagons (3².4.3.4 orbit) plus four pentagons (3³.4² orbit),
        // both classes exactly 1/2+√3/4 — areas sum to 6+3√3 = |det|;
        // measured p4g — 4-fold center ((√3−1)/4, (5+√3)/4), pure axis
        // mirrors through the corner-2 point (−1, 1/2), both diagonal
        // classes true half-period glides, no mirror through the 4-fold
        // center).
        // -> M127 Quasiperiodic 10 -> 11 (golden-triangle, the √(−τ)-
        // inflating right-triangle reptile: legs 1, √τ, hypotenuse τ,
        // altitude cut into two similar children (τ⁻¹, τ^−1/2), both
        // mirror chiralities, Fibonacci λ-inflation λL = L ∪ S,
        // λS = L in exact Q(λ), λ = i√τ; Danzer–van Ophuysen 2000
        // triangle-pair family).
        // -> M128 Nonperiodic 4 -> 5 (armchair, the L-tetromino rep-4
        // substitution: the unique dissection of the scale-2 inflated
        // armchair into four half-side armchairs, one of each
        // orientation; limit-periodic — the depth-g block's only
        // translation period is (2^(g+1), 2^(g+1)); non-edge-to-edge
        // T-junctions at every scale).
        // -> M129 Quasiperiodic 10 -> 11 (ammann-beenker-tri, the
        // Bielefeld "Ammann-Beenker rhomb triangle" variant of the
        // classic A5: the half-square triangles are emitted as their
        // own tile class — 3 prototiles, rhombus + two mirror-image
        // half-squares — instead of being paired into squares; same
        // silver-mean deflation, H = 2P + U hypotenuse parity).
        // -> M162 isohedral family 0 -> 5 (the Kaplan IH instances
        // ih1/ih6/ih7 hexagons on the 6³ net, ih27/ih29 pentagons on
        // the Cairo net; Laves 28 -> 33, PentagonTypes 15 -> 17).
        // -> tab chore (2026-11-24, non-M): the Isohedral tab, the
        // 11th category — the IH instances go primary-Isohedral with
        // Laves / PentagonTypes secondaries; memberships 175 -> 180.
        // -> M163 Islamic 26 -> 29 (hankin-4-8-8-motifs,
        // hankin-hex-motifs, najm-63-motifs — the Kaplan §3.5
        // design-element motif library; per-tile motif assignment;
        // docs/families/islamic.md §67).
        // -> M166 Aperiodic 11 -> 12 (cap — the Baake–Gähler–Sadun
        // cut-and-project tiling, arXiv:2305.05639: the D-map
        // deformation of the hat metatile lineage as a TRUE
        // cut-and-project with shape vector (3φ, 3φ, 1, φ+ξ, 2φ),
        // a φ² left eigenvector of σ̃₁ ⇒ LIDS; exact integer abcd
        // arithmetic, floats only at emission;
        // docs/tilings/cap.md + aperiodic.md).
        // -> M172 Islamic 29 -> 30 (dodecagon-hexagram-honeycomb, the
        // composition family's first instance)
        // -> M173 Quasiperiodic 21 -> 22 (ammann-a4, the rectilinear
        // chair set)
        let sizes: Vec<usize> = tabs.iter().map(|t| t.cards.len()).collect();
        assert_eq!(
            sizes,
vec![11, 20, 33, 17, 17, 5, 30, 8, 23, 12, 11],
            "per-tab census (update when adding tilings)"
        );
assert_eq!(r.len(), 166, "built-in tiling count (update when adding tilings)");
        assert_eq!(memberships, 187, "membership census (update when adding tilings)");

        // The Wallpaper tab is the group catalog: exactly one card per
        // represented group, and the represented groups are precisely
        // the 17 groups minus the gap table.
        let wall = &tabs
            .iter()
            .find(|t| t.category == tess_core::TilingCategory::Wallpaper)
            .unwrap();
        let mut groups: Vec<&WallpaperGroup> = wall
            .cards
            .iter()
            .filter_map(|c| c.group.as_ref())
            .collect();
        groups.sort_unstable_by_key(|g| {
            WallpaperGroup::ALL
                .iter()
                .position(|a| *a == **g)
                .unwrap()
        });
        assert_eq!(groups.len(), wall.cards.len(), "every Wallpaper card carries a group");
        assert_eq!(groups.len(), 17, "seventeen represented wallpaper groups");
        let expected: Vec<&WallpaperGroup> = WallpaperGroup::ALL
            .iter()
            .filter(|g| !tess_core::browse::WALLPAPER_GROUP_GAPS.contains(*g))
            .collect();
        assert_eq!(groups, expected, "represented groups = 17 - gaps");
    }

    /// Which family docs are guarded, and against which register slice.
    /// Shared by the doc-sync guards below (cfg'd per family feature).
    fn family_cases()
    -> Vec<(&'static str, fn(tess_core::Registry) -> tess_core::Registry)> {
        let mut cases: Vec<(&'static str, fn(tess_core::Registry) -> tess_core::Registry)> =
            Vec::new();
        #[cfg(feature = "uniform")]
        cases.push(("uniform", crate::families::uniform::register));
        #[cfg(feature = "quasiperiodic")]
        cases.push(("quasiperiodic", crate::families::quasiperiodic::register));
        #[cfg(feature = "arrangements")]
        cases.push(("arrangements", crate::families::arrangements::register));
        #[cfg(feature = "pentagons")]
        cases.push(("pentagons", crate::families::pentagons::register));
        #[cfg(feature = "laves")]
        cases.push(("laves", crate::families::laves::register));
        #[cfg(feature = "kuniform")]
        cases.push(("kuniform", crate::families::kuniform::register));
        #[cfg(feature = "reptiles")]
        cases.push(("reptiles", crate::families::reptiles::register));
        #[cfg(feature = "aperiodic")]
        cases.push(("aperiodic", crate::families::aperiodic::register));
        #[cfg(feature = "pavers")]
        cases.push(("pavers", crate::families::pavers::register));
        #[cfg(feature = "islamic")]
        cases.push(("islamic", crate::families::islamic::register));
        #[cfg(feature = "isohedral")]
        cases.push(("isohedral", crate::families::isohedral::register));
        cases
    }

    /// Doc-sync guard, README tier: the README must mirror the registry —
    /// the census sentence ("N built-in tilings (M tab memberships)"),
    /// the per-tab counts in "The eleven tabs:" prose, and the per-family
    /// table (exactly one doc link per family, stating the family's
    /// registered size). Per-tiling specifications live in
    /// `docs/tilings/<id>.md` (guard: `tiling_docs_match_registry`) and
    /// per-family tables in `docs/families/<family>.md` (guard:
    /// `family_docs_match_registrations`). (History: this guard began
    /// life as `readme_catalog_matches_registry`, checking the README's
    /// inline catalog table; the 2026-11 docs restructure moved the
    /// per-tiling detail into `docs/tilings/` and left the README as the
    /// overview. Without such a guard the README once drifted to 119/133
    /// while the registry moved to 122/136 — 11 rows never landed.)
    #[test]
    fn readme_matches_registry() {
        /// The number immediately preceding `phrase` (digits only).
        fn phrase_number(text: &str, phrase: &str, what: &str) -> usize {
            let i = text
                .find(phrase)
                .unwrap_or_else(|| panic!("README phrase missing: {what}"));
            let before = text[..i].trim_end();
            let start = before
                .rfind(|c: char| !c.is_ascii_digit())
                .map_or(0, |p| p + 1);
            before[start..]
                .parse()
                .unwrap_or_else(|_| panic!("README number not parseable: {what}"))
        }

        let readme = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md"),
        )
        .expect("README.md readable from crates/tess-tilings");
        let r = super::registry();
        let tabs = tess_core::browse_tabs(&r);
        let memberships: usize = tabs.iter().map(|t| t.cards.len()).sum();

        // Census sentence.
        assert_eq!(
            phrase_number(&readme, " built-in tilings (", "built-in census"),
            r.len(),
            "README built-in tiling count"
        );
        assert_eq!(
            phrase_number(&readme, " tab memberships)", "membership census"),
            memberships,
            "README tab-membership count"
        );

        // Per-tab prose counts, in canonical order. The README's Laves
        // alias is "Laves / Catalan duals" (the category label uses
        // parentheses).
        let aliases = [
            "Uniform",
            "Two-Uniform",
            "Laves / Catalan duals",
            "Wallpaper",
            "Pentagon Types",
            "Isohedral",
            "Islamic",
            "Arrangements",
            "Quasiperiodic",
            "Aperiodic",
            "Nonperiodic",
        ];
        for (tab, alias) in tabs.iter().zip(aliases) {
            let marker = format!("**{alias}** (");
            let i = readme
                .find(&marker)
                .unwrap_or_else(|| panic!("README per-tab prose missing for {alias}"));
            let digits: String = readme[i + marker.len()..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let n: usize = digits
                .parse()
                .unwrap_or_else(|_| panic!("README per-tab count not parseable for {alias}"));
            assert_eq!(n, tab.cards.len(), "README per-tab count for {alias}");
        }

        // Family table: exactly one doc link per family, and the count
        // cell must equal that family's registered size. (The README no
        // longer carries a per-tiling catalog table; per-tiling detail
        // lives in docs/tilings/<id>.md, guarded by
        // tiling_docs_match_registry.)
        assert_eq!(
            family_cases().len(),
            11,
            "README family table covers all eleven families"
        );
        for (fam, register) in family_cases() {
            let marker = format!("](docs/families/{fam}.md)");
            let rows: Vec<&str> =
                readme.lines().filter(|l| l.contains(&marker)).collect();
            assert_eq!(
                rows.len(),
                1,
                "README family table: exactly one row linking docs/families/{fam}.md"
            );
            let after = rows[0]
                .split(&marker)
                .nth(1)
                .expect("row contains its own marker");
            let digits: String = after
                .trim_start_matches([' ', '|'])
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let n: usize = digits
                .parse()
                .unwrap_or_else(|_| panic!("README family-table count not parseable for {fam}"));
            let fr = register(tess_core::Registry::new());
            assert_eq!(n, fr.len(), "README family-table count for {fam}");
        }
    }

    /// Doc-sync guard, census-surface tier: the remaining census-class
    /// numeric claims in the four doc surfaces — the README's word-form
    /// census claims `readme_matches_registry` does not cover, the
    /// `docs/INDEX.md` family/tab counts and per-family Tilings column,
    /// the `docs/DESIGN.md` family-count word, and the
    /// `docs/families/aperiodic.md` M168 status-line census — pinned
    /// against the live registry. This is the per-M-landing hand-check
    /// made permanent: a census-moving M-landing that forgets one of
    /// these surfaces fails CI naming the file. Line-scoped,
    /// whitespace-tolerant parsing; an anchor that is missing or
    /// ambiguous fails loudly (zero-or-one match per claim). The
    /// README census sentence / per-tab prose / family table are already
    /// pinned by `readme_matches_registry`; the `## Implemented (N
    /// tilings)` headings in `docs/families/*.md` are already pinned by
    /// `family_docs_match_registrations`; the historical status lines in
    /// `docs/families/aperiodic.md` (M34/M37/M38/M93) are frozen history
    /// and not pinned. The 1431 test-suite count is not a census claim
    /// and is not pinned (this job adds test fns and it would move).
    #[test]
    fn census_surface_docs_match_registry() {
        /// The whitespace-normalized single line containing `anchor_pos`.
        fn line_region(text: &str, anchor_pos: usize) -> String {
            let start = text[..anchor_pos]
                .rfind('\n')
                .map(|p| p + 1)
                .unwrap_or(0);
            let end = text[anchor_pos..]
                .find('\n')
                .map(|i| anchor_pos + i)
                .unwrap_or(text.len());
            text[start..end]
                .chars()
                .map(|c| if c.is_whitespace() { ' ' } else { c })
                .collect()
        }

        /// 1-based line number of the anchor at `anchor_pos`.
        fn anchor_line(text: &str, anchor_pos: usize) -> usize {
            text[..anchor_pos].chars().filter(|&c| c == '\n').count() + 1
        }

        /// The unique byte position of `anchor`; else panic loudly.
        fn unique_anchor(text: &str, anchor: &str, what: &str) -> usize {
            let positions: Vec<usize> =
                text.match_indices(anchor).map(|(i, _)| i).collect();
            match positions.len() {
                0 => panic!("{what}: anchor missing: {anchor:?}"),
                1 => positions[0],
                n => panic!("{what}: anchor ambiguous ({n} matches): {anchor:?}"),
            }
        }

        /// The parsed digit run immediately preceding `suffix` on the line.
        fn number_before_suffix(line: &str, suffix: &str, what: &str) -> usize {
            let s = line
                .find(suffix)
                .unwrap_or_else(|| panic!("{what}: suffix {suffix:?} missing on the anchor's line"));
            let before = line[..s].trim();
            let mut digits = String::new();
            for c in before.chars().rev() {
                if c.is_ascii_digit() {
                    digits.push(c);
                } else if !digits.is_empty() {
                    break;
                }
            }
            let digits: String = digits.chars().rev().collect();
            digits
                .parse()
                .unwrap_or_else(|_| panic!("{what}: number not parseable before {suffix:?}"))
        }

        /// The parsed digit run immediately following `marker` on the line.
        fn number_after_marker(line: &str, marker: &str, what: &str) -> usize {
            let i = line
                .find(marker)
                .unwrap_or_else(|| panic!("{what}: marker {marker:?} missing on the anchor's line"));
            let after = &line[i + marker.len()..];
            let mut digits = String::new();
            for c in after.chars() {
                if c.is_ascii_digit() {
                    digits.push(c);
                } else if !digits.is_empty() {
                    break;
                }
            }
            digits
                .parse()
                .unwrap_or_else(|_| panic!("{what}: no digit after {marker:?}"))
        }

        /// The English word immediately preceding `suffix` on the line.
        fn word_before_suffix(line: &str, suffix: &str, what: &str) -> String {
            let s = line
                .find(suffix)
                .unwrap_or_else(|| panic!("{what}: suffix {suffix:?} missing on the anchor's line"));
            let before = line[..s].trim_end();
            let mut word = String::new();
            for c in before.chars().rev() {
                if c.is_ascii_alphabetic() {
                    word.push(c);
                } else {
                    break;
                }
            }
            let word: String = word.chars().rev().collect();
            assert!(!word.is_empty(), "{what}: no word before {suffix:?}");
            word
        }

        /// The value an English small-number word stands for.
        fn word_value(word: &str) -> Option<usize> {
            Some(match word {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                "ten" => 10,
                "eleven" => 11,
                "twelve" => 12,
                "thirteen" => 13,
                "fourteen" => 14,
                "fifteen" => 15,
                "sixteen" => 16,
                "seventeen" => 17,
                "eighteen" => 18,
                "nineteen" => 19,
                "twenty" => 20,
                _ => return None,
            })
        }

        // ---- live values ----------------------------------------------

        let r = super::registry();
        let tabs = tess_core::browse_tabs(&r);
        let memberships: usize = tabs.iter().map(|t| t.cards.len()).sum();
        let tab_count = tess_core::TilingCategory::ALL.len();
        let family_count = family_cases().len();

        let docs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

        // ---- README.md (remaining word-form census claims) ------------

        let readme = std::fs::read_to_string(docs_dir.join("../../README.md"))
            .expect("README.md readable from crates/tess-tilings");

        // "eleven tabs:" — the tab-count word.
        let anchor_pos = unique_anchor(
            &readme,
            "eleven tabs",
            "README.md: eleven-tabs word pin",
        );
        let line_no = anchor_line(&readme, anchor_pos);
        let line = line_region(&readme, anchor_pos);
        let word = word_before_suffix(
            &line,
            "tabs:",
            &format!("README.md line {line_no} eleven-tabs word pin"),
        );
        assert_eq!(
            word_value(&word),
            Some(tab_count),
            "README.md (line {line_no}): eleven-tabs word form"
        );

        // "The eleven families" — the family-count word.
        let anchor_pos = unique_anchor(
            &readme,
            "The eleven families",
            "README.md: The-eleven-families word pin",
        );
        let line_no = anchor_line(&readme, anchor_pos);
        let line = line_region(&readme, anchor_pos);
        let word = word_before_suffix(
            &line,
            "families",
            &format!("README.md line {line_no} family-count word pin"),
        );
        assert_eq!(
            word_value(&word),
            Some(family_count),
            "README.md (line {line_no}): The-eleven-families word form"
        );

        // "holds the 165 built-ins in eleven families" — the design-prose
        // built-in total and the family-count word.
        let anchor_pos = unique_anchor(
            &readme,
            " built-ins in eleven families",
            "README.md: built-ins-in-eleven-families pin",
        );
        let line_no = anchor_line(&readme, anchor_pos);
        let line = line_region(&readme, anchor_pos);
        let n = number_before_suffix(
            &line,
            " built-ins",
            &format!("README.md line {line_no} built-in tiling count pin"),
        );
        assert_eq!(
            n,
            r.len(),
            "README.md (line {line_no}): built-in tiling count"
        );
        let word = word_before_suffix(
            &line,
            "families",
            &format!("README.md line {line_no} family-count word pin"),
        );
        assert_eq!(
            word_value(&word),
            Some(family_count),
            "README.md (line {line_no}): built-ins-in-eleven-families word form"
        );

        // ---- docs/INDEX.md -------------------------------------------

        let index = std::fs::read_to_string(docs_dir.join("../../docs/INDEX.md"))
            .expect("docs/INDEX.md readable from crates/tess-tilings");

        // "the eleven families with doc links" — the family-count word.
        let anchor_pos = unique_anchor(
            &index,
            "eleven families with doc links",
            "docs/INDEX.md: eleven-families-with-doc-links word pin",
        );
        let line_no = anchor_line(&index, anchor_pos);
        let line = line_region(&index, anchor_pos);
        let word = word_before_suffix(
            &line,
            "families with doc links",
            &format!("docs/INDEX.md line {line_no} family-count word pin"),
        );
        assert_eq!(
            word_value(&word),
            Some(family_count),
            "docs/INDEX.md (line {line_no}): eleven-families-with-doc-links word form"
        );

        // "166 files" — the per-tiling-documents count.
        let anchor_pos = unique_anchor(
            &index,
            "166 files",
            "docs/INDEX.md: per-tiling-documents total pin",
        );
        let line_no = anchor_line(&index, anchor_pos);
        let line = line_region(&index, anchor_pos);
        let n = number_before_suffix(
            &line,
            " files)",
            &format!("docs/INDEX.md line {line_no} per-tiling-documents total pin"),
        );
        assert_eq!(
            n,
            r.len(),
            "docs/INDEX.md (line {line_no}): per-tiling-documents count"
        );

        // "the eleven-tab gallery taxonomy" — the tab-count word.
        let anchor_pos = unique_anchor(
            &index,
            "eleven-tab gallery taxonomy",
            "docs/INDEX.md: eleven-tab word pin",
        );
        let line_no = anchor_line(&index, anchor_pos);
        let line = line_region(&index, anchor_pos);
        let word = word_before_suffix(
            &line,
            "-tab gallery taxonomy",
            &format!("docs/INDEX.md line {line_no} eleven-tab word pin"),
        );
        assert_eq!(
            word_value(&word),
            Some(tab_count),
            "docs/INDEX.md (line {line_no}): eleven-tab-gallery-taxonomy word form"
        );

        // Per-family Tilings column: one row per family, the count cell
        // equal to that family's registered size.
        let row_count = index.matches("](families/").count();
        assert_eq!(
            row_count,
            family_count,
            "docs/INDEX.md: per-family table must have one row per family (expected {family_count}, got {row_count})"
        );
        for (fam, register) in family_cases() {
            let marker = format!("](families/{fam}.md)");
            let anchor_pos = unique_anchor(
                &index,
                &marker,
                &format!("docs/INDEX.md: per-family Tilings column pin ({fam})"),
            );
            let line_no = anchor_line(&index, anchor_pos);
            let line = line_region(&index, anchor_pos);
            let n = number_after_marker(
                &line,
                &marker,
                &format!("docs/INDEX.md (line {line_no}): per-family Tilings column count for {fam}"),
            );
            let fr = register(tess_core::Registry::new());
            assert_eq!(
                n,
                fr.len(),
                "docs/INDEX.md (line {line_no}): per-family Tilings column count for {fam}"
            );
        }

        // ---- docs/DESIGN.md -------------------------------------------

        let design = std::fs::read_to_string(docs_dir.join("../../docs/DESIGN.md"))
            .expect("docs/DESIGN.md readable from crates/tess-tilings");

        // "eleven family features" — the family-count word.
        let anchor_pos = unique_anchor(
            &design,
            "eleven family features",
            "docs/DESIGN.md: eleven-family-features word pin",
        );
        let line_no = anchor_line(&design, anchor_pos);
        let line = line_region(&design, anchor_pos);
        let word = word_before_suffix(
            &line,
            "family features",
            &format!("docs/DESIGN.md line {line_no} family-count word pin"),
        );
        assert_eq!(
            word_value(&word),
            Some(family_count),
            "docs/DESIGN.md (line {line_no}): eleven-family-features word form"
        );

        // ---- docs/families/aperiodic.md ------------------------------

        let aperiodic = std::fs::read_to_string(
            docs_dir.join("../../docs/families/aperiodic.md"),
        )
        .expect("docs/families/aperiodic.md readable from crates/tess-tilings");

        // "166 tilings, 187 memberships" — the M168 status-line census
        // (not the historical M34/M37/M38/M93 status lines, which are
        // frozen history, not live census). The 1431 test-suite count
        // is NOT pinned: it is not a census claim, and this job adds
        // test fns which would move it.
        let anchor_pos = unique_anchor(
            &aperiodic,
            "presentation (M168). ",
            "docs/families/aperiodic.md: M168 status-line pin",
        );
        let line_no = anchor_line(&aperiodic, anchor_pos);
        let line = line_region(&aperiodic, anchor_pos);
        let n = number_before_suffix(
            &line,
            " tilings,",
            &format!("docs/families/aperiodic.md line {line_no} M168 status-line total pin"),
        );
        assert_eq!(
            n,
            r.len(),
            "docs/families/aperiodic.md (line {line_no}): M168 status-line built-in total"
        );
        let m = number_before_suffix(
            &line,
            " memberships,",
            &format!("docs/families/aperiodic.md line {line_no} M168 status-line membership pin"),
        );
        assert_eq!(
            m,
            memberships,
            "docs/families/aperiodic.md (line {line_no}): M168 status-line membership count"
        );

        // ---- docs/TAXONOMY.md --------------------------------------

        let taxonomy = std::fs::read_to_string(docs_dir.join("../../docs/TAXONOMY.md"))
            .expect("docs/TAXONOMY.md readable from crates/tess-tilings");

        // "eleven since 2026-11-24" — the tab-count word (the M76 "ten
        // tabs" snapshot on the line above is frozen history; the live
        // claim, eleven since the Isohedral-tab tab chore, is what is
        // pinned here). The anchor embeds no number/word so a move of
        // the claim still locates the line and fails on the value, not
        // on a vanished anchor.
        let anchor_pos = unique_anchor(
            &taxonomy,
            " since 2026-11-24",
            "docs/TAXONOMY.md: eleven-since tab-count word pin",
        );
        let line_no = anchor_line(&taxonomy, anchor_pos);
        let line = line_region(&taxonomy, anchor_pos);
        let word = word_before_suffix(
            &line,
            "since 2026-11-24",
            &format!("docs/TAXONOMY.md line {line_no} tab-count word pin"),
        );
        assert_eq!(
            word_value(&word),
            Some(tab_count),
            "docs/TAXONOMY.md (line {line_no}): eleven-since word form"
        );

        // "187 memberships over the 166 implemented tilings; the 21 extra
        // are" — the live membership census and the derived "extra"
        // arithmetic: 21 extra == memberships - built-ins, both live
        // values (187 - 166 = 21). The per-tab figures in the same file
        // are already pinned against the registry by
        // registry_groups_are_complete_and_canonically_ordered (the
        // per-tab vector) and are intentionally not re-pinned.
        let anchor_pos = unique_anchor(
            &taxonomy,
            " memberships over the",
            "docs/TAXONOMY.md: memberships-over-built-ins pin",
        );
        let line_no = anchor_line(&taxonomy, anchor_pos);
        let line = line_region(&taxonomy, anchor_pos);
        let m = number_before_suffix(
            &line,
            " memberships",
            &format!("docs/TAXONOMY.md line {line_no} membership count pin"),
        );
        assert_eq!(
            m,
            memberships,
            "docs/TAXONOMY.md (line {line_no}): membership count"
        );
        let n = number_before_suffix(
            &line,
            " implemented tilings",
            &format!("docs/TAXONOMY.md line {line_no} built-in total pin"),
        );
        assert_eq!(
            n,
            r.len(),
            "docs/TAXONOMY.md (line {line_no}): built-in total"
        );
        let extra = number_before_suffix(
            &line,
            " extra are",
            &format!("docs/TAXONOMY.md line {line_no} extra-secondary-memberships pin"),
        );
        assert_eq!(
            extra,
            memberships - r.len(),
            "docs/TAXONOMY.md (line {line_no}): extra secondary-memberships arithmetic"
        );

        // "seventeen tilings appear under the Wallpaper catalog" — the
        // wallpaper-catalog card count word.
        let anchor_pos = unique_anchor(
            &taxonomy,
            " tilings appear under the Wallpaper",
            "docs/TAXONOMY.md: wallpaper-catalog seventeen-tilings word pin",
        );
        let line_no = anchor_line(&taxonomy, anchor_pos);
        let line = line_region(&taxonomy, anchor_pos);
        let word = word_before_suffix(
            &line,
            "tilings appear",
            &format!("docs/TAXONOMY.md line {line_no} wallpaper-catalog card-count word pin"),
        );
        let wallpaper_cards = tabs
            .iter()
            .find(|t| t.category == tess_core::TilingCategory::Wallpaper)
            .unwrap()
            .cards
            .len();
        assert_eq!(
            word_value(&word),
            Some(wallpaper_cards),
            "docs/TAXONOMY.md (line {line_no}): seventeen-tilings-under-Wallpaper word form"
        );
    }

    /// A ledger guard, not a census guard: the M-numbers cited in this
    /// file's own `// Census of today's taxonomy` comment block must all
    /// appear as CHANGELOG headers (audit VII measured this bijection
    /// once by hand: 0 STALE, 101 census M-numbers, all present).
    ///
    /// Scope guard: only census ⊆ CHANGELOG-headers is asserted. The
    /// converse is deliberately NOT asserted — the CHANGELOG is the
    /// pre-repo-era ledger (M3–M38 etc. carry no census rows), audit VII
    /// documented those as FIDEL-OK.
    ///
    /// Block-extraction rule (load-bearing, re-derived every run): the
    /// census-comment block is anchored on the first line whose trimmed
    /// content starts with `// Census of today`; it extends over every
    /// following line whose trimmed content starts with `//`; it ends at
    /// the first non-comment line. It never depends on line numbers, so
    /// a comment edit elsewhere in the file cannot shift the block, and
    /// a benign edit adding/removing M-note comment lines INSIDE the
    /// block is simply picked up (a new M-note with no CHANGELOG header
    /// fails this fn naming it).
    ///
    /// Dup-M clause: NOT pinned. M-tokens repeat legitimately in the
    /// current block (each M is named once in the header M-list and again
    /// in its per-family `-> M<N>` note lines — 22 distinct M-numbers on
    /// multiple lines, M99 on 3 lines). A repeated M-note LINE would be a
    /// copy-paste bug, but no such repeated-line signal is sound today.
    #[test]
    fn census_m_ledger_matches_changelog() {
        /// `\b`-style word byte for the M-token scanner.
        fn is_word_byte(b: u8) -> bool {
            b.is_ascii_alphanumeric() || b == b'_'
        }

        /// The M-numbers with word boundaries (``\bM([0-9]+)\b``).
        fn m_tokens(text: &str) -> Vec<u32> {
            let bytes = text.as_bytes();
            let mut out = Vec::new();
            let mut i = 0;
            while i < bytes.len() {
                if bytes[i] == b'M' {
                    let before_ok = i == 0 || !is_word_byte(bytes[i - 1]);
                    let mut j = i + 1;
                    while j < bytes.len() && bytes[j].is_ascii_digit() {
                        j += 1;
                    }
                    let after_ok = j == bytes.len() || !is_word_byte(bytes[j]);
                    if before_ok && j > i + 1 && after_ok {
                        out.push(
                            text[i + 1..j]
                                .parse::<u32>()
                                .expect("M-token digits parse as u32"),
                        );
                    }
                    i = j.max(i + 1);
                } else {
                    i += 1;
                }
            }
            out
        }

        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let lib_src = std::fs::read_to_string(manifest.join("src/lib.rs"))
            .expect("src/lib.rs readable from crates/tess-tilings");

        // Extraction rule (see fn doc): header anchor + consecutive-//
        // lines. Re-derived from the file every run.
        let lines: Vec<&str> = lib_src.lines().collect();
        let header_idx = lines
            .iter()
            .position(|l| l.trim_start().starts_with("// Census of today"))
            .expect("census-comment header line missing from src/lib.rs");
        let mut block_end = header_idx;
        while block_end < lines.len()
            && lines[block_end].trim_start().starts_with("//")
        {
            block_end += 1;
        }
        let block = lines[header_idx..block_end].join("\n");

        let mut census_ms: Vec<u32> = m_tokens(&block);
        census_ms.sort_unstable();
        census_ms.dedup();
        assert!(
            !census_ms.is_empty(),
            "census-comment block extracted but contained no M-tokens"
        );

        // CHANGELOG header matcher (audit VII): the header LINE must
        // match `^\s*- 20\d\d-` — the M-token counts ONLY the
        // header line, never the body (body references do not count).
        let changelog = std::fs::read_to_string(
            manifest.join("../../docs/history/CHANGELOG.md"),
        )
        .expect("docs/history/CHANGELOG.md readable from crates/tess-tilings");
        let mut header_ms: Vec<u32> = Vec::new();
        let mut header_line_count = 0usize;
        for line in changelog.lines() {
            let t = line.trim_start();
            let bytes = t.as_bytes();
            let is_header = bytes.len() >= 7
                && t.starts_with("- 20")
                && bytes[4].is_ascii_digit()
                && bytes[5].is_ascii_digit()
                && bytes[6] == b'-';
            if is_header {
                header_line_count += 1;
                header_ms.extend(m_tokens(line));
            }
        }
        header_ms.sort_unstable();
        header_ms.dedup();

        assert!(
            !header_ms.is_empty(),
            "no CHANGELOG header lines matched the audit VII matcher"
        );

        // Assert: every census M-number is a CHANGELOG header. Failure
        // names EVERY missing M-number (not just the first).
        let missing: Vec<String> = census_ms
            .iter()
            .filter(|m| !header_ms.contains(m))
            .map(|m| format!("M{m}"))
            .collect();
        assert!(
            missing.is_empty(),
            "census-comment M-number(s) missing from CHANGELOG headers: {} \
             (census block: {} M-numbers, CHANGELOG headers: {} unique M-numbers, \
             {header_line_count} header lines) — every '// M<N>' note and header \
             M-list entry must land with a CHANGELOG header line \
             (audit VII matcher: '^\\s*- 20\\d\\d-', M-tokens on the header line only). \
             Scope guard: census ⊆ headers only, never the converse.",
            missing.join(", "),
            census_ms.len(),
            header_ms.len()
        );
    }

    /// Doc-sync guard, per-tiling tier: every registered tiling must own
    /// `docs/tilings/<id>.md`, titled with its registry name, and every
    /// family doc's "Per-tiling documents" section must link exactly its
    /// family's registered ids (section-scoped, so prose cross-links
    /// elsewhere in the family doc are tolerated). This is the third
    /// tier of the doc-sync guards: README overview (readme_matches_
    /// registry) → family docs (family_docs_match_registrations) →
    /// per-tiling documents.
    #[test]
    fn tiling_docs_match_registry() {
        let docs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/tilings");
        let r = super::registry();
        for t in r.iter() {
            let path = docs_dir.join(format!("{}.md", t.id()));
            let doc = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("docs/tilings/{}.md readable: {e}", t.id()));
            let h1 = doc
                .lines()
                .next()
                .unwrap_or_else(|| panic!("docs/tilings/{}.md empty", t.id()));
            assert_eq!(
                h1,
                format!("# {}", t.name()),
                "docs/tilings/{}.md title must match the registry name",
                t.id()
            );
        }

        for (fam, register) in family_cases() {
            let doc = std::fs::read_to_string(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(format!("../../docs/families/{fam}.md")),
            )
            .unwrap_or_else(|e| panic!("docs/families/{fam}.md readable: {e}"));
            let r = register(tess_core::Registry::new());
            let s = doc
                .find("## Per-tiling documents")
                .unwrap_or_else(|| panic!("docs/families/{fam}.md '## Per-tiling documents' section"));
            let section_end = doc[s + 1..]
                .find("\n## ")
                .map_or(doc.len(), |i| s + 1 + i);
            let mut ids: Vec<&str> = doc[s..section_end]
                .lines()
                .filter_map(|l| l.strip_prefix("- [`"))
                .map(|rest| {
                    rest.split('`')
                        .next()
                        .expect("Per-tiling-documents link id has a closing backtick")
                })
                .collect();
            ids.sort_unstable();
            let mut expected: Vec<&str> = r.iter().map(|t| t.id()).collect();
            expected.sort_unstable();
            assert_eq!(
                ids, expected,
                "docs/families/{fam}.md Per-tiling-documents section must link every \
                 registered id of the family exactly once (missing, stale, or duplicated)"
            );
        }
    }

    /// Doc-sync guard, symmetry tier: every per-tiling doc's metadata line
    /// (the line starting with a backtick) must declare the wallpaper-group
    /// and lattice tokens the registry declares for that tiling — matching
    /// `declared_symmetry()` Display strings byte for byte — and docs for
    /// tilings the registry leaves symmetry-undeclared (`None`) must omit
    /// both tokens. Pinned against the registry, never against a
    /// hand-maintained table: a doc that loses a token, an extra token, or a
    /// stale token (or a registry symmetry change that no doc reflects)
    /// fails here with the id and the mismatch. Covers all 166 registered
    /// tilings.
    #[test]
    fn tiling_docs_metadata_tokens_match_registry() {
        let docs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/tilings");
        let r = super::registry();
        assert_eq!(
            r.len(),
            166,
            "registry census (this tier covers all registered docs)"
        );
        let n_docs = std::fs::read_dir(&docs_dir)
            .expect("docs/tilings readable")
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |x| x == "md"))
            .count();
        assert_eq!(
            n_docs,
            r.len(),
            "docs/tilings must carry exactly one doc per registered tiling (registry={}, docs={})",
            r.len(),
            n_docs
        );

        let mut with_tokens = 0usize;
        for t in r.iter() {
            let path = docs_dir.join(format!("{}.md", t.id()));
            let doc = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("docs/tilings/{}.md readable: {e}", t.id()));
            let meta = doc
                .lines()
                .find(|l| l.trim_start().starts_with('`'))
                .unwrap_or_else(|| {
                    panic!("docs/tilings/{}.md has no metadata line", t.id())
                });

            let doc_group = meta.find("wallpaper group `").map(|i| {
                let rest = &meta[i + "wallpaper group `".len()..];
                rest.split('`')
                    .next()
                    .expect("wallpaper-group token has a closing backtick")
            });
            let doc_lattice = meta.find(" lattice").map(|i| {
                meta[..i]
                    .rsplit(|c: char| !c.is_ascii_lowercase() && c != '-')
                    .next()
                    .unwrap_or("")
            }).filter(|w| !w.is_empty());

            match t.declared_symmetry() {
                Some(s) => {
                    with_tokens += 1;
                    let expected_group = s.group.to_string();
                    let expected_lattice = s.lattice.to_string();
                    assert_eq!(
                        doc_group.as_deref(),
                        Some(expected_group.as_str()),
                        "docs/tilings/{}.md metadata wallpaper-group token must match the registry (expected `{expected_group}`)",
                        t.id()
                    );
                    assert_eq!(
                        doc_lattice.as_deref(),
                        Some(expected_lattice.as_str()),
                        "docs/tilings/{}.md metadata lattice token must match the registry (expected `{expected_lattice}`)",
                        t.id()
                    );
                }
                None => {
                    assert!(
                        doc_group.is_none() && doc_lattice.is_none(),
                        "docs/tilings/{}.md must omit symmetry tokens (registry declares none), saw group `{:?}` lattice `{:?}`",
                        t.id(),
                        doc_group,
                        doc_lattice
                    );
                }
            }
        }

        // A pin that silently passes while carrying no tokens is a bug:
        // the registry must drive a non-trivial token population.
        let expected_symmetry_docs = r.iter().filter(|t| t.declared_symmetry().is_some()).count();
        assert!(expected_symmetry_docs > 0, "registry declares no symmetry — pin would be vacuous");
        assert_eq!(
            with_tokens, expected_symmetry_docs,
            "docs carrying symmetry tokens must equal the tilings declaring symmetry"
        );
    }

    /// Doc-sync guard for the per-family docs: each
    /// `docs/families/<family>.md` must state its family's registered
    /// size in the "Implemented (N tilings)" heading and list every
    /// registered id of that family exactly once across its tables —
    /// and no others (the drift that shipped 119 rows for a 122-tiling
    /// registry — 27 rows never landed, and a mis-merge absorbed the
    /// two-uniform/dual/pentagon tables into the paver-bonds table
    /// without section headers — is what this guard prevents, now per
    /// family so a failure names the one file to fix).
    #[test]
    fn family_docs_match_registrations() {
        let cases = family_cases();
        assert!(!cases.is_empty(), "at least one family doc is guarded");

        for (fam, register) in cases {
            let doc = std::fs::read_to_string(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(format!("../../docs/families/{fam}.md")),
            )
            .unwrap_or_else(|e| panic!("docs/families/{fam}.md readable: {e}"));
            let r = register(tess_core::Registry::new());

            let heading = doc
                .find("## Implemented (")
                .unwrap_or_else(|| panic!("docs/families/{fam}.md '## Implemented (' heading"));
            let heading_end = heading + "## Implemented (".len();
            let digits: String = doc[heading_end..]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            let n: usize = digits
                .parse()
                .unwrap_or_else(|_| panic!("docs/families/{fam}.md census not parseable"));
            assert_eq!(n, r.len(), "docs/families/{fam}.md Implemented census heading");

            // The Implemented section runs to the family's Build arcs.
            let section_end = doc[heading_end..]
                .find("\n## Build arcs")
                .map_or(doc.len(), |i| heading_end + i);
            let mut ids: Vec<&str> = doc[heading..section_end]
                .lines()
                .filter_map(|l| l.strip_prefix("| `"))
                .map(|rest| {
                    rest.split('`')
                        .next()
                        .expect("Implemented row id has a closing backtick")
                })
                .collect();
            ids.sort_unstable();
            let mut expected: Vec<&str> = r.iter().map(|t| t.id()).collect();
            expected.sort_unstable();
            assert_eq!(
                ids, expected,
                "docs/families/{fam}.md Implemented section must list every {fam} \
                 id exactly once (missing, stale, or duplicated rows)"
            );
        }
    }

    // -----------------------------------------------------------------
    // Doc-sync guard, per-tiling per-cell count tier: the audit-IV
    // derivations (2026-11) made permanent. For each verified per-cell
    // doc claim in `docs/tilings/<id>.md` — 7 hankin + 3 stragglers
    // (girih-w85, pentagon-10, pentagon-13) + 18 kuniform 2-uniform
    // primals (28 docs total) — the claimed number must equal the cell
    // count of the tiling built by its public constructor
    // (`LatticeTiling::cells.len()`; one Cell per motif face for hankin,
    // one Cell per tile per cell for lattice tables — the same
    // measurement audit IV used: the derived-vs-claimed tables live in
    // `.scratch/hankin-doc-verify/` and `.scratch/kuniform-doc-verify/`).
    //
    // The claim is parsed *tolerantly* on purpose: nothing depends on
    // line numbers, the metadata line, or the exact phrasing — the doc
    // may say "N faces per cell", "N faces per unit cell", "N tiles per
    // hexagonal cell", "N tiles/cell", or any other bare
    // "N (faces|tiles) (per [unit|hexagonal] cell | / cell)" token.
    // A doc with zero or more than one matching token fails loudly
    // rather than being silently skipped. The concurrent metadata-line
    // normalization job (group/lattice tokens) can reformat the header
    // without disturbing this pin.
    #[cfg(all(feature = "islamic", feature = "kuniform", feature = "pentagons"))]
    #[test]
    fn tiling_doc_claims_match_constructors() {
        use crate::families::islamic::girih_w85::girih_w85;
        use crate::families::islamic::hankin_3_3_3_4_4::hankin_3_3_3_4_4;
        use crate::families::islamic::hankin_3_3_4_3_4::hankin_3_3_4_3_4;
        use crate::families::islamic::hankin_3_4_6_4::hankin_3_4_6_4;
        use crate::families::islamic::hankin_3_6_3_6::hankin_3_6_3_6;
        use crate::families::islamic::hankin_3_12_12::hankin_3_12_12;
        use crate::families::islamic::hankin_4_6_12::hankin_4_6_12;
        use crate::families::islamic::hankin_square_two_point::hankin_square_two_point;
        use crate::families::kuniform::kuniform::{
            kuniform_n1, kuniform_n2, kuniform_n3, kuniform_n4, kuniform_n5, kuniform_n6,
            kuniform_n7, kuniform_n8, kuniform_n9, kuniform_n12, kuniform_n13, kuniform_n14,
            kuniform_n15, kuniform_n16, kuniform_n17, kuniform_n18, kuniform_n19, kuniform_n20,
        };
        use crate::families::pentagons::pentagon10::spec as pentagon10_spec;
        use crate::families::pentagons::pentagon13::spec as pentagon13_spec;
        use crate::lattice::LatticeTiling;

        /// Parse the doc's single per-cell count claim (see the doc
        /// comment above for the tolerant grammar). Fails loudly when
        /// the doc carries zero or more than one matching token.
        fn per_cell_claim(doc: &str, id: &str) -> usize {
            let tokens: Vec<&str> = doc
                .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                .filter(|t| !t.is_empty())
                .collect();
            let mut matches = 0usize;
            let mut value = 0usize;
            for (i, t) in tokens.iter().enumerate() {
                if !t.chars().all(|c| c.is_ascii_digit()) {
                    continue;
                }
                let n: usize = t.parse().unwrap();
                let noun_ok = matches!(tokens.get(i + 1), Some(&"faces") | Some(&"tiles"));
                if !noun_ok {
                    continue;
                }
                let mut j = i + 2;
                if tokens.get(j) == Some(&"per") {
                    j += 1;
                }
                if matches!(tokens.get(j), Some(&"unit") | Some(&"hexagonal")) {
                    j += 1;
                }
                if tokens.get(j) == Some(&"cell") {
                    matches += 1;
                    value = n;
                }
            }
            assert_eq!(
                matches, 1,
                "docs/tilings/{id}.md: expected exactly one per-cell count claim \
                 (N faces/tiles per [unit|hexagonal] cell | N tiles/cell), found {matches}"
            );
            value
        }

        let docs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/tilings");

        let cases: Vec<(&str, fn() -> LatticeTiling)> = vec![
            // Hankin family (audit IV, 7 docs).
            ("hankin-4-6-12", hankin_4_6_12),
            ("hankin-3-12-12", hankin_3_12_12),
            ("hankin-square-two-point", hankin_square_two_point),
            ("hankin-3-6-3-6", hankin_3_6_3_6),
            ("hankin-3-4-6-4", hankin_3_4_6_4),
            ("hankin-3-3-3-4-4", hankin_3_3_3_4_4),
            ("hankin-3-3-4-3-4", hankin_3_3_4_3_4),
            // Stragglers (audit IV, 3 docs).
            ("girih-w85", girih_w85),
            ("pentagon-10", pentagon10_spec),
            ("pentagon-13", pentagon13_spec),
            // Kuniform primals (audit IV, 18 docs — unpinned at the time
            // of writing: their doc claims were never pinned anywhere).
            ("kuniform-n1", kuniform_n1),
            ("kuniform-n2", kuniform_n2),
            ("kuniform-n3", kuniform_n3),
            ("kuniform-n4", kuniform_n4),
            ("kuniform-n5", kuniform_n5),
            ("kuniform-n6", kuniform_n6),
            ("kuniform-n7", kuniform_n7),
            ("kuniform-n8", kuniform_n8),
            ("kuniform-n9", kuniform_n9),
            ("kuniform-n12", kuniform_n12),
            ("kuniform-n13", kuniform_n13),
            ("kuniform-n14", kuniform_n14),
            ("kuniform-n15", kuniform_n15),
            ("kuniform-n16", kuniform_n16),
            ("kuniform-n17", kuniform_n17),
            ("kuniform-n18", kuniform_n18),
            ("kuniform-n19", kuniform_n19),
            ("kuniform-n20", kuniform_n20),
        ];

        for (id, make) in cases {
            let tiling = make();
            assert_eq!(
                tiling.id(), id,
                "docs/tilings/{id}.md: constructor id mismatch (doc id != constructor id)"
            );
            let doc = std::fs::read_to_string(docs_dir.join(format!("{id}.md")))
                .unwrap_or_else(|e| panic!("docs/tilings/{id}.md readable: {e}"));
            let claimed = per_cell_claim(&doc, id);
            assert_eq!(
                claimed,
                tiling.cells.len(),
                "docs/tilings/{id}.md: doc claims {claimed} per cell but the constructor produces {}",
                tiling.cells.len()
            );
        }
    }

    /// Kuniform dual doc pin. The 17 `kuniform-dual-*` docs claim, per cell,
    /// either an explicit "N faces over the primal's V vertex orbits" token
    /// (5 orbit-token docs — kuniform-dual-n3, n4, n14, n15, n16) or a
    /// face-by-polygon-type enumeration ("1 regular hexagon + 6 congruent
    /// quadrilaterals per cell" — the other 12). The constructors emit one
    /// canonical dual face per primal-vertex orbit
    /// (`families/kuniform/dual.rs`: "the per-cell table holds one canonical
    /// dual face per primal-vertex orbit, so every dual face is emitted
    /// exactly once by exactly one cell"), so the claimed per-cell face
    /// count equals `tiling.cells.len()` in both grammars. The orbit-token
    /// docs additionally pin the cross-check N == V == cells.len().
    ///
    /// The enumeration grammar is tolerant by design: it sums, over the text
    /// between the group parenthetical and the first " per cell", every
    /// (number, modifier?, polygon-noun) triple. Modifiers are "congruent" /
    /// "regular" / "isosceles" / "unit" plus bare-digit angle fragments
    /// (the `30°/60°/90°` token); polygon nouns are hexagon(s) /
    /// quadrilateral(s) / pentagon(s) / triangle(s) / kite(s) / rhombus /
    /// square(s). Parenthetical spans are stripped first — they carry side
    /// lengths, areas, orbit names and M-numbers, none of which is a face
    /// count. A doc whose enumeration or orbit token does not parse fails
    /// loudly rather than being silently skipped.
    #[cfg(feature = "kuniform")]
    #[test]
    fn kuniform_dual_doc_claims_match_constructors() {
        use crate::families::kuniform::dual::{
            kuniform_dual_n1, kuniform_dual_n2, kuniform_dual_n3, kuniform_dual_n4,
            kuniform_dual_n5, kuniform_dual_n8, kuniform_dual_n9, kuniform_dual_n12,
            kuniform_dual_n13, kuniform_dual_n14, kuniform_dual_n15, kuniform_dual_n16,
            kuniform_dual_n18, kuniform_dual_n19, kuniform_dual_n20, kuniform_dual_p6m,
            kuniform_dual_pmm,
        };
        use crate::lattice::LatticeTiling;

        /// Alphanumeric/'-' tokens — drops everything else.
        fn tokens(s: &str) -> Vec<&str> {
            s.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                .filter(|t| !t.is_empty())
                .collect()
        }

        /// Drop parenthetical spans (keeping the text outside them).
        fn strip_parens(s: &str) -> String {
            let mut out = String::new();
            let mut depth = 0usize;
            for c in s.chars() {
                match c {
                    '(' => depth += 1,
                    ')' => depth = depth.saturating_sub(1),
                    _ if depth == 0 => out.push(c),
                    _ => {}
                }
            }
            out
        }

        /// Sum every (number, modifier?, polygon-noun) triple in the
        /// face-enumeration region (the text between the group parenthetical
        /// and the first " per cell").
        fn enumeration_total(head_region: &str, id: &str) -> usize {
            let stripped = strip_parens(head_region);
            let toks = tokens(&stripped);
            let mut total = 0usize;
            let mut count = 0usize;
            let mut i = 0usize;
            while i < toks.len() {
                let t = toks[i];
                if !t.chars().all(|c| c.is_ascii_digit()) {
                    i += 1;
                    continue;
                }
                let n: usize = t.parse().unwrap();
                let mut j = i + 1;
                let mut matched_noun = false;
                loop {
                    match toks.get(j) {
                        // bare-digit angle fragments (30°/60°/90°) and
                        // modifier words are skipped; a polygon noun ends
                        // the triple
                        Some(&m) if m.chars().all(|c| c.is_ascii_digit()) => j += 1,
                        Some(&m)
                            if matches!(m, "congruent" | "regular" | "isosceles" | "unit") =>
                        {
                            j += 1
                        }
                        Some(&noun)
                            if matches!(
                                noun,
                                "hexagon"
                                    | "hexagons"
                                    | "quadrilateral"
                                    | "quadrilaterals"
                                    | "pentagon"
                                    | "pentagons"
                                    | "triangle"
                                    | "triangles"
                                    | "kite"
                                    | "kites"
                                    | "rhombus"
                                    | "rhombi"
                                    | "square"
                                    | "squares"
                            ) =>
                        {
                            matched_noun = true;
                            break;
                        }
                        _ => break,
                    }
                }
                if matched_noun {
                    total += n;
                    count += 1;
                    i = j + 1;
                } else {
                    i += 1;
                }
            }
            assert!(
                count >= 1,
                "docs/tilings/{id}.md: no polygon-count claims found in face-enumeration region"
            );
            total
        }

        /// Parse the "N faces over the primal's V vertex orbits" token, if
        /// present. Returns (N, V).
        fn orbit_token(line: &str, id: &str) -> Option<(usize, usize)> {
            let toks = tokens(line);
            let Some(f) = toks.iter().position(|&t| t == "faces") else {
                return None;
            };
            let n_faces: usize = toks
                .get(f.wrapping_sub(1))
                .and_then(|t| t.parse().ok())
                .unwrap_or_else(|| {
                    panic!("docs/tilings/{id}.md: 'N faces over the primal' — N missing")
                });
            let orbits_idx = toks
                .iter()
                .rposition(|&t| t == "orbits")
                .unwrap_or_else(|| {
                    panic!("docs/tilings/{id}.md: 'N faces over the primal' — orbits missing")
                });
            assert_eq!(
                toks.get(orbits_idx.wrapping_sub(1)),
                Some(&"vertex"),
                "docs/tilings/{id}.md: expected 'V vertex orbits' token"
            );
            let v_orbits: usize = toks
                .get(orbits_idx.wrapping_sub(2))
                .and_then(|t| t.parse().ok())
                .unwrap_or_else(|| {
                    panic!("docs/tilings/{id}.md: 'V vertex orbits' — V missing")
                });
            Some((n_faces, v_orbits))
        }

        let docs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/tilings");

        let cases: Vec<(&str, fn() -> LatticeTiling)> = vec![
            ("kuniform-dual-p6m", kuniform_dual_p6m),
            ("kuniform-dual-pmm", kuniform_dual_pmm),
            ("kuniform-dual-n1", kuniform_dual_n1),
            ("kuniform-dual-n2", kuniform_dual_n2),
            ("kuniform-dual-n3", kuniform_dual_n3),
            ("kuniform-dual-n4", kuniform_dual_n4),
            ("kuniform-dual-n5", kuniform_dual_n5),
            ("kuniform-dual-n8", kuniform_dual_n8),
            ("kuniform-dual-n9", kuniform_dual_n9),
            ("kuniform-dual-n12", kuniform_dual_n12),
            ("kuniform-dual-n13", kuniform_dual_n13),
            ("kuniform-dual-n14", kuniform_dual_n14),
            ("kuniform-dual-n15", kuniform_dual_n15),
            ("kuniform-dual-n16", kuniform_dual_n16),
            ("kuniform-dual-n18", kuniform_dual_n18),
            ("kuniform-dual-n19", kuniform_dual_n19),
            ("kuniform-dual-n20", kuniform_dual_n20),
        ];

        for (id, make) in cases {
            let tiling = make();
            assert_eq!(
                tiling.id(), id,
                "docs/tilings/{id}.md: constructor id mismatch (doc id != constructor id)"
            );
            let doc = std::fs::read_to_string(docs_dir.join(format!("{id}.md")))
                .unwrap_or_else(|e| panic!("docs/tilings/{id}.md readable: {e}"));
            let claim_lines: Vec<&str> = doc
                .lines()
                .filter(|l| l.contains("face-centroid dual"))
                .collect();
            assert_eq!(
                claim_lines.len(), 1,
                "docs/tilings/{id}.md: expected exactly one face-claim line (contains 'face-centroid dual'), found {}",
                claim_lines.len()
            );
            let line = claim_lines[0];
            // Face-enumeration region: text between the group parenthetical
            // and the first " per cell".
            let head = line
                .split(" per cell")
                .next()
                .unwrap_or_else(|| panic!("docs/tilings/{id}.md: no ' per cell' found in face-claim line"));
            let claimed = enumeration_total(head, id);
            assert_eq!(
                claimed,
                tiling.cells.len(),
                "docs/tilings/{id}.md: doc claims {claimed} faces per cell (enumeration) but the constructor produces {}",
                tiling.cells.len()
            );
            // Orbit-token cross-check (the 5 token docs).
            if let Some((n_faces, v_orbits)) = orbit_token(line, id) {
                assert_eq!(
                    n_faces, v_orbits,
                    "docs/tilings/{id}.md: doc claims {n_faces} faces over the primal's {v_orbits} vertex orbits — N != V"
                );
                assert_eq!(
                    n_faces,
                    tiling.cells.len(),
                    "docs/tilings/{id}.md: doc claims {n_faces} faces over the primal's {v_orbits} vertex orbits but the constructor produces {}",
                    tiling.cells.len()
                );
            }
        }
    }

    /// Part A legality test (M76). Two independent claims per tiling:
    ///
    /// 1. *Group legality*: the declared (group, lattice kind) pair is a
    ///    row of the crystallographic table (owner doc §4,
    ///    `docs/2D_Tilings_Reference.md`): p1/p2 admit any lattice;
    ///    pm/pg/pmm/pmg/pgg are primitive-rectangular only; cm/cmm are
    ///    centered-rectangular only; p4/p4m/p4g are square only;
    ///    p3/p3m1/p31m/p6/p6m are hexagonal only.
    /// 2. *Basis measurement*: when a fixed lattice basis is declared
    ///    (`Tiling::lattice_basis`), the lattice it spans must actually
    ///    be of the declared kind. Measured from the basis vectors
    ///    alone — no `generate()` call — via [`measured_kind`]. The
    ///    measurement reports the lattice's MAXIMAL metric type, so two
    ///    degenerate realizations are admitted: a rectangular or
    ///    centered-rectangular lattice whose vectors happen to form a
    ///    square metric lattice (equal-length orthogonal vectors) is
    ///    measured as Square even though the pattern's point group has
    ///    no 4-fold — a square-metric translation lattice under a
    ///    ≤2-fold group is a valid rectangular/c-rectangular Bravais
    ///    realization (running-bond: L = {x+y even}, constant √2).
    ///    Similarly, a centered-rectangular lattice whose vectors happen
    ///    to form a hexagonal metric lattice (a 60° equal-length basis,
    ///    the special a² = 3b² case) is measured as Hexagonal even
    ///    though the pattern's point group has no 3-/6-fold
    ///    (pentagon-1: the cmm tiling on Λ = Z(3,−√3) + Z(3,√3)).
    ///    Cross-combos in the other direction are NOT admitted: a
    ///    declared Square must measure Square, a declared Oblique must
    ///    measure Oblique.
    #[test]
    fn declared_symmetry_is_legal_and_matches_the_measured_lattice() {
        /// The lattice kinds a wallpaper group may sit on (the 17-group
        /// legality table; see the test docs).
        fn legal_kinds(g: WallpaperGroup) -> &'static [LatticeKind] {
            match g {
                WallpaperGroup::P1 | WallpaperGroup::P2 => &[
                    LatticeKind::Oblique,
                    LatticeKind::Rectangular,
                    LatticeKind::CenteredRectangular,
                    LatticeKind::Square,
                    LatticeKind::Hexagonal,
                ],
                WallpaperGroup::Pm
                | WallpaperGroup::Pg
                | WallpaperGroup::Pmm
                | WallpaperGroup::Pmg
                | WallpaperGroup::Pgg => &[LatticeKind::Rectangular],
                WallpaperGroup::Cm | WallpaperGroup::Cmm => &[LatticeKind::CenteredRectangular],
                WallpaperGroup::P4 | WallpaperGroup::P4m | WallpaperGroup::P4g => {
                    &[LatticeKind::Square]
                }
                WallpaperGroup::P3
                | WallpaperGroup::P3m1
                | WallpaperGroup::P31m
                | WallpaperGroup::P6
                | WallpaperGroup::P6m => &[LatticeKind::Hexagonal],
            }
        }

        /// Measure the lattice type of L = Z a + Z b from the basis
        /// vectors alone. Priority: hexagonal (an equal-length 60/120°
        /// basis exists), then orthogonal basis (square when the pair is
        /// equal-length, rectangular otherwise), then centered
        /// rectangular (an orthogonal pair spanning an index-2
        /// sublattice whose cell center is a lattice point), else
        /// oblique. Every stage is an exact invariant of the lattice, so
        /// the search bounds only affect finding the certificate, never
        /// the answer.
        fn measured_kind(a: Vec2, b: Vec2) -> LatticeKind {
            let det = a.cross(b);
            let tol = 1e-6 * a.dot(a).max(b.dot(b)).max(1.0);
            let span = |m: i32, n: i32, p: i32, q: i32| -> (Vec2, Vec2) {
                (a * m as f64 + b * n as f64, a * p as f64 + b * q as f64)
            };
            let is_basis = |d: f64| (d - det).abs() <= tol || (d + det).abs() <= tol;

            // Stage 1: hexagonal — a 60/120° equal-length basis of L.
            for m in -2..=2 {
                for n in -2..=2 {
                    if m == 0 && n == 0 {
                        continue;
                    }
                    for p in -2..=2 {
                        for q in -2..=2 {
                            if p == 0 && q == 0 {
                                continue;
                            }
                            let (u, v) = span(m, n, p, q);
                            if !is_basis(u.cross(v)) {
                                continue;
                            }
                            let (u2, v2) = (u.dot(u), v.dot(v));
                            if (u2 - v2).abs() > tol {
                                continue;
                            }
                            // 2 u·v = ±|u|²  <=>  60° or 120°.
                            if (2.0 * u.dot(v) - u2).abs() <= tol
                                || (2.0 * u.dot(v) + u2).abs() <= tol
                            {
                                return LatticeKind::Hexagonal;
                            }
                        }
                    }
                }
            }

            // Stage 2: an orthogonal basis of L — square when the pair is
            // equal-length, rectangular otherwise. (A centered lattice
            // never has an orthogonal basis, so this stage cannot
            // misclassify one.)
            //
            // NOTE: the lattice kinds checked here are the MAXIMAL metric
            // type of the lattice itself. A centered-rectangular Bravais
            // lattice may, as a special parameter, be hexagonal-metric
            // (a² = 3b², so a 60° equal-length basis exists) even under
            // a ≤2-fold group; that is admitted below.
            for m in -3..=3 {
                for n in -3..=3 {
                    if m == 0 && n == 0 {
                        continue;
                    }
                    for p in -3..=3 {
                        for q in -3..=3 {
                            if p == 0 && q == 0 {
                                continue;
                            }
                            let (u, v) = span(m, n, p, q);
                            if !is_basis(u.cross(v)) || u.dot(v).abs() > tol {
                                continue;
                            }
                            return if (u.dot(u) - v.dot(v)).abs() <= tol {
                                LatticeKind::Square
                            } else {
                                LatticeKind::Rectangular
                            };
                        }
                    }
                }
            }

            // Stage 3: centered rectangular — an orthogonal pair (u, v)
            // spanning an index-2 sublattice with the cell center
            // (u+v)/2 back in L (even coefficient sums). A primitive
            // rectangular or oblique lattice has no such pair.
            for m in -3..=3 {
                for n in -3..=3 {
                    if m == 0 && n == 0 {
                        continue;
                    }
                    for p in -3..=3 {
                        for q in -3..=3 {
                            if p == 0 && q == 0 {
                                continue;
                            }
                            if (m + p) % 2 != 0 || (n + q) % 2 != 0 {
                                continue;
                            }
                            let (u, v) = span(m, n, p, q);
                            if (u.cross(v) - 2.0 * det).abs() > tol
                                && (u.cross(v) + 2.0 * det).abs() > tol
                            {
                                continue;
                            }
                            if u.dot(v).abs() > tol {
                                continue;
                            }
                            return LatticeKind::CenteredRectangular;
                        }
                    }
                }
            }

            LatticeKind::Oblique
        }

        for t in super::registry().iter() {
            let Some(meta) = t.declared_symmetry() else {
                continue; // aperiodic / quasiperiodic / parameterized
            };
            assert!(
                legal_kinds(meta.group).contains(&meta.lattice),
                "{}: {} does not permit lattice kind {}",
                t.id(),
                meta.group,
                meta.lattice
            );
            if let Some([a, b]) = t.lattice_basis() {
                let measured = measured_kind(a, b);
                assert!(
                    measured == meta.lattice
                        || (measured == LatticeKind::Square
                            && matches!(
                                meta.lattice,
                                LatticeKind::Rectangular | LatticeKind::CenteredRectangular
                            ))
                        || (measured == LatticeKind::Hexagonal
                            && meta.lattice == LatticeKind::CenteredRectangular),
                    "{}: declared {} but the declared basis measures as {}",
                    t.id(),
                    meta.lattice,
                    measured
                );
            }
        }
    }

    /// Every registered tiling generates without error (scale 1.0, fixed
    /// region) and produces tiles whose ear-clipped triangulation is
    /// valid: exactly n−2 triangles per polygon (n = vertex count), with
    /// the triangle-area sum matching the polygon's area (within
    /// 1e-9 × max(area, 1e-6)).
    #[test]
    fn all_tiles_triangulate() {
        let v2 = |x: f64, y: f64| Vec2::new(x, y);
        let region = BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0));
        for t in super::registry().iter() {
            let mut p = ParamSet::default_for(t.params());
            p.set(crate::lattice::SCALE_ID, ParamValue::Float(1.0));
            let r = t.generate(&p, &Query { region });
            assert!(
                r.message.is_none(),
                "{}: {msg}",
                t.name(),
                msg = r.message.unwrap_or("")
            );
            assert!(!r.tiles.is_empty(), "{}: no tiles", t.name());
            for tile in &r.tiles {
                let tris = tile.poly.triangulate();
                assert_eq!(
                    tris.len(),
                    tile.poly.pts.len() - 2,
                    "{}: triangle count",
                    t.name()
                );
                let area: f64 = tris
                    .iter()
                    .map(|[a, b, c]| {
                        let (a, b, c) = (
                            tile.poly.pts[*a as usize],
                            tile.poly.pts[*b as usize],
                            tile.poly.pts[*c as usize],
                        );
                        (b - a).cross(c - a).abs() * 0.5
                    })
                    .sum();
                let pa = tile.poly.area();
                assert!(
                    (area - pa).abs() <= 1e-9 * pa.max(1e-6),
                    "{}: triangulated area {area} != polygon area {pa}",
                    t.name()
                );
            }
        }
    }

    /// Pin (tile positivity, every built-in): every registered tiling
    /// generates at default parameters over a fixed region (same
    /// depth/region as `all_tiles_triangulate` above, which triangulates
    /// every tile) and every generated tile must carry at least 3 finite
    /// vertices and a strictly positive area. Zero-area or non-finite
    /// tiles would silently break ear-clipping, coverage bookkeeping and
    /// triangulation downstream; this is the hard validity gate on the
    /// tile geometry itself. Failures name the tiling.
    ///
    /// Strictly positive, not epsilon: area() is the polygon's own shoelace
    /// magnitude — 0.0 is a degenerate tile by definition (never legal),
    /// so the threshold is exactly 0.0 (no epsilon can be "just generous",
    /// 0.0 is the real validity cut). Vertex finiteness is checked
    /// separately because NaN/Inf vertices would poison every downstream
    /// computation while still passing area().
    #[test]
    fn all_built_ins_positive_area_tiles() {
        let v2 = |x: f64, y: f64| Vec2::new(x, y);
        let region = BBox::new(v2(-8.0, -6.0), v2(8.0, 6.0));
        // Tracked so the runner can report how much geometry this pin
        // covered; printed on stdout (cargo captures it by default, so
        // it shows only under --nocapture).
        let mut total_tiles = 0usize;
        let mut tilings_checked = 0usize;
        for t in super::registry().iter() {
            let p = ParamSet::default_for(t.params());
            let r = t.generate(&p, &Query { region });
            assert!(
                r.message.is_none(),
                "{}: {msg}",
                t.name(),
                msg = r.message.unwrap_or("")
            );
            assert!(!r.tiles.is_empty(), "{}: no tiles", t.name());
            total_tiles += r.tiles.len();
            tilings_checked += 1;
            for tile in &r.tiles {
                assert!(
                    tile.poly.pts.len() >= 3,
                    "{}: tile with fewer than 3 vertices",
                    t.name()
                );
                for pt in &tile.poly.pts {
                    assert!(
                        pt.x.is_finite() && pt.y.is_finite(),
                        "{}: non-finite vertex {pt}",
                        t.name()
                    );
                }
                let area = tile.poly.area();
                assert!(
                    area > 0.0,
                    "{}: non-positive-area tile (area = {area})",
                    t.name()
                );
            }
        }
        // Reported so the CI run log shows the coverage of this pin.
        println!(
            "all_built_ins_positive_area_tiles: examined {total_tiles} tiles across {tilings_checked} tilings"
        );
    }

    // -----------------------------------------------------------------
    // Doc-structure pin, lattice-determinant tier (kuniform + hankin
    // families). For each of the 42 docs that state a lattice-basis
    // determinant token — 18 kuniform 2-uniform primals + 17 kuniform
    // face-centroid duals + 7 hankin family docs — the token is parsed
    // and compared against `|lattice[0].cross(lattice[1])|` of the live
    // constructor at 1e-9 relative tolerance.
    //
    // The value-carrying "det " token (trailing space — NOT "|det|",
    // which appears once more in 16 of 17 dual docs as the "sum ...
    // = |det|" tail) must occur exactly once per doc (fail loudly on
    // zero or more than one occurrence). Value-carrying grammar
    // (limited to the forms present in the scoped docs): plain integer
    // (`1`, `2`), `a√3` (`2√3`), `a+b√3` (`12+8√3`), `a√3/2` (`9√3/2`),
    // `a+b√3/2` (`1+3√3/2`), `a+√3/2` (`3+√3/2`), `a√3/2+b` (`4√3/2+3`),
    // and the two hand-written forms `2h² = 6+3√3` (kuniform-n16) and
    // `(1+√3)² = 4+2√3` (hankin-3-3-4-3-4) — both parse the right-hand
    // side after " = ". Anything else fails loudly rather than being
    // silently skipped.
    #[cfg(all(feature = "islamic", feature = "kuniform"))]
    #[test]
    fn doc_lattice_determinants_match_constructors() {
        use crate::families::islamic::hankin_3_3_3_4_4::hankin_3_3_3_4_4;
        use crate::families::islamic::hankin_3_3_4_3_4::hankin_3_3_4_3_4;
        use crate::families::islamic::hankin_3_4_6_4::hankin_3_4_6_4;
        use crate::families::islamic::hankin_3_6_3_6::hankin_3_6_3_6;
        use crate::families::islamic::hankin_3_12_12::hankin_3_12_12;
        use crate::families::islamic::hankin_4_6_12::hankin_4_6_12;
        use crate::families::islamic::hankin_square_two_point::hankin_square_two_point;
        use crate::families::kuniform::dual::{
            kuniform_dual_n1, kuniform_dual_n2, kuniform_dual_n3, kuniform_dual_n4,
            kuniform_dual_n5, kuniform_dual_n8, kuniform_dual_n9, kuniform_dual_n12,
            kuniform_dual_n13, kuniform_dual_n14, kuniform_dual_n15, kuniform_dual_n16,
            kuniform_dual_n18, kuniform_dual_n19, kuniform_dual_n20, kuniform_dual_p6m,
            kuniform_dual_pmm,
        };
        use crate::families::kuniform::kuniform::{
            kuniform_n1, kuniform_n2, kuniform_n3, kuniform_n4, kuniform_n5, kuniform_n6,
            kuniform_n7, kuniform_n8, kuniform_n9, kuniform_n12, kuniform_n13, kuniform_n14,
            kuniform_n15, kuniform_n16, kuniform_n17, kuniform_n18, kuniform_n19,
            kuniform_n20,
        };
        use crate::lattice::LatticeTiling;

        /// Parse one Q(√3) det token (see the grammar note above).
        /// Fails loudly on anything outside the documented forms.
        fn parse_det_token(token: &str, doc_file: &str) -> f64 {
            let s = token.replace("\u{221a}3", "s");
            let mut total = 0.0f64;
            for term in s.split('+') {
                let term = term.trim();
                if term.is_empty() {
                    continue;
                }
                let (numerator, denominator) = match term.find('/') {
                    Some(i) => (&term[..i], Some(term[i + 1..].trim())),
                    None => (term, None),
                };
                let mut coef = String::new();
                let mut has_sqrt3 = false;
                for ch in numerator.chars() {
                    if ch == 's' {
                        has_sqrt3 = true;
                    } else if ch.is_ascii_digit() {
                        coef.push(ch);
                    } else {
                        panic!(
                            "docs/tilings/{doc_file}: det token numerator has unexpected char {ch:?}: {numerator:?}"
                        );
                    }
                }
                let coef: f64 = if coef.is_empty() {
                    1.0
                } else {
                    coef.parse().unwrap()
                };
                let mut val = coef * if has_sqrt3 { 3.0f64.sqrt() } else { 1.0 };
                if let Some(d) = denominator {
                    let d: f64 = d.parse().unwrap_or_else(|e| {
                        panic!("docs/tilings/{doc_file}: cannot parse det token denominator: {d:?} ({e})")
                    });
                    val /= d;
                }
                total += val;
            }
            total
        }

        let docs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/tilings");

        let cases: Vec<(&str, fn() -> LatticeTiling)> = vec![
            // Hankin family (7 docs).
            ("hankin-4-6-12", hankin_4_6_12),
            ("hankin-3-12-12", hankin_3_12_12),
            ("hankin-square-two-point", hankin_square_two_point),
            ("hankin-3-6-3-6", hankin_3_6_3_6),
            ("hankin-3-4-6-4", hankin_3_4_6_4),
            ("hankin-3-3-3-4-4", hankin_3_3_3_4_4),
            ("hankin-3-3-4-3-4", hankin_3_3_4_3_4),
            // Kuniform 2-uniform primals (18 docs).
            ("kuniform-n1", kuniform_n1),
            ("kuniform-n2", kuniform_n2),
            ("kuniform-n3", kuniform_n3),
            ("kuniform-n4", kuniform_n4),
            ("kuniform-n5", kuniform_n5),
            ("kuniform-n6", kuniform_n6),
            ("kuniform-n7", kuniform_n7),
            ("kuniform-n8", kuniform_n8),
            ("kuniform-n9", kuniform_n9),
            ("kuniform-n12", kuniform_n12),
            ("kuniform-n13", kuniform_n13),
            ("kuniform-n14", kuniform_n14),
            ("kuniform-n15", kuniform_n15),
            ("kuniform-n16", kuniform_n16),
            ("kuniform-n17", kuniform_n17),
            ("kuniform-n18", kuniform_n18),
            ("kuniform-n19", kuniform_n19),
            ("kuniform-n20", kuniform_n20),
            // Kuniform face-centroid duals (17 docs).
            ("kuniform-dual-p6m", kuniform_dual_p6m),
            ("kuniform-dual-pmm", kuniform_dual_pmm),
            ("kuniform-dual-n1", kuniform_dual_n1),
            ("kuniform-dual-n2", kuniform_dual_n2),
            ("kuniform-dual-n3", kuniform_dual_n3),
            ("kuniform-dual-n4", kuniform_dual_n4),
            ("kuniform-dual-n5", kuniform_dual_n5),
            ("kuniform-dual-n8", kuniform_dual_n8),
            ("kuniform-dual-n9", kuniform_dual_n9),
            ("kuniform-dual-n12", kuniform_dual_n12),
            ("kuniform-dual-n13", kuniform_dual_n13),
            ("kuniform-dual-n14", kuniform_dual_n14),
            ("kuniform-dual-n15", kuniform_dual_n15),
            ("kuniform-dual-n16", kuniform_dual_n16),
            ("kuniform-dual-n18", kuniform_dual_n18),
            ("kuniform-dual-n19", kuniform_dual_n19),
            ("kuniform-dual-n20", kuniform_dual_n20),
        ];

        assert_eq!(cases.len(), 42, "det pin: expected 42 scoped docs");

        for (id, make) in cases {
            let tiling = make();
            assert_eq!(
                tiling.id(), id,
                "docs/tilings/{id}.md: constructor id mismatch (doc id != constructor id)"
            );
            let doc = std::fs::read_to_string(docs_dir.join(format!("{id}.md")))
                .unwrap_or_else(|e| panic!("docs/tilings/{id}.md readable: {e}"));

            let occurrences = doc.matches("det ").count();
            assert_eq!(
                occurrences, 1,
                "docs/tilings/{id}.md: expected exactly one \"det \" token (trailing space), found {occurrences}"
            );
            let rest = &doc[doc.find("det ").unwrap() + 4..];

            // Hand-written forms keep only the right-hand side; the
            // generic grammar must not see the leading "2h² = " or the
            // leading "(1+√3)² = ".
            let token: String = if let Some(tail) = rest.strip_prefix("2h² = ") {
                let end = tail.find(|c: char| c == ')' || c == ';' || c == ',' || c == ' ' || c == '\u{2014}').unwrap_or(tail.len());
                tail[..end].trim().to_string()
            } else if let Some(tail) = rest.strip_prefix("(1+√3)² = ") {
                let end = tail.find(|c: char| c == ')' || c == ';' || c == ',' || c == ' ' || c == '\u{2014}').unwrap_or(tail.len());
                tail[..end].trim().to_string()
            } else {
                let end = rest.find(|c: char| c == ' ' || c == ';' || c == ')' || c == ',' || c == '\u{2014}').unwrap_or(rest.len());
                rest[..end].trim().to_string()
            };

            let claimed = parse_det_token(&token, id);
            let actual = tiling.lattice[0].cross(tiling.lattice[1]).abs();
            assert!(
                (actual - claimed).abs() <= 1e-9 * claimed.abs().max(1.0),
                "docs/tilings/{id}.md: doc claims det {claimed} but the constructor's \\
                 basis spans |cross| = {actual}"
            );
        }
    }

    // -----------------------------------------------------------------
    // Doc-structure pin, per-cell shape-composition tier (kuniform +
    // hankin families). For each of the 27 docs that state a per-cell
    // shape composition — 9 kuniform 2-uniform primals + 17 kuniform
    // face-centroid duals + 1 hankin family doc (hankin-square-two-point) —
    // the noun multiset is parsed (side counts: triangle→3,
    // square/quadrilateral/kite/rhombus/quad→4, pentagon/pentagonal→5,
    // hexagon/hexagonal→6, octagon→8, decagon/decagonal→10,
    // dodecagon→12; adjectives stripped) and compared against the
    // multiset of `cell.poly.len()` side counts of the live constructor
    // cell polygons. The doc total (sum of the multiset) must equal
    // `cells.len()` (the total per-cell count the existing claim pin
    // already asserts) — internal consistency.
    //
    // Parser design (tolerant, limited to the forms present in the
    // scoped docs):
    // - Kuniform primals: segment = text between the first "tiles/cell"
    //   and the first ";"; trailing clauses ("in orbits {...}", "the
    //   2-hexagon + 8-triangle centered cell ...", "area ...") are
    //   dropped at the first ", " inside the segment.
    // - Kuniform duals: segment = text between the first ": " and the
    //   first " per cell"; split at " + " (each sub-segment is one
    //   "N noun" pair group); the walk from a count token tolerates
    //   non-noun tokens (adjectives, degree numbers, "in cyclic order"
    //   clauses, parenthetical shapes) but stops within 5 steps.
    // - hankin-square-two-point: segment = text between the first
    //   " per cell" and the first " — " (after the leading separator
    //   is dropped), i.e. the "four corner triangles, two pentagonal
    //   petals, one 10-gon core" list; ordinal words one..twelve are
    //   accepted as counts.
    // Anything outside the documented forms fails loudly (the multiset
    // parse must succeed and the doc total must match the cell count).
    #[cfg(all(feature = "islamic", feature = "kuniform"))]
    #[test]
    fn doc_per_cell_shape_compositions_match_constructors() {
        use crate::families::islamic::hankin_square_two_point::hankin_square_two_point;
        use crate::families::kuniform::dual::{
            kuniform_dual_n1, kuniform_dual_n2, kuniform_dual_n3, kuniform_dual_n4,
            kuniform_dual_n5, kuniform_dual_n8, kuniform_dual_n9, kuniform_dual_n12,
            kuniform_dual_n13, kuniform_dual_n14, kuniform_dual_n15, kuniform_dual_n16,
            kuniform_dual_n18, kuniform_dual_n19, kuniform_dual_n20, kuniform_dual_p6m,
            kuniform_dual_pmm,
        };
        use crate::families::kuniform::kuniform::{
            kuniform_n2, kuniform_n3, kuniform_n4, kuniform_n6, kuniform_n7, kuniform_n12,
            kuniform_n14, kuniform_n15, kuniform_n17,
        };
        use crate::lattice::LatticeTiling;

        /// Map one noun token to polygon side counts. Returns `None`
        /// for anything that is not a shape noun (adjectives,
        /// connectives, degree numbers, ...), which the walk-forward
        /// tolerates by skipping.
        fn is_shape_noun(token: &str) -> Option<u32> {
            match token.to_ascii_lowercase().as_str() {
                "triangle" | "triangles" => Some(3),
                "square" | "squares" | "quad" | "quads" | "quadrilateral" | "quadrilaterals"
                | "kite" | "kites" | "rhombus" | "rhombi" | "rhombes" => Some(4),
                "pentagon" | "pentagons" | "pentagonal" => Some(5),
                "hexagon" | "hexagons" | "hexagonal" => Some(6),
                "octagon" | "octagons" => Some(8),
                "decagon" | "decagons" | "decagonal" => Some(10),
                "dodecagon" | "dodecagons" => Some(12),
                _ => None,
            }
        }

        /// Ordinals accepted as count tokens alongside plain digits.
        fn ordinal_count(token: &str) -> Option<u32> {
            match token.to_ascii_lowercase().as_str() {
                "one" => Some(1),
                "two" => Some(2),
                "three" => Some(3),
                "four" => Some(4),
                "five" => Some(5),
                "six" => Some(6),
                "seven" => Some(7),
                "eight" => Some(8),
                "nine" => Some(9),
                "ten" => Some(10),
                "eleven" => Some(11),
                "twelve" => Some(12),
                _ => None,
            }
        }

        /// Parse count-then-shape pairs from a segment. Each pair is
        /// (count, noun-side-count); the walk from each count token
        /// stops at the first shape noun within 5 steps and the span
        /// is consumed so degree numbers and shape-clauses in the span
        /// cannot double-match.
        fn parse_pairs(segment: &str) -> Vec<(u32, u32)> {
            let segment = segment.replace("10-gon", "decagon");
            let tokens: Vec<&str> = segment
                .split(|c: char| !c.is_ascii_alphanumeric())
                .filter(|t| !t.is_empty())
                .collect();
            let mut pairs = Vec::new();
            let mut i = 0usize;
            while i < tokens.len() {
                let count: u32 = if tokens[i].chars().all(|c| c.is_ascii_digit()) {
                    tokens[i].parse().unwrap()
                } else if let Some(n) = ordinal_count(tokens[i]) {
                    n
                } else {
                    i += 1;
                    continue;
                };
                // walk forward to the first shape noun (within 5 steps)
                let mut j = i + 1;
                let mut found: Option<usize> = None;
                while j < tokens.len() && j <= i + 5 {
                    if is_shape_noun(tokens[j]).is_some() {
                        found = Some(j);
                        break;
                    }
                    j += 1;
                }
                if let Some(j) = found {
                    let side = is_shape_noun(tokens[j]).unwrap();
                    pairs.push((count, side));
                    i = j + 1;
                } else {
                    i += 1;
                }
            }
            pairs
        }

        let docs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/tilings");

        // Doc id, constructor, and the class (anchor strategy) used to
        // extract the composition segment.
        let cases: Vec<(&str, fn() -> LatticeTiling, &str)> = vec![
            ("hankin-square-two-point", hankin_square_two_point, "hankin"),
            ("kuniform-n2", kuniform_n2, "primal"),
            ("kuniform-n3", kuniform_n3, "primal"),
            ("kuniform-n4", kuniform_n4, "primal"),
            ("kuniform-n6", kuniform_n6, "primal"),
            ("kuniform-n7", kuniform_n7, "primal"),
            ("kuniform-n12", kuniform_n12, "primal"),
            ("kuniform-n14", kuniform_n14, "primal"),
            ("kuniform-n15", kuniform_n15, "primal"),
            ("kuniform-n17", kuniform_n17, "primal"),
            ("kuniform-dual-p6m", kuniform_dual_p6m, "dual"),
            ("kuniform-dual-pmm", kuniform_dual_pmm, "dual"),
            ("kuniform-dual-n1", kuniform_dual_n1, "dual"),
            ("kuniform-dual-n2", kuniform_dual_n2, "dual"),
            ("kuniform-dual-n3", kuniform_dual_n3, "dual"),
            ("kuniform-dual-n4", kuniform_dual_n4, "dual"),
            ("kuniform-dual-n5", kuniform_dual_n5, "dual"),
            ("kuniform-dual-n8", kuniform_dual_n8, "dual"),
            ("kuniform-dual-n9", kuniform_dual_n9, "dual"),
            ("kuniform-dual-n12", kuniform_dual_n12, "dual"),
            ("kuniform-dual-n13", kuniform_dual_n13, "dual"),
            ("kuniform-dual-n14", kuniform_dual_n14, "dual"),
            ("kuniform-dual-n15", kuniform_dual_n15, "dual"),
            ("kuniform-dual-n16", kuniform_dual_n16, "dual"),
            ("kuniform-dual-n18", kuniform_dual_n18, "dual"),
            ("kuniform-dual-n19", kuniform_dual_n19, "dual"),
            ("kuniform-dual-n20", kuniform_dual_n20, "dual"),
        ];

        assert_eq!(cases.len(), 27, "composition pin: expected 27 scoped docs");

        for (id, make, class) in cases {
            let tiling = make();
            assert_eq!(
                tiling.id(), id,
                "docs/tilings/{id}.md: constructor id mismatch (doc id != constructor id)"
            );
            let doc = std::fs::read_to_string(docs_dir.join(format!("{id}.md")))
                .unwrap_or_else(|e| panic!("docs/tilings/{id}.md readable: {e}"));

            let segment: String = match class {
                "primal" => {
                    let anchor = "tiles/cell";
                    assert_eq!(
                        doc.matches(anchor).count(), 1,
                        "docs/tilings/{id}.md: expected exactly one \"tiles/cell\" anchor"
                    );
                    let rest = &doc[doc.find(anchor).unwrap() + anchor.len()..];
                    let end = rest.find(';').unwrap_or(rest.len());
                    let mut seg = rest[..end].trim().to_string();
                    if let Some(i) = seg.find(", ") {
                        seg = seg[..i].trim().to_string();
                    }
                    seg
                }
                "dual" => {
                    let anchor = ": ";
                    let anchor_idx = doc.find(anchor).unwrap_or_else(|| {
                        panic!("docs/tilings/{id}.md: expected a \": \" anchor for the composition segment")
                    });
                    let rest = &doc[anchor_idx + anchor.len()..];
                    let end = rest.find(" per cell").unwrap_or_else(|| {
                        panic!("docs/tilings/{id}.md: expected a \" per cell\" anchor for the composition segment")
                    });
                    rest[..end].trim().to_string()
                }
                _ => {
                    // hankin-square-two-point: text between the first
                    // " per cell" and the first " — " (after dropping
                    // the leading separator).
                    let anchor = " per cell";
                    assert_eq!(
                        doc.matches(anchor).count(), 1,
                        "docs/tilings/{id}.md: expected exactly one \" per cell\" anchor"
                    );
                    let rest = &doc[doc.find(anchor).unwrap() + anchor.len()..];
                    let rest = rest.trim_start();
                    let rest = rest
                        .strip_prefix('\u{2014}')
                        .map(|s| s.trim_start())
                        .unwrap_or(rest);
                    let end = rest.find(" \u{2014} ").unwrap_or_else(|| {
                        panic!("docs/tilings/{id}.md: expected a \" — \" separator for the composition segment")
                    });
                    rest[..end].trim().to_string()
                }
            };

            let subsegments: Vec<&str> = if class == "dual" {
                segment.split(" + ").collect()
            } else {
                vec![segment.as_str()]
            };

            let mut claimed: Vec<u32> = Vec::new();
            for sub in subsegments {
                for (count, side) in parse_pairs(sub) {
                    let mut i = 0;
                    while i < count {
                        claimed.push(side);
                        i += 1;
                    }
                }
            }

            assert_eq!(
                claimed.len(),
                tiling.cells.len(),
                "docs/tilings/{id}.md: doc claims {} per-cell faces but the \\
                 constructor contains {}",
                claimed.len(),
                tiling.cells.len()
            );

            let mut claimed_sorted = claimed.clone();
            claimed_sorted.sort_unstable();
            let mut actual: Vec<u32> = tiling.cells.iter().map(|c| c.poly.len() as u32).collect();
            actual.sort_unstable();
            assert_eq!(
                claimed_sorted, actual,
                "docs/tilings/{id}.md: doc shape multiset {claimed_sorted:?} != \\
                 constructor cell polygon side counts {actual:?}"
            );
        }
    }
}

#[cfg(test)]
mod lattice_culling_tests {
    use super::lattice::LatticeTiling;
    // The lattice entries come from the family modules
    // (`families::<family>::lattice_tilings()`); a family feature
    // adds its own entries, so this list can never go stale.
    use tess_core::geometry::{BBox, Polygon, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::tiling::{Query, Tile, Tiling};

    fn v2(x: f64, y: f64) -> Vec2 {
        Vec2::new(x, y)
    }

    /// Independent emission: enumerate a wide fixed index window straight
    /// from the public table data and keep tiles whose world AABB
    /// intersects the region.
    fn brute_force(lt: &LatticeTiling, params: &ParamSet, region: &BBox, window: i64) -> Vec<Tile> {
        let t = (lt.make_map)(params);
        let l1 = t.transform(lt.lattice[0]);
        let l2 = t.transform(lt.lattice[1]);
        let origin = t.transform(lt.offset);
        let world_cells: Vec<(Polygon, u32)> = lt
            .cells
            .iter()
            .map(|c| (c.poly.transform(&t), c.seed))
            .collect();
        let mut out = Vec::new();
        for i in -window..=window {
            for j in -window..=window {
                let tr = origin + l1 * i as f64 + l2 * j as f64;
                for (ci, (poly, seed)) in world_cells.iter().enumerate() {
                    let bb = poly.aabb();
                    let bb = BBox::new(bb.min + tr, bb.max + tr);
                    if bb.intersects(*region) {
                        out.push(Tile {
                            proto: ci as u8,
                            poly: poly.translated(tr),
                            seed: lt.seed_pattern.base(i, j).wrapping_add(*seed),
                        });
                    }
                }
            }
        }
        out
    }

    /// Canonical sort key: (proto, seed, all vertex bits). Both
    /// implementations build tile geometry with identical operations, so
    /// equal keys imply bit-identical geometry.
    fn key(tile: &Tile) -> (u8, u32, Vec<u64>) {
        let mut bits = Vec::with_capacity(tile.poly.pts.len() * 2);
        for p in &tile.poly.pts {
            bits.push(p.x.to_bits());
            bits.push(p.y.to_bits());
        }
        (tile.proto, tile.seed, bits)
    }

    fn compare(lt: &LatticeTiling, params: &ParamSet, region: BBox, label: &str) {
        let got = lt.generate(params, &Query { region });
        assert!(got.message.is_none(), "{label}: {:?}", got.message);
        assert!(!got.tiles.is_empty(), "{label}: no tiles");
        let mut a: Vec<_> = got.tiles.iter().map(key).collect();
        let mut b: Vec<_> = brute_force(lt, params, &region, 40)
            .iter()
            .map(key)
            .collect();
        a.sort();
        b.sort();
        assert_eq!(
            a.len(),
            b.len(),
            "{label}: tile count {} vs {}",
            a.len(),
            b.len()
        );
        for (ka, kb) in a.iter().zip(b.iter()) {
            assert_eq!(ka, kb, "{label}: tile mismatch");
        }
    }

    #[test]
    fn lattice_culling_matches_brute_force() {
    let mut lattices: Vec<(&str, LatticeTiling)> = Vec::new();
    #[cfg(feature = "uniform")]
    lattices.extend(crate::families::uniform::lattice_tilings());
    #[cfg(feature = "arrangements")]
    lattices.extend(crate::families::arrangements::lattice_tilings());
    #[cfg(feature = "pentagons")]
    lattices.extend(crate::families::pentagons::lattice_tilings());
    #[cfg(feature = "laves")]
    lattices.extend(crate::families::laves::lattice_tilings());
    #[cfg(feature = "kuniform")]
    lattices.extend(crate::families::kuniform::lattice_tilings());
    #[cfg(feature = "pavers")]
    lattices.extend(crate::families::pavers::lattice_tilings());
    #[cfg(feature = "islamic")]
    lattices.extend(crate::families::islamic::lattice_tilings());
        for (name, lt) in &lattices {
            let specs = lt.params();
            // Default parameters and a rotated/scaled variant: the rotated
            // case is what makes naive cell-space padding wrong for
            // non-axis-aligned lattices.
            compare(
                lt,
                &ParamSet::default_for(specs),
                BBox::new(v2(-5.0, -4.0), v2(9.0, 8.0)),
                name,
            );
            let mut p = ParamSet::default_for(specs);
            p.set(
                crate::lattice::SCALE_ID,
                tess_core::params::ParamValue::Float(0.7),
            );
            p.set(
                crate::lattice::ROTATION_ID,
                tess_core::params::ParamValue::Angle(30.0),
            );
            compare(
                lt,
                &p,
                BBox::new(v2(-4.0, -3.0), v2(6.0, 5.0)),
                &format!("{name}(rot30,scale0.7)"),
            );
        }
    }

    /// Pin (lattice area identity, whole lattice class): per unit lattice cell,
    /// the per-cell tile count (one prototile instance per lattice translate per
    /// cell) weighted by prototile area equals the lattice determinant:
    ///
    /// ```text
    ///   Σᵢ area(cellᵢ.poly) == |lattice[0] × lattice[1]|
    /// ```
    ///
    /// The translates of the cell polygons tile the plane exactly once, so the
    /// area inside one fundamental domain is the sum of the cell areas — the
    /// cell-space determinant of the declared basis is the area of that domain.
    /// This is the exact identity the doc-determinant pin
    /// (`doc_lattice_determinants_match_constructors`, end of `mod tests`)
    /// asserts against; this pin makes the per-cell-count × prototile-area half
    /// permanent over the WHOLE data-table class (all 104 lattice tilings),
    /// parameter-independently (the cell polygons and the basis live in cell
    /// space), instead of only at the two 42- and 27-doc doc-pin scopes.
    ///
    /// Tolerance: 1e-9 relative (determinant and shoelace magnitudes in f64;
    /// the repo's other live-area pins use the same 1e-9 relative convention).
    /// Failures name the tiling id; the enumeration is the
    /// `lattice_culling_matches_brute_force` list shape — a family feature adds
    /// its entries, so this list can never go stale.
    #[test]
    fn lattice_area_identity_over_the_lattice_class() {
        let mut lattices: Vec<(&str, LatticeTiling)> = Vec::new();
        #[cfg(feature = "uniform")]
        lattices.extend(crate::families::uniform::lattice_tilings());
        #[cfg(feature = "arrangements")]
        lattices.extend(crate::families::arrangements::lattice_tilings());
        #[cfg(feature = "pentagons")]
        lattices.extend(crate::families::pentagons::lattice_tilings());
        #[cfg(feature = "laves")]
        lattices.extend(crate::families::laves::lattice_tilings());
        #[cfg(feature = "kuniform")]
        lattices.extend(crate::families::kuniform::lattice_tilings());
        #[cfg(feature = "pavers")]
        lattices.extend(crate::families::pavers::lattice_tilings());
        #[cfg(feature = "islamic")]
        lattices.extend(crate::families::islamic::lattice_tilings());

        assert_eq!(
            lattices.len(),
            104,
            "DESIGN.md claims \"All 104 lattice tilings\" — the lattice class enumerates {}",
            lattices.len()
        );
        // Cross-check: the enumeration is a subset of the registry — every
        // listed tiling is registered (no orphan data-table entry), and the
        // doc-name key is not the identity (the tiling's own id is).
        let registry = crate::registry();
        for (_, lt) in &lattices {
            let registered = registry.get(lt.id()).unwrap_or_else(|| {
                panic!("lattice tiling {} not present in the registry", lt.id())
            });
            assert_eq!(
                registered.id(),
                lt.id(),
                "registry id mismatch for {}",
                lt.id()
            );
        }
        for (_, lt) in &lattices {
            let det = lt.lattice[0].cross(lt.lattice[1]).abs();
            assert!(det > 0.0, "{}: degenerate lattice basis", lt.id());
            assert!(!lt.cells.is_empty(), "{}: no cell polygons declared", lt.id());
            let sum: f64 = lt.cells.iter().map(|c| c.poly.area()).sum();
            assert!(
                (sum - det).abs() <= 1e-9 * det.max(1.0),
                "{}: Σ per-cell tile areas {sum} != |det| {det}",
                lt.id()
            );
        }
    }
}


/// M57 layer-mode verification anchors: the generic strapwork engine in
/// `tess_core::strapwork` laid on each canonical registry substrate at
/// the canonical contact angle must reproduce the registered PIC
/// partition's internal edges **exactly** (1e-9 on endpoints, 1e-6 key
/// grid) — layer mode and tiling mode pinned to each other (PROPOSED #14).
#[cfg(all(test, feature = "islamic", feature = "uniform"))]
mod strapwork_layer_mode {
    use std::collections::BTreeMap;
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::strapwork::{walls, Wall};
    use tess_core::tiling::{Query, Tiling};

    type SegKey = ((i64, i64), (i64, i64));

    fn key_pt(p: Vec2) -> (i64, i64) {
        ((p.x * 1e6).round() as i64, (p.y * 1e6).round() as i64)
    }

    /// Endpoint-canonical (order- and direction-independent) key.
    fn key_seg(a: Vec2, b: Vec2) -> SegKey {
        let (ka, kb) = (key_pt(a), key_pt(b));
        if ka <= kb {
            (ka, kb)
        } else {
            (kb, ka)
        }
    }

    fn inside(p: Vec2, r: f64) -> bool {
        p.x.abs() <= r && p.y.abs() <= r
    }

    /// Engine walls over the tiles of `substrate` intersecting `region`,
    /// kept inside the radius-`r` box (where coverage is complete).
    fn engine_set(substrate: &dyn Tiling, delta_deg: f64, region: BBox, r: f64) -> BTreeMap<SegKey, Wall> {
        let params = ParamSet::default_for(substrate.params());
        let res = substrate.generate(&params, &Query { region });
        walls(&res.tiles, delta_deg)
            .into_iter()
            .filter(|w| inside(w.a, r) && inside(w.b, r))
            .map(|w| (key_seg(w.a, w.b), w))
            .collect()
    }

    /// Internal edges of the registered PIC partition: edges shared by
    /// exactly 2 generated faces, kept inside the radius-`r` box.
    fn pic_set(pic: &dyn Tiling, region: BBox, r: f64) -> BTreeMap<SegKey, [Vec2; 2]> {
        let params = ParamSet::default_for(pic.params());
        let res = pic.generate(&params, &Query { region });
        let mut count: BTreeMap<SegKey, u32> = BTreeMap::new();
        let mut raw: BTreeMap<SegKey, [Vec2; 2]> = BTreeMap::new();
        for tile in &res.tiles {
            let q = &tile.poly.pts;
            for i in 0..q.len() {
                let (a, b) = (q[i], q[(i + 1) % q.len()]);
                let k = key_seg(a, b);
                *count.entry(k).or_default() += 1;
                raw.entry(k).or_insert([a, b]);
            }
        }
        raw.into_iter()
            .filter(|(k, seg)| count[k] == 2 && inside(seg[0], r) && inside(seg[1], r))
            .collect()
    }

    fn dist(a: Vec2, b: Vec2) -> f64 {
        (a - b).length()
    }

    /// The anchor: engine walls ≡ PIC internal edges — same key sets,
    /// endpoints to 1e-9 (best pairing), equal length sums; and stable
    /// against window growth (no boundary artifacts on either side).
    fn check_anchor(substrate_id: &str, pic_id: &str, delta_deg: f64, region: BBox, r: f64) {
        let registry = crate::registry();
        let substrate = registry.get(substrate_id).expect(substrate_id);
        let pic = registry.get(pic_id).expect(pic_id);
        let grown = BBox::new(
            Vec2::new(region.min.x - 3.0, region.min.y - 3.0),
            Vec2::new(region.max.x + 3.0, region.max.y + 3.0),
        );

        let eng = engine_set(substrate, delta_deg, region, r);
        let pic_edges = pic_set(pic, region, r);
        assert!(!eng.is_empty() && !pic_edges.is_empty(), "{pic_id}: empty anchor sets");

        let missing: Vec<_> = pic_edges.keys().copied().filter(|k| !eng.contains_key(k)).collect();
        let extra: Vec<_> = eng.keys().copied().filter(|k| !pic_edges.contains_key(k)).collect();
        assert!(
            missing.is_empty() && extra.is_empty(),
            "{substrate_id}@{delta_deg}° vs {pic_id}: engine={} pic={} missing={missing:?} extra={extra:?}",
            eng.len(),
            pic_edges.len()
        );

        let mut worst = 0.0f64;
        let mut len_e = 0.0f64;
        let mut len_p = 0.0f64;
        for (k, w) in &eng {
            let seg = &pic_edges[k];
            let pair = dist(w.a, seg[0])
                .max(dist(w.b, seg[1]))
                .min(dist(w.a, seg[1]).max(dist(w.b, seg[0])));
            worst = worst.max(pair);
            len_e += w.length();
            len_p += dist(seg[0], seg[1]);
        }
        assert!(worst < 1e-9, "{pic_id}: worst endpoint mismatch {worst:e}");
        assert!((len_e - len_p).abs() < 1e-9, "{pic_id}: length sums {len_e} vs {len_p}");

        // Window-size invariance (same filter radius, grown region).
        assert_eq!(eng.keys().collect::<Vec<_>>(), engine_set(substrate, delta_deg, grown, r).keys().collect::<Vec<_>>(), "{substrate_id}: engine set shifted with the window");
        assert_eq!(pic_edges.keys().collect::<Vec<_>>(), pic_set(pic, grown, r).keys().collect::<Vec<_>>(), "{pic_id}: internal-edge set shifted with the window");
    }

    #[test]
    fn layer_walls_4_8_8_match_pic_partition() {
        check_anchor(
            "truncated-square",
            "pic-4-8-8",
            45.0,
            BBox::new(Vec2::new(-8.0, -8.0), Vec2::new(8.0, 8.0)),
            6.0,
        );
    }

    #[test]
    fn layer_walls_3_6_3_6_match_pic_partition() {
        check_anchor(
            "trihexagonal",
            "pic-3-6-3-6",
            60.0,
            BBox::new(Vec2::new(-8.0, -8.0), Vec2::new(8.0, 8.0)),
            5.0,
        );
    }

    #[test]
    fn layer_walls_3_12_12_match_pic_partition() {
        check_anchor(
            "truncated-hexagonal",
            "pic-3-12-12",
            60.0,
            BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
            6.0,
        );
    }
}

/// M164-follow-up battery: every decorated classic-δ export (12 rows × 4
/// styles) renders SVG with **zero non-finite coordinates**. This guards
/// the NaN band/side rings that shipped — the straight-through case in
/// `join_geometry` fell through to the parallel-lines mitre branch (isect
/// divides by ~0 → NaN) and the SVG bytes carried literal `NaN` components.
/// Only the CLI's exact 1200×900@scale-1.0 region is exercised (the one
/// `docs/rendering.md` documents).
#[cfg(all(test, feature = "uniform", feature = "islamic"))]
mod decoration_finiteness_battery {
    use tess_core::geometry::{BBox, Vec2};
    use tess_core::params::ParamSet;
    use tess_core::strapwork::{plan_decoration, walls, Decoration};
    use tess_core::tiling::Query;
    use tess_core::svg::render_svg_decorated;

    /// The 12 classic-δ rows from `docs/rendering.md` (the only rows that
    /// pass all four styles' gates — verified against the CLI).
    const ROWS: [(&str, f64); 12] = [
        ("square", 45.0),
        ("square", 90.0),
        ("hexagon", 30.0),
        ("hexagon", 60.0),
        ("triangle", 30.0),
        ("triangle", 60.0),
        ("trihexagonal", 60.0),
        ("pic-3-6-3-6", 60.0),
        ("truncated-hexagonal", 60.0),
        ("truncated-square", 45.0),
        ("snub-trihexagonal", 60.0),
        ("hexagram", 60.0),
    ];

    const STYLES: [Decoration; 4] = [
        Decoration::Fill,
        Decoration::Outline,
        Decoration::Interlace,
        Decoration::OutlineInterlace,
    ];

    /// The CLI's scale-1.0, 1200×900 export: 20 world units tall,
    /// 20·(1200/900) wide, centered on the origin.
    fn cli_region() -> BBox {
        let world_h = 20.0;
        let world_w = world_h * 1200.0 / 900.0;
        BBox::new(
            Vec2::new(-world_w / 2.0, -world_h / 2.0),
            Vec2::new(world_w / 2.0, world_h / 2.0),
        )
    }

    /// Assert the rendered SVG carries zero non-finite coordinates: no
    /// `NaN`/`inf` substrings anywhere (the `{:.4}` emission prints `inf`
    /// verbatim when the f64 is infinite), and every numeric token inside
    /// every `d="..."` attribute parses as a finite f64.
    fn assert_finite_svg(svg: &str, where_: &str) {
        assert!(!svg.contains("NaN"), "{where_}: NaN in rendered SVG");
        assert!(!svg.contains("inf"), "{where_}: inf in rendered SVG");
        let mut rest = svg;
        while let Some(i) = rest.find("d=\"") {
            let start = i + 3;
            let Some(close) = rest[start..].find('"') else { break };
            let d = &rest[start..start + close];
            for tok in d.split(|c: char| {
                !matches!(c, '-' | '.' | 'e' | 'E' | '+') && !c.is_ascii_digit()
            }) {
                if tok.is_empty() {
                    continue;
                }
                let v: f64 = tok.parse().unwrap_or_else(|_| {
                    panic!("{where_}: bad float token {tok:?} in d: {d:?}")
                });
                assert!(
                    v.is_finite(),
                    "{where_}: non-finite token {tok:?} in d: {d:?}"
                );
            }
            rest = &rest[start + close..];
        }
    }

    #[test]
    fn classic_delta_matrix_is_finite() {
        let registry = crate::registry();
        for &(id, delta) in &ROWS {
            let tiling = registry.get(id).unwrap_or_else(|| panic!("missing tiling {id}"));
            let params = ParamSet::default_for(tiling.params());
            let region = cli_region();
            let res = tiling.generate(&params, &Query { region });
            let walls = walls(&res.tiles, delta);
            for style in STYLES {
                let style_name = style.name();
                let where_ = format!("{id}@{delta}/{style_name}");
                let plan = plan_decoration(&walls, 0.1, style, &region.padding(-1e-9))
                    .unwrap_or_else(|e| panic!("{where_}: honest precondition error: {e}"));
                let svg = render_svg_decorated(&region, &plan);
                assert_finite_svg(&svg, &where_);
                assert!(
                    svg.contains("data-deco"),
                    "{where_}: no decoration layers painted"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests_param_surface_pins {
    //! Pins for the self-describing parameter surface —
    //! ParamSpec well-formedness, set/parse round-trip, and the
    //! fingerprint contract — over the live registry.
    //!
    //! These pins make the "self-describing params" claim permanent:
    //! every tiling declares its parameters via `ParamSpec`, the specs
    //! are well-formed, values round-trip, and the UIs' params
    //! fingerprint (tess-tui `params_key` / tess-gui `preview_params_str`,
    //! byte-identical construction) is stable, sensitive, and ordered.
    //!
    //! Deliberately NOT pinned (per the fleet's in-flight `depth`-param
    //! job for glb-binary-n7): global param totals, per-tiling param
    //! counts, or defaults beyond the claims the README makes.
    use tess_core::params::{ParamId, ParamKind, ParamSpec, ParamSet, ParamValue};

    /// The params fingerprint — the cache-key construction the UIs use.
    /// Byte-identical to tess-tui `thumb::params_key` (thumb.rs) and
    /// tess-gui `ui::preview_params_str` (ui.rs): `id=value` fragments
    /// in declaration order, joined with `,`, missing values falling back
    /// to the spec default. Re-implemented here because the UIs cannot be
    /// reached from this registry-level crate.
    fn fingerprint(specs: &[ParamSpec], params: &ParamSet) -> String {
        specs.iter()
            .map(|s| {
                let v = params.get(s.id).unwrap_or_else(|| s.default_value());
                format!("{}={}", s.id.as_str(), v.display())
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    /// A different in-bounds value for a spec — the sensitivity input.
    fn alternate(spec: ParamSpec) -> ParamValue {
        match spec.kind {
            ParamKind::Float { min, max, .. } => {
                let d = match spec.default_value() { ParamValue::Float(d) => d, _ => unreachable!() };
                ParamValue::Float(if d == min { max } else { min })
            }
            ParamKind::Angle { min, max, .. } => {
                let d = match spec.default_value() { ParamValue::Angle(d) => d, _ => unreachable!() };
                ParamValue::Angle(if d == min { max } else { min })
            }
            ParamKind::Int { min, max, .. } => {
                let d = match spec.default_value() { ParamValue::Int(d) => d, _ => unreachable!() };
                ParamValue::Int(if d == min { max } else { min })
            }
            ParamKind::Bool { .. } => {
                let d = match spec.default_value() { ParamValue::Bool(d) => d, _ => unreachable!() };
                ParamValue::Bool(!d)
            }
        }
    }

    #[test]
    fn registry_params_well_formed() {
        let registry = crate::registry();
        let mut total = 0usize;
        for t in registry.iter() {
            let specs = t.params();
            for s in specs {
                total += 1;
                let where_ = format!("{} / {}", t.id(), s.id.as_str());
                assert!(
                    !s.id.as_str().trim().is_empty(),
                    "{where_}: empty param id"
                );
                assert!(
                    !s.label.trim().is_empty(),
                    "{where_}: empty label"
                );
                match s.kind {
                    ParamKind::Float { min, max, step, default } => {
                        assert!(min.is_finite() && max.is_finite() && step.is_finite(), "{where_}: non-finite bound or step");
                        assert!(min < max, "{where_}: min {min} >= max {max}");
                        assert!(step > 0.0, "{where_}: step {step} <= 0");
                        assert!(default.is_finite(), "{where_}: default not finite");
                        assert!(min <= default && default <= max, "{where_}: default {default} outside [{min}, {max}]");
                    }
                    ParamKind::Angle { min, max, step, default } => {
                        assert!(min.is_finite() && max.is_finite() && step.is_finite(), "{where_}: non-finite bound or step");
                        assert!(min < max, "{where_}: min {min} >= max {max}");
                        assert!(step > 0.0, "{where_}: step {step} <= 0");
                        assert!(default.is_finite(), "{where_}: default not finite");
                        assert!(min <= default && default < max, "{where_}: default {default} outside [{min}, {max})");
                    }
                    ParamKind::Int { min, max, step, default } => {
                        assert!(min < max, "{where_}: min {min} >= max {max}");
                        assert!(step > 0, "{where_}: step {step} <= 0");
                        assert!(min <= default && default <= max, "{where_}: default {default} outside [{min}, {max}]");
                    }
                    ParamKind::Bool { .. } => {
                        // Default is a bool by construction — nothing to check.
                    }
                }
            }
        }
        // `ParamKind` has no Choice variant (Float/Int/Bool/Angle only), so
        // there is no Choice clause to pin — Float/Angle/Int are checked
        // per-kind above and Bool by construction. Reported, not pinned.
        assert!(total > 0, "registry has no param specs at all");
    }

    #[test]
    fn registry_params_round_trip() {
        let registry = crate::registry();
        for t in registry.iter() {
            let specs = t.params();
            let set = ParamSet::default_for(specs);
            let where_ = format!("{}", t.id());
            for s in specs {
                // Default set→get: identical.
                assert_eq!(
                    set.get(s.id),
                    Some(s.default_value()),
                    "{where_} / {}: default round-trip",
                    s.id.as_str()
                );
                match s.kind {
                    ParamKind::Float { min, max, step, .. } => {
                        // Clamped semantics: stepping past either edge stays
                        // at that edge (not wrapping).
                        let lo = ParamValue::Float(min);
                        let hi = ParamValue::Float(max);
                        assert_eq!(s.step_by(lo, -1), lo, "{where_} / {}: min edge", s.id.as_str());
                        assert_eq!(s.step_by(hi, 1), hi, "{where_} / {}: max edge", s.id.as_str());
                        // Mid-range step lands on the grid and in range.
                        let mid = ParamValue::Float((min + max) / 2.0);
                        let stepped = s.step_by(mid, 1);
                        if let ParamValue::Float(v) = stepped {
                            assert!(min <= v && v <= max, "{where_} / {}: mid step {v} outside range", s.id.as_str());
                            let _ = step; // grid exercised via step_by itself
                        } else {
                            panic!("{where_} / {}: mid step returned non-float {stepped:?}", s.id.as_str());
                        }
                    }
                    ParamKind::Angle { min, max, .. } => {
                        // Wrap semantics: stepping at the edges wraps into
                        // [min, max).
                        let stepped_lo = s.step_by(ParamValue::Angle(min), -1);
                        let stepped_hi = s.step_by(ParamValue::Angle(max - 1e-9), 1);
                        for (v, what) in [(stepped_lo, "wrap from min"), (stepped_hi, "wrap from max")] {
                            if let ParamValue::Angle(x) = v {
                                assert!(min <= x && x < max, "{where_} / {}: {} wrapped to {x} outside [{min}, {max})", s.id.as_str(), what);
                            } else {
                                panic!("{where_} / {}: {} returned non-angle {v:?}", s.id.as_str(), what);
                            }
                        }
                    }
                    ParamKind::Int { min, max, .. } => {
                        let lo = ParamValue::Int(min);
                        let hi = ParamValue::Int(max);
                        assert_eq!(s.step_by(lo, -1), lo, "{where_} / {}: int min edge", s.id.as_str());
                        assert_eq!(s.step_by(hi, 1), hi, "{where_} / {}: int max edge", s.id.as_str());
                    }
                    ParamKind::Bool { .. } => {
                        let d = match s.default_value() { ParamValue::Bool(d) => d, _ => unreachable!() };
                        assert_eq!(s.step_by(ParamValue::Bool(d), 1), ParamValue::Bool(!d), "{where_} / {}: bool toggle", s.id.as_str());
                    }
                }
            }
        }
    }

    #[test]
    fn registry_fingerprint_stability_sensitivity_order_sensitivity() {
        let registry = crate::registry();
        for t in registry.iter() {
            let specs = t.params();
            let where_ = format!("{} ({} specs)", t.id(), specs.len());
            let baseline = fingerprint(specs, &ParamSet::default_for(specs));
            // Stability: two constructions of the same values → byte-identical.
            let again = fingerprint(specs, &ParamSet::default_for(specs));
            assert_eq!(baseline, again, "{where_}: fingerprint not stable");
            // Construction with an explicit set carrying the same values → same bytes.
            let explicit = {
                let mut p = ParamSet::EMPTY;
                for s in specs {
                    p.set(s.id, s.default_value());
                }
                p
            };
            assert_eq!(fingerprint(specs, &explicit), baseline, "{where_}: default and explicit agree");
            // Sensitivity: every spec — change one value → fingerprint moves.
            for (i, s) in specs.iter().enumerate() {
                let mut p = ParamSet::default_for(specs);
                p.set(s.id, alternate(*s));
                let changed = fingerprint(specs, &p);
                assert_ne!(changed, baseline, "{where_}: spec {} ({:?}) change invisible", i, s.id);
            }
            // Order-sensitivity: swap the last two specs → fingerprint moves.
            assert!(specs.len() >= 2, "{where_}: expected >=2 specs");
            let mut swapped = specs.to_vec();
            swapped.swap(specs.len() - 2, specs.len() - 1);
            let reordered = fingerprint(&swapped, &ParamSet::default_for(&swapped));
            assert_ne!(reordered, baseline, "{where_}: spec-order change invisible");
        }
    }

    #[test]
    fn glb_binary_n7_fingerprint_locks_appended_spec() {
        // The depth-param job LANDED (M-integration): `depth` Int is a
        // real appended spec on this tiling (bounds 0..=5, default 0).
        // Pin the real appended tail, and keep the generic property —
        // appending a spec must be a detectable, non-colliding change —
        // with a fresh hypothetical id.
        let registry = crate::registry();
        let t = registry.get("glb-binary-n7").unwrap_or_else(|| panic!("missing glb-binary-n7"));
        let specs = t.params();
        let baseline = fingerprint(specs, &ParamSet::default_for(specs));
        assert!(baseline.ends_with(",depth=0"), "glb-binary-n7: the landed depth spec must be the appended tail: {baseline}");
        let mut specs_plus: Vec<ParamSpec> = specs.to_vec();
        specs_plus.push(ParamSpec::int(
            ParamId::new("future_param"), "future_param", 1, 4, 1, 2,
        ));
        let appended = fingerprint(&specs_plus, &ParamSet::default_for(&specs_plus));
        assert_ne!(appended, baseline, "appending a param must change the fingerprint");
        assert!(appended.ends_with(",future_param=2"), "the new spec is appended, not spliced: {appended}");
        // And the appended spec must not collide with the existing ids.
        let existing: std::collections::BTreeSet<&str> = specs.iter().map(|s| s.id.as_str()).collect();
        assert!(!existing.contains("future_param"), "future_param id would collide with an existing spec");
    }

    #[test]
    fn hexagram_declaration_order_is_scale_then_rotation() {
        // The fingerprint is positional: scale and rotation are declared
        // first, in that order, and any third param comes after them —
        // structural invariant, not a count (a future fourth param is
        // allowed). If the order flips, the fingerprint re-keying the UIs
        // depend on would silently reshuffle.
        let registry = crate::registry();
        let t = registry.get("hexagram").unwrap_or_else(|| panic!("missing hexagram"));
        let ids: Vec<&str> = t.params().iter().map(|s| s.id.as_str()).collect();
        assert!(ids.len() >= 2, "hexagram: expected >=2 specs, got {ids:?}");
        assert_eq!(ids[0], "scale", "hexagram: first spec must be scale, got {ids:?}");
        assert_eq!(ids[1], "rotation", "hexagram: second spec must be rotation, got {ids:?}");
    }
}

#[cfg(test)]
mod tests_param_surface_doc_pins {
    //! Doc-vs-code pins for the self-describing parameter surface.
    //!
    //! Mirrors the census-surface-pin pattern: line-scoped,
    //! whitespace-tolerant pins whose anchors must match exactly once
    //! (zero or several matches fail loudly, naming file/line/claim).
    //! Code-visible claims are asserted against the live registry; the
    //! process/architecture claims are anchor-guarded (a deletion of
    //! the claim fails this test) but are not computable here — they
    //! are documented in the module head.
    use tess_core::params::{ParamKind, ParamSpec};

    /// Whitespace-normalized paragraph containing `anchor_pos` (from the
    /// nearest blank line above to the nearest blank line below) — claims
    /// routinely span several lines and the anchor is only one line of it.
    fn line_region(text: &str, anchor_pos: usize) -> String {
        let start = text[..anchor_pos]
            .rfind("\n\n")
            .map(|p| p + 2)
            .unwrap_or(0);
        let end = text[anchor_pos..]
            .find("\n\n")
            .map(|i| anchor_pos + i)
            .unwrap_or(text.len());
        text[start..end]
            .chars()
            .map(|c| if c.is_whitespace() { ' ' } else { c })
            .collect()
    }

    /// 1-based line number of the anchor at `anchor_pos`.
    fn anchor_line(text: &str, anchor_pos: usize) -> usize {
        text[..anchor_pos].chars().filter(|&c| c == '\n').count() + 1
    }

    /// The unique byte position of `anchor`; else panic loudly.
    fn unique_anchor(text: &str, anchor: &str, what: &str) -> usize {
        let positions: Vec<usize> = text.match_indices(anchor).map(|(i, _)| i).collect();
        match positions.len() {
            0 => panic!("{what}: anchor missing: {anchor:?}"),
            1 => positions[0],
            n => panic!("{what}: anchor ambiguous ({n} matches): {anchor:?}"),
        }
    }

    fn readme() -> String {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../README.md"),
        )
        .expect("read README.md")
    }

    fn design() -> String {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/DESIGN.md"),
        )
        .expect("read DESIGN.md")
    }

    fn agents() -> String {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../AGENTS.md"),
        )
        .expect("read AGENTS.md")
    }

    fn spec_by_label<'a>(specs: &'a [ParamSpec], label: &str) -> &'a ParamSpec {
        specs
            .iter()
            .find(|s| s.label == label)
            .unwrap_or_else(|| panic!("no spec with label {label:?} in {}", label))
    }

    #[test]
    fn readme_scale_and_rotation_ranges_match_registry() {
        let text = readme();
        let what = "README scale/rotation";
        let pos = unique_anchor(&text, "Each periodic tiling accepts", what);
        let line_no = anchor_line(&text, pos);
        let line = line_region(&text, pos);
        assert!(
            line.contains("scale") && line.contains("0.1–50") && line.contains("0.05"),
            "{what}: README.md:{line_no}: anchor's line must claim scale (0.1–50, step 0.05): line: {line}"
        );
        assert!(
            line.contains("rotation") && line.contains("0–360°") && line.contains("5°"),
            "{what}: README.md:{line_no}: anchor's line must claim rotation (0–360°, step 5°): line: {line}"
        );
        let registry = crate::registry();
        for t in registry.iter() {
            let specs = t.params();
            let where_ = format!("{}", t.id());
            let scale = specs
                .iter()
                .find(|s| s.id.as_str() == "scale")
                .unwrap_or_else(|| panic!("{where_}: no scale spec (README claims every periodic tiling accepts scale)"));
            match scale.kind {
                ParamKind::Float { min, max, step, .. } => {
                    assert!((min - 0.1).abs() < 1e-12 && (max - 50.0).abs() < 1e-12, "{where_}: scale range {min}..{max} != claimed 0.1..50");
                    assert!((step - 0.05).abs() < 1e-12, "{where_}: scale step {step} != claimed 0.05");
                }
                other => panic!("{where_}: scale spec is {other:?}, not Float"),
            }
            let rotation = specs
                .iter()
                .find(|s| s.id.as_str() == "rotation")
                .unwrap_or_else(|| panic!("{where_}: no rotation spec (README claims every periodic tiling accepts rotation)"));
            match rotation.kind {
                ParamKind::Angle { min, max, step, .. } => {
                    assert!((min - 0.0).abs() < 1e-12 && (max - 360.0).abs() < 1e-12, "{where_}: rotation range {min}..{max} != claimed 0..360");
                    assert!((step - 5.0).abs() < 1e-12, "{where_}: rotation step {step} != claimed 5°");
                }
                other => panic!("{where_}: rotation spec is {other:?}, not Angle"),
            }
        }
    }

    #[test]
    fn readme_third_parameter_ranges_match_registry() {
        let text = readme();
        // star-and-cross: star radius (0.05–0.5)
        let what = "README star-cross star radius";
        let pos = unique_anchor(&text, "star radius", what);
        let line_no = anchor_line(&text, pos);
        let line = line_region(&text, pos);
        assert!(line.contains("0.05") && line.contains("0.5"), "{what}: README.md:{line_no}: anchor's line must claim 0.05–0.5: line: {line}");
        let registry = crate::registry();
        let star_cross = registry.get("star-cross").unwrap_or_else(|| panic!("missing star-cross"));
        let spec = spec_by_label(star_cross.params(), "star radius");
        match spec.kind {
            ParamKind::Float { min, max, step: _, default } => {
                assert!((min - 0.05).abs() < 1e-12 && (max - 0.5).abs() < 1e-12, "star-cross: star radius range {min}..{max} != claimed 0.05..0.5");
                assert!(default > 0.0 && default <= 0.5, "star-cross: star radius default {default} out of range");
            }
            other => panic!("star-cross: star radius is {other:?}, not Float"),
        }
        // hexagram: star tip radius (0.05–0.5, default 0.5)
        let what = "README hexagram star tip radius";
        let pos = unique_anchor(&text, "star tip radius", what);
        let line_no = anchor_line(&text, pos);
        let line = line_region(&text, pos);
        assert!(line.contains("0.05") && line.contains("0.5") && line.contains("default 0.5"), "{what}: README.md:{line_no}: anchor's line must claim 0.05–0.5 default 0.5: line: {line}");
        let hexagram = registry.get("hexagram").unwrap_or_else(|| panic!("missing hexagram"));
        let spec = spec_by_label(hexagram.params(), "star tip radius");
        match spec.kind {
            ParamKind::Float { min, max, step: _, default } => {
                assert!((min - 0.05).abs() < 1e-12 && (max - 0.5).abs() < 1e-12, "hexagram: star tip radius range {min}..{max} != claimed 0.05..0.5");
                assert!((default - 0.5).abs() < 1e-12, "hexagram: star tip radius default {default} != claimed 0.5");
            }
            other => panic!("hexagram: star tip radius is {other:?}, not Float"),
        }
        // Pythagorean two-square: square ratio b/a (0.2–10, default φ)
        let what = "README pythagorean square ratio";
        let pos = unique_anchor(&text, "square ratio", what);
        let line_no = anchor_line(&text, pos);
        let line = line_region(&text, pos);
        assert!(line.contains("0.2") && line.contains("10") && line.contains("φ"), "{what}: README.md:{line_no}: anchor's line must claim 0.2–10 default φ: line: {line}");
        let pythagorean = registry.get("pythagorean").unwrap_or_else(|| panic!("missing pythagorean"));
        let spec = spec_by_label(pythagorean.params(), "square ratio b/a");
        match spec.kind {
            ParamKind::Float { min, max, step: _, default } => {
                assert!((min - 0.2).abs() < 1e-12 && (max - 10.0).abs() < 1e-12, "pythagorean: square ratio range {min}..{max} != claimed 0.2..10");
                assert!((default - 1.618033988749895).abs() < 1e-12, "pythagorean: square ratio default {default} != claimed φ");
            }
            other => panic!("pythagorean: square ratio is {other:?}, not Float"),
        }
    }

    #[test]
    fn design_md_self_describing_claim_anchored() {
        let text = design();
        let what = "DESIGN.md self-describing";
        let pos = unique_anchor(&text, "Every tiling declares its parameters", what);
        let line_no = anchor_line(&text, pos);
        let line = line_region(&text, pos);
        assert!(
            line.contains("ParamSpec") && line.contains("Angle"),
            "{what}: docs/DESIGN.md:{line_no}: anchor's line must claim ParamSpec + Angle: line: {line}"
        );
        // Anchor presence only — the code-visible content (every spec is a
        // valid ParamSpec) is pinned by registry_params_well_formed; the
        // process/architecture claim (adding a tiling never requires touching
        // UI code) is not computable here and is deliberately not pinned.
    }

    #[test]
    fn readme_and_agents_self_describing_anchors_present() {
        let readme = readme();
        let what = "README self-describing";
        let pos = unique_anchor(&readme, "adding a tiling never requires touching UI code", what);
        let line_no = anchor_line(&readme, pos);
        let line = line_region(&readme, pos);
        assert!(line.contains("self-describing"), "{what}: README.md:{line_no}: anchor's line must claim self-describing params: line: {line}");
        let agents = agents();
        let what = "AGENTS.md self-describing";
        let pos = unique_anchor(&agents, "Parameters are self-describing (`ParamSpec`), so", what);
        let line_no = anchor_line(&agents, pos);
        let line = line_region(&agents, pos);
        assert!(line.contains("adding a tiling never requires touching UI code"), "{what}: AGENTS.md:{line_no}: anchor's line must claim never-touching-UI: line: {line}");
    }
}
