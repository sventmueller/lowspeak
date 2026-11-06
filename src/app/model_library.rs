//! The local models on this Mac, with the download and the deletion that may be running.

use std::collections::HashSet;

use crate::models::LocalModel;

pub struct ModelLibrary {
    /// Without a model whose files are being deleted, so nothing selects or loads it meanwhile.
    installed: HashSet<LocalModel>,
    /// The model and the fraction downloaded.
    download: Option<(LocalModel, f64)>,
    deleting: Option<LocalModel>,
}

impl ModelLibrary {
    pub fn new(installed: HashSet<LocalModel>) -> Self {
        Self {
            installed,
            download: None,
            deleting: None,
        }
    }

    pub fn installed(&self) -> &HashSet<LocalModel> {
        &self.installed
    }

    pub fn is_installed(&self, model: LocalModel) -> bool {
        self.installed.contains(&model)
    }

    pub fn download(&self) -> Option<(LocalModel, f64)> {
        self.download
    }

    pub fn deleting(&self) -> Option<LocalModel> {
        self.deleting
    }

    pub fn start_download(&mut self, model: LocalModel) -> bool {
        if self.download.is_some() || self.is_installed(model) || self.deleting == Some(model) {
            return false;
        }
        self.download = Some((model, 0.0));
        true
    }

    pub fn download_progressed(&mut self, progress: f64) {
        if let Some((_, reported)) = &mut self.download {
            *reported = progress;
        }
    }

    /// `installed` is what the disk holds now.
    pub fn download_finished(&mut self, installed: HashSet<LocalModel>) {
        self.download = None;
        self.set_installed(installed);
    }

    pub fn start_deletion(&mut self, model: LocalModel) -> bool {
        if self.deleting.is_some()
            || self
                .download
                .is_some_and(|(downloading, _)| downloading == model)
            || !self.is_installed(model)
        {
            return false;
        }
        self.deleting = Some(model);
        self.installed.remove(&model);
        true
    }

    /// `installed` is what the disk holds now.
    pub fn deletion_finished(&mut self, installed: HashSet<LocalModel>) {
        self.deleting = None;
        self.set_installed(installed);
    }

    fn set_installed(&mut self, mut installed: HashSet<LocalModel>) {
        if let Some(deleting) = self.deleting {
            installed.remove(&deleting);
        }
        self.installed = installed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn library(models: &[LocalModel]) -> ModelLibrary {
        ModelLibrary::new(models.iter().copied().collect())
    }

    #[test]
    fn one_download_at_a_time() {
        let mut library = library(&[LocalModel::Base]);
        assert!(!library.start_download(LocalModel::Base));
        assert!(library.start_download(LocalModel::Tiny));
        assert!(!library.start_download(LocalModel::Small));
        library.download_progressed(0.5);
        assert_eq!(library.download(), Some((LocalModel::Tiny, 0.5)));
        library.download_finished([LocalModel::Base, LocalModel::Tiny].into());
        assert!(library.is_installed(LocalModel::Tiny) && library.download().is_none());
    }

    #[test]
    fn a_model_being_deleted_counts_as_gone() {
        let mut library = library(&[LocalModel::Base, LocalModel::ParakeetUltra]);
        assert!(library.start_deletion(LocalModel::Base));
        assert!(!library.is_installed(LocalModel::Base));
        assert!(!library.start_deletion(LocalModel::ParakeetUltra));
        assert!(!library.start_download(LocalModel::Base));
        assert!(library.start_download(LocalModel::Tiny));
        library.download_finished([LocalModel::Base, LocalModel::Tiny].into());
        assert!(!library.is_installed(LocalModel::Base));
        library.deletion_finished([LocalModel::Tiny, LocalModel::ParakeetUltra].into());
        assert_eq!(library.deleting(), None);
        assert!(library.is_installed(LocalModel::Tiny) && !library.is_installed(LocalModel::Base));
    }

    #[test]
    fn a_model_that_is_downloading_or_missing_cannot_be_deleted() {
        let mut library = library(&[LocalModel::Base]);
        assert!(!library.start_deletion(LocalModel::Small));
        assert!(library.start_download(LocalModel::Small));
        assert!(!library.start_deletion(LocalModel::Small));
    }
}
