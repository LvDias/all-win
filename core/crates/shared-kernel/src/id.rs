//! Typed identifiers. Core entities use UUIDv7 (ADR-0001 §5, data model §6).

/// Declares a UUID-backed identifier newtype (crate-local: the `sqlx` feature is this crate's).
macro_rules! uuid_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[derive(::serde::Serialize, ::serde::Deserialize)]
        #[serde(transparent)]
        #[cfg_attr(feature = "sqlx", derive(::sqlx::Type), sqlx(transparent))]
        pub struct $name(::uuid::Uuid);

        impl $name {
            /// New time-ordered (v7) identifier.
            pub fn new_v7() -> Self {
                Self(::uuid::Uuid::now_v7())
            }

            pub const fn from_uuid(id: ::uuid::Uuid) -> Self {
                Self(id)
            }

            pub const fn as_uuid(&self) -> ::uuid::Uuid {
                self.0
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
}

uuid_id!(
    /// Authenticated owner of every user-scoped row. Every user-owned query is scoped by it.
    UserId
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_is_v7_and_serializes_as_plain_uuid() {
        let id = UserId::new_v7();
        assert_eq!(id.as_uuid().get_version_num(), 7);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, format!("\"{id}\""));
        let back: UserId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
    }

    #[test]
    fn v7_ids_are_time_ordered() {
        let a = UserId::new_v7();
        let b = UserId::new_v7();
        assert!(a < b);
    }
}
