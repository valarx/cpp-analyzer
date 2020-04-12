mod compilation_database;
mod translation_unit;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub use compilation_database::{CompilationDatabase, Parsed};
use translation_unit::index::Index;
pub use translation_unit::index::{DeclarationFromPHCMode, DiagnosticsMode};
pub use translation_unit::TUOptionsBuilder;
use translation_unit::TU;
pub use translation_unit::{
    AccessSpecifierType, CodeSpan, ConstructorType, CursorKind, CursorType, Entry, Position,
    TemplateArgumentKind, Virtuality,
};

#[derive(Debug)]
pub enum ParsingError {
    FileNameConversionProblem(String),
    IndexCreationFailure,
    GenericFailure(String),
    Crash(String),
    InvalidArguments(String),
    ASTReadError(String),
    UnknownError(i32),
}

pub struct Source {
    pub translation_units: Vec<Result<TU, ParsingError>>,
}

impl Source {
    pub fn from_file(
        file_name: String,
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
        command_line_args: Vec<String>,
        options: TUOptionsBuilder,
    ) -> Result<Source, ParsingError> {
        let index = Index::new(phc_mode, diagnostics_mode)?;
        let mut result = Source {
            translation_units: vec![],
        };
        result
            .translation_units
            .push(TU::new(file_name, &index, command_line_args, &options));
        Ok(result)
    }

    pub fn from_compilation_database(
        phc_mode: DeclarationFromPHCMode,
        diagnostics_mode: DiagnosticsMode,
        compilation_database: CompilationDatabase,
        options: TUOptionsBuilder,
    ) -> Result<Source, ParsingError> {
        let n_workers = 16; // TODO make it configurable
        let n_tasks = compilation_database.commands.len();
        let thread_pool = ThreadPool::new(n_workers);
        let (tx, rx) = channel();
        for command in compilation_database.commands {
            let tx = tx.clone();
            thread_pool.execute(move || {
                let index = Index::new(phc_mode, diagnostics_mode);
                let err_msg = "Failed to create syncronization channel for thread pool";
                match index {
                    Ok(index) => tx
                        .send(TU::new(command.file, &index, command.args, &options))
                        .expect(err_msg),
                    Err(err) => tx.send(Err(err)).expect(err_msg),
                };
            });
        }
        let result = Source {
            translation_units: rx.iter().take(n_tasks).collect(),
        };
        Ok(result)
    }
}
