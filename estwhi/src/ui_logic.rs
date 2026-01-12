use rand::Rng;

#[derive(Clone, Debug)]
pub struct RandomThingsConfig {
    pub enabled: bool,
    pub icon_twirl_enabled: bool,
    pub multiplier: i32,
    pub count: usize,
    pub interval_ms: u32,
}

impl Default for RandomThingsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            icon_twirl_enabled: true,
            multiplier: 6,
            count: 4,
            interval_ms: 200,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RandomThingState {
    pub x: i32,
    pub y: i32,
    pub bitmap_index: usize,
}

#[derive(Default)]
pub struct RandomThings {
    pub config: RandomThingsConfig,
    pub things: Vec<RandomThingState>,
    pub random_timer_active: bool,
    pub icon_timer_active: bool,
    pub icon_count: usize,
}

impl RandomThings {
    pub fn validate_and_fix_config(&mut self) {
        self.config.multiplier = self.config.multiplier.clamp(1, 20);
        self.config.count = self.config.count.clamp(1, 4);
        self.config.interval_ms = self.config.interval_ms.clamp(20, 1000);
    }

    pub fn resize_things(&mut self, client_width: i32, client_height: i32) {
        let current_count = self.things.len();
        let new_count = self.config.count;

        match new_count.cmp(&current_count) {
            std::cmp::Ordering::Greater => {
                for i in current_count..new_count {
                    // Initialize in the table area (left side), not center of whole window
                    // The table is roughly 2/3 of width and most of height
                    let safe_x = (client_width / 3).max(100);
                    let safe_y = (client_height / 3).max(100);
                    self.things.push(RandomThingState {
                        x: safe_x,
                        y: safe_y,
                        bitmap_index: i % 4,
                    });
                }
            }
            std::cmp::Ordering::Less => {
                self.things.truncate(new_count);
            }
            std::cmp::Ordering::Equal => {}
        }
    }

    // Extracted math for updating positions (random walk + bounds check + logo collision)
    // Returns true if position changed (always true here essentially)
    pub fn update_positions(
        &mut self,
        bounds: (i32, i32, i32, i32),    // left, top, right, bottom
        logo_rect: (i32, i32, i32, i32), // left, top, right, bottom
        thing_size: i32,
        rng: &mut impl Rng,
    ) {
        let mult = self.config.multiplier;

        for thing in &mut self.things {
            let dx = mult * rng.gen_range(-1..=1);
            let dy = mult * rng.gen_range(-1..=1);
            thing.x += dx;
            thing.y += dy;

            // Constrain to table area boundaries
            if thing.x < bounds.0 {
                thing.x = bounds.0;
            }
            if thing.x > bounds.2 - thing_size {
                thing.x = bounds.2 - thing_size;
            }
            if thing.y < bounds.1 {
                thing.y = bounds.1;
            }
            if thing.y > bounds.3 - thing_size {
                thing.y = bounds.3 - thing_size;
            }

            // Avoid logo
            // Logic from main.rs: if inside logo rect, push out
            // main.rs logic checks center? No, checks top-left.
            // Check intersection of Thing rect and Logo rect?
            // main.rs logic:
            /*
            if thing.x >= logo_left
                && thing.x <= logo_right
                && thing.y >= logo_top
                && thing.y <= logo_bottom
            */
            // This treats logo_rect as an inclusive "forbidden zone" for the thing's top-left corner.
            if thing.x >= logo_rect.0
                && thing.x <= logo_rect.2
                && thing.y >= logo_rect.1
                && thing.y <= logo_rect.3
            {
                // Push to nearest edge of logo area
                let logo_x = logo_rect.0 + 31;
                let logo_y = logo_rect.1 + 31;
                let logo_left = logo_rect.0;
                let logo_right = logo_rect.2;
                let logo_top = logo_rect.1;
                let logo_bottom = logo_rect.3;

                if thing.x < logo_x {
                    thing.x = logo_left;
                } else {
                    thing.x = logo_right;
                }
                if thing.y < logo_y {
                    thing.y = logo_top;
                } else {
                    thing.y = logo_bottom;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let mut things = RandomThings::default();
        things.config.multiplier = 100;
        things.config.count = 100;
        things.config.interval_ms = 0;

        things.validate_and_fix_config();

        assert_eq!(things.config.multiplier, 20);
        assert_eq!(things.config.count, 4);
        assert_eq!(things.config.interval_ms, 20);
    }

    #[test]
    fn test_resize() {
        let mut things = RandomThings::default();
        things.config.count = 2;
        things.resize_things(1000, 1000);
        assert_eq!(things.things.len(), 2);

        things.config.count = 4;
        things.resize_things(1000, 1000);
        assert_eq!(things.things.len(), 4);

        things.config.count = 1;
        things.resize_things(1000, 1000);
        assert_eq!(things.things.len(), 1);
    }

    #[test]
    fn test_update_positions_bounds() {
        let mut things = RandomThings::default();
        things.config.multiplier = 10; // Large steps
        things.things.push(RandomThingState {
            x: 0,
            y: 0,
            bitmap_index: 0,
        });

        let bounds = (0, 0, 100, 100);
        let logo = (-100, -100, -50, -50); // Far away
        let thing_size = 10;

        let mut rng = rand::thread_rng();

        // Run updates
        for _ in 0..100 {
            things.update_positions(bounds, logo, thing_size, &mut rng);
            let t = &things.things[0];
            // Verify bounds
            assert!(t.x >= bounds.0);
            assert!(t.x <= bounds.2 - thing_size);
            assert!(t.y >= bounds.1);
            assert!(t.y <= bounds.3 - thing_size);
        }
    }

    #[test]
    fn test_update_positions_collision() {
        let mut things = RandomThings::default();
        things.config.multiplier = 1;
        // Place thing inside logo rect
        let logo = (50, 50, 100, 100);
        // Center of logo zone is approx 50 + 31 = 81.
        // 75 is < 81, so it should snap left/top.
        things.things.push(RandomThingState {
            x: 75,
            y: 75,
            bitmap_index: 0,
        });

        let bounds = (0, 0, 200, 200);
        let thing_size = 10;
        let mut rng = rand::thread_rng();

        things.update_positions(bounds, logo, thing_size, &mut rng);

        let t = &things.things[0];
        // The logic snaps to the edge. Since collision check is inclusive, it remains "on" the edge.
        // We verify it moved to the expected corner.
        assert_eq!(t.x, 50);
        assert_eq!(t.y, 50);
    }
}
