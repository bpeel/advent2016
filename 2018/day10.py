#!/usr/bin/env python3

import sys
import re
from OpenGL import GL
import pygame
import struct
import ctypes

SCREEN_SIZE = (800, 600)

STAR_RE = re.compile(r'position=< *(-?[0-9]+), *(-?[0-9]+)> +'
                     r'velocity=< *(-?[0-9]+), *(-?[0-9]+)>')


def compile_shader(shader_type, source):
        shader = GL.glCreateShader(shader_type)
        GL.glShaderSource(shader, source)
        GL.glCompileShader(shader)

        if GL.glGetShaderiv(shader, GL.GL_COMPILE_STATUS) != 1:
            print(GL.glGetShaderInfoLog(shader))
            sys.exit(1)
 
        return shader


class Renderer:
    def __init__(self, stars):
        self.n_points = len(stars) // 4
        self.screen = pygame.display.set_mode(SCREEN_SIZE,
                                              pygame.OPENGL | pygame.DOUBLEBUF)

        with open('day10.glsl', 'r') as f:
            (vs_source, fs_source) = f.read().split('@@')

        vs_shader = compile_shader(GL.GL_VERTEX_SHADER, vs_source)
        fs_shader = compile_shader(GL.GL_FRAGMENT_SHADER, fs_source)

        self.program = GL.glCreateProgram()
        GL.glAttachShader(self.program, vs_shader)
        GL.glAttachShader(self.program, fs_shader)

        GL.glDeleteShader(vs_shader)
        GL.glDeleteShader(fs_shader)

        GL.glLinkProgram(self.program)
        assert(GL.glGetProgramiv(self.program, GL.GL_LINK_STATUS) == 1)

        self.buf = GL.glGenBuffers(1)
        GL.glBindBuffer(GL.GL_ARRAY_BUFFER, self.buf)
        buf_data = struct.pack("{}f".format(len(stars)), *stars)
        GL.glBufferData(GL.GL_ARRAY_BUFFER,
                        len(buf_data), buf_data,
                        GL.GL_STATIC_DRAW)

        position = GL.glGetAttribLocation(self.program, 'position')
        GL.glEnableVertexAttribArray(position)
        GL.glVertexAttribPointer(position, 2, GL.GL_FLOAT, GL.GL_FALSE,
                                 4 * 4, ctypes.c_void_p(0))

        velocity = GL.glGetAttribLocation(self.program, 'velocity')
        GL.glEnableVertexAttribArray(velocity)
        GL.glVertexAttribPointer(velocity, 2, GL.GL_FLOAT, GL.GL_FALSE,
                                 4 * 4, ctypes.c_void_p(2 * 4))

        GL.glPointSize(8.0)

        GL.glUseProgram(self.program)
        self.scale = GL.glGetUniformLocation(self.program, 'scale')

        GL.glUseProgram(self.program)
        self.time = GL.glGetUniformLocation(self.program, 'time')

    def render(self, time):
        GL.glClear(GL.GL_COLOR_BUFFER_BIT)

        max_x = max(stars[i] + stars[i + 2] * time
                    for i in range(0, len(stars), 4))
        min_x = min(stars[i] + stars[i + 2] * time
                    for i in range(0, len(stars), 4))
        max_y = max(stars[i + 1] + stars[i + 3] * time
                    for i in range(0, len(stars), 4))
        min_y = min(stars[i + 1] + stars[i + 3] * time
                    for i in range(0, len(stars), 4))

        off_x = (max_x + min_x) / -2.0
        off_y = (max_y + min_y) / -2.0

        max_range = max(max_x - min_x, max_y - min_y)
        scale_y = 1.9 / max_range
        scale_x = scale_y * SCREEN_SIZE[1] / SCREEN_SIZE[0]

        GL.glUniform4f(self.scale, scale_x, -scale_y, off_x, off_y)
        GL.glUniform1f(self.time, time)

        GL.glDrawArrays(GL.GL_POINTS, 0, self.n_points)
                
        pygame.display.flip()


stars = []

for line in sys.stdin:
    md = STAR_RE.match(line)
    stars.extend(int(x) for x in md.groups())

pygame.init()

renderer = Renderer(stars)

time = 0
time_step = 1
running = False

while True:
    while True:
        event = pygame.event.poll()
        if not event:
            break

        if event.type == pygame.QUIT:
            break
        if event.type == pygame.KEYDOWN:
            if event.key == pygame.K_LEFT:
                time -= time_step
            if event.key == pygame.K_RIGHT:
                time += time_step
            if event.key == pygame.K_UP:
                time_step *= 10
            if event.key == pygame.K_DOWN:
                time_step /= 10
            if event.key == pygame.K_SPACE:
                running = not running
                print(running)
            if event.key == ord('b'):
                time_step = -time_step
            print(time, time_step)

    print(time)
    renderer.render(time)

    if running:
        time += time_step
