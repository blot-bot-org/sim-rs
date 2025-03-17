
// Converts cartesian coordinates into belt lengths.
pub fn cartesian_to_belt(x: f64, y: f64, motor_distance: f64) -> (f64, f64) {
    let left_belt = f64::sqrt(f64::powi(x, 2) + f64::powi(y, 2));
    let right_belt = f64::sqrt(f64::powi(motor_distance - x, 2) + f64::powi(y, 2));

    return (left_belt, right_belt);
}

// Converts belt lengths into cartesian coordinates.
pub fn belt_to_cartesian(left_length: f64, right_length: f64, motor_distance: f64) -> (f64, f64) {
    let x = (f64::powi(motor_distance, 2) + f64::powi(left_length, 2) - f64::powi(right_length, 2)) / (2. * motor_distance);
    let y = f64::sqrt(f64::powi(left_length, 2) - f64::powi(x, 2));
    return (x, y);
}


const steps_per_rev: f64 = 3200.;
const wheel_diameter: f64 = 12.63;

// Returns the amount of steps required for the belt to move 1mm.
pub fn steps_per_mm() -> f64 {
    return steps_per_rev / (std::f64::consts::PI * wheel_diameter);
}

// Returns the amount of MM a belt will move given an amount of steps.
pub fn steps_to_mm(steps: f64) -> f64 {
    // println!("{} steps is {} mm", steps, steps / (steps_per_mm()));
    return steps / (steps_per_mm());
}






pub struct Belts {
    left_belt_length: f64,
    right_belt_length: f64,
    motor_distance: f64
}

impl Belts {
    pub fn new_by_length(left_belt_length: f64, right_belt_length: f64, motor_distance: f64) -> Self {
        return Self { left_belt_length, right_belt_length, motor_distance };
    }

    pub fn new_by_cartesian(page_x: f64, page_y: f64, motor_distance: f64) -> Self {
        let (left_belt_length, right_belt_length) = cartesian_to_belt(page_x, page_y, motor_distance);
        println!("Starting belts at {} {}", left_belt_length, right_belt_length);
        return Self { left_belt_length, right_belt_length, motor_distance };
    }

    // Moves the left belt by a given amount. Uses += because a positive number of steps turns the
    // left motor clockwise, which increases the belt length.
    pub fn move_left(&mut self, steps: i16) {
        // println!("old left belt length: {}", self.left_belt_length);
        self.left_belt_length += steps_to_mm(steps as f64);
        // println!("new left belt length: {}", self.left_belt_length);
    }

    // Moves the right belt by a given amount. Uses -= because a positive number of steps turns the
    // right motor clockwise, which reduces the belt length.
    pub fn move_right(&mut self, steps: i16) {
        self.right_belt_length += steps_to_mm(steps as f64);
    }

    pub fn move_belts(&mut self, left_steps: i16, right_steps: i16) -> () {
        self.move_left(left_steps);
        self.move_right(right_steps);
    }

    pub fn get_cartesian(&self) -> (f64, f64) {
        return belt_to_cartesian(self.left_belt_length, self.right_belt_length, self.motor_distance);
    }

    pub fn get_lengths(&self) -> (f64, f64) {
        return (self.left_belt_length, self.right_belt_length);
    }
}
