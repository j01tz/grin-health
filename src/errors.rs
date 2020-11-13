// Make it easier to propagate accurate errors from external API calls using reqwest
error_chain!{
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
    }
}