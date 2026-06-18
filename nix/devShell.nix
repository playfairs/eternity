{ mkShell
, cargo
, clippy
, just
, pkg-config
, rustc
, rustfmt
, sqlite
}:

mkShell {
  packages = [
    cargo
    clippy
    just
    pkg-config
    rustc
    rustfmt
    sqlite
  ];
}
