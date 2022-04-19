use std::{error::Error, path::{PathBuf, Path}};

use sqlite::State;

use crate::platform::Platform;

use super::{AmazonSettings, AmazonGame};


pub struct AmazonPlatform{
    pub settings:AmazonSettings
}

impl Platform<AmazonGame, Box<dyn Error>> for AmazonPlatform{
    #[cfg(windows)]
    fn enabled(&self) -> bool {       
        self.settings.enabled
    }
    
    #[cfg(target_family="unix")]
    fn enabled(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        "Amazon"
    }

    fn get_shortcuts(&self) -> Result<Vec<AmazonGame>, Box<dyn Error>> {
        let sqllite_path = get_sqlite_path().expect("This should enver get called if settings are invalid");
        let mut result = vec![];
        let connection = sqlite::open(sqllite_path)?;
        let mut statement = connection.prepare("SELECT Id, ProductTitle FROM DbSet WHERE Installed = 1")?;            
        while let State::Row = statement.next().unwrap() {
            let id = statement.read::<String>(0);
            let title = statement.read::<String>(1);
            if let (Ok(id),Ok(title)) = (id,title){
                result.push(AmazonGame { title, id });
            }
        }
        Ok(result)
    }

    fn settings_valid(&self) -> crate::platform::SettingsValidity {
        let path = get_sqlite_path();
        if path.is_some(){
            crate::platform::SettingsValidity::Valid
        }else{
            crate::platform::SettingsValidity::Invalid { reason: format!("Could not find Amazon Games installation")} 
        }
        
    }

    #[cfg(target_family ="unix")]
    fn create_symlinks(&self) -> bool {
        false
    }

    fn needs_proton(&self, _input: &AmazonGame) -> bool {
        false
    }
}


fn get_sqlite_path() -> Option<PathBuf> {
    match std::env::var("LOCALAPPDATA") {
        Ok(localdata) => {            
            let path = Path::new(&localdata).join("Amazon Games").join("Data").join("Games").join("Sql").join("GameInstallInfo.sqlite");
            if path.exists() {
                Some(path)
            }else{
                None
            }
        }
        Err(_e) => None,
    }
}