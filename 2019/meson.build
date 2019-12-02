project('advent2019', 'c')

cc = meson.get_compiler('c')

foreach a : ['-Wall', '-Wuninitialized', '-Wempty-body', '-Wformat',
      '-Wformat-security', '-Winit-self', '-Wundef',
      '-Wvla', '-Wpointer-arith', '-Wmissing-declarations']
  if cc.has_argument(a)
    add_project_arguments(a, language : ['c'])
  endif
endforeach

day2_src = [
        'intcode.c',
        'pcx-util.c',
        'pcx-error.c',
        'pcx-buffer.c',
        'read-memory.c',
        'day2.c'
]

executable('day2', day2_src)
