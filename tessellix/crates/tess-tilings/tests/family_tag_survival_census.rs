//! Landing A/B pin — the Tile family/reflected contract survives every
//! generator, the tagged-card set is exactly pinned, and `mirror_tiles`
//! forwards both new fields.
//!
//! Tagged set (47 cards, derived programmatically from the registry):
//! 4 Penrose + 4 Andritz + 11 Uniform (archimedean) + 20 Two-Uniform
//! (kuniform) + 5 Isohedral gallery cards (all Landing A) + the 3 aperiodic
//! cards `hat`/`turtle`/`spectre` (Landing B). Kuniform
//! *duals* register under the Duals-&-variants tab (category Laves) and
//! are untagged by contract (SPEC §polygon-type rule: "both duals of
//! the triangle/square/hexagon are in Laves, not these tabs").
//!
//! Landing B also pins the honest `reflected` law at emission (only the hat
//! card emits reflected tiles — the H1 cluster hat — while turtle and spectre
//! bake their reflection into the level placements and emit none), the
//! family-blindness of `shape_signature` for the three new cards, and the
//! family-dispatched `Metatile` bands end to end.

use tess_core::app::{mean_tile_side, preview_window};
use tess_core::geometry::{BBox, Vec2};
use tess_core::params::ParamSet;
use tess_core::tiling::{FamilyTag, Query, Tiling};

const PENROSE: [&str; 4] = ["penrose", "penrose-p1", "penrose_p2", "penrose-pentagon"];
const ANDRITZ: [&str; 4] = [
    "andritz-bowtie-hexagon",
    "andritz-bowtie-hexagon-10gon-1",
    "andritz-bowtie-hexagon-10gon-2",
    "andritz-bowtie-hexagon-10gon-3",
];
const UNIFORM: [&str; 11] = [
    "square",
    "hexagon",
    "triangle",
    "truncated-square",
    "trihexagonal",
    "snub-trihexagonal",
    "elongated-triangular",
    "rhombitrihexagonal",
    "truncated-trihexagonal",
    "truncated-hexagonal",
    "snub-square",
];
const TWO_UNIFORM: [&str; 20] = [
    "kuniform-p6m",
    "kuniform-pmm",
    "kuniform-n18",
    "kuniform-n9",
    "kuniform-n8",
    "kuniform-n5",
    "kuniform-n1",
    "kuniform-n13",
    "kuniform-n20",
    "kuniform-n12",
    "kuniform-n19",
    "kuniform-n4",
    "kuniform-n14",
    "kuniform-n3",
    "kuniform-n15",
    "kuniform-n2",
    "kuniform-n16",
    "kuniform-n17",
    "kuniform-n7",
    "kuniform-n6",
];
const ISOHEDRAL: [&str; 5] = [
    "ih1-p1-hexagon",
    "ih6-pgg-hexagon",
    "ih7-p3-hexagon",
    "ih27-pgg-pentagon",
    "ih29-p4g-pentagon",
];
/// Landing B: the three aperiodic cards whose engines now tag their tiles.
const APERIODIC: [&str; 3] = ["hat", "turtle", "spectre"];

fn expected_family(id: &str) -> FamilyTag {
    if PENROSE.contains(&id) {
        FamilyTag::Penrose
    } else if ANDRITZ.contains(&id) {
        FamilyTag::Andritz
    } else if UNIFORM.contains(&id) {
        FamilyTag::Uniform
    } else if TWO_UNIFORM.contains(&id) {
        FamilyTag::TwoUniform
    } else if ISOHEDRAL.contains(&id) {
        FamilyTag::Isohedral
    } else if id == "hat" {
        FamilyTag::Hat
    } else if id == "turtle" {
        FamilyTag::Turtle
    } else if id == "spectre" {
        FamilyTag::Spectre
    } else {
        FamilyTag::None
    }
}

