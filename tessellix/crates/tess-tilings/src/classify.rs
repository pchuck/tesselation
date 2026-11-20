//! The per-tiling classification table: [`classification`] maps a
//! registry id to its declared [`TilingClass`] — the optional,
//! per-family display facts the UIs render (only the fields that are
//! present; a Penrose card shows no wallpaper group, a pentagon card
//! shows no vertex configuration, and nothing renders as "N/A").
//!
//! Classification is *declared*, not derived: like the primary
//! category it states the taxonomy honestly (a nonperiodic
//! realization of a tile that also admits periodic tilings must not
//! claim quasiperiodicity). The table lives here, beside the tilings
//! it classifies; `tess-core` only defines the [`TilingClass`] shape
//! and the rendering rules (`tess_core::browse::attr_lines`), and both
//! UIs inject `classification` into
//! [`tess_core::browse::browse_tabs_with`], which also drives the
//! declared tab sort orders (pentagon-type sequence, 2-uniform vertex
//! configurations, Laves ranks, wallpaper crystal systems).
//!
//! Conventions:
//! - `vertex_config` / `face_config` use the registry's notation
//!   (superscript counts, `;` between vertex orbits, `V` prefix on a
//!   Laves face configuration). For the two 2-uniform pairs that
//!   share a configuration, the disambiguating subscript (₁/₂) is part
//!   of the configuration, matching the card names (M113).
//! - `dual_of` names the dual partner (short form; "self-dual" for
//!   the regular trio).
//! - `contact_angle` and `inflation` are display-ready strings
//!   ("θK 45°", "1+√2", "λ = 2").
//! - `laves_rank` orders the Laves tab: 0 = self-dual regular, 1 =
//!   Laves proper, 2 = 2-uniform dual.
//! - `pentagon_type` orders the Pentagon tab (types 1–15; cairo = 4
//!   and floret = 5 join as secondary members).

use tess_core::classify::TilingClass;

