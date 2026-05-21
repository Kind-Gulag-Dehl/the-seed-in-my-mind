pub(crate) const GOLDEN_ANGLE: f64 = 2.399_963_229_728_653;
pub(crate) const DEFAULT_SPACING: f64 = 300.0;
pub(crate) const DEFAULT_THETA_JITTER: f64 = 0.12;
pub(crate) const DEFAULT_RADIAL_JITTER: f64 = DEFAULT_SPACING * 0.10;
pub(crate) const DEFAULT_MIN_DISTANCE: f64 = DEFAULT_SPACING * 0.80;
pub(crate) const DEFAULT_RELAXATION_ITERATIONS: usize = 8;
pub(crate) const DEFAULT_RELAXATION_TETHER: f64 = 0.05;
pub(crate) const DEFAULT_RELAXATION_MAX_POINTS: usize = 600;

#[derive(Clone, Copy, Debug)]
pub(crate) struct CoordinateScatterConfig {
    pub(crate) spacing: f64,
    pub(crate) theta_jitter: f64,
    pub(crate) radial_jitter: f64,
    pub(crate) min_distance: f64,
    pub(crate) relaxation_iterations: usize,
    pub(crate) relaxation_tether: f64,
    pub(crate) relaxation_max_points: usize,
}

impl Default for CoordinateScatterConfig {
    fn default() -> Self {
        Self {
            spacing: DEFAULT_SPACING,
            theta_jitter: DEFAULT_THETA_JITTER,
            radial_jitter: DEFAULT_RADIAL_JITTER,
            min_distance: DEFAULT_MIN_DISTANCE,
            relaxation_iterations: DEFAULT_RELAXATION_ITERATIONS,
            relaxation_tether: DEFAULT_RELAXATION_TETHER,
            relaxation_max_points: DEFAULT_RELAXATION_MAX_POINTS,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CoordinateSlotInput {
    pub(crate) id: String,
    pub(crate) slot_idx: usize,
    pub(crate) hash_key: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoordinatePoint {
    pub(crate) id: String,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CoordinateProjection {
    pub(crate) points: Vec<CoordinatePoint>,
    pub(crate) relaxed: bool,
}

#[derive(Clone, Debug)]
struct RelaxablePoint {
    id: String,
    x: f64,
    y: f64,
    target_radius: f64,
}

pub(crate) fn fnv1a_64(value: &str) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    let mut hash = OFFSET_BASIS;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

fn signed_unit_from_u32(value: u32) -> f64 {
    ((value as f64) / (u32::MAX as f64)) * 2.0 - 1.0
}

fn hash_jitter(hash_key: &str) -> (f64, f64) {
    let hash = fnv1a_64(hash_key);
    let lower = hash as u32;
    let upper = (hash >> 32) as u32;
    let mixed = upper ^ lower.rotate_left(7);
    (
        signed_unit_from_u32(lower),
        signed_unit_from_u32(mixed.rotate_left(11)),
    )
}

pub(crate) fn project_slots(
    slots: &[CoordinateSlotInput],
    config: CoordinateScatterConfig,
) -> CoordinateProjection {
    let mut points = slots
        .iter()
        .map(|slot| {
            let k = slot.slot_idx as f64;
            let theta = k * GOLDEN_ANGLE;
            let radius = (k + 1.0).sqrt() * config.spacing;
            let (j1, j2) = hash_jitter(&slot.hash_key);
            let theta_prime = theta + j1 * config.theta_jitter;
            let radius_prime = (radius + j2 * config.radial_jitter).max(0.0);

            RelaxablePoint {
                id: slot.id.clone(),
                x: radius_prime * theta_prime.cos(),
                y: radius_prime * theta_prime.sin(),
                target_radius: radius_prime,
            }
        })
        .collect::<Vec<_>>();

    let relaxed = if points.len() > 1 && points.len() <= config.relaxation_max_points {
        relax_points(&mut points, config);
        true
    } else {
        false
    };

    CoordinateProjection {
        points: points
            .into_iter()
            .map(|point| CoordinatePoint {
                id: point.id,
                x: point.x,
                y: point.y,
            })
            .collect(),
        relaxed,
    }
}

fn relax_points(points: &mut [RelaxablePoint], config: CoordinateScatterConfig) {
    const EPSILON: f64 = 1e-9;

    for _ in 0..config.relaxation_iterations {
        for left_idx in 0..points.len() {
            for right_idx in (left_idx + 1)..points.len() {
                let dx = points[right_idx].x - points[left_idx].x;
                let dy = points[right_idx].y - points[left_idx].y;
                let distance = dx.hypot(dy);
                if distance >= config.min_distance {
                    continue;
                }

                let overlap = config.min_distance - distance;
                let (unit_x, unit_y) = if distance > EPSILON {
                    (dx / distance, dy / distance)
                } else {
                    let angle = (left_idx + right_idx + 1) as f64 * GOLDEN_ANGLE;
                    (angle.cos(), angle.sin())
                };
                let delta = overlap / 2.0;

                points[left_idx].x -= unit_x * delta;
                points[left_idx].y -= unit_y * delta;
                points[right_idx].x += unit_x * delta;
                points[right_idx].y += unit_y * delta;
            }
        }

        for point in points.iter_mut() {
            let current_radius = point.x.hypot(point.y);
            if current_radius <= EPSILON {
                continue;
            }
            let next_radius =
                current_radius + (point.target_radius - current_radius) * config.relaxation_tether;
            let scale = next_radius / current_radius;
            point.x *= scale;
            point.y *= scale;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fnv1a_hash_matches_known_vector() {
        assert_eq!(fnv1a_64("hello"), 0xa430_d846_80aa_bd0b);
    }

    #[test]
    fn coordinate_projection_matches_fixed_numeric_snapshot() {
        let slots = vec![
            CoordinateSlotInput {
                id: "idea-a".to_string(),
                slot_idx: 0,
                hash_key: "idea:idea-a:birth:1:0".to_string(),
            },
            CoordinateSlotInput {
                id: "idea-b".to_string(),
                slot_idx: 1,
                hash_key: "idea:idea-b:birth:1:1".to_string(),
            },
            CoordinateSlotInput {
                id: "idea-c".to_string(),
                slot_idx: 2,
                hash_key: "edge:edge-c:idx:1:2:idea:idea-c".to_string(),
            },
        ];

        let coords = project_slots(&slots, CoordinateScatterConfig::default());
        assert!(coords.relaxed);
        let expected = [
            ("idea-a", 297.5841171483545, 3.0565943668073907),
            ("idea-b", -295.2776980483248, 263.369792287459),
            ("idea-c", 97.14846217609131, -504.51314094876864),
        ];

        let epsilon = 1e-9;
        assert_eq!(coords.points.len(), expected.len());
        for (coord, (expected_id, expected_x, expected_y)) in
            coords.points.iter().zip(expected.iter())
        {
            assert_eq!(coord.id, *expected_id);
            assert!(
                (coord.x - expected_x).abs() <= epsilon,
                "x mismatch for {}: expected {}, got {}",
                coord.id,
                expected_x,
                coord.x
            );
            assert!(
                (coord.y - expected_y).abs() <= epsilon,
                "y mismatch for {}: expected {}, got {}",
                coord.id,
                expected_y,
                coord.y
            );
        }
    }

    #[test]
    fn coordinate_projection_skips_relaxation_above_cap() {
        let slots = (0..601)
            .map(|slot_idx| CoordinateSlotInput {
                id: format!("idea-{slot_idx}"),
                slot_idx,
                hash_key: format!("idea:idea-{slot_idx}:birth:1:{slot_idx}"),
            })
            .collect::<Vec<_>>();

        let coords = project_slots(&slots, CoordinateScatterConfig::default());
        assert!(!coords.relaxed);
        assert_eq!(coords.points.len(), slots.len());
    }
}
