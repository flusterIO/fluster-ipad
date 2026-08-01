use serde::{Deserialize, Serialize};

/// ## Warning
/// Right now this is all very insecure, and you should always assume that
/// anyone installing anything on your computer can access your notes. That's no
/// different than them being on your file system really, but the connection to
/// the API and the ability to write to the db is almost completely
/// unprotected. If someone's on your system tha that has the ability to
/// manipulate things, you have bigger problems anyways.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApplicationPermission {
    /// The ability to write to the file system, **through** the Conundrum
    /// api's.
    /// This will not allow the application to write to Conundrum files
    /// directly, although they can probably do that anyways if they're on
    /// your system, but will instead allow their data to
    /// overwrite conundrum files on your system making them a co-equal source
    /// of truth when it comes to your notes.
    /// If you prefer to keep your file system as the source of truth, do not
    /// add this to any applications. If you're perpetually online and
    /// aren't worried about syncying conflicts, then add this to multiple
    /// applications, and so long as there aren't updates in-between
    /// syncs, there shouldn't be any unexpected conflicts.
    ///
    /// Use the 'priority' field in the application data to prioritize one
    /// application over the other, which will be used for comparison when
    /// syncing if both applications provide updates.
    ManagedFileSytemWrite,
}
