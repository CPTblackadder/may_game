

# Resources:
https://bevy-cheatbook.github.io/platforms/wasm.html
https://bevy-cheatbook.github.io/platforms/wasm/gh-pages.html
https://trunkrs.dev/


# My process
Install trunk:
cargo install --locked trunk

Run trunk:
trunk build

Move files to desired location manually and change web links in index.html to be relative.
I.E.

<!DOCTYPE html><html><head>
        <meta charset="utf-8">
        <title>Hello, World!</title>
    
<link rel="preload" href="/may_game-367964530c03d2cc_bg.wasm" as="fetch" type="application/wasm" crossorigin="">
<link rel="modulepreload" href="/may_game-367964530c03d2cc.js"></head>
<body><script type="module">import init from '/may_game-367964530c03d2cc.js';init('/may_game-367964530c03d2cc_bg.wasm');</script></body></html>

to

<!DOCTYPE html><html><head>
        <meta charset="utf-8">
        <title>Hello, World!</title>
    
<link rel="preload" href="may_game-367964530c03d2cc_bg.wasm" as="fetch" type="application/wasm" crossorigin="">
<link rel="modulepreload" href="may_game-367964530c03d2cc.js"></head>
<body><script type="module">import init from 'may_game-367964530c03d2cc.js';init('may_game-367964530c03d2cc_bg.wasm');</script></body></html>