# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run it on this file.
Output "target/show-live-travel-modes.gif"
Set Padding 10
Set Theme "AtomOneLight"
Set Shell bash
Set Height 520
# Sleep 1s
Hide
Type@0ms "reset && echo '$ cargo run --example show-live -- --schema=travel-modes'"
Type@0ms " && cargo run -q --example show-live -- --schema=travel-modes"
Type@0ms " --tui-height=18"
# Backspace
Enter
Wait+Screen /Output/
Show
Sleep 1s
Type@100ms '[{"mode": "foot", "benefit": "healthy"}, '
Enter
Type@100ms ' {"mode": "aeroplane", "benefit": "fast"}]'
Sleep 1s
Hide
Escape
