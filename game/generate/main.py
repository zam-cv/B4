import os

# TODO: remove not necessary files
# TODO: post your own images

PATH = os.path.dirname(__file__)
FILE = os.path.join(PATH, 'template.html')
OUTPUT = os.path.join(PATH, "..", "..", "page", 'index.html')

with open(FILE, 'r') as f:
    with open(OUTPUT, 'w') as o:
        o.write(f.read())