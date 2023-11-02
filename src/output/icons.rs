use ansiterm::Style;
use phf::{phf_map, Map};

use crate::fs::File;

#[non_exhaustive]
struct Icons;

#[rustfmt::skip]
impl Icons {
    const AUDIO: char           = '\u{f001}';  // 
    const BINARY: char          = '\u{eae8}';  // 
    const BOOK: char            = '\u{e28b}';  // 
    const CALENDAR: char        = '\u{eab0}';  // 
    const CLOCK: char           = '\u{f43a}';  // 
    const COMPRESSED: char      = '\u{f410}';  // 
    const CONFIG: char          = '\u{e615}';  // 
    const CSS3: char            = '\u{e749}';  // 
    const DATABASE: char        = '\u{f1c0}';  // 
    const DIFF: char            = '\u{f440}';  // 
    const DISK_IMAGE: char      = '\u{e271}';  // 
    const DOCKER: char          = '\u{e650}';  // 
    const DOCUMENT: char        = '\u{f1c2}';  // 
    const DOWNLOAD: char        = '\u{f01da}'; // 󰇚
    const EMACS: char           = '\u{e632}';  // 
    const ESLINT: char          = '\u{e655}';  // 
    const FILE: char            = '\u{f15b}';  // 
    const FILE_OUTLINE: char    = '\u{f016}';  // 
    const FOLDER: char          = '\u{e5ff}';  // 
    const FOLDER_CONFIG: char   = '\u{e5fc}';  // 
    const FOLDER_GIT: char      = '\u{e5fb}';  // 
    const FOLDER_GITHUB: char   = '\u{e5fd}';  // 
    const FOLDER_HIDDEN: char   = '\u{f179e}'; // 󱞞
    const FOLDER_KEY: char      = '\u{f08ac}'; // 󰢬
    const FOLDER_NPM: char      = '\u{e5fa}';  // 
    const FOLDER_OPEN: char     = '\u{f115}';  // 
    const FONT: char            = '\u{f031}';  // 
    const GIST_SECRET: char     = '\u{eafa}';  // 
    const GIT: char             = '\u{f1d3}';  // 
    const GRADLE: char          = '\u{e660}';  // 
    const GRAPHQL: char         = '\u{e662}';  // 
    const GRUNT: char           = '\u{e611}';  // 
    const GULP: char            = '\u{e610}';  // 
    const HEADER: char          = '\u{f0fd}';  // 
    const HTML5: char           = '\u{f13b}';  // 
    const IMAGE: char           = '\u{f1c5}';  // 
    const INTELLIJ: char        = '\u{e7b5}';  // 
    const JSON: char            = '\u{e60b}';  // 
    const KEY: char             = '\u{eb11}';  // 
    const KEYS: char            = '\u{e60a}';  // 
    const KEYPASS: char         = '\u{f23e}';  // 
    const LANG_ASSEMBLY: char   = '\u{e637}';  // 
    const LANG_C: char          = '\u{e61e}';  // 
    const LANG_CPP: char        = '\u{e61d}';  // 
    const LANG_CSHARP: char     = '\u{f031b}'; // 󰌛
    const LANG_D: char          = '\u{e7af}';  // 
    const LANG_ELIXIR: char     = '\u{e62d}';  // 
    const LANG_ERLANG: char     = '\u{e7b1}';  // 
    const LANG_FSHARP: char     = '\u{e7a7}';  // 
    const LANG_GO: char         = '\u{e627}';  // 
    const LANG_GROOVY: char     = '\u{e775}';  // 
    const LANG_HASKELL: char    = '\u{e777}';  // 
    const LANG_JAVA: char       = '\u{e256}';  // 
    const LANG_JAVASCRIPT: char = '\u{e74e}';  // 
    const LANG_KOTLIN: char     = '\u{e634}';  // 
    const LANG_OCAML: char      = '\u{e67a}';  // 
    const LANG_PERL: char       = '\u{e67e}';  // 
    const LANG_PHP: char        = '\u{e608}';  // 
    const LANG_PYTHON: char     = '\u{e606}';  // 
    const LANG_R: char          = '\u{e68a}';  // 
    const LANG_RUBY: char       = '\u{e21e}';  // 
    const LANG_RUBYRAILS: char  = '\u{e73b}';  // 
    const LANG_RUST: char       = '\u{e68b}';  // 
    const LANG_SASS: char       = '\u{e603}';  // 
    const LANG_STYLUS: char     = '\u{e600}';  // 
    const LANG_TEX: char        = '\u{e69b}';  // 
    const LANG_TYPESCRIPT: char = '\u{e628}';  // 
    const LANG_ZIG: char        = '\u{e6a9}';  // 
    const LIBRARY: char         = '\u{eb9c}';  // 
    const LICENSE: char         = '\u{f02d}';  // 
    const LOCK: char            = '\u{f023}';  // 
    const MAKE: char            = '\u{e673}';  // 
    const MARKDOWN: char        = '\u{f48a}';  // 
    const MUSTACHE: char        = '\u{e60f}';  // 
    const NODEJS: char          = '\u{e718}';  // 
    const NPM: char             = '\u{e71e}';  // 
    const OS_ANDROID: char      = '\u{e70e}';  // 
    const OS_APPLE: char        = '\u{f179}';  // 
    const OS_LINUX: char        = '\u{f17c}';  // 
    const OS_LINUX_ARCH: char   = '\u{f303}';  // 
    const OS_LINUX_DEBIAN: char = '\u{e77d}';  // 
    const OS_LINUX_GENTOO: char = '\u{f30d}';  // 
    const OS_LINUX_REDHAT: char = '\u{e7bb}';  // 
    const OS_LINUX_VOID: char   = '\u{f32e}';  // 
    const OS_WINDOWS: char      = '\u{f17a}';  // 
    const OS_WINDOWS_CMD: char  = '\u{ebc4}';  // 
    const PLAYLIST: char        = '\u{f0cb9}'; // 󰲹
    const POWERSHELL: char      = '\u{ebc7}';  // 
    const PRIVATE_KEY: char     = '\u{f0306}'; // 󰌆
    const PUBLIC_KEY: char      = '\u{f0dd6}'; // 󰷖
    const RAZOR: char           = '\u{f1fa}';  // 
    const REACT: char           = '\u{e7ba}';  // 
    const README: char          = '\u{f00ba}'; // 󰂺
    const SHEET: char           = '\u{f1c3}';  // 
    const SHELL: char           = '\u{f1183}'; // 󱆃
    const SHELL_CMD: char       = '\u{f489}';  // 
    const SHIELD_CHECK: char    = '\u{f0565}'; // 󰕥
    const SHIELD_KEY: char      = '\u{f0bc4}'; // 󰯄
    const SHIELD_LOCK: char     = '\u{f099d}'; // 󰦝
    const SIGNED_FILE: char     = '\u{f19c3}'; // 󱧃
    const SLIDE: char           = '\u{f1c4}';  // 
    const SUBLIME: char         = '\u{e7aa}';  // 
    const SUBTITLE: char        = '\u{f0a16}'; // 󰨖
    const SYMLINK: char         = '\u{f481}';  // 
    const SYMLINK_DIR: char     = '\u{f482}';  // 
    const TERRAFORM: char       = '\u{f1062}'; // 󱁢
    const TEXT: char            = '\u{f15c}';  // 
    const TYPST: char           = '\u{1D42D}'; // 𝐭
    const UNITY: char           = '\u{e721}';  // 
    const VECTOR: char          = '\u{f0559}'; // 󰕙
    const VIDEO: char           = '\u{f03d}';  // 
    const VIM: char             = '\u{e7c5}';  // 
    const WRENCH: char          = '\u{f0ad}';  // 
    const XML: char             = '\u{f05c0}'; // 󰗀
    const YARN: char            = '\u{e6a7}';  // 
}