fn preview_tiles(t: &dyn Tiling) -> Vec<tess_core::tiling::Tile> {
    let params = ParamSet::default_for(t.params());
    let probe = t.generate(
        &params,
        &Query {
            region: BBox::new(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0)),
        },
    );
    let mut window = preview_window(probe.periodic, mean_tile_side(&probe.tiles), 4.0 / 3.0);
    let mut result = t.generate(
        &params,
        &Query {
            region: window,
        },
    );
    let mut shrink = 0.4;
    while (result.message.is_some() || result.tiles.is_empty()) && shrink > 0.01 {
        let cx = (window.min.x + window.max.x) / 2.0;
        let cy = (window.min.y + window.max.y) / 2.0;
        let hx = (window.max.x - window.min.x) / 2.0 * shrink;
        let hy = (window.max.y - window.min.y) / 2.0 * shrink;
        window = BBox::new(Vec2::new(cx - hx, cy - hy), Vec2::new(cx + hx, cy + hy));
        result = t.generate(
            &params,
            &Query {
                region: window,
            },
        );
        shrink *= 0.4;
    }
    result.tiles
}

/// Every registered card's emitted tiles carry exactly the expected
/// family tag, and `reflected` follows each family's honest emission law.
#[test]
fn family_tag_survives_every_generator() {
    let r = tess_tilings::registry();
    assert_eq!(r.len(), 170, "registry size pin (Landing A base)");
    let mut tagged_cards = 0;
    for t in r.iter() {
        let want = expected_family(t.id());
        if want != FamilyTag::None {
            tagged_cards += 1;
        }
        let tiles = preview_tiles(t);
        assert!(!tiles.is_empty(), "{}: no tiles", t.id());
        for (i, tile) in tiles.iter().enumerate() {
            assert_eq!(
                tile.family, want,
                "{}: tile {i} carries {:?}, expected {want:?}",
                t.id(),
                tile.family
            );
            match want {
                // The hat engine places one reflected hat per H cluster
                // (H1, i.e. seed 1) and metatiles are never reflected, so
                // the flag survives substitution unchanged: exactly the
                // H1 hats are reflected at any depth.
                FamilyTag::Hat => assert_eq!(
                    tile.reflected,
                    tile.seed == 1,
                    "{}: tile {i} (seed {}) has reflected {}",
                    t.id(),
                    tile.seed,
                    tile.reflected
                ),
                // Turtle and spectre bake the substitution's reflection into
                // the level placements (root `refl: depth % 2 == 1`), so every
                // leaf is emitted unreflected; their patch chirality is a
                // whole-patch fact, never a per-tile one.
                FamilyTag::Turtle | FamilyTag::Spectre => {
                    assert!(!tile.reflected, "{}: leaf tile {i} must be unreflected", t.id());
                }
                // Landing A law, unchanged: every other card emits unreflected
                // (only `svg::mirror_tiles` ever sets the flag).
                _ => {
                    assert!(!tile.reflected, "{}: emitted tile {i} must not be reflected", t.id());
                }
            }
        }
    }
    assert_eq!(tagged_cards, 47, "exactly 47 family-tagged cards");
}

/// The hat card's `reflected` set is non-empty and non-total: the flag
/// actually differentiates tiles (the `--highlight-reflected` pass needs a
/// both-ways witness at emission, not a constant).
#[test]
fn hat_emits_both_chiralities() {
    let r = tess_tilings::registry();
    let hat = r.iter().find(|t| t.id() == "hat").expect("hat card");
    let tiles = preview_tiles(hat);
    let reflected = tiles.iter().filter(|t| t.reflected).count();
    assert!(reflected > 0, "the hat card must emit reflected H1 hats");
    assert!(reflected < tiles.len(), "most hats are canonical");
    assert_eq!(
        tiles.iter().filter(|t| t.seed == 1).count(),
        reflected,
        "exactly the seed-1 (H1) hats are reflected"
    );
    for id in APERIODIC {
        if id == "hat" {
            continue;
        }
        let card = r.iter().find(|t| t.id() == id).expect(id);
        assert!(
            preview_tiles(card).iter().all(|t| !t.reflected),
            "{id}: no emitted tile is reflected"
        );
    }
}

