use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};
use bevy_enoki::prelude::*;

/// Handle for [`Particle2dEffect`].
#[derive(Resource, Default)]
pub(crate) struct Particle2dMaterialHandle<T>(pub(crate) Handle<T>)
where
    T: Particle2dMaterial + Default;

/// [Particle2dMaterial] for [BloodParticle](crate::render::prelude::BloodParticle).
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub(crate) struct BloodParticleMaterial {}
impl Particle2dMaterial for BloodParticleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/particles/blood.wgsl".into()
    }
}

/// [Particle2dMaterial] for [DeathParticle](crate::render::prelude::DeathParticle).
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub(crate) struct DeathParticleMaterial {}
impl Particle2dMaterial for DeathParticleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/particles/death.wgsl".into()
    }
}

/// [Particle2dMaterial] for [DustTrailParticle](crate::render::prelude::DustTrailParticle).
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub(crate) struct DustTrailParticleMaterial {}
impl Particle2dMaterial for DustTrailParticleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/particles/dust_trail.wgsl".into()
    }
}

/// [Particle2dMaterial] for [MeleeParticle](crate::render::prelude::MeleeParticle).
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub(crate) struct MeleeParticleMaterial {}
impl Particle2dMaterial for MeleeParticleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/particles/melee.wgsl".into()
    }
}
