{ lib
, rustPlatform
, pkg-config
, sqlite
}:

rustPlatform.buildRustPackage {
  pname = "eternity";
  version = "0.1.0";

  src = lib.cleanSourceWith {
    src = ../.;
    filter = path: type:
      let
        baseName = baseNameOf path;
      in
        !(baseName == "target" || baseName == ".git" || baseName == "data");
  };

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    sqlite
  ];

  meta = {
    description = "Where will you spend eternity?";
    homepage = "https://github.com/playfairs/eternity";
    license = lib.licenses.gpl3Only;
    mainProgram = "eternity";
  };
}
