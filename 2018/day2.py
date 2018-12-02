#!/usr/bin/env python3

from mako.template import Template
import sys
import struct
from tempfile import NamedTemporaryFile
import subprocess
import itertools

TEMPLATE = """\
[compute shader]
#version 450

#define ID_SIZE ${id_size}

layout(binding = 0, std140) buffer inbuf {
        int ids[];
};

layout(binding = 1, std140) buffer outbuf {
        int doubles;
        int triples;
};

void
main()
{
        bool have_double = false, have_triple = false;

        for (int i = 0; i < ID_SIZE; i++) {
                int letter = ids[gl_WorkGroupID.x * ID_SIZE + i];
                int count = 0;

                for (int j = 0; j < ID_SIZE; j++) {
                        int other_letter = ids[gl_WorkGroupID.x * ID_SIZE + j];
                        if (other_letter == letter)
                                count++;
                }

                switch (count) {
                case 2:
                        have_double = true;
                        break;
                case 3:
                        have_triple = true;
                        break;
                }
        }

        if (have_double)
                atomicAdd(doubles, 1);
        if (have_triple)
                atomicAdd(triples, 1);
}

[test]

# Clear the sums
ssbo 1 subdata int 0 0
ssbo 1 subdata int 4 0

# Initialise the input data
ssbo 0 subdata int 0 ${" ".join(str(x) for x in ids)}

# Calculate the sum
compute ${len(ids) // id_size} 1 1
"""

ids = [x.strip() for x in sys.stdin]
flat_ids = [ord(x) for id in ids for x in id]

with NamedTemporaryFile('w+') as infile, NamedTemporaryFile() as outfile:
    template = Template(TEMPLATE)
    source = template.render(ids=flat_ids, id_size=len(ids[0]))
    print(source, file=infile)
    infile.flush()
    subprocess.check_call(["vkrunner",
                           "-q",
                           "-B", "1",
                           "-b", outfile.name,
                           infile.name])
    output = outfile.read()
    doubles, triples = struct.unpack("ii", output[0:8])
    print("doubles: {}, triples {}".format(doubles, triples))
    print("Part 1: {}".format(doubles * triples))
