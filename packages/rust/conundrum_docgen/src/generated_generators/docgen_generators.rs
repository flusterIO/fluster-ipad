
use askama::Template;

pub fn run_generated_generators() -> crate::errors::DocGenResult<()> {
crate::generated_generators::usage::intro::DocumentationGenerator::default().render()
        .inspect(|x| {
            let _ = std::fs::create_dir_all("/Users/bigsexy/Desktop/swift/Fluster/docs/generated/vector/usage")
                .inspect_err(|e| {
                    log::error!("Create Directory Error: {:#?}", e);
                });
            if let Err(err) = std::fs::write("/Users/bigsexy/Desktop/swift/Fluster/docs/generated/vector/usage/intro.cdrm", x.as_bytes()) {
                log::error!("Docgen Error: {:#?}", err);
            }
        })
        .map_err(|e| {
                                    log::error!("Error: {:#?}", e);
                                    crate::errors::DocGenError::GeneralError
                                })?;

crate::generated_generators::protocol::conundrum_protocol::DocumentationGenerator::default().render()
        .inspect(|x| {
            let _ = std::fs::create_dir_all("/Users/bigsexy/Desktop/swift/Fluster/docs/generated/vector/protocol")
                .inspect_err(|e| {
                    log::error!("Create Directory Error: {:#?}", e);
                });
            if let Err(err) = std::fs::write("/Users/bigsexy/Desktop/swift/Fluster/docs/generated/vector/protocol/conundrum_protocol.cdrm", x.as_bytes()) {
                log::error!("Docgen Error: {:#?}", err);
            }
        })
        .map_err(|e| {
                                    log::error!("Error: {:#?}", e);
                                    crate::errors::DocGenError::GeneralError
                                })?;

crate::generated_generators::protocol::language::conundrum_lang::DocumentationGenerator::default().render()
        .inspect(|x| {
            let _ = std::fs::create_dir_all("/Users/bigsexy/Desktop/swift/Fluster/docs/generated/vector/protocol/language")
                .inspect_err(|e| {
                    log::error!("Create Directory Error: {:#?}", e);
                });
            if let Err(err) = std::fs::write("/Users/bigsexy/Desktop/swift/Fluster/docs/generated/vector/protocol/language/conundrum_lang.cdrm", x.as_bytes()) {
                log::error!("Docgen Error: {:#?}", err);
            }
        })
        .map_err(|e| {
                                    log::error!("Error: {:#?}", e);
                                    crate::errors::DocGenError::GeneralError
                                })?;

crate::generated_generators::legal::license::DocumentationGenerator::default().render()
        .inspect(|x| {
            let _ = std::fs::create_dir_all("/Users/bigsexy/Desktop/swift/Fluster/docs/generated/vector/legal")
                .inspect_err(|e| {
                    log::error!("Create Directory Error: {:#?}", e);
                });
            if let Err(err) = std::fs::write("/Users/bigsexy/Desktop/swift/Fluster/docs/generated/vector/legal/license.cdrm", x.as_bytes()) {
                log::error!("Docgen Error: {:#?}", err);
            }
        })
        .map_err(|e| {
                                    log::error!("Error: {:#?}", e);
                                    crate::errors::DocGenError::GeneralError
                                })?;

Ok(())
}