# Notifications D-Bus Interface

Control desktop notifications and Do Not Disturb mode.

- **Service:** `com.wayle.Notifications1`
- **Path:** `/com/wayle/Notifications`

## Methods

| Method             | Arguments       | Returns   | Description                                  |
| ------------------ | --------------- | --------- | -------------------------------------------- |
| `DismissAll`       | -               | -         | Dismiss all notifications                    |
| `Dismiss`          | `u id`          | -         | Dismiss a specific notification by ID        |
| `SetDnd`           | `b enabled`     | -         | Enable/disable Do Not Disturb                |
| `ToggleDnd`        | -               | -         | Toggle Do Not Disturb mode                   |
| `SetPopupDuration` | `u duration_ms` | -         | Set popup display time in milliseconds       |
| `List`             | -               | `a(usss)` | List notifications: (id, app, summary, body) |

## Properties

| Property        | Type | Access | Description                  |
| --------------- | ---- | ------ | ---------------------------- |
| `Dnd`           | `b`  | read   | Do Not Disturb status        |
| `PopupDuration` | `u`  | read   | Popup display duration in ms |
| `Count`         | `u`  | read   | Number of notifications      |
| `PopupCount`    | `u`  | read   | Number of active popups      |
