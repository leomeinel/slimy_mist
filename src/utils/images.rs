/// Offsets of pixel neighbors.
const NEIGHBOR_OFFSETS: &[(i32, i32)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

/// Index of pixel coordinates x, y in [`Image::data`](bevy::prelude::Image).
pub(crate) fn pixel_index(x: u32, y: u32, width: u32) -> usize {
    ((y * width + x) * 4) as usize
}

/// Whether the pixel index `i` in data is transparent.
pub(crate) fn is_transparent_pixel(data: &[u8], i: usize) -> bool {
    data[i + 3] == 0
}

/// Whether pixel coordinates x, y have any opaqua neighbor.
///
/// This is computed with the help of [`NEIGHBOR_OFFSETS`].
pub(crate) fn has_opaque_neighbor(data: &[u8], x: u32, y: u32, width: u32, height: u32) -> bool {
    NEIGHBOR_OFFSETS.iter().any(|(ox, oy)| {
        let (nx, ny) = (x as i32 + ox, y as i32 + oy);
        nx >= 0
            && ny >= 0
            && nx < width as i32
            && ny < height as i32
            && !is_transparent_pixel(data, pixel_index(nx as u32, ny as u32, width))
    })
}
