# Power Profiles D-Bus Interface

Control system power profiles.

- **Service:** `com.wayle.PowerProfiles1`
- **Path:** `/com/wayle/PowerProfiles`

## Methods

| Method         | Arguments   | Returns | Description             |
| -------------- | ----------- | ------- | ----------------------- |
| `SetProfile`   | `s profile` | -       | Set power profile       |
| `Cycle`        | -           | -       | Cycle to next profile   |
| `ListProfiles` | -           | `as`    | List available profiles |

### Profile Values

| Value         | Description                               |
| ------------- | ----------------------------------------- |
| `power-saver` | Maximum battery life, reduced performance |
| `balanced`    | Default, balances power and performance   |
| `performance` | Maximum performance, higher power usage   |

## Properties

| Property              | Type | Access | Description                                   |
| --------------------- | ---- | ------ | --------------------------------------------- |
| `ActiveProfile`       | `s`  | read   | Currently active power profile                |
| `PerformanceDegraded` | `s`  | read   | Degradation reason (e.g., thermal throttling) |
| `ProfileCount`        | `u`  | read   | Number of available profiles                  |
