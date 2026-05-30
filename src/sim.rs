use serde::Serialize;

const WORLD_HALF_WIDTH: f64 = 60.0;
const STEER_STEP: f64 = 1.0;
const START_SPEED: f64 = 1.0;
const NORMAL_CAP: f64 = 7.0;
const BOOST_CAP: f64 = 10.5;
const NORMAL_ACCEL: f64 = 0.12;
const BOOST_ACCEL: f64 = 0.28;
const COLLISION_X_RADIUS: f64 = 1.25;
const MONSTER_SPAWN_DISTANCE: f64 = 2000.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SkierMode {
    Skiing,
    Crashed,
    Airborne,
    Eaten,
}

impl SkierMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Skiing => "skiing",
            Self::Crashed => "crashed",
            Self::Airborne => "airborne",
            Self::Eaten => "eaten",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MonsterMode {
    Chasing,
    Fleeing,
}

impl MonsterMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Chasing => "chasing",
            Self::Fleeing => "fleeing",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Input {
    pub steer: i32,
    pub boost: bool,
    pub jump: bool,
}

#[derive(Clone, Debug)]
pub struct Game {
    seed: i64,
    tick: u64,
    x: f64,
    y: f64,
    speed: f64,
    mode: SkierMode,
    distance_m: f64,
    style: f64,
    crash_ticks: u8,
    recovery_freeze_ticks: u8,
    airborne_ticks: u8,
    monster: Option<Monster>,
    game_over: bool,
}

#[derive(Clone, Debug)]
struct Monster {
    x: f64,
    y: f64,
    mode: MonsterMode,
}

#[derive(Clone, Debug)]
struct FieldObstacle {
    kind: &'static str,
    x: f64,
    y: f64,
}

#[derive(Serialize)]
pub struct GameState {
    skier: SkierState,
    #[serde(rename = "distanceM")]
    distance_m: f64,
    style: f64,
    obstacles: Vec<ObstacleState>,
    monster: Option<MonsterState>,
    #[serde(rename = "gameOver")]
    game_over: bool,
    tick: u64,
}

#[derive(Serialize)]
struct SkierState {
    x: f64,
    y: f64,
    speed: f64,
    mode: &'static str,
}

#[derive(Serialize)]
struct ObstacleState {
    #[serde(rename = "type")]
    kind: &'static str,
    x: f64,
    y: f64,
}

#[derive(Serialize)]
struct MonsterState {
    x: f64,
    y: f64,
    mode: &'static str,
}

impl Game {
    pub fn new(seed: i64) -> Self {
        Self {
            seed,
            tick: 0,
            x: 0.0,
            y: 0.0,
            speed: START_SPEED,
            mode: SkierMode::Skiing,
            distance_m: 0.0,
            style: 0.0,
            crash_ticks: 0,
            recovery_freeze_ticks: 0,
            airborne_ticks: 0,
            monster: None,
            game_over: false,
        }
    }

    pub fn state(&self) -> GameState {
        GameState {
            skier: SkierState {
                x: rounded(self.x),
                y: rounded(self.y),
                speed: rounded(self.speed),
                mode: self.mode.as_str(),
            },
            distance_m: rounded(self.distance_m),
            style: rounded(self.style),
            obstacles: self
                .obstacles_in_window((self.y - 12.0).max(0.0), self.y + 420.0)
                .into_iter()
                .map(|obstacle| ObstacleState {
                    kind: obstacle.kind,
                    x: rounded(obstacle.x),
                    y: rounded(obstacle.y),
                })
                .collect(),
            monster: self.monster.as_ref().map(|monster| MonsterState {
                x: rounded(monster.x),
                y: rounded(monster.y),
                mode: monster.mode.as_str(),
            }),
            game_over: self.game_over,
            tick: self.tick,
        }
    }