/// Mapping from full filenames to directory icon. This mapping should contain
/// all the directories that have a custom icon.
const DIRECTORY_ICONS: Map<&'static str, char> = phf_map! {
    ".config"             => Icons::FOLDER_CONFIG,  // 
    ".git"                => Icons::FOLDER_GIT,     // 
    ".github"             => Icons::FOLDER_GITHUB,  // 
    ".gnupg"              => Icons::FOLDER_KEY,     // 󰢬
    ".local"              => Icons::FOLDER_CONFIG,  // 
    ".npm"                => Icons::FOLDER_NPM,     // 
    ".ssh"                => Icons::FOLDER_KEY,     // 󰢬
    ".Trash"              => '\u{f1f8}',            // 
    "bin"                 => Icons::FOLDER_CONFIG,  // 
    "config"              => Icons::FOLDER_CONFIG,  // 
    "Contacts"            => '\u{f024c}',           // 󰉌
    "cron.d"              => Icons::FOLDER_CONFIG,  // 
    "cron.daily"          => Icons::FOLDER_CONFIG,  // 
    "cron.hourly"         => Icons::FOLDER_CONFIG,  // 
    "cron.monthly"        => Icons::FOLDER_CONFIG,  // 
    "cron.weekly"         => Icons::FOLDER_CONFIG,  // 
    "Desktop"             => '\u{f108}',            // 
    "Downloads"           => '\u{f024d}',           // 󰉍
    "etc"                 => Icons::FOLDER_CONFIG,  // 
    "Favorites"           => '\u{f069d}',           // 󰚝
    "hidden"              => Icons::FOLDER_HIDDEN,  // 󱞞
    "home"                => '\u{f10b5}',           // 󱂵
    "include"             => Icons::FOLDER_CONFIG,  // 
    "Mail"                => '\u{f01f0}',           // 󰇰
    "Movies"              => '\u{f0fce}',           // 󰿎
    "Music"               => '\u{f1359}',           // 󱍙
    "node_modules"        => Icons::FOLDER_NPM,     // 
    "npm_cache"           => Icons::FOLDER_NPM,     // 
    "pam.d"               => Icons::FOLDER_KEY,     // 󰢬
    "Pictures"            => '\u{f024f}',           // 󰉏
    "root"                => '\u{f0250}',           // 󰉐
    "src"                 => '\u{f19fc}',           // 󱧼
    "ssh"                 => Icons::FOLDER_KEY,     // 󰢬
    "sudoers.d"           => Icons::FOLDER_KEY,     // 󰢬
    "Videos"              => '\u{f03d}',            // 
    "xbps.d"              => Icons::FOLDER_CONFIG,  // 
    "xorg.conf.d"         => Icons::FOLDER_CONFIG,  // 
};

