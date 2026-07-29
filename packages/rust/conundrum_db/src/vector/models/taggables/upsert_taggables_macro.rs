#[macro_export]
macro_rules! upsert_taggables {
    ( $($self_alias:ty),* ) => {
        {
        $(
        for t in $self_alias.tags.iter() {
            t.upsert_self(db).await?;
        }
        if let Some(subject) = &$self_alias.subject {
            subject.upsert_self(db).await?;
        }
        if let Some(topic) = &$self_alias.topic {
            topic.upsert_self(db).await?;
        }
        )*
        }
    };
}

