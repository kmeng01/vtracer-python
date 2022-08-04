from rustpy_vtracer import trace
import cv2

img = cv2.imread("dog.jpg")
img = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)
img = cv2.cvtColor(img, cv2.COLOR_RGB2RGBA).tolist()

svg = trace(img)
with open("out.svg", "w") as f:
    f.write(svg)
