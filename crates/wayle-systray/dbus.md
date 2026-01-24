# System Tray D-Bus Interface

Interact with system tray items.

- **Service:** `com.wayle.SystemTray1`
- **Path:** `/com/wayle/SystemTray`

## Methods

| Method     | Arguments | Returns   | Description                                |
| ---------- | --------- | --------- | ------------------------------------------ |
| `List`     | -         | `a(ssss)` | List tray items: (id, title, icon, status) |
| `Activate` | `s id`    | -         | Activate/click a tray item                 |

## Properties

| Property    | Type | Access | Description                                |
| ----------- | ---- | ------ | ------------------------------------------ |
| `Count`     | `u`  | read   | Number of tray items                       |
| `IsWatcher` | `b`  | read   | Whether operating as StatusNotifierWatcher |