/// Mapping from full filenames to file icon. This mapping should also contain
/// all the "dot" files that have a custom icon.
const FILENAME_ICONS: Map<&'static str, char> = phf_map! {
    ".atom"                  => '\u{e764}',            // 
    ".bashrc"                => Icons::SHELL,          // 󱆃
    ".bash_history"          => Icons::SHELL,          // 󱆃
    ".bash_logout"           => Icons::SHELL,          // 󱆃
    ".bash_profile"          => Icons::SHELL,          // 󱆃
    ".CFUserTextEncoding"    => Icons::OS_APPLE,       // 
    ".clang-format"          => Icons::CONFIG,         // 
    ".cshrc"                 => Icons::SHELL,          // 󱆃
    ".DS_Store"              => Icons::OS_APPLE,       // 
    ".emacs"                 => Icons::EMACS,          // 
    ".eslintrc.cjs"          => Icons::ESLINT,         // 
    ".eslintrc.js"           => Icons::ESLINT,         // 
    ".eslintrc.json"         => Icons::ESLINT,         // 
    ".eslintrc.yaml"         => Icons::ESLINT,         // 
    ".eslintrc.yml"          => Icons::ESLINT,         // 
    ".git-blame-ignore-revs" => Icons::GIT,            // 
    ".gitattributes"         => Icons::GIT,            // 
    ".gitconfig"             => Icons::GIT,            // 
    ".gitignore"             => Icons::GIT,            // 
    ".gitignore_global"      => Icons::GIT,            // 
    ".gitlab-ci.yml"         => '\u{f296}',            // 
    ".gitmodules"            => Icons::GIT,            // 
    ".htaccess"              => Icons::CONFIG,         // 
    ".htpasswd"              => Icons::CONFIG,         // 
    ".idea"                  => Icons::INTELLIJ,       // 
    ".ideavimrc"             => Icons::VIM,            // 
    ".inputrc"               => Icons::CONFIG,         // 
    ".kshrc"                 => Icons::SHELL,          // 󱆃
    ".login"                 => Icons::SHELL,          // 󱆃
    ".logout"                => Icons::SHELL,          // 󱆃
    ".nix-channels"          => '\u{f313}',            // 
    ".mailcap"               => '\u{f01f0}',           // 󰇰
    ".mailmap"               => Icons::GIT,            // 
    ".mime.types"            => '\u{f462}',            // 
    ".node_repl_history"     => Icons::NODEJS,         // 
    ".npmignore"             => Icons::NPM,            // 
    ".npmrc"                 => Icons::NPM,            // 
    ".profile"               => Icons::SHELL,          // 󱆃
    ".python_history"        => Icons::LANG_PYTHON,    // 
    ".rustfmt.toml"          => Icons::LANG_RUST,      // 
    ".rvm"                   => Icons::LANG_RUBY,      // 
    ".rvmrc"                 => Icons::LANG_RUBY,      // 
    ".tcshrc"                => Icons::SHELL,          // 󱆃
    ".travis.yml"            => '\u{e77e}',            // 
    ".viminfo"               => Icons::VIM,            // 
    ".vimrc"                 => Icons::VIM,            // 
    ".Xauthority"            => Icons::CONFIG,         // 
    ".xinitrc"               => Icons::CONFIG,         // 
    ".Xresources"            => Icons::CONFIG,         // 
    ".yarnrc"                => Icons::YARN,           // 
    ".zlogin"                => Icons::SHELL,          // 󱆃
    ".zlogout"               => Icons::SHELL,          // 󱆃
    ".zprofile"              => Icons::SHELL,          // 󱆃
    ".zshenv"                => Icons::SHELL,          // 󱆃
    ".zshrc"                 => Icons::SHELL,          // 󱆃
    ".zsh_history"           => Icons::SHELL,          // 󱆃
    ".zsh_sessions"          => Icons::SHELL,          // 󱆃
    "._DS_Store"             => Icons::OS_APPLE,       // 
    "a.out"                  => Icons::SHELL_CMD,      // 
    "authorized_keys"        => Icons::KEYS,           // 
    "bashrc"                 => Icons::SHELL,          // 󱆃
    "bspwmrc"                => Icons::CONFIG,         // 
    "build.ninja"            => '\u{f0774}',           // 󰝴
    "build.gradle.kts"       => Icons::GRADLE,         // 
    "Cargo.lock"             => Icons::LANG_RUST,      // 
    "Cargo.toml"             => Icons::LANG_RUST,      // 
    "CMakeLists.txt"         => Icons::MAKE,           // 
    "composer.json"          => Icons::LANG_PHP,       // 
    "composer.lock"          => Icons::LANG_PHP,       // 
    "config"                 => Icons::CONFIG,         // 
    "config.el"              => Icons::EMACS,          // 
    "config.in"              => Icons::CONFIG,         // 
    "config.mk"              => Icons::MAKE,           // 
    "config.status"          => Icons::CONFIG,         // 
    "configure"              => Icons::WRENCH,         // 
    "configure.ac"           => Icons::CONFIG,         // 
    "configure.in"           => Icons::CONFIG,         // 
    "constraints.txt"        => Icons::LANG_PYTHON,    // 
    "COPYING"                => Icons::LICENSE,        // 
    "COPYRIGHT"              => Icons::LICENSE,        // 
    "cron.deny"              => Icons::CONFIG,         // 
    "crontab"                => Icons::CONFIG,         // 
    "crypttab"               => Icons::CONFIG,         // 
    "csh.cshrc"              => Icons::SHELL,          // 󱆃
    "csh.login"              => Icons::SHELL,          // 󱆃
    "csh.logout"             => Icons::SHELL,          // 󱆃
    "docker-compose.yml"     => Icons::DOCKER,         // 
    "Dockerfile"             => Icons::DOCKER,         // 
    "Earthfile"              => '\u{f0ac}',            // 
    "environment"            => Icons::CONFIG,         // 
    "fstab"                  => Icons::CONFIG,         // 
    "GNUmakefile"            => Icons::MAKE,           // 
    "go.mod"                 => Icons::LANG_GO,        // 
    "go.sum"                 => Icons::LANG_GO,        // 
    "go.work"                => Icons::LANG_GO,        // 
    "gradle"                 => Icons::GRADLE,         // 
    "gradle.properties"      => Icons::GRADLE,         // 
    "gradlew"                => Icons::GRADLE,         // 
    "gradlew.bat"            => Icons::GRADLE,         // 
    "group"                  => Icons::LOCK,           // 
    "gruntfile.coffee"       => Icons::GRUNT,          // 
    "gruntfile.js"           => Icons::GRUNT,          // 
    "gruntfile.ls"           => Icons::GRUNT,          // 
    "gshadow"                => Icons::LOCK,           // 
    "gulpfile.coffee"        => Icons::GULP,           // 
    "gulpfile.js"            => Icons::GULP,           // 
    "gulpfile.ls"            => Icons::GULP,           // 
    "heroku.yml"             => '\u{e77b}',            // 
    "hostname"               => Icons::CONFIG,         // 
    "hosts"                  => Icons::CONFIG,         // 
    "id_dsa"                 => Icons::PRIVATE_KEY,    // 󰌆
    "id_ecdsa"               => Icons::PRIVATE_KEY,    // 󰌆
    "id_ecdsa_sk"            => Icons::PRIVATE_KEY,    // 󰌆
    "id_ed25519"             => Icons::PRIVATE_KEY,    // 󰌆
    "id_ed25519_sk"          => Icons::PRIVATE_KEY,    // 󰌆
    "id_rsa"                 => Icons::PRIVATE_KEY,    // 󰌆
    "init.el"                => Icons::EMACS,          // 
    "inputrc"                => Icons::CONFIG,         // 
    "Jenkinsfile"            => '\u{e66e}',            // 
    "jsconfig.json"          => Icons::LANG_JAVASCRIPT,// 
    "Justfile"               => Icons::WRENCH,         // 
    "Kbuild"                 => Icons::OS_LINUX,       // 
    "Kconfig"                => Icons::OS_LINUX,       // 
    "known_hosts"            => Icons::KEYS,           // 
    "LICENCE"                => Icons::LICENSE,        // 
    "LICENCE.md"             => Icons::LICENSE,        // 
    "LICENCE.txt"            => Icons::LICENSE,        // 
    "LICENSE"                => Icons::LICENSE,        // 
    "LICENSE-APACHE"         => Icons::LICENSE,        // 
    "LICENSE-MIT"            => Icons::LICENSE,        // 
    "LICENSE.md"             => Icons::LICENSE,        // 
    "LICENSE.txt"            => Icons::LICENSE,        // 
    "localized"              => Icons::OS_APPLE,       // 
    "localtime"              => Icons::CLOCK,          // 
    "Makefile"               => Icons::MAKE,           // 
    "makefile"               => Icons::MAKE,           // 
    "Makefile.ac"            => Icons::MAKE,           // 
    "Makefile.am"            => Icons::MAKE,           // 
    "Makefile.in"            => Icons::MAKE,           // 
    "MANIFEST"               => Icons::LANG_PYTHON,    // 
    "MANIFEST.in"            => Icons::LANG_PYTHON,    // 
    "meson.build"            => Icons::WRENCH,         // 
    "mime.types"             => '\u{f462}',            // 
    "npm-shrinkwrap.json"    => Icons::NPM,            // 
    "npmrc"                  => Icons::NPM,            // 
    "package-lock.json"      => Icons::NPM,            // 
    "package.json"           => Icons::NPM,            // 
    "passwd"                 => Icons::LOCK,           // 
    "PKGBUILD"               => Icons::OS_LINUX_ARCH,  // 
    "php.ini"                => Icons::LANG_PHP,       // 
    "pom.xml"                => '\u{e674}',            // 
    "Procfile"               => '\u{e77b}',            // 
    "profile"                => Icons::SHELL,          // 󱆃
    "pyproject.toml"         => Icons::LANG_PYTHON,    // 
    "Rakefile"               => Icons::LANG_RUBY,      // 
    "README"                 => Icons::README,         // 󰂺
    "release.toml"           => Icons::LANG_RUST,      // 
    "requirements.txt"       => Icons::LANG_PYTHON,    // 
    "robots.txt"             => '\u{f06a9}',           // 󰚩
    "rubydoc"                => Icons::LANG_RUBYRAILS, // 
    "rustfmt.toml"           => Icons::LANG_RUST,      // 
    "rvmrc"                  => Icons::LANG_RUBY,      // 
    "settings.gradle.kts"    => Icons::GRADLE,         // 
    "shadow"                 => Icons::LOCK,           // 
    "shells"                 => Icons::CONFIG,         // 
    "subgid"                 => Icons::CONFIG,         // 
    "subuid"                 => Icons::CONFIG,         // 
    "sudoers"                => Icons::LOCK,           // 
    "timezone"               => Icons::CLOCK,          // 
    "tsconfig.json"          => Icons::LANG_TYPESCRIPT,// 
    "Vagrantfile"            => '\u{2371}',            // ⍱
    "webpack.config.js"      => '\u{f072b}',           // 󰜫
    "xbps-src"               => Icons::OS_LINUX_VOID,  // 
    "yarn.lock"              => Icons::YARN,           // 
    "zlogin"                 => Icons::SHELL,          // 󱆃
    "zlogout"                => Icons::SHELL,          // 󱆃
    "zprofile"               => Icons::SHELL,          // 󱆃
    "zshenv"                 => Icons::SHELL,          // 󱆃
    "zshrc"                  => Icons::SHELL,          // 󱆃
};