/// The declared classification facts for the tiling registered under
/// `id` (empty when the id has no entry — the coverage test below
/// fails on any registered tiling missing from this table).
pub fn classification(id: &str) -> TilingClass {
    match id {
        // ---- Uniform (11) -------------------------------------------------
        "square" => TilingClass {
            families: &["Uniform", "Laves"],
            vertex_config: Some("4.4.4.4"),
            schlafli: Some("{4,4}"),
            uniformity: Some("regular"),
            dual_of: Some("self-dual"),
            k: Some(1),
            laves_rank: Some(0),
            ..TilingClass::EMPTY
        },
        "triangle" => TilingClass {
            families: &["Uniform", "Laves"],
            vertex_config: Some("3.3.3.3.3.3"),
            schlafli: Some("{3,6}"),
            uniformity: Some("regular"),
            dual_of: Some("self-dual"),
            k: Some(1),
            laves_rank: Some(0),
            ..TilingClass::EMPTY
        },
        "hexagon" => TilingClass {
            families: &["Uniform", "Laves", "Wallpaper"],
            vertex_config: Some("6.6.6"),
            schlafli: Some("{6,3}"),
            uniformity: Some("regular"),
            dual_of: Some("self-dual"),
            k: Some(1),
            laves_rank: Some(0),
            ..TilingClass::EMPTY
        },
        "truncated-square" => TilingClass {
            families: &["Uniform", "Wallpaper"],
            vertex_config: Some("4.8.8"),
            schlafli: Some("t{4,4}"),
            uniformity: Some("semiregular"),
            dual_of: Some("tetrakis square"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "truncated-hexagonal" => TilingClass {
            families: &["Uniform"],
            vertex_config: Some("3.12.12"),
            schlafli: Some("t{6,3}"),
            uniformity: Some("semiregular"),
            dual_of: Some("triakis triangular"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "truncated-trihexagonal" => TilingClass {
            families: &["Uniform"],
            vertex_config: Some("4.6.12"),
            schlafli: Some("tr{3,6}"),
            uniformity: Some("semiregular"),
            dual_of: Some("kisrhombille"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "rhombitrihexagonal" => TilingClass {
            families: &["Uniform"],
            vertex_config: Some("3.4.6.4"),
            schlafli: Some("rr{3,6}"),
            uniformity: Some("semiregular"),
            dual_of: Some("deltoidal trihexagonal"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "trihexagonal" => TilingClass {
            families: &["Uniform"],
            vertex_config: Some("3.6.3.6"),
            schlafli: Some("r{3,6}"),
            uniformity: Some("semiregular"),
            dual_of: Some("rhombille"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "elongated-triangular" => TilingClass {
            families: &["Uniform", "Wallpaper"],
            vertex_config: Some("3.3.3.4.4"),
            uniformity: Some("semiregular"),
            dual_of: Some("prismatic pentagonal"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "snub-square" => TilingClass {
            families: &["Uniform", "Wallpaper"],
            vertex_config: Some("3.3.4.3.4"),
            schlafli: Some("s{4,4}"),
            uniformity: Some("semiregular"),
            dual_of: Some("cairo pentagonal"),
            k: Some(1),
            ..TilingClass::EMPTY
        },
        "snub-trihexagonal" => TilingClass {
            families: &["Uniform", "Wallpaper"],
            vertex_config: Some("3.3.3.3.6"),
            schlafli: Some("sr{3,6}"),
            uniformity: Some("semiregular"),
            dual_of: Some("floret pentagonal"),
            k: Some(1),
            chirality: Some("homochiral"),
            ..TilingClass::EMPTY
        },

        // ---- Two-Uniform (15) ---------------------------------------------
        "kuniform-p6m" => two_uniform(&["Two-Uniform"], "(3⁶; 3².6²)"),
        "kuniform-pmm" => two_uniform(&["Two-Uniform", "Wallpaper"], "(3.6.3.6; 3².6²)"),
        "kuniform-n18" => two_uniform(&["Two-Uniform"], "(3⁶; 3².4.3.4)"),
        "kuniform-n9" => two_uniform(&["Two-Uniform"], "(3.4.6.4; 3².4.3.4)"),
        "kuniform-n8" => two_uniform(&["Two-Uniform"], "(3.4.6.4; 3³.4²)"),
        "kuniform-n5" => two_uniform(&["Two-Uniform"], "(3.4.6.4; 3.4².6)"),
        "kuniform-n1" => two_uniform(&["Two-Uniform"], "(4.6.12; 3.4.6.4)"),
        "kuniform-n13" => two_uniform(&["Two-Uniform"], "(3⁶; 3².4.12)"),
        "kuniform-n20" => two_uniform(&["Two-Uniform"], "(3⁶; 3⁴.6)₁"),
        "kuniform-n12" => two_uniform(&["Two-Uniform"], "(3².6²; 3⁴.6)"),
        "kuniform-n19" => two_uniform(&["Two-Uniform"], "(3⁶; 3⁴.6)₂"),
        "kuniform-n4" => two_uniform(&["Two-Uniform"], "(4⁴; 3³.4²)₁"),
        "kuniform-n14" => two_uniform(&["Two-Uniform"], "(3⁶; 3³.4²)₁"),
        "kuniform-n3" => two_uniform(&["Two-Uniform"], "(4⁴; 3³.4²)₂"),
        "kuniform-n15" => two_uniform(&["Two-Uniform"], "(3⁶; 3³.4²)₂"),
        "kuniform-n2" => two_uniform(&["Two-Uniform"], "(3.12.12; 3.4.3.12)"),
        "kuniform-n16" => two_uniform(&["Two-Uniform"], "(3³.4²; 3².4.3.4)"),

        // ---- Laves proper (8, incl. cairo) --------------------------------
        "tetrakis-square" => laves(&["Laves"], "V4.8.8", "truncated square"),
        "rhombille" => laves(&["Laves"], "V3.6.3.6", "trihexagonal"),
        "triakis-triangular" => laves(&["Laves"], "V3.12.12", "truncated hexagonal"),
        "deltoidal-trihexagonal" => laves(&["Laves"], "V3.4.6.4", "rhombitrihexagonal"),
        "kisrhombille" => laves(&["Laves"], "V4.6.12", "truncated trihexagonal"),
        "prismatic-pentagonal" => laves(&["Laves"], "V3.3.3.4.4", "elongated triangular"),
        "floret-pentagonal" => TilingClass {
            pentagon_type: Some(5),
            isohedral: Some(1),
            ..laves(&["Laves", "Pentagon"], "V3.3.3.3.6", "snub trihexagonal")
        },
        "cairo-pentagonal" => TilingClass {
            pentagon_type: Some(4),
            isohedral: Some(1),
            ..laves(&["Laves", "Pentagon"], "V3.3.4.3.4", "snub square")
        },

        // ---- 2-uniform duals (11) — Laves rank 2 ---------------------------
        "kuniform-dual-p6m" => kuniform_dual("V3⁶; V3².6²", "(3⁶; 3².6²)"),
        "kuniform-dual-n18" => kuniform_dual("V3⁶; V3².4.3.4", "(3⁶; 3².4.3.4)"),
        "kuniform-dual-n9" => kuniform_dual("V3.4.6.4; V3².4.3.4", "(3.4.6.4; 3².4.3.4)"),
        "kuniform-dual-n8" => kuniform_dual("V3.4.6.4; V3³.4²", "(3.4.6.4; 3³.4²)"),
        "kuniform-dual-n5" => kuniform_dual("V3.4.6.4; V3.4².6", "(3.4.6.4; 3.4².6)"),
        "kuniform-dual-n1" => kuniform_dual("V4.6.12; V3.4.6.4", "(4.6.12; 3.4.6.4)"),
        "kuniform-dual-n13" => kuniform_dual("V3⁶; V3².4.12", "(3⁶; 3².4.12)"),
        "kuniform-dual-n20" => kuniform_dual("V3⁶; V3⁴.6", "(3⁶; 3⁴.6)₁"),
        "kuniform-dual-pmm" => kuniform_dual("V3.6.3.6; V3².6²", "(3.6.3.6; 3².6²)"),
        "kuniform-dual-n12" => kuniform_dual("V3².6²; V3⁴.6", "(3².6²; 3⁴.6)"),
        "kuniform-dual-n19" => kuniform_dual("V3⁶; V3⁴.6", "(3⁶; 3⁴.6)₂"),
        "kuniform-dual-n2" => kuniform_dual("V3.4.3.12; V3.12.12", "(3.12.12; 3.4.3.12)"),
        "kuniform-dual-n16" => kuniform_dual("V3².4.3.4; V3³.4²", "(3³.4²; 3².4.3.4)"),

        // ---- Pentagon types (13 primaries; cairo = 4, floret = 5 above) ----
        "pentagon-1" => pentagon(&["Pentagon"], 1, 1, "Reinhardt", "1918", Some("achiral")),
        "pentagon-2" => pentagon(&["Pentagon"], 2, 1, "Reinhardt", "1918", Some("mixed")),
        "pentagon-3" => pentagon(&["Pentagon", "Wallpaper"], 3, 1, "Reinhardt", "1918", Some("homochiral")),
        "pentagon-6" => pentagon(&["Pentagon"], 6, 2, "Kershner", "1968", Some("achiral")),
        "pentagon-7" => pentagon(&["Pentagon"], 7, 2, "Kershner", "1968", Some("mixed")),
        "pentagon-8" => pentagon(&["Pentagon"], 8, 2, "Kershner", "1968", Some("mixed")),
        "pentagon-9" => pentagon(&["Pentagon"], 9, 2, "Rice", "1977", None),
        "pentagon-10" => pentagon(&["Pentagon"], 10, 3, "James", "1975", None),
        "pentagon-11" => pentagon(&["Pentagon"], 11, 2, "Rice", "1977", None),
        "pentagon-12" => pentagon(&["Pentagon"], 12, 2, "Rice", "1977", None),
        "pentagon-13" => pentagon(&["Pentagon"], 13, 2, "Rice", "1977", Some("mixed")),
        "pentagon-14" => pentagon(&["Pentagon"], 14, 3, "Stein", "1985", None),
        "pentagon-15" => pentagon(&["Pentagon"], 15, 3, "Mann et al.", "2015", None),
        "hirschhorn" => TilingClass {
            families: &["Pentagon", "Nonperiodic"],
            source: Some("Hirschhorn & Hunt"),
            year: Some("1985"),
            translational: Some("non-periodic"),
            ..TilingClass::EMPTY
        },

        // ---- Wallpaper catalog primaries (8) -------------------------------
        "stepped-hexominoes" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("staircase hexomino"),
            cell_content: Some("1 hexomino per cell"),
            ..TilingClass::EMPTY
        },
        "trapezoid-courses" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("two incongruent trapezoids"),
            cell_content: Some("4 trapezoids per cell"),
            ..TilingClass::EMPTY
        },
        "glide-hexagon-strips" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("fat + waist hexagons"),
            cell_content: Some("2 hexagons per cell"),
            ..TilingClass::EMPTY
        },
        "shifted-trapezoid-courses" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("two incongruent trapezoids"),
            cell_content: Some("8 trapezoids per cell"),
            ..TilingClass::EMPTY
        },
        "wavy-courses" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("two incongruent triangles"),
            cell_content: Some("4 triangles per cell"),
            ..TilingClass::EMPTY
        },
        "split-triangle-grid" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("inner triangle + 3 corner quads"),
            cell_content: Some("8 tiles per cell"),
            ..TilingClass::EMPTY
        },
        "chiral-cut-honeycomb" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("chiral cut hexagon"),
            cell_content: Some("3 tiles per cell"),
            chirality: Some("homochiral"),
            ..TilingClass::EMPTY
        },
        "alcove-cut-grid" => TilingClass {
            families: &["Wallpaper"],
            tile_set: Some("alcove-cut triangles"),
            cell_content: Some("18 tiles per cell"),
            ..TilingClass::EMPTY
        },

        // ---- Islamic (21) ---------------------------------------------------
        "star-cross" => TilingClass {
            families: &["Islamic"],
            method: Some("star-and-cross"),
            star_type: Some("{8/3}"),
            tile_set: Some("octagram stars + concave crosses"),
            ..TilingClass::EMPTY
        },
        "girih-shesh-band" => TilingClass {
            families: &["Islamic"],
            method: Some("girih strapwork"),
            tile_set: Some("shesh-band hexagon"),
            vertex_config: Some("6.6.6"),
            ..TilingClass::EMPTY
        },
        "girih-star-cross" => TilingClass {
            families: &["Islamic"],
            method: Some("girih strapwork"),
            tile_set: Some("10-pointed star and cross"),
            star_type: Some("{10/3}"),
            ..TilingClass::EMPTY
        },
        "girih-w85" => TilingClass {
            families: &["Islamic"],
            method: Some("Wichmann panel data"),
            tile_set: Some("12-pointed star interlace"),
            ..TilingClass::EMPTY
        },
        "girih-quasiperiodic" => TilingClass {
            families: &["Islamic", "Quasiperiodic"],
            method: Some("Darb-i Imam substitution"),
            inflation: Some("2φ³"),
            point_order: Some(10),
            prototiles: Some(3),
            source: Some("Lu & Steinhardt"),
            year: Some("2007"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "pic-4-8-8" => hankin_pic("4.8.8", "θK 45°"),
        "pic-3-6-3-6" => hankin_pic("3.6.3.6", "θK 60°"),
        "pic-3-12-12" => hankin_pic("3.12.12", "θK 60°"),
        "hankin-4-8-8-rosette" => hankin_greedy("4.8.8", "θK 67.5°", &[]),
        "hankin-4-6-12" => hankin_greedy("4.6.12", "θK 60°", &[]),
        "hankin-hex-two-point" => TilingClass {
            families: &["Islamic"],
            method: Some("Kaplan two-point"),
            base_tiling: Some("6.6.6"),
            contact_angle: Some("θK 45°, δ = 1/2"),
            ..TilingClass::EMPTY
        },
        "hankin-square-two-point" => TilingClass {
            families: &["Islamic"],
            method: Some("Kaplan two-point"),
            base_tiling: Some("4.4.4.4"),
            contact_angle: Some("θK 45°, δ = 1/2"),
            ..TilingClass::EMPTY
        },
        "hankin-4-8-8-rosette-transform" => TilingClass {
            families: &["Islamic"],
            method: Some("rosette transform + Hankin inference"),
            base_tiling: Some("4.8.8"),
            contact_angle: Some("θK 45°"),
            ..TilingClass::EMPTY
        },
        "hankin-3-6-3-6" => hankin_greedy("3.6.3.6", "θK 60°", &["{6/2}"]),
        "hankin-3-4-6-4" => hankin_greedy("3.4.6.4", "θK 60°", &["{6/2}"]),
        "hankin-3-3-3-4-4" => hankin_greedy("3.3.3.4.4", "θK 60°", &[]),
        "hankin-3-3-4-3-4" => hankin_greedy("3.3.4.3.4", "θK 60°", &[]),
        "hankin-3-12-12" => hankin_greedy("3.12.12", "θK 45°", &[]),
        "khatam-girih" => TilingClass {
            families: &["Islamic"],
            method: Some("girih decomposition"),
            star_type: Some("{8/3}"),
            tile_set: Some("kissing khatam star-and-cross"),
            ..TilingClass::EMPTY
        },
        "najm-63-2e-0-3e-ac" => TilingClass {
            families: &["Islamic"],
            method: Some("Najm template"),
            base_tiling: Some("[6,3]; 2e, 0, 3e (AC)"),
            tile_set: Some("dodecagon + enneagon substrate"),
            ..TilingClass::EMPTY
        },
        "najm-44-3e-0-0-a" => TilingClass {
            families: &["Islamic"],
            method: Some("Najm template"),
            base_tiling: Some("[4,4]; 3e, 0, 0 (A)"),
            tile_set: Some("dodecagon + concave 4-point stars"),
            ..TilingClass::EMPTY
        },
        "rosette-transformed-4-8-8" => TilingClass {
            families: &["Islamic"],
            method: Some("rosette transform (Kaplan §3.8)"),
            base_tiling: Some("4.8.8"),
            ..TilingClass::EMPTY
        },
        "hexagram" => TilingClass {
            // The name and description already carry "{6/2}"; the
            // schema adds nothing here and the card falls back to its
            // description fragment.
            families: &["Islamic"],
            ..TilingClass::EMPTY
        },

        // ---- Arrangements (5) -----------------------------------------------
        "running-bond" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("1 domino per cell"),
            ..TilingClass::EMPTY
        },
        "stack-bond" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("1 domino per cell"),
            ..TilingClass::EMPTY
        },
        "herringbone-45" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("2 dominoes per cell"),
            chirality: Some("homochiral"),
            ..TilingClass::EMPTY
        },
        "basketweave-3" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("15 dominoes per cell"),
            ..TilingClass::EMPTY
        },
        "basketweave" => TilingClass {
            families: &["Arrangement"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("4 dominoes per cell"),
            ..TilingClass::EMPTY
        },
        "herringbone" => TilingClass {
            families: &["Arrangement", "Wallpaper"],
            method: Some("paver bond"),
            tile_set: Some("2:1 dominoes"),
            cell_content: Some("2 dominoes per cell"),
            chirality: Some("homochiral"),
            ..TilingClass::EMPTY
        },
        "windmill" => TilingClass {
            families: &["Arrangement", "Wallpaper"],
            tile_set: Some("1–2–√5 right triangles"),
            chirality: Some("homochiral"),
            ..TilingClass::EMPTY
        },
        "pythagorean" => TilingClass {
            families: &["Arrangement"],
            method: Some("Pythagorean dissection"),
            tile_set: Some("two squares per cell"),
            ..TilingClass::EMPTY
        },

        // ---- Quasiperiodic (10) ----------------------------------------------
        "ammann-beenker" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("1+√2"),
            cut_project: Some("R⁴ → R², octagonal window"),
            point_order: Some(8),
            source: Some("Ammann & Beenker"),
            year: Some("1982"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "shield" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("√(2+√3)"),
            point_order: Some(12),
            source: Some("Gähler"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "stampfli" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("2+√3"),
            point_order: Some(12),
            source: Some("Stampfli"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "golden-bee" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("√φ"),
            source: Some("Vince"),
            year: Some("2014"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "fractalizing-3-12-12" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("fractalizing substitution"),
            base_tiling: Some("3.12.12"),
            inflation: Some("2+√3"),
            point_order: Some(12),
            translational: Some("limit-periodic"),
            ..TilingClass::EMPTY
        },
        "penrose" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("φ"),
            point_order: Some(5),
            source: Some("Penrose"),
            year: Some("1974"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "penrose-p1" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("pentagon deflation"),
            inflation: Some("φ²"),
            point_order: Some(5),
            source: Some("Penrose"),
            year: Some("1974"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "penrose_p2" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("half-tile substitution"),
            inflation: Some("φ"),
            point_order: Some(5),
            source: Some("Penrose"),
            year: Some("1974"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },
        "penrose-pentagon" => TilingClass {
            families: &["Quasiperiodic"],
            method: Some("substitution"),
            inflation: Some("φ"),
            point_order: Some(5),
            tile_set: Some("five pentagons"),
            chirality: Some("homochiral"),
            translational: Some("quasiperiodic"),
            ..TilingClass::EMPTY
        },

        // ---- Aperiodic (6) ----------------------------------------------------
        "pinwheel" => TilingClass {
            families: &["Aperiodic"],
            method: Some("substitution"),
            inflation: Some("√5"),
            source: Some("Conway & Radin"),
            year: Some("1994"),
            aperiodic_status: Some("dense tile orientations"),
            tile_set: Some("1–2–√5 right triangle"),
            translational: Some("non-periodic"),
            ..TilingClass::EMPTY
        },
        "hat" => TilingClass {
            families: &["Aperiodic"],
            method: Some("metatile substitution"),
            inflation: Some("φ²"),
            prototiles: Some(1),
            reflections: Some(true),
            chirality: Some("mixed"),
            source: Some("Smith et al."),
            year: Some("2023"),
            aperiodic_status: Some("einstein"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },
        "turtle" => TilingClass {
            families: &["Aperiodic"],
            method: Some("9-label substitution"),
            prototiles: Some(1),
            chirality: Some("homochiral"),
            source: Some("Smith et al."),
            year: Some("2023"),
            aperiodic_status: Some("einstein"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },
        "spectre" => TilingClass {
            families: &["Aperiodic"],
            method: Some("9-label substitution"),
            prototiles: Some(1),
            chirality: Some("homochiral"),
            source: Some("Smith et al."),
            year: Some("2023"),
            aperiodic_status: Some("chiral einstein"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },
        "taylor-socolar" => TilingClass {
            families: &["Aperiodic"],
            method: Some("substitution"),
            prototiles: Some(1),
            source: Some("Taylor & Socolar"),
            year: Some("2010"),
            aperiodic_status: Some("strongly aperiodic"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },
        "jeandel-rao" => TilingClass {
            families: &["Aperiodic"],
            method: Some("Wang tiles"),
            prototiles: Some(11),
            source: Some("Jeandel & Rao"),
            year: Some("2015"),
            aperiodic_status: Some("strongly aperiodic"),
            tile_set: Some("Wang tile set"),
            translational: Some("aperiodic"),
            ..TilingClass::EMPTY
        },

        // ---- Nonperiodic (4; hirschhorn above) --------------------------------
        "chair" => rep_tile("L-tromino"),
        "sphinx" => rep_tile("sphinx hexiamond"),
        "p-pentomino" => rep_tile("P-pentomino"),

        _ => TilingClass::EMPTY,
    }
}

/// A 2-uniform tiling class: `k = 2` and the pair of vertex
/// configurations (with the ₁/₂ subscript when a configuration pair
/// needs disambiguating, matching the M113 card names).
fn two_uniform(families: &'static [&'static str], vertex_config: &'static str) -> TilingClass {
    TilingClass {
        families,
        vertex_config: Some(vertex_config),
        k: Some(2),
        ..TilingClass::EMPTY
    }
}

/// A Laves-proper class: face configuration, dual partner, rank 1.
fn laves(
    families: &'static [&'static str],
    face_config: &'static str,
    dual_of: &'static str,
) -> TilingClass {
    TilingClass {
        families,
        face_config: Some(face_config),
        dual_of: Some(dual_of),
        laves_rank: Some(1),
        ..TilingClass::EMPTY
    }
}

/// A machine-dualized 2-uniform dual: V-face configuration of both
/// vertex orbits, primal reference, rank 2.
fn kuniform_dual(face_config: &'static str, primal: &'static str) -> TilingClass {
    TilingClass {
        families: &["Laves"],
        face_config: Some(face_config),
        dual_of: Some(primal),
        laves_rank: Some(2),
        k: Some(2),
        ..TilingClass::EMPTY
    }
}

/// A pentagon-type class: type number (the Pentagon-tab sort key),
/// isohedral class, attribution.
fn pentagon(
    families: &'static [&'static str],
    pentagon_type: u8,
    isohedral: u8,
    source: &'static str,
    year: &'static str,
    chirality: Option<&'static str>,
) -> TilingClass {
    TilingClass {
        families,
        pentagon_type: Some(pentagon_type),
        isohedral: Some(isohedral),
        source: Some(source),
        year: Some(year),
        chirality,
        ..TilingClass::EMPTY
    }
}

/// A Hankin polygons-in-contact entry.
fn hankin_pic(base_tiling: &'static str, contact_angle: &'static str) -> TilingClass {
    TilingClass {
        families: &["Islamic"],
        method: Some("Hankin polygons-in-contact"),
        base_tiling: Some(base_tiling),
        contact_angle: Some(contact_angle),
        ..TilingClass::EMPTY
    }
}

/// A Hankin greedy-inference entry (optional star polygon).
fn hankin_greedy(base_tiling: &'static str, contact_angle: &'static str, star: &'static [&'static str]) -> TilingClass {
    TilingClass {
        families: &["Islamic"],
        method: Some("Hankin greedy inference"),
        base_tiling: Some(base_tiling),
        contact_angle: Some(contact_angle),
        star_type: star.first().copied(),
        ..TilingClass::EMPTY
    }
}

/// A limit-periodic rep-tile realization (chair, sphinx, P-pentomino).
fn rep_tile(tile_set: &'static str) -> TilingClass {
    TilingClass {
        families: &["Nonperiodic"],
        method: Some("substitution"),
        inflation: Some("λ = 2"),
        prototiles: Some(1),
        tile_set: Some(tile_set),
        translational: Some("limit-periodic"),
        ..TilingClass::EMPTY
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tess_core::category::TilingCategory;

    /// Every registered tiling must carry a non-empty class — this is
    /// the guard that forces a classify-table entry whenever a new
    /// tiling registers (and the reason the "hide absent fields" UI
    /// rule never degrades to a bare description).
    #[test]
    fn classification_covers_every_registered_tiling() {
        for t in crate::registry().iter() {
            let class = classification(t.id());
            assert!(
                !class.is_empty(),
                "{} ({}) has no classification entry",
                t.id(),
                t.name()
            );
            assert!(
                !class.families.is_empty(),
                "{} declares no families",
                t.id()
            );
        }
    }

    /// The Pentagon tab sort key: types 1–15 all declared (type 1 is
    /// M117, pending integration at the time this table shipped),
    /// cairo = 4 and floret = 5 as secondary members.
    #[test]
    fn pentagon_types_are_declared_sequentially() {
        for (id, ty) in [
            ("cairo-pentagonal", 4),
            ("floret-pentagonal", 5),
            ("pentagon-2", 2),
            ("pentagon-3", 3),
            ("pentagon-6", 6),
            ("pentagon-7", 7),
            ("pentagon-8", 8),
            ("pentagon-9", 9),
            ("pentagon-10", 10),
            ("pentagon-11", 11),
            ("pentagon-12", 12),
            ("pentagon-13", 13),
            ("pentagon-14", 14),
            ("pentagon-15", 15),
        ] {
            assert_eq!(classification(id).pentagon_type, Some(ty), "{id}");
        }
    }

    /// Every Laves-tab member declares a rank (0 self-dual regular,
    /// 1 Laves proper, 2 2-uniform dual) and a face configuration or
    /// Schläfli symbol.
    #[test]
    fn laves_members_declare_ranks() {
        for t in crate::registry().iter() {
            let member = t.category() == TilingCategory::Laves
                || t.secondary_categories().contains(&TilingCategory::Laves);
            if !member {
                continue;
            }
            let class = classification(t.id());
            assert!(class.laves_rank.is_some(), "{} lacks laves_rank", t.id());
            assert!(
                class.face_config.is_some() || class.schlafli.is_some(),
                "{} lacks a face configuration",
                t.id()
            );
        }
    }

    /// Every 2-uniform primary declares its vertex configuration (the
    /// Two-Uniform tab sort key) and k = 2.
    #[test]
    fn two_uniform_primaries_declare_configs() {
        for t in crate::registry().iter() {
            if t.category() != TilingCategory::TwoUniform {
                continue;
            }
            let class = classification(t.id());
            assert!(
                class.vertex_config.is_some(),
                "{} lacks vertex_config",
                t.id()
            );
            assert_eq!(class.k, Some(2), "{}", t.id());
        }
    }
}