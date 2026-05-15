// Rust Program for Server Room Humidity Monitoring and Control
// Utilizing Object-Oriented Design and Numerical Methods

use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

// --- 1. SENSOR OBJECT ---
struct HumiditySensor {
    id: u32,
    raw_data: f64,
    history: VecDeque<f64>,
    calibration_slope: f64,
    calibration_offset: f64,
}

impl HumiditySensor {
    fn new(id: u32) -> Self {
        Self {
            id,
            raw_data: 0.0,
            history: VecDeque::with_capacity(10),
            calibration_slope: 1.02, // Example calibration gain
            calibration_offset: -0.5, // Example calibration offset
        }
    }

    // Input & Validation
    fn read_sensor(&mut self, val: f64) -> Result<f64, String> {
        if val < 0.0 || val > 100.0 {
            return Err(format!("Sensor ID {}: Invalid Humidity Range!", self.id));
        }
        self.raw_data = val;
        Ok(val)
    }

    // Numerical Method: Calibration
    fn apply_calibration(&self, val: f64) -> f64 {
        (val * self.calibration_slope) + self.calibration_offset
    }

    // Numerical Method: Low Pass Filtering
    fn filter_data(&self, current: f64, previous: f64) -> f64 {
        let alpha = 0.2; // Smoothing factor
        (alpha * current) + (1.0 - alpha) * previous
    }
}

// --- 2. CONTROLLER OBJECT ---
struct Controller {
    target_low: f64,
    target_high: f64,
    is_humidifier_on: bool,
    is_dehumidifier_on: bool,
}

impl Controller {
    fn new() -> Self {
        Self {
            target_low: 40.0,
            target_high: 50.0,
            is_humidifier_on: false,
            is_dehumidifier_on: false,
        }
    }

    // Decision Logic
    fn evaluate_state(&mut self, current_h: f64) -> String {
        if current_h < 20.0 {
            self.is_humidifier_on = true;
            self.is_dehumidifier_on = false;
            String::from("CRITICAL LOW: Activating Humidifier!")
        } else if current_h > 60.0 {
            self.is_humidifier_on = false;
            self.is_dehumidifier_on = true;
            String::from("CRITICAL HIGH: Activating Dehumidifier!")
        } else if current_h >= self.target_low && current_h <= self.target_high {
            self.is_humidifier_on = false;
            self.is_dehumidifier_on = false;
            String::from("Status: Optimal (ASHRAE Standard)")
        } else {
            String::from("Status: Marginal - Monitoring...")
        }
    }

    // Numerical Method: Root Finding (Bisection) 
    // To find at what "adjustment factor" the humidity reaches target
    fn find_target_adjustment(&self, current: f64) -> f64 {
        let mut low = -10.0;
        let mut high = 10.0;
        let target = 45.0; // Mid-point target
        
        for _ in 0..10 { // 10 iterations of Bisection
            let mid = (low + high) / 2.0;
            if (current + mid) < target {
                low = mid;
            } else {
                high = mid;
            }
        }
        (low + high) / 2.0
    }
}

// --- 3. MONITORING SYSTEM OBJECT ---
struct MonitoringSystem {
    sensor: HumiditySensor,
    controller: Controller,
    reading_count: usize,
}

impl MonitoringSystem {
    fn new() -> Self {
        Self {
            sensor: HumiditySensor::new(101),
            controller: Controller::new(),
            reading_count: 0,
        }
    }

    // Numerical Method: Moving Average
    fn calculate_moving_average(&self) -> f64 {
        if self.sensor.history.is_empty() { return 0.0; }
        let sum: f64 = self.sensor.history.iter().sum();
        sum / self.sensor.history.len() as f64
    }

    // Numerical Method: Linear Regression (Simple trend)
    fn calculate_trend(&self) -> f64 {
        let n = self.sensor.history.len();
        if n < 2 { return 0.0; }
        
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xx = 0.0;

        for (i, &y) in self.sensor.history.iter().enumerate() {
            let x = i as f64;
            sum_x += x;
            sum_y += y;
            sum_xy += x * y;
            sum_xx += x * x;
        }

        let slope = (n as f64 * sum_xy - sum_x * sum_y) / (n as f64 * sum_xx - sum_x * sum_x);
        slope
    }

    // Main Operation Loop
    fn run(&mut self, simulated_inputs: Vec<f64>) {
        println!("--- DATA CENTER HUMIDITY MONITORING SYSTEM START ---");
        println!("Standard: ASHRAE Class A1 (Target 40-50%)\n");

        let mut prev_val = 45.0;

        for input in simulated_inputs {
            self.reading_count += 1;
            println!("Reading #{}", self.reading_count);

            // 1. Data Input & Validation
            match self.sensor.read_sensor(input) {
                Ok(raw) => {
                    // 2. Numerical Processing
                    let calibrated = self.sensor.apply_calibration(raw);
                    let filtered = self.sensor.filter_data(calibrated, prev_val);
                    
                    // Update history for Moving Average and Regression
                    if self.sensor.history.len() >= 5 { self.sensor.history.pop_front(); }
                    self.sensor.history.push_back(filtered);
                    
                    let avg = self.calculate_moving_average();
                    let trend = self.calculate_trend();
                    let adj_needed = self.controller.find_target_adjustment(filtered);

                    // 3. Control Decision
                    let status = self.controller.evaluate_state(filtered);

                    // 4. Output Monitoring
                    println!("  [SENS] Raw: {:.2}% | Processed: {:.2}%", raw, filtered);
                    println!("  [MATH] Moving Avg: {:.2}% | Trend: {:.4} units/tick", avg, trend);
                    println!("  [ROOT] Required Adjustment to 45%: {:.2}", adj_needed);
                    println!("  [CTRL] {}", status);
                    println!("--------------------------------------------------");

                    prev_val = filtered;
                }
                Err(e) => println!("  [ALARM] ERROR: {}", e),
            }
            thread::sleep(Duration::from_millis(500));
        }
    }
}

fn main() {
    let mut sys = MonitoringSystem::new();

    // Simulated sensor data stream (Normal -> Rising -> Error -> Dropping)
    let data_stream = vec![45.0, 46.5, 48.0, 55.0, 65.0, 105.0, 62.0, 45.0, 30.0, 15.0];

    sys.run(data_stream);
}