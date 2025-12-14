from PIL import Image, ImageDraw

# Create simple blue icons
for size, name in [(32, "32x32.png"), (128, "128x128.png"), (256, "128x128@2x.png"), (512, "icon.png")]:
    img = Image.new('RGB', (size, size), color='blue')
    draw = ImageDraw.Draw(img)
    # Draw a simple X
    draw.line([(size//4, size//4), (3*size//4, 3*size//4)], fill='white', width=max(2, size//32))
    draw.line([(3*size//4, size//4), (size//4, 3*size//4)], fill='white', width=max(2, size//32))
    img.save(name)
    print(f"Created {name}")

# Create .ico file
img = Image.new('RGB', (256, 256), color='blue')
draw = ImageDraw.Draw(img)
draw.line([(64, 64), (192, 192)], fill='white', width=8)
draw.line([(192, 64), (64, 192)], fill='white', width=8)
img.save("icon.ico", format='ICO', sizes=[(256, 256)])
print("Created icon.ico")

# Create .icns file (macOS) - just use PNG as fallback
img.save("icon.icns")
print("Created icon.icns")
