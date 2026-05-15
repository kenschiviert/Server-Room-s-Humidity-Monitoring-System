Humidity Monitoring & Control System (Rust Implementation)

This project is a high-reliability environmental monitoring system designed for data centers. It ensures that server room humidity remains within the ASHRAE (American Society of Heating, Refrigerating and Air-Conditioning Engineers) recommended range of 40%–50% RH.

🚀 System Architecture

The program is built using Object-Oriented Programming (OOP) principles in Rust, specifically utilizing struct and impl blocks to encapsulate logic:

HumiditySensor: Handles raw data acquisition, validation, calibration, and signal filtering.

Controller: The "brain" of the system. It evaluates the processed data against ASHRAE standards and manages actuator states (Humidifier/Dehumidifier).

MonitoringSystem: Orchestrates the workflow, maintains data buffers for numerical analysis, and provides real-time telemetry.

🧮 Numerical Methods Implementation

To ensure precision and predictive capabilities, the system implements several computational numeric techniques:

Calibration & Filtering: Raw sensor data is adjusted using a linear calibration model ($y = mx + c$) and passed through a Recursive Low-Pass Filter to eliminate electronic jitter.

Moving Average: Uses a sliding window buffer to smooth out transient spikes in humidity, preventing unnecessary actuator toggling.

Linear Regression: Analyzes the rate of change (slope) over the last few readings to determine if humidity is rising or falling.

Linear Interpolation: Estimates precise values between measurement timestamps.

Root Finding (Bisection Method): A mathematical approach used to calculate the specific humidity "offset" required to reach the target 45% midpoint of the ideal range.

🛠 Standard Operating Logic (ASHRAE)

The system enforces the following logic thresholds:

Condition

Humidity Range

Action

Critical High

$> 60\%$

Trigger High Alarm + Start Dehumidifier

Optimal

$40\% - 50\%$

System Idle / Status OK

Critical Low

$< 20\%$

Trigger Low Alarm + Start Humidifier

Sensor Error

Outside $0-100\%$

Panic Halt / Maintenance Mode

📦 How to Run

Ensure you have Rust installed.

Navigate to the project directory.

Run the simulation using:

cargo run


📝 Example Output

The terminal will display real-time statistics including:

Calibrated Value: The immediate "true" reading.

Moving Average: The smoothed trend value.

Prediction: Based on linear regression (e.g., "Humidity is rising at 0.5%/cycle").

Actuator Status: Current state of the server room climate control hardware.

Developed for College Instrumentation Project - Data Center Environmental Control.