/// Mapping from lowercase file extension to icons.  If an image, video, or audio extension is add
/// also update the extension filetype map.
const EXTENSION_ICONS: Map<&'static str, char> = phf_map! {
    "7z"             => Icons::COMPRESSED,       // 
    "a"              => Icons::OS_LINUX,         // 
    "acc"            => Icons::AUDIO,            // 
    "acf"            => '\u{f1b6}',              // 
    "ai"             => '\u{e7b4}',              // 
    "aif"            => Icons::AUDIO,            // 
    "aifc"           => Icons::AUDIO,            // 
    "aiff"           => Icons::AUDIO,            // 
    "alac"           => Icons::AUDIO,            // 
    "android"        => Icons::OS_ANDROID,       // 
    "ape"            => Icons::AUDIO,            // 
    "apk"            => Icons::OS_ANDROID,       // 
    "apple"          => Icons::OS_APPLE,         // 
    "ar"             => Icons::COMPRESSED,       // 
    "arj"            => Icons::COMPRESSED,       // 
    "arw"            => Icons::IMAGE,            // 
    "asc"            => Icons::SHIELD_LOCK,      // 󰦝
    "asm"            => Icons::LANG_ASSEMBLY,    // 
    "asp"            => '\u{f121}',              // 
    "avi"            => Icons::VIDEO,            // 
    "avif"           => Icons::IMAGE,            // 
    "avro"           => Icons::JSON,             // 
    "awk"            => Icons::SHELL_CMD,        // 
    "bash"           => Icons::SHELL_CMD,        // 
    "bat"            => Icons::OS_WINDOWS_CMD,   // 
    "bats"           => Icons::SHELL_CMD,        // 
    "bdf"            => Icons::FONT,             // 
    "bib"            => Icons::LANG_TEX,         // 
    "bin"            => Icons::BINARY,           // 
    "bmp"            => Icons::IMAGE,            // 
    "br"             => Icons::COMPRESSED,       // 
    "bst"            => Icons::LANG_TEX,         // 
    "bundle"         => Icons::OS_APPLE,         // 
    "bz"             => Icons::COMPRESSED,       // 
    "bz2"            => Icons::COMPRESSED,       // 
    "bz3"            => Icons::COMPRESSED,       // 
    "c"              => Icons::LANG_C,           // 
    "c++"            => Icons::LANG_CPP,         // 
    "cab"            => Icons::OS_WINDOWS,       // 
    "cbr"            => Icons::IMAGE,            // 
    "cbz"            => Icons::IMAGE,            // 
    "cc"             => Icons::LANG_CPP,         // 
    "cert"           => Icons::GIST_SECRET,      // 
    "cfg"            => Icons::CONFIG,           // 
    "cjs"            => Icons::LANG_JAVASCRIPT,  // 
    "class"          => Icons::LANG_JAVA,        // 
    "clj"            => '\u{e768}',              // 
    "cljs"           => '\u{e76a}',              // 
    "cls"            => Icons::LANG_TEX,         // 
    "cmake"          => Icons::MAKE,             // 
    "cmd"            => Icons::OS_WINDOWS,       // 
    "coffee"         => '\u{f0f4}',              // 
    "com"            => Icons::OS_WINDOWS_CMD,   // 
    "conf"           => Icons::CONFIG,           // 
    "config"         => Icons::CONFIG,           // 
    "cp"             => Icons::LANG_CPP,         // 
    "cpio"           => Icons::COMPRESSED,       // 
    "cpp"            => Icons::LANG_CPP,         // 
    "cr"             => '\u{e62f}',              // 
    "cr2"            => Icons::IMAGE,            // 
    "crdownload"     => Icons::DOWNLOAD,         // 󰇚
    "crt"            => Icons::GIST_SECRET,      // 
    "cs"             => Icons::LANG_CSHARP,      // 󰌛
    "csh"            => Icons::SHELL_CMD,        // 
    "cshtml"         => Icons::RAZOR,            // 
    "csproj"         => Icons::LANG_CSHARP,      // 󰌛
    "css"            => Icons::CSS3,             // 
    "csv"            => Icons::SHEET,            // 
    "csx"            => Icons::LANG_CSHARP,      // 󰌛
    "cts"            => Icons::LANG_TYPESCRIPT,  // 
    "cu"             => '\u{e64b}',              // 
    "cue"            => Icons::PLAYLIST,         // 󰲹
    "cxx"            => Icons::LANG_CPP,         // 
    "d"              => Icons::LANG_D,           // 
    "dart"           => '\u{e798}',              // 
    "db"             => Icons::DATABASE,         // 
    "deb"            => Icons::OS_LINUX_DEBIAN,  // 
    "desktop"        => '\u{ebd1}',              // 
    "di"             => Icons::LANG_D,           // 
    "diff"           => Icons::DIFF,             // 
    "djv"            => Icons::DOCUMENT,         // 
    "djvu"           => Icons::DOCUMENT,         // 
    "dll"            => Icons::LIBRARY,          // 
    "dmg"            => Icons::DISK_IMAGE,       // 
    "doc"            => Icons::DOCUMENT,         // 
    "docx"           => Icons::DOCUMENT,         // 
    "dot"            => '\u{f1049}',             // 󱁉
    "download"       => Icons::DOWNLOAD,         // 󰇚
    "drawio"         => '\u{ebba}',              // 
    "dump"           => Icons::DATABASE,         // 
    "dvi"            => Icons::IMAGE,            // 
    "dylib"          => Icons::OS_APPLE,         // 
    "ebook"          => Icons::BOOK,             // 
    "ebuild"         => Icons::OS_LINUX_GENTOO,  // 
    "eclass"         => Icons::OS_LINUX_GENTOO,  // 
    "editorconfig"   => Icons::CONFIG,           // 
    "ejs"            => '\u{e618}',              // 
    "el"             => Icons::EMACS,            // 
    "elc"            => Icons::EMACS,            // 
    "elf"            => Icons::OS_LINUX,         // 
    "elm"            => '\u{e62c}',              // 
    "eml"            => '\u{f003}',              // 
    "env"            => '\u{f462}',              // 
    "eot"            => Icons::FONT,             // 
    "eps"            => Icons::VECTOR,           // 󰕙
    "epub"           => Icons::BOOK,             // 
    "erb"            => Icons::LANG_RUBYRAILS,   // 
    "erl"            => Icons::LANG_ERLANG,      // 
    "ex"             => Icons::LANG_ELIXIR,      // 
    "exe"            => Icons::OS_WINDOWS_CMD,   // 
    "exs"            => Icons::LANG_ELIXIR,      // 
    "fdmdownload"    => Icons::DOWNLOAD,         // 󰇚
    "fish"           => Icons::SHELL_CMD,        // 
    "flac"           => Icons::AUDIO,            // 
    "flv"            => Icons::VIDEO,            // 
    "fnt"            => Icons::FONT,             // 
    "fon"            => Icons::FONT,             // 
    "font"           => Icons::FONT,             // 
    "fs"             => Icons::LANG_FSHARP,      // 
    "fsi"            => Icons::LANG_FSHARP,      // 
    "fsx"            => Icons::LANG_FSHARP,      // 
    "gdoc"           => Icons::DOCUMENT,         // 
    "gem"            => Icons::LANG_RUBY,        // 
    "gemfile"        => Icons::LANG_RUBY,        // 
    "gemspec"        => Icons::LANG_RUBY,        // 
    "gform"          => '\u{f298}',              // 
    "gif"            => Icons::IMAGE,            // 
    "git"            => Icons::GIT,              // 
    "go"             => Icons::LANG_GO,          // 
    "gpg"            => Icons::SHIELD_LOCK,      // 󰦝
    "gql"            => Icons::GRAPHQL,          // 
    "gradle"         => Icons::GRADLE,           // 
    "graphql"        => Icons::GRAPHQL,          // 
    "groovy"         => Icons::LANG_GROOVY,      // 
    "gsheet"         => Icons::SHEET,            // 
    "gslides"        => Icons::SLIDE,            // 
    "guardfile"      => Icons::LANG_RUBY,        // 
    "gv"             => '\u{f1049}',             // 󱁉
    "gvy"            => Icons::LANG_GROOVY,      // 
    "gz"             => Icons::COMPRESSED,       // 
    "h"              => Icons::HEADER,           // 
    "h++"            => Icons::HEADER,           // 
    "h264"           => Icons::VIDEO,            // 
    "haml"           => '\u{e664}',              // 
    "hbs"            => Icons::MUSTACHE,         // 
    "heic"           => Icons::IMAGE,            // 
    "heics"          => Icons::VIDEO,            // 
    "heif"           => Icons::IMAGE,            // 
    "hrl"            => Icons::LANG_ERLANG,      // 
    "hpp"            => Icons::HEADER,           // 
    "hs"             => Icons::LANG_HASKELL,     // 
    "htm"            => Icons::HTML5,            // 
    "html"           => Icons::HTML5,            // 
    "hxx"            => Icons::HEADER,           // 
    "ical"           => Icons::CALENDAR,         // 
    "icalendar"      => Icons::CALENDAR,         // 
    "ico"            => Icons::IMAGE,            // 
    "ics"            => Icons::CALENDAR,         // 
    "ifb"            => Icons::CALENDAR,         // 
    "image"          => Icons::DISK_IMAGE,       // 
    "img"            => Icons::DISK_IMAGE,       // 
    "iml"            => Icons::INTELLIJ,         // 
    "inl"            => Icons::LANG_C,           // 
    "ini"            => Icons::CONFIG,           // 
    "ipynb"          => '\u{e678}',              // 
    "iso"            => Icons::DISK_IMAGE,       // 
    "j2c"            => Icons::IMAGE,            // 
    "j2k"            => Icons::IMAGE,            // 
    "jad"            => Icons::LANG_JAVA,        // 
    "jar"            => Icons::LANG_JAVA,        // 
    "java"           => Icons::LANG_JAVA,        // 
    "jfi"            => Icons::IMAGE,            // 
    "jfif"           => Icons::IMAGE,            // 
    "jif"            => Icons::IMAGE,            // 
    "jl"             => '\u{e624}',              // 
    "jmd"            => Icons::MARKDOWN,         // 
    "jp2"            => Icons::IMAGE,            // 
    "jpe"            => Icons::IMAGE,            // 
    "jpeg"           => Icons::IMAGE,            // 
    "jpf"            => Icons::IMAGE,            // 
    "jpg"            => Icons::IMAGE,            // 
    "jpx"            => Icons::IMAGE,            // 
    "js"             => Icons::LANG_JAVASCRIPT,  // 
    "json"           => Icons::JSON,             // 
    "jsx"            => Icons::REACT,            // 
    "jxl"            => Icons::IMAGE,            // 
    "kbx"            => Icons::SHIELD_KEY,       // 󰯄
    "kdb"            => Icons::KEYPASS,          // 
    "kdbx"           => Icons::KEYPASS,          // 
    "key"            => Icons::KEY,              // 
    "ko"             => Icons::OS_LINUX,         // 
    "ksh"            => Icons::SHELL_CMD,        // 
    "kt"             => Icons::LANG_KOTLIN,      // 
    "kts"            => Icons::LANG_KOTLIN,      // 
    "latex"          => Icons::LANG_TEX,         // 
    "ld"             => Icons::WRENCH,           // 
    "ldb"            => Icons::DATABASE,         // 
    "less"           => '\u{e758}',              // 
    "lhs"            => Icons::LANG_HASKELL,     // 
    "lib"            => Icons::LIBRARY,          // 
    "license"        => Icons::LICENSE,          // 
    "lisp"           => '\u{f0172}',             // 󰅲
    "localized"      => Icons::OS_APPLE,         // 
    "lock"           => Icons::LOCK,             // 
    "log"            => '\u{f18d}',              // 
    "ltx"            => Icons::LANG_TEX,         // 
    "lua"            => '\u{e620}',              // 
    "lz"             => Icons::COMPRESSED,       // 
    "lz4"            => Icons::COMPRESSED,       // 
    "lzh"            => Icons::COMPRESSED,       // 
    "lzma"           => Icons::COMPRESSED,       // 
    "lzo"            => Icons::COMPRESSED,       // 
    "m"              => Icons::LANG_C,           // 
    "m2ts"           => Icons::VIDEO,            // 
    "m2v"            => Icons::VIDEO,            // 
    "m3u"            => Icons::PLAYLIST,         // 󰲹
    "m3u8"           => Icons::PLAYLIST,         // 󰲹
    "m4a"            => Icons::AUDIO,            // 
    "m4v"            => Icons::VIDEO,            // 
    "magnet"         => '\u{f076}',              // 
    "markdown"       => Icons::MARKDOWN,         // 
    "md"             => Icons::MARKDOWN,         // 
    "md5"            => Icons::SHIELD_CHECK,     // 󰕥
    "mdb"            => Icons::DATABASE,         // 
    "mid"            => '\u{f08f2}',             // 󰣲
    "mjs"            => Icons::LANG_JAVASCRIPT,  // 
    "mk"             => Icons::MAKE,             // 
    "mka"            => Icons::AUDIO,            // 
    "mkd"            => Icons::MARKDOWN,         // 
    "mkv"            => Icons::VIDEO,            // 
    "ml"             => Icons::LANG_OCAML,       // 
    "mli"            => Icons::LANG_OCAML,       // 
    "mll"            => Icons::LANG_OCAML,       // 
    "mly"            => Icons::LANG_OCAML,       // 
    "mm"             => Icons::LANG_CPP,         // 
    "mobi"           => Icons::BOOK,             // 
    "mov"            => Icons::VIDEO,            // 
    "mp2"            => Icons::AUDIO,            // 
    "mp3"            => Icons::AUDIO,            // 
    "mp4"            => Icons::VIDEO,            // 
    "mpeg"           => Icons::VIDEO,            // 
    "mpg"            => Icons::VIDEO,            // 
    "msi"            => Icons::OS_WINDOWS,       // 
    "mts"            => Icons::LANG_TYPESCRIPT,  // 
    "mustache"       => Icons::MUSTACHE,         // 
    "nef"            => Icons::IMAGE,            // 
    "ninja"          => '\u{f0774}',             // 󰝴
    "nix"            => '\u{f313}',              // 
    "node"           => Icons::NODEJS,           // 
    "o"              => Icons::BINARY,           // 
    "odp"            => Icons::SLIDE,            // 
    "ods"            => Icons::SHEET,            // 
    "odt"            => Icons::DOCUMENT,         // 
    "ogg"            => Icons::AUDIO,            // 
    "ogm"            => Icons::VIDEO,            // 
    "ogv"            => Icons::VIDEO,            // 
    "old"            => '\u{f006f}',             // 󰁯
    "opus"           => Icons::AUDIO,            // 
    "orf"            => Icons::IMAGE,            // 
    "org"            => '\u{e633}',              // 
    "otf"            => Icons::FONT,             // 
    "out"            => '\u{eb2c}',              // 
    "p12"            => Icons::KEY,              // 
    "par"            => Icons::COMPRESSED,       // 
    "part"           => Icons::DOWNLOAD,         // 󰇚
    "patch"          => Icons::DIFF,             // 
    "pbm"            => Icons::IMAGE,            // 
    "pcm"            => Icons::AUDIO,            // 
    "pdf"            => '\u{f1c1}',              // 
    "pem"            => Icons::KEY,              // 
    "pfx"            => Icons::KEY,              // 
    "pgm"            => Icons::IMAGE,            // 
    "phar"           => Icons::LANG_PHP,         // 
    "php"            => Icons::LANG_PHP,         // 
    "pkg"            => '\u{eb29}',              // 
    "pl"             => Icons::LANG_PERL,        // 
    "plist"          => Icons::OS_APPLE,         // 
    "plx"            => Icons::LANG_PERL,        // 
    "pm"             => Icons::LANG_PERL,        // 
    "png"            => Icons::IMAGE,            // 
    "pnm"            => Icons::IMAGE,            // 
    "pod"            => Icons::LANG_PERL,        // 
    "pp"             => '\u{e631}',              // 
    "ppm"            => Icons::IMAGE,            // 
    "pps"            => Icons::SLIDE,            // 
    "ppsx"           => Icons::SLIDE,            // 
    "ppt"            => Icons::SLIDE,            // 
    "pptx"           => Icons::SLIDE,            // 
    "properties"     => Icons::JSON,             // 
    "ps"             => Icons::VECTOR,           // 󰕙
    "ps1"            => Icons::POWERSHELL,       // 
    "psd"            => '\u{e7b8}',              // 
    "psd1"           => Icons::POWERSHELL,       // 
    "psf"            => Icons::FONT,             // 
    "psm1"           => Icons::POWERSHELL,       // 
    "pub"            => Icons::PUBLIC_KEY,       // 󰷖
    "purs"           => '\u{e630}',              // 
    "pxm"            => Icons::IMAGE,            // 
    "py"             => Icons::LANG_PYTHON,      // 
    "pyc"            => Icons::LANG_PYTHON,      // 
    "pyd"            => Icons::LANG_PYTHON,      // 
    "pyi"            => Icons::LANG_PYTHON,      // 
    "pyo"            => Icons::LANG_PYTHON,      // 
    "qcow"           => Icons::DISK_IMAGE,       // 
    "qcow2"          => Icons::DISK_IMAGE,       // 
    "r"              => Icons::LANG_R,           // 
    "rar"            => Icons::COMPRESSED,       // 
    "raw"            => Icons::IMAGE,            // 
    "razor"          => Icons::RAZOR,            // 
    "rb"             => Icons::LANG_RUBY,        // 
    "rdata"          => Icons::LANG_R,           // 
    "rdb"            => '\u{e76d}',              // 
    "rdoc"           => Icons::MARKDOWN,         // 
    "rds"            => Icons::LANG_R,           // 
    "readme"         => Icons::README,           // 󰂺
    "rlib"           => Icons::LANG_RUST,        // 
    "rmd"            => Icons::MARKDOWN,         // 
    "rmeta"          => Icons::LANG_RUST,        // 
    "rpm"            => Icons::OS_LINUX_REDHAT,  // 
    "rs"             => Icons::LANG_RUST,        // 
    "rspec"          => Icons::LANG_RUBY,        // 
    "rspec_parallel" => Icons::LANG_RUBY,        // 
    "rspec_status"   => Icons::LANG_RUBY,        // 
    "rss"            => '\u{f09e}',              // 
    "rst"            => Icons::TEXT,             // 
    "rtf"            => Icons::TEXT,             // 
    "ru"             => Icons::LANG_RUBY,        // 
    "rubydoc"        => Icons::LANG_RUBYRAILS,   // 
    "s"              => Icons::LANG_ASSEMBLY,    // 
    "sass"           => Icons::LANG_SASS,        // 
    "sbt"            => Icons::SUBTITLE,         // 󰨖
    "scala"          => '\u{e737}',              // 
    "scpt"           => Icons::OS_APPLE,         // 
    "scss"           => Icons::LANG_SASS,        // 
    "service"        => '\u{eba2}',              // 
    "sh"             => Icons::SHELL_CMD,        // 
    "sha1"           => Icons::SHIELD_CHECK,     // 󰕥
    "sha224"         => Icons::SHIELD_CHECK,     // 󰕥
    "sha256"         => Icons::SHIELD_CHECK,     // 󰕥
    "sha384"         => Icons::SHIELD_CHECK,     // 󰕥
    "sha512"         => Icons::SHIELD_CHECK,     // 󰕥
    "shell"          => Icons::SHELL_CMD,        // 
    "shtml"          => Icons::HTML5,            // 
    "sig"            => Icons::SIGNED_FILE,      // 󱧃
    "signature"      => Icons::SIGNED_FILE,      // 󱧃
    "slim"           => Icons::LANG_RUBYRAILS,   // 
    "sln"            => '\u{e70c}',              // 
    "so"             => Icons::OS_LINUX,         // 
    "sql"            => Icons::DATABASE,         // 
    "sqlite3"        => '\u{e7c4}',              // 
    "srt"            => Icons::SUBTITLE,         // 󰨖
    "ssa"            => Icons::SUBTITLE,         // 󰨖
    "stl"            => Icons::IMAGE,            // 
    "sty"            => Icons::LANG_TEX,         // 
    "styl"           => Icons::LANG_STYLUS,      // 
    "stylus"         => Icons::LANG_STYLUS,      // 
    "sub"            => Icons::SUBTITLE,         // 󰨖
    "sublime-build"  => Icons::SUBLIME,          // 
    "sublime-keymap" => Icons::SUBLIME,          // 
    "sublime-menu"   => Icons::SUBLIME,          // 
    "sublime-options"=> Icons::SUBLIME,          // 
    "sublime-package"=> Icons::SUBLIME,          // 
    "sublime-project"=> Icons::SUBLIME,          // 
    "sublime-session"=> Icons::SUBLIME,          // 
    "sublime-settings"=>Icons::SUBLIME,          // 
    "sublime-snippet"=> Icons::SUBLIME,          // 
    "sublime-theme"  => Icons::SUBLIME,          // 
    "svelte"         => '\u{e697}',              // 
    "svg"            => Icons::VECTOR,           // 󰕙
    "swift"          => '\u{e755}',              // 
    "sym"            => '\u{eae8}',              // 
    "t"              => Icons::LANG_PERL,        // 
    "tar"            => Icons::COMPRESSED,       // 
    "taz"            => Icons::COMPRESSED,       // 
    "tbz"            => Icons::COMPRESSED,       // 
    "tbz2"           => Icons::COMPRESSED,       // 
    "tc"             => Icons::DISK_IMAGE,       // 
    "tex"            => Icons::LANG_TEX,         // 
    "tf"             => Icons::TERRAFORM,        // 󱁢
    "tfstate"        => Icons::TERRAFORM,        // 󱁢
    "tfvars"         => Icons::TERRAFORM,        // 󱁢
    "tgz"            => Icons::COMPRESSED,       // 
    "tif"            => Icons::IMAGE,            // 
    "tiff"           => Icons::IMAGE,            // 
    "tlz"            => Icons::COMPRESSED,       // 
    "tml"            => Icons::CONFIG,           // 
    "toml"           => Icons::CONFIG,           // 
    "torrent"        => '\u{e275}',              // 
    "ts"             => Icons::LANG_TYPESCRIPT,  // 
    "tsv"            => Icons::SHEET,            // 
    "tsx"            => Icons::REACT,            // 
    "ttc"            => Icons::FONT,             // 
    "ttf"            => Icons::FONT,             // 
    "twig"           => '\u{e61c}',              // 
    "txt"            => Icons::TEXT,             // 
    "typ"            => Icons::TYPST,            // 𝐭
    "txz"            => Icons::COMPRESSED,       // 
    "tz"             => Icons::COMPRESSED,       // 
    "tzo"            => Icons::COMPRESSED,       // 
    "unity"          => Icons::UNITY,            // 
    "unity3d"        => Icons::UNITY,            // 
    "url"            => '\u{f0ac}',              // 
    "v"              => '\u{e6ac}',              // 
    "vdi"            => Icons::DISK_IMAGE,       // 
    "vhd"            => Icons::DISK_IMAGE,       // 
    "video"          => Icons::VIDEO,            // 
    "vim"            => Icons::VIM,              // 
    "vmdk"           => Icons::DISK_IMAGE,       // 
    "vob"            => Icons::VIDEO,            // 
    "vue"            => '\u{f0844}',             // 󰡄
    "war"            => Icons::LANG_JAVA,        // 
    "wav"            => Icons::AUDIO,            // 
    "webm"           => Icons::VIDEO,            // 
    "webmanifest"    => Icons::JSON,             // 
    "webp"           => Icons::IMAGE,            // 
    "whl"            => Icons::LANG_PYTHON,      // 
    "windows"        => Icons::OS_WINDOWS,       // 
    "wma"            => Icons::AUDIO,            // 
    "wmv"            => Icons::VIDEO,            // 
    "webloc"         => '\u{f0ac}',              // 
    "woff"           => Icons::FONT,             // 
    "woff2"          => Icons::FONT,             // 
    "wv"             => Icons::AUDIO,            // 
    "xbps"           => '\u{f32e}',              // 
    "xcf"            => Icons::IMAGE,            // 
    "xhtml"          => Icons::HTML5,            // 
    "xlr"            => Icons::SHEET,            // 
    "xls"            => Icons::SHEET,            // 
    "xlsm"           => Icons::SHEET,            // 
    "xlsx"           => Icons::SHEET,            // 
    "xml"            => Icons::XML,              // 󰗀
    "xpm"            => Icons::IMAGE,            // 
    "xul"            => Icons::XML,              // 󰗀
    "xz"             => Icons::COMPRESSED,       // 
    "yaml"           => Icons::CONFIG,           // 
    "yml"            => Icons::CONFIG,           // 
    "z"              => Icons::COMPRESSED,       // 
    "zig"            => Icons::LANG_ZIG,         // 
    "zip"            => Icons::COMPRESSED,       // 
    "zsh"            => Icons::SHELL_CMD,        // 
    "zsh-theme"      => Icons::SHELL,            // 󱆃
    "zst"            => Icons::COMPRESSED,       // 
};

