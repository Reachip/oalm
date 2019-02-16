extern crate oalm;

#[cfg(test)]
mod test_core_install {
    #[cfg(test)]
    fn test_determine_arg_http() {
        assert_eq!(
            oalm::core::install::determine_arg("http"),
            Some(oalm::core::install::Arg::IsHttpOrHttps)
        );
    }

    #[cfg(test)]
    fn test_determine_arg_git() {
        assert_eq!(
            oalm::core::install::determine_arg("https://github.com/"),
            Some(oalm::core::install::Arg::IsGitRepo)
        );
    }

    #[cfg(test)]
    fn test_determine_arg_zip() {
        assert_eq!(
            oalm::core::install::determine_arg("file.zip"),
            Some(oalm::core::install::Arg::IsZipFile)
        );
    }

    #[cfg(test)]
    fn test_determine_arg_nothing_else() {
        assert_eq!(oalm::core::install::determine_arg("kind of"), None);
    }

}