    pub fn step(&mut self, input: Input) {
        self.tick += 1;

        if self.mode == SkierMode::Eaten || self.game_over {
            self.mode = SkierMode::Eaten;
            self.speed = 0.0;
            self.update_monster();
            return;
        }

        match self.mode {
            SkierMode::Crashed => self.step_crashed(),
            SkierMode::Airborne => self.step_airborne(input),
            SkierMode::Skiing => self.step_skiing(input),
            SkierMode::Eaten => {}
        }

        self.spawn_monster_if_needed();
        self.update_monster();
    }

    fn step_skiing(&mut self, input: Input) {
        self.x = wrap_x(self.x + input.steer as f64 * STEER_STEP);

        if self.recovery_freeze_ticks > 0 {
            self.recovery_freeze_ticks -= 1;
            self.speed = 0.0;
            return;
        }

        let prev_y = self.y;
        let cap = if input.boost { BOOST_CAP } else { NORMAL_CAP };
        let accel = if input.boost {
            BOOST_ACCEL
        } else {
            NORMAL_ACCEL
        };
        self.speed = (self.speed + accel).min(cap);
        if !input.boost && self.speed > NORMAL_CAP {
            self.speed = (self.speed - 0.18).max(NORMAL_CAP);
        }
        self.advance_downhill();

        if let Some(obstacle) = self.collided_obstacle(prev_y, self.y) {
            match obstacle.kind {
                "ramp" if input.jump => {
                    self.mode = SkierMode::Airborne;
                    self.airborne_ticks = 5;
                    self.speed = (self.speed + 0.35).min(BOOST_CAP);
                }
                "tree" | "bigtree" | "stump" | "rock" => self.crash(),
                _ => {}
            }
        }
    }

    fn step_crashed(&mut self) {
        self.speed = 0.0;
        self.crash_ticks += 1;
        if self.crash_ticks >= 3 {
            self.mode = SkierMode::Skiing;
            self.crash_ticks = 0;
            self.recovery_freeze_ticks = 3;
        }
    }

    fn step_airborne(&mut self, input: Input) {
        self.x = wrap_x(self.x + input.steer as f64 * STEER_STEP * 0.75);
        self.speed = (self.speed + 0.04).min(BOOST_CAP);
        self.advance_downhill();

        if self.airborne_ticks > 0 {
            self.airborne_ticks -= 1;
        }
        if self.airborne_ticks == 0 {
            self.mode = SkierMode::Skiing;
            self.style += if input.jump { 12.0 } else { 8.0 };
        }
    }

    fn advance_downhill(&mut self) {
        self.y += self.speed;
        self.distance_m = self.y;
    }

    fn crash(&mut self) {
        self.mode = SkierMode::Crashed;
        self.speed = 0.0;
        self.crash_ticks = 0;
        self.recovery_freeze_ticks = 0;
        self.style -= 5.0;
    }

    fn spawn_monster_if_needed(&mut self) {
        if self.monster.is_none() && self.distance_m >= MONSTER_SPAWN_DISTANCE {
            self.monster = Some(Monster {
                x: wrap_x(self.x + 32.0),
                y: self.y - 90.0,
                mode: MonsterMode::Chasing,
            });
        }
    }

    fn update_monster(&mut self) {
        let Some(monster) = &mut self.monster else {
            return;
        };

        match monster.mode {
            MonsterMode::Chasing => {
                let dx = shortest_x_delta(monster.x, self.x);
                let dy = self.y - monster.y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance <= 9.5 {
                    monster.x = self.x;
                    monster.y = self.y;
                    monster.mode = MonsterMode::Fleeing;
                    self.mode = SkierMode::Eaten;
                    self.game_over = true;
                    self.speed = 0.0;
                    return;
                }

                let step = 10.5_f64.min(distance);
                monster.x = wrap_x(monster.x + dx / distance * step);
                monster.y += dy / distance * step;
            }
            MonsterMode::Fleeing => {
                let dx = shortest_x_delta(self.x, monster.x);
                let dy = monster.y - self.y;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance <= f64::EPSILON {
                    monster.y -= 8.0;
                } else {
                    monster.x = wrap_x(monster.x + dx / distance * 8.0);
                    monster.y += dy / distance * 8.0;
                }
            }
        }
    }

