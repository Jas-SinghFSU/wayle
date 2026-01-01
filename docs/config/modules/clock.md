# 󰥔 Clock Module

Controls the clock display and calendar settings

## Button Behavior

**Config path:** `[modules.clock.button]`

| Property    | Type      | Description                                              | Default |
| ----------- | --------- | -------------------------------------------------------- | ------- |
| `show_icon` | `boolean` | Whether to display a clock icon alongside the time text. | `false` |

## Dropdown Behavior

**Config path:** `[modules.clock.dropdown]`

| Property        | Type      | Description                                                | Default |
| --------------- | --------- | ---------------------------------------------------------- | ------- |
| `show_calendar` | `boolean` | Whether to display a calendar widget in the dropdown menu. | `false` |

## General Behavior

**Config path:** `[modules.clock.general]`

| Property | Type     | Description                               | Default |
| -------- | -------- | ----------------------------------------- | ------- |
| `format` | `string` | Time format string using strftime syntax. | `""`    |

## Button Styling

**Config path:** `[modules.clock.styling.button]`

| Property | Type     | Description                                | Default |
| -------- | -------- | ------------------------------------------ | ------- |
| `icon`   | `string` | Color of the clock icon in the bar button. | `""`    |

## Dropdown Styling

**Config path:** `[modules.clock.styling.dropdown]`

| Property | Type     | Description                                       | Default |
| -------- | -------- | ------------------------------------------------- | ------- |
| `clock`  | `string` | Color of the clock display in the dropdown panel. | `""`    |
