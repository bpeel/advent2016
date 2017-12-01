import sys

def valid_triangle(tri):
    return sum(tri) > max(tri) * 2

def count_triangles(tris):
    return sum(valid_triangle(tri) for tri in tris)

def rotate_triangle_list(tris):
    for y in range(0, len(tris), 3):
        for x in range(3):
            yield [tris[y + i][x] for i in range(3)]

triangles_in_rows = [[int(x) for x in line.split()] for line in sys.stdin]

print("Part 1:", count_triangles(triangles_in_rows))
print("Part 2:", count_triangles(rotate_triangle_list(triangles_in_rows)))
