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

day10_src = [
        'pcx-util.c',
        'pcx-buffer.c',
        'day10.c'
]

executable('day10', day10_src)

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

day13_src = [
        'intcode.c',
        'pcx-util.c',
        'pcx-error.c',
        'pcx-buffer.c',
        'read-memory.c',
        'grid.c',
        'day13.c'
]

executable('day13', day13_src)

day15_src = [
        'intcode.c',
        'pcx-util.c',
        'pcx-error.c',
        'pcx-buffer.c',
        'read-memory.c',
        'grid.c',
        'day15.c'
]

executable('day15', day15_src)

day19_src = [
        'intcode.c',
        'pcx-util.c',
        'pcx-error.c',
        'pcx-buffer.c',
        'read-memory.c',
        'day19.c'
]

executable('day19', day19_src)

day20_src = [
        'pcx-util.c',
        'pcx-buffer.c',
        'pcx-error.c',
        'day20.c'
]

executable('day20', day20_src)

runner_src = [
        'intcode.c',
        'pcx-util.c',
        'pcx-error.c',
        'pcx-buffer.c',
        'read-memory.c',
        'runner.c'
]

executable('runner', runner_src)

day22_src = [
        'pcx-util.c',
        'pcx-buffer.c',
        'day22.c'
]

executable('day22', day22_src)
