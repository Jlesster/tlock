fn main() {
    pkg_config::Config::new()
        .atleast_version("0.1")
        .probe("pam")
        .expect(
            "libpam not found.  \
             Install it with: \
             sudo pacman -S pam",
        );
}
