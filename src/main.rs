use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadingMode {
    SinglePage,     
    DoublePage,     
    VerticalScroll, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadingDirection {
    RightToLeft, 
    LeftToRight, 
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderConfig {
    pub mode: ReadingMode,
    pub direction: ReadingDirection,
}


pub struct ReaderSession {
    pub pages: Vec<String>,  
    pub current_index: usize,  
    pub config: ReaderConfig,   
}

impl ReaderSession {
    pub fn new(pages: Vec<String>, config: ReaderConfig) -> Self {
        Self {
            pages,
            current_index: 0,
            config,
        }
    }

    
    pub fn next(&mut self) -> bool {
        let step = match self.config.mode {
            ReadingMode::DoublePage => 2,
            _ => 1,
        };

        if self.current_index + step < self.pages.len() {
            self.current_index += step;
            true
        } else {
            false
        }
    }

    
    pub fn prev(&mut self) -> bool {
        let step = match self.config.mode {
            ReadingMode::DoublePage => 2,
            _ => 1,
        };

        if self.current_index >= step {
            self.current_index -= step;
            true
        } else {
            self.current_index = 0;
            false
        }
    }

    pub fn visible_pages(&self) -> &[String] {
        if self.pages.is_empty() {
            return &[];
        }

        match self.config.mode {
            ReadingMode::SinglePage => &self.pages[self.current_index..=self.current_index],
            ReadingMode::DoublePage => {
                let end = (self.current_index + 2).min(self.pages.len());
                &self.pages[self.current_index..end]
            }
            ReadingMode::VerticalScroll => &self.pages[..],
        }
    }
}

fn main() {
    println!("--- Iniciando motor de lectura ---");

    let pages = vec![
        "pagina_01.png".to_string(),
        "pagina_02.png".to_string(),
        "pagina_03.png".to_string(),
        "pagina_04.png".to_string(),
        "pagina_05.png".to_string(),
    ];

    let config = ReaderConfig {
        mode: ReadingMode::SinglePage,
        direction: ReadingDirection::RightToLeft,
    };

    let mut session = ReaderSession::new(pages, config);

    println!("Inicio: {:?}", session.visible_pages());

    session.next();
    println!("Siguiente página: {:?}", session.visible_pages());

    session.config.mode = ReadingMode::DoublePage;
    println!("Modo doble página activo: {:?}", session.visible_pages());
}