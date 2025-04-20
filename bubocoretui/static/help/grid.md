The grid is the central view of BuboCoreTUI. It provides ways to organize and manipulate the scripts that compose a scene loaded on the server.

## Organization

- `Scene` - The scene represents everything that is currently playing. It is composed of one or more `lines`, itself composed of one or more `frames`.
- `Line` - A line is a linear sequence of `frames`. One or more line can be played at the same time.
- `Frame` - A frame is a single unit of execution. It is the smallest unit that can be manipulated in the grid. It is essentially a script that can be enabled or disabled. Each frame has a length (in beats).

## Navigation & Selection

*   `↑` / `↓` / `←` / `→` : Move the cursor (single cell selection).
*   `Shift` + `Arrows` : Extend the selection range.
*   `Esc` : If multiple cells are selected, reset to a single cell selection at the start of the previous range.

## Line Manipulation

*   `a`: Add a new line column.
*   `d`: Remove the *last* line column.
*   `c`: Copy the selected cells to the clipboard.
*   `p`: Paste the copied frame under cursor.

## Frame Manipulation (within Line)

*   `+`: Add a new frame (length 1.0) to the *end* of the line under the cursor.
*   `-`: Remove the *last* frame from the line under the cursor.
*   `Space` : Toggle the enabled/disabled state of the selected frame(s).
*   `Enter` : Request the script for the selected frame (opens in Editor).
*   `>` or `.`: Increase the length of selected frame(s) by 0.25.
*   `<` or `,`: Decrease the length of selected frame(s) by 0.25 (minimum length 0.01).
*   `b`: Toggle the selected frame as the *start marker* for its line.
*   `e`: Toggle the selected frame as the *end marker* for its line.
