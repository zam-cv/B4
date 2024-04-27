import os
import shutil

# TODO: remove not necessary files
# TODO: post your own images

PATH = os.path.dirname(__file__)
FILE = os.path.join(PATH, 'template.html')

OUTPUT = os.path.join(PATH, "..", "..", "page", 'index.html')
BUILD = os.path.join(PATH, "..", "..", "page", "Build")
LOGO = os.path.join(PATH, "..", "..", "page", "verqor.png")
BACKGROUND = os.path.join(PATH, "..", "..", "page", "background.png")
V = os.path.join(PATH, "..", "..", "page", "v.png")

with open(FILE, 'r') as f:
    with open(OUTPUT, 'w') as o:
        content = f.read()
        
        files = os.listdir(BUILD)
        
        for file in files:
            if file.endswith('.wasm.unityweb'):
                content = content.replace('/Qrops.wasm.unityweb', "/" + file)
            
            if file.endswith('.data.unityweb'):
                content = content.replace('/Qrops.data.unityweb', "/" + file)
            
            if file.endswith('.js.unityweb'):
                content = content.replace('/Qrops.framework.js.unityweb', "/" + file)
        
        o.write(content)

shutil.copy(os.path.join(PATH, "verqor.png"), LOGO)
shutil.copy(os.path.join(PATH, "background.png"), BACKGROUND)
shutil.copy(os.path.join(PATH, "v.png"), V)