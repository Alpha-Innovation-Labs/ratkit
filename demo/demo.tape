# ratatui-toolkit Demo Tape
# Usage: just demo-record
# This tape demonstrates all the key features of ratatui-toolkit

# Set up the terminal environment
Set FontSize 14
Set Shell "bash"
Set TypingSpeed 50ms

# Run the showcase demo using just
Type "just dev"
Enter

# Wait for the demo to start
Sleep 3s

# Navigate through Layout tab
Type "l"
Sleep 800ms

# Show split resize
Type "Right"
Sleep 300ms
Type "Right"
Sleep 300ms
Type "Left"
Sleep 300ms
Type "Left"
Sleep 300ms

# Go to Tree View tab
Type "t"
Sleep 800ms

# Navigate tree with keyboard
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "l"
Sleep 200ms
Type "h"
Sleep 200ms

# Go to Dialogs tab
Type "d"
Sleep 800ms

# Open info dialog
Type "i"
Sleep 800ms
Type "Escape"
Sleep 500ms

# Open warning dialog
Type "w"
Sleep 800ms
Type "Escape"
Sleep 500ms

# Open error dialog
Type "e"
Sleep 800ms
Type "Escape"
Sleep 500ms

# Open confirm dialog
Type "c"
Sleep 800ms
Type "Escape"
Sleep 500ms

# Go to Markdown tab
Type "m"
Sleep 800ms

# Scroll markdown content
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "k"
Sleep 200ms

# Go to Scrollbar tab
Sleep 500ms
Type "s"
Sleep 800ms

# Scroll with keyboard
Type "Down"
Sleep 300ms
Type "Down"
Sleep 300ms
Type "Up"
Sleep 300ms
Type "End"
Sleep 300ms
Type "Home"
Sleep 300ms

# Go to StatusLine tab
Type "b"
Sleep 800ms

# Switch vim modes to show status changes
Type "i"
Sleep 400ms
Type "Escape"
Sleep 400ms
Type "v"
Sleep 400ms
Type "Escape"
Sleep 400ms
Type ":"
Sleep 400ms
Type "Escape"
Sleep 400ms

# Go to Terminal tab
Type "r"
Sleep 800ms

# Type some commands in the embedded terminal
Type "echo 'ratatui-toolkit demo'"
Enter
Sleep 500ms

# Clear screen
Type "Ctrl+l"
Sleep 300ms

# Exit demo
Type "q"
Sleep 500ms