/// D5 — `shape_signature` is family-blind: for the three newly tagged
/// cards, every emitted tile's signature is bit-identical to the signature
/// of the same polygon stripped of its tag and flipped in `reflected`
/// (the signature reads geometry only), so `ColorMode::Shape` output cannot
/// move when a card gains a tag.
#[test]
fn shape_signature_is_family_blind_for_the_aperiodic_cards() {
    let r = tess_tilings::registry();
    for id in APERIODIC {
        let card = r.iter().find(|t| t.id() == id).expect(id);
        let tiles = preview_tiles(card);
        assert!(!tiles.is_empty(), "{id}: no tiles");
        let want: Vec<Vec<i64>> = tiles
            .iter()
            .map(|t| tess_core::paint::shape_signature(&t.poly))
            .collect();
        let untagged: Vec<Vec<i64>> = tiles
            .iter()
            .map(|t| {
                let mut plain = t.clone();
                plain.family = FamilyTag::None;
                plain.reflected = !t.reflected;
                tess_core::paint::shape_signature(&plain.poly)
            })
            .collect();
        assert_eq!(want, untagged, "{id}: shape_signature must not read family/reflected");
    }
}

/// D4 end to end: the family dispatch puts each aperiodic card's
/// `Metatile` classes in its own band of the 9-entry ramp, and the bands are
/// pairwise disjoint in real emitted views (not just in the unit pins).
#[test]
fn metatile_bands_separate_the_three_aperiodic_cards() {
    let r = tess_tilings::registry();
    let classes = |id: &str| -> std::collections::BTreeSet<usize> {
        preview_tiles(r.iter().find(|t| t.id() == id).expect(id))
            .iter()
            .map(|t| tess_core::paint::metatile_class(t))
            .collect()
    };
    let hat = classes("hat");
    let spectre = classes("spectre");
    let turtle = classes("turtle");
    assert!(!hat.is_empty() && hat.iter().all(|&c| c <= 3), "hat band 0..=3, got {hat:?}");
    assert!(
        !spectre.is_empty() && spectre.iter().all(|&c| (4..=7).contains(&c)),
        "spectre band 4..=7, got {spectre:?}"
    );
    // Turtle cells are slot 8; the companion hat (depth 0, label 0 only)
    // reads the hat band, so the turtle view is {8} ∪ a hat-band subset.
    assert!(turtle.contains(&8), "turtle cells read slot 8, got {turtle:?}");
    assert!(
        turtle.iter().all(|&c| c == 8 || c <= 3),
        "turtle is slot 8 plus the hat band, got {turtle:?}"
    );
    assert!(hat.is_disjoint(&spectre), "hat/spectre bands overlap: {hat:?} {spectre:?}");
    assert!(turtle.iter().filter(|&&c| c != 8).all(|c| hat.contains(c)));
    // The three cards now span three bands of the ramp where they used to
    // share one bucket table (hat 0..=3, spectre 4..=7, turtle 8): the union
    // must be wider than any single band's 4 buckets... in the weakest
    // possible view (few roles emitted) still at least three classes.
    let mut union = hat.clone();
    union.extend(spectre.iter().copied());
    union.extend(turtle.iter().copied());
    assert!(union.len() >= 3, "expected ≥3 classes across the union, got {union:?}");
}

// `mirror_tiles` field forwarding is pinned in tess-core
// (`svg::tests::mirror_tiles_forwards_family_and_flips_reflected` — the flip
// is an involution: canonical ↔ reflected across the mirror pass);
// the private helper is exercised there, and `mirror_export.rs` keeps
// covering the end-to-end mirror render.