/// Converts the style used to paint a file name into the style that should be
/// used to paint an icon.
///
/// - The background colour should be preferred to the foreground colour, as
///   if one is set, it’s the more “obvious” colour choice.
/// - If neither is set, just use the default style.
/// - Attributes such as bold or underline should not be used to paint the
///   icon, as they can make it look weird.
pub fn iconify_style(style: Style) -> Style {
    style
        .background
        .or(style.foreground)
        .map(Style::from)
        .unwrap_or_default()
}

/// Lookup the icon for a file based on the file's name, if the entry is a
/// directory, or by the lowercase file extension.
#[cfg(unix)]
pub fn icon_for_file(file: &File<'_>) -> char {
    if file.points_to_directory() {
        *DIRECTORY_ICONS.get(file.name.as_str()).unwrap_or_else(|| {
            if file.is_empty_dir() {
                &Icons::FOLDER_OPEN // 
            } else if file.is_link() {
                &Icons::SYMLINK_DIR // 
            } else {
                &Icons::FOLDER // 
            }
        })
    } else if file.is_link() {
        Icons::SYMLINK // 
    } else if let Some(icon) = FILENAME_ICONS.get(file.name.as_str()) {
        *icon
    } else if let Some(ext) = file.ext.as_ref() {
        *EXTENSION_ICONS.get(ext.as_str()).unwrap_or(&Icons::FILE) // 
    } else if file.is_executable_file() {
        Icons::SHELL_CMD // 
    } else {
        Icons::FILE_OUTLINE // 
    }
}

/// Lookup the icon for a file based on the file's name, if the entry is a
/// directory, or by the lowercase file extension
#[cfg(windows)]
pub fn icon_for_file(file: &File<'_>) -> char {
    if file.points_to_directory() {
        *DIRECTORY_ICONS.get(file.name.as_str()).unwrap_or_else(|| {
            if file.is_empty_dir() {
                &Icons::FOLDER_OPEN // 
            } else if file.is_link() {
                &Icons::SYMLINK_DIR // 
            } else {
                &Icons::FOLDER // 
            }
        })
    } else if file.is_link() {
        Icons::SYMLINK // 
    } else if let Some(icon) = FILENAME_ICONS.get(file.name.as_str()) {
        *icon
    } else if let Some(ext) = file.ext.as_ref() {
        *EXTENSION_ICONS.get(ext.as_str()).unwrap_or(&Icons::FILE) // 
    } else {
        Icons::FILE_OUTLINE // 
    }
}