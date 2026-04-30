#import bevy_enoki::particle_vertex_out::{ VertexOutput }

const HALF_UV = vec2<f32>(0.5);
const LINE_WIDTH = 0.25;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let dist = in.uv - HALF_UV;
    let diag_main = dist.y - dist.x;
    let diag_cross = dist.y + dist.x;
    let in_cross = abs(diag_main) <= LINE_WIDTH || abs(diag_cross) <= LINE_WIDTH;

    return in.color * select(0., 1., in_cross);
}
