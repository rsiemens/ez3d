Simple in software rendering program using the rasterization algorithim. Mostly building as
this as I learn graphic programming concepts and rust so things are a bit messy :D.

The algorithim basically works like so:

1. For each triangle in your scene transfrom it from world space -> camera space -> screen space -> NDC space -> raster space.
2. Get a bounding box around the triangle
3. For each pixel in the bounding box check if it intersects the triangle
4. Interpolate the z coordinate of the pixel with barycentric coordinates
5. Use a depth buffer with the interpolated z coordinate to check if the triangle is occluded or not. If it is discard it.
6. Get the dot product between the triangle normal and the viewing direction to calculate shading
7. Store in framebuffer.
8. Output framebuffer to mode of choice (ppm file in this program).

> Example output for running this on the examples file, `ez3d examples/dragon.obj`

![](https://cdn.zappy.app/6ccfcab0ee56aad1df6d03f719ed29b8.png)

Simple improvements to be made:

- Use a perspective matrix
- Add anti aliasing by sub-dividing pixels
- Seperate the rendering pipeline out of the main program
- Control camera settings with cli args
- Support different vertex attributes (like texture mapping)
- Fix obj loader to handle negative offsets
