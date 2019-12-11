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

day4_src = [
        'day4.c',
]

executable('day4', day4_src)

day5_src = [
        'intcode.c',
        'pcx-util.c',
        'pcx-error.c',
        'pcx-buffer.c',
        'read-memory.c',
        'day5.c'
]

executable('day5', day5_src)

day6_src = [
        'pcx-util.c',
        'pcx-buffer.c',
        'day6.c'
]

executable('day6', day6_src)

day7_src = [
        'intcode.c',
        'pcx-util.c',
        'pcx-error.c',
        'pcx-buffer.c',
        'read-memory.c',
        'permutations.c',
        'day7.c'
]

executable('day7', day7_src, link_args : '-pthread')

day11_src = [
        'intcode.c',
        'pcx-util.c',
        'pcx-error.c',
        'pcx-buffer.c',
        'read-memory.c',
        'grid.c',
        'day11.c'
]

executable('day11', day11_src)
