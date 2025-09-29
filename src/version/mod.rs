//! Game versions and protocol IDs.


/// Game versions by protocol ID.
#[derive(Clone, Copy)]
pub struct Version {
    id    : u32,
    names : &'static [&'static str]
}

impl Version {

    /// The current version supported by pipework.
    pub const CURRENT : Self = Self::by_id(772).unwrap(); // TODO: Check against current datagen

    /// Get version information by protocol ID.
    pub const fn by_id(id : u32) -> Option<Self> {
        let mut i = 0;
        while (i < Self::LOOKUP.len()) {
            let (checking_id, names,) = Self::LOOKUP[i];
            if (id == checking_id) {
                return Some(Self { id, names });
            }
            i += 1;
        }
        None
    }

    /// Get version information by version or snapshot name.
    pub const fn by_name(name : &'static str) -> Option<Self> {
        let mut i = 0;
        while (i < Self::LOOKUP.len()) {
            let (id, checking_names,) = Self::LOOKUP[i];
            let mut j = 0;
            while (j < checking_names.len()) {
                let checking_name = checking_names[j];
                if (name.len() == checking_name.len()) {
                    let mut k = 0;
                    while (k < name.len()) {
                        if (name.as_bytes()[k] == checking_name.as_bytes()[k]) {
                            return Some(Self { id, names : checking_names });
                        }
                        k += 1;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        None
    }

}

impl Version {

    /// The protocol ID of this version.
    #[inline]
    pub const fn id(self) -> u32 { self.id }

    /// The version or snapshot names supporting this protocol ID.
    #[inline]
    pub const fn names(self) -> &'static [&'static str] { self.names }

    /// The earliest version or snapshot name supporting this protocol ID.
    #[inline]
    pub const fn earliest_name(self) -> &'static str { self.names[0] }

    /// The latest version or snapshot name supporting this protocol ID.
    #[inline]
    pub const fn latest_name(self) -> &'static str { self.names.last().unwrap() }

}

include!("out.rs");
