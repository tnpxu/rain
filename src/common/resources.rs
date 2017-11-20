#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Resources {
    pub n_cpus: u32,
}

impl Resources {

    pub fn from_capnp(reader: &::capnp_gen::common_capnp::resources::Reader) -> Self {
        Resources {
            n_cpus: reader.get_n_cpus()
        }
    }

    pub fn to_capnp(&self, builder: &mut ::capnp_gen::common_capnp::resources::Builder) {
        builder.set_n_cpus(self.n_cpus);
    }

}