    fn collided_obstacle(&self, prev_y: f64, current_y: f64) -> Option<FieldObstacle> {
        self.obstacles_in_window(prev_y + 0.001, current_y + 0.001)
            .into_iter()
            .find(|obstacle| {
                obstacle.y > prev_y
                    && obstacle.y <= current_y + 0.001
                    && (self.x - obstacle.x).abs() <= COLLISION_X_RADIUS
            })
    }

    fn obstacles_in_window(&self, min_y: f64, max_y: f64) -> Vec<FieldObstacle> {
        let mut obstacles = Vec::new();
        let lane = self.seed_lane();
        let start_band = ((min_y - 160.0) / 220.0).floor().max(0.0) as i32;
        let end_band = ((max_y + 260.0) / 220.0).ceil().max(0.0) as i32;

        for band in start_band..=end_band {
            let base = band as f64;
            let crash_y = 90.0 + base * 220.0;
            if crash_y >= min_y && crash_y <= max_y {
                obstacles.push(FieldObstacle {
                    kind: self.crash_kind(band),
                    x: 7.0 + lane,
                    y: crash_y,
                });
            }

            let ramp_y = 135.0 + base * 240.0;
            if ramp_y >= min_y && ramp_y <= max_y {
                obstacles.push(FieldObstacle {
                    kind: "ramp",
                    x: -7.0 + lane,
                    y: ramp_y,
                });
            }

            let side_y = 185.0 + base * 220.0;
            if side_y >= min_y && side_y <= max_y {
                obstacles.push(FieldObstacle {
                    kind: "mogul",
                    x: 18.0 - lane,
                    y: side_y,
                });
            }
        }

        obstacles.sort_by(|a, b| {
            a.y.partial_cmp(&b.y)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.kind.cmp(b.kind))
        });
        obstacles
    }

    fn seed_lane(&self) -> f64 {
        let mixed = (self.seed as i128)
            .wrapping_mul(1_103_515_249)
            .wrapping_add(12_347);
        let lane = mixed.rem_euclid(5) as f64 - 2.0;
        lane * 0.5
    }

    fn crash_kind(&self, band: i32) -> &'static str {
        match (self.seed + band as i64).rem_euclid(4) {
            0 => "tree",
            1 => "stump",
            2 => "rock",
            _ => "bigtree",
        }
    }
}

fn wrap_x(x: f64) -> f64 {
    if x > WORLD_HALF_WIDTH {
        x - WORLD_HALF_WIDTH * 2.0
    } else if x < -WORLD_HALF_WIDTH {
        x + WORLD_HALF_WIDTH * 2.0
    } else {
        x
    }
}

fn shortest_x_delta(from: f64, to: f64) -> f64 {
    let direct = to - from;
    if direct > WORLD_HALF_WIDTH {
        direct - WORLD_HALF_WIDTH * 2.0
    } else if direct < -WORLD_HALF_WIDTH {
        direct + WORLD_HALF_WIDTH * 2.0
    } else {
        direct
    }
}

fn rounded(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steering_and_distance_advance() {
        let mut game = Game::new(2);
        game.step(Input {
            steer: 1,
            boost: false,
            jump: false,
        });
        assert!(game.x > 0.0);
        assert!(game.distance_m > 0.0);
        assert_eq!(game.mode, SkierMode::Skiing);
    }

    #[test]
    fn neutral_line_reaches_monster() {
        let mut game = Game::new(12);
        for _ in 0..400 {
            game.step(Input {
                steer: 0,
                boost: false,
                jump: false,
            });
            if game.monster.is_some() {
                assert!(game.distance_m >= MONSTER_SPAWN_DISTANCE);
                return;
            }
        }
        panic!("monster did not spawn");
    }

    #[test]
    fn seeded_obstacles_differ() {
        let a = Game::new(501).state().obstacles[0].x;
        let b = Game::new(502).state().obstacles[0].x;
        assert_ne!(a, b);
    }
}
