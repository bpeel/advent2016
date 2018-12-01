#!/usr/bin/env python3

from mako.template import Template
import sys
import struct
from tempfile import NamedTemporaryFile
import subprocess

TEMPLATE = """\
[compute shader]
#version 450

layout(binding = 0, std140) buffer inbuf {
        int changes[];
};

layout(binding = 1, std140) buffer outbuf {
        int sum;
};

void
main()
{
        atomicAdd(sum, changes[gl_WorkGroupID.x]);
}

[test]

# Clear the sum
ssbo 1 subdata int 0 0

# Initialise the input data
ssbo 0 subdata int 0 ${" ".join(str(x) for x in inputs)}

# Calculate the sum
compute ${len(inputs)} 1 1
"""

inputs = [int(x) for x in sys.stdin]

with NamedTemporaryFile('w+') as infile, NamedTemporaryFile() as outfile:
    template = Template(TEMPLATE)
    source = template.render(inputs=inputs)
    print(source, file=infile)
    infile.flush()
    subprocess.check_call(["vkrunner",
                           "-q",
                           "-B", "1",
                           "-b", outfile.name,
                           infile.name])
    output = outfile.read()
    part1 = struct.unpack("i", output)[0]
    print("Part 1: {}".format(part1))
