# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run it on this file.
Output "target/du-show-live.gif"
Set Padding 20
Set Theme "AtomOneLight"
Set Shell bash
Set Height 540
Hide
Type@0ms "reset && echo '$ cargo run -q --example du -- /dev /etc /mnt /var /root /usr | cargo run -q --example show-live -- -o nushell-light'"
Type@0ms " && { sleep 2 ; cargo run -q --example du -- /dev /etc /mnt /var /root /usr ; sleep 5 ; } | cargo run -q --example show-live -- -o nushell-light"
Type@0ms " --tui-height=17"
# Backspace
Enter
Wait+Screen /Output/
Show
Wait+Screen@30s /usr.*iB/
Sleep 3s
Hide