let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/code/kostalui
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +29 src/main.rs
badd +66 ~/code/kostalui/src/log/mod.rs
badd +9 ~/code/kostalui/src/log/file.rs
badd +1 log.txt
badd +26 ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/plotly-0.8.4/src/bindings.rs
badd +446 ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/plotly-0.8.4/src/plot.rs
argglobal
%argdel
$argadd src/main.rs
set stal=2
tabnew +setlocal\ bufhidden=wipe
tabnew +setlocal\ bufhidden=wipe
tabrewind
edit src/main.rs
argglobal
balt ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/plotly-0.8.4/src/bindings.rs
setlocal fdm=indent
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal fen
10
normal! zo
24
normal! zo
29
normal! zo
36
normal! zo
53
normal! zo
64
normal! zo
let s:l = 29 - ((13 * winheight(0) + 24) / 49)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 29
normal! 012|
tabnext
edit ~/code/kostalui/src/log/mod.rs
argglobal
balt src/main.rs
setlocal fdm=indent
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal fen
26
normal! zo
32
normal! zo
54
normal! zo
56
normal! zo
57
normal! zo
64
normal! zo
69
normal! zo
let s:l = 66 - ((25 * winheight(0) + 24) / 49)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 66
normal! 043|
tabnext
edit log.txt
argglobal
balt ~/code/kostalui/src/log/file.rs
setlocal fdm=indent
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=99
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 5943 - ((23 * winheight(0) + 24) / 49)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 5943
normal! 0
tabnext 2
set stal=1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
