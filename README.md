Simple 3D renderer of `.obj` files using the rasterization algorithim.

The algorithim basically works like so:

1. For each triangle in your scene transfrom it from world space -> camera space -> screen space -> NDC space -> raster space.
2. Get a bounding box around the triangle
3. For each pixel in the bounding box check if it intersects the triangle
4. Interpolate the z coordinate of the pixel with barycentric coordinates
5. Use a depth buffer with the interpolated z coordinate to check if the triangle is occluded or not. If it is discard it.

![](examples/dragon.ppm)